//! Integration tests with in-process TCP mock HTTP server.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::Value;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Recorded {
    method: String,
    path: String,
    method_call: String,
    body: String,
}

#[allow(dead_code)]
struct Mock {
    url: String,
    log: Arc<Mutex<Vec<Recorded>>>,
}

impl Mock {
    fn start(responses: HashMap<&'static str, (u16, String)>) -> Mock {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/RPCSERV", listener.local_addr().unwrap());
        let log = Arc::new(Mutex::new(Vec::new()));
        let log2 = log.clone();

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { continue };
                let responses = responses.clone();
                let log = log2.clone();

                std::thread::spawn(move || {
                    let mut reader = BufReader::new(stream.try_clone().unwrap());
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap_or(0) == 0 {
                        return;
                    }
                    let mut parts = line.split_whitespace();
                    let method = parts.next().unwrap_or("").to_string();
                    let path = parts.next().unwrap_or("").to_string();

                    let mut len = 0usize;
                    loop {
                        let mut h = String::new();
                        reader.read_line(&mut h).unwrap();
                        let h = h.trim_end();
                        if h.is_empty() {
                            break;
                        }
                        if let Some((k, v)) = h.split_once(':')
                            && k.trim().eq_ignore_ascii_case("content-length")
                        {
                            len = v.trim().parse().unwrap_or(0);
                        }
                    }

                    let mut buf = vec![0; len];
                    reader.read_exact(&mut buf).unwrap();
                    let body = String::from_utf8_lossy(&buf).to_string();

                    let method_call = if let Some(start) = body.find("<methodName>") {
                        let after = &body[start + 12..];
                        if let Some(end) = after.find("</methodName>") {
                            after[..end].trim().to_string()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    log.lock().unwrap().push(Recorded {
                        method: method.clone(),
                        path: path.clone(),
                        method_call: method_call.clone(),
                        body: body.clone(),
                    });

                    let (status, resp_xml) = if let Some(resp) = responses.get(method_call.as_str()) {
                        resp.clone()
                    } else if method_call == "fault_auth" {
                        (
                            200,
                            r#"<?xml version="1.0"?><methodResponse><fault><value><struct><member><name>faultCode</name><value><int>401</int></value></member><member><name>faultString</name><value><string>Bad credentials</string></value></member></struct></value></fault></methodResponse>"#.to_string(),
                        )
                    } else if method_call == "http_500" {
                        (500, "Internal Server Error".to_string())
                    } else {
                        (
                            200,
                            r#"<?xml version="1.0"?><methodResponse><params><param><value><string>OK</string></value></param></params></methodResponse>"#.to_string(),
                        )
                    };

                    let _ = write!(
                        stream,
                        "HTTP/1.1 {status} OK\r\nContent-Type: text/xml; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{resp_xml}",
                        resp_xml.len()
                    );
                });
            }
        });

        Mock { url, log }
    }

    #[allow(dead_code)]
    fn calls(&self) -> Vec<String> {
        self.log.lock().unwrap().iter().map(|r| r.method_call.clone()).collect()
    }
}

struct Env {
    dir: PathBuf,
    api: String,
}

impl Env {
    fn new(mock: &Mock) -> Env {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "loopia-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        Env { dir, api: mock.url.clone() }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_loopia"))
            .args(args)
            .env("LOOPIA_CONFIG_DIR", &self.dir)
            .env("LOOPIA_SECRET_STORE", "plaintext")
            .env("LOOPIA_ALLOW_PLAINTEXT_STORE", "1")
            .env("LOOPIA_API_ENDPOINT", &self.api)
            .output()
            .unwrap()
    }

    fn json(&self, args: &[&str]) -> (i32, Value, Value) {
        let mut all = args.to_vec();
        all.push("--json");
        let out = self.run(&all);
        let parse = |b: &[u8]| {
            let s = String::from_utf8_lossy(b);
            let filtered = s.lines().filter(|l| !l.starts_with("warning:")).collect::<Vec<_>>().join("\n");
            serde_json::from_str(&filtered).unwrap_or(Value::Null)
        };
        (out.status.code().unwrap(), parse(&out.stdout), parse(&out.stderr))
    }

    fn with_account(self) -> Env {
        let (code, out, err) =
            self.json(&["accounts", "add", "work", "--username", "u@loopiaapi", "--password", "p123"]);
        assert_eq!(code, 0, "{err}");
        assert_eq!(out["status"], "ok");
        self
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn standard_mock() -> Mock {
    let mut map: HashMap<&'static str, (u16, String)> = HashMap::new();
    map.insert(
        "getDomains",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><array><data><value><struct><member><name>domain</name><value><string>example.se</string></value></member></struct></value></data></array></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getDomain",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><struct><member><name>domain</name><value><string>example.se</string></value></member><member><name>paid</name><value><boolean>1</boolean></value></member></struct></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "domainIsFree",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><string>OK</string></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getSubdomains",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><array><data><value><string>www</string></value><value><string>mail</string></value></data></array></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getZoneRecords",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><array><data><value><struct><member><name>record_id</name><value><int>101</int></value></member><member><name>type</name><value><string>A</string></value></member><member><name>rdata</name><value><string>192.0.2.1</string></value></member><member><name>ttl</name><value><int>3600</int></value></member><member><name>priority</name><value><int>0</int></value></member></struct></value></data></array></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getCreditsAmount",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><double>1500.5</double></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getUnpaidInvoices",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><array><data><value><struct><member><name>reference_no</name><value><string>INV-101</string></value></member><member><name>amount</name><value><double>250.0</double></value></member></struct></value></data></array></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getInvoice",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><struct><member><name>reference_no</name><value><string>INV-101</string></value></member></struct></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getCustomers",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><array><data><value><struct><member><name>customer_id</name><value><string>CUST-100</string></value></member></struct></value></data></array></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "createNewAccount",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><struct><member><name>status</name><value><string>OK</string></value></member><member><name>order_reference</name><value><string>ORD-12345</string></value></member></struct></value></param></params></methodResponse>"#.to_string(),
        ),
    );
    map.insert(
        "getOrderStatus",
        (
            200,
            r#"<?xml version="1.0"?><methodResponse><params><param><value><struct><member><name>status</name><value><string>COMPLETED</string></value></member></struct></value></param></params></methodResponse>"#.to_string(),
        ),
    );

    Mock::start(map)
}

#[test]
fn accounts_lifecycle() {
    let mock = standard_mock();
    let env = Env::new(&mock).with_account();

    let (code, out, _) = env.json(&["accounts", "list"]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["account"], "work");
    assert_eq!(out[0]["username"], "u@loopiaapi");
    assert_eq!(out[0]["isDefault"], true);

    // Duplicate add without force fails with code 6 (InvalidInput)
    let (code, _, err) = env.json(&["accounts", "add", "work", "--username", "other", "--password", "p"]);
    assert_eq!(code, 6);
    assert!(err["error"].as_str().unwrap().contains("already exists"));

    // Overwrite with force succeeds
    let (code, out, _) = env.json(&["accounts", "add", "work", "--username", "other", "--password", "p", "--force"]);
    assert_eq!(code, 0);
    assert_eq!(out["username"], "other");

    // Test account
    let (code, out, _) = env.json(&["accounts", "test", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["valid"], true);

    // Remove account
    let (code, out, _) = env.json(&["accounts", "remove", "work", "--yes"]);
    assert_eq!(code, 0);
    assert_eq!(out["deleted"], true);
    let (code, out, _) = env.json(&["accounts", "list"]);
    assert_eq!(code, 0);
    assert_eq!(out.as_array().unwrap().len(), 0);
}

#[test]
fn login_command() {
    let mock = standard_mock();
    let env = Env::new(&mock);

    let (code, out, _) = env.json(&["login", "main", "--username", "admin@loopiaapi", "--password", "secret"]);
    assert_eq!(code, 0);
    assert_eq!(out["account"], "main");
    assert_eq!(out["username"], "admin@loopiaapi");
}

#[test]
fn domains_commands() {
    let mock = standard_mock();
    let env = Env::new(&mock).with_account();

    // List domains
    let (code, out, _) = env.json(&["domains", "list", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["domain"], "example.se");

    // Get domain
    let (code, out, _) = env.json(&["domains", "get", "--domain", "example.se", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["domain"], "example.se");
    assert_eq!(out["paid"], true);

    // Check domain available
    let (code, out, _) = env.json(&["domains", "check", "--domain", "newdomain.se", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["available"], true);

    // Order domain requires accept terms
    let (code, _, err) = env.json(&["domains", "order", "--domain", "newdomain.se", "-a", "work"]);
    assert_eq!(code, 6);
    assert!(err["error"].as_str().unwrap().contains("accept-terms"));

    let (code, out, _) = env.json(&["domains", "order", "--domain", "newdomain.se", "--accept-terms", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");

    // Add domain
    let (code, out, _) = env.json(&["domains", "add", "--domain", "added.se", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");

    // Transfer domain
    let (code, out, _) = env.json(&["domains", "transfer", "--domain", "trans.se", "--auth-code", "123", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["transferStarted"], true);

    // Remove domain
    let (code, out, _) = env.json(&["domains", "remove", "--domain", "del.se", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["removed"], true);

    // Nameservers requires at least two
    let (code, _, err) =
        env.json(&["domains", "nameservers", "--domain", "ns.se", "--nameserver", "ns1.loopia.se", "-a", "work"]);
    assert_eq!(code, 6);
    assert!(err["error"].as_str().unwrap().contains("At least two"));

    let (code, out, _) = env.json(&[
        "domains",
        "nameservers",
        "--domain",
        "ns.se",
        "--nameserver",
        "ns1.loopia.se",
        "--nameserver",
        "ns2.loopia.se",
        "-a",
        "work",
    ]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");
}

#[test]
fn subdomains_commands() {
    let mock = standard_mock();
    let env = Env::new(&mock).with_account();

    let (code, out, _) = env.json(&["subdomains", "list", "--domain", "example.se", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out, serde_json::json!(["www", "mail"]));

    let (code, out, _) =
        env.json(&["subdomains", "add", "--domain", "example.se", "--subdomain", "blog", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");

    let (code, out, _) =
        env.json(&["subdomains", "remove", "--domain", "example.se", "--subdomain", "blog", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");
}

#[test]
fn records_commands() {
    let mock = standard_mock();
    let env = Env::new(&mock).with_account();

    let (code, out, _) = env.json(&["records", "list", "--domain", "example.se", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["record_id"], 101);
    assert_eq!(out[0]["type"], "A");

    let (code, out, _) =
        env.json(&["records", "add", "--domain", "example.se", "--type", "A", "--rdata", "192.0.2.1", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");

    let (code, out, _) = env.json(&[
        "records",
        "update",
        "--domain",
        "example.se",
        "--record-id",
        "101",
        "--type",
        "A",
        "--rdata",
        "192.0.2.2",
        "-a",
        "work",
    ]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");

    let (code, out, _) = env.json(&["records", "remove", "--domain", "example.se", "--record-id", "101", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "ok");
}

#[test]
fn billing_and_invoices_commands() {
    let mock = standard_mock();
    let env = Env::new(&mock).with_account();

    let (code, out, _) = env.json(&["billing", "credits", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out, 1500.5);

    let (code, out, _) = env.json(&["billing", "unpaid-invoices", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["reference_no"], "INV-101");

    let (code, out, _) = env.json(&["billing", "invoice", "--reference-no", "INV-101", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["reference_no"], "INV-101");

    let (code, out, _) = env.json(&["billing", "pay-invoice", "--reference-no", "INV-101", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["paid"], true);

    // Invoices alias group
    let (code, out, _) = env.json(&["invoices", "list", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["reference_no"], "INV-101");
}

#[test]
fn reseller_commands() {
    let mock = standard_mock();
    let env = Env::new(&mock).with_account();

    let (code, out, _) = env.json(&["reseller", "customers", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["customer_id"], "CUST-100");

    let (code, out, _) = env.json(&[
        "reseller",
        "create-account",
        "--domain",
        "newclient.se",
        "--account-type",
        "STARTER",
        "--accept-terms",
        "--firstname",
        "Alice",
        "--lastname",
        "Smith",
        "--street",
        "Main St 1",
        "--zip",
        "12345",
        "--city",
        "Stockholm",
        "--country",
        "SE",
        "--email",
        "alice@example.se",
        "-a",
        "work",
    ]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "OK");
    assert_eq!(out["order_reference"], "ORD-12345");

    let (code, out, _) = env.json(&["reseller", "order-status", "--order-reference", "ORD-12345", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "COMPLETED");

    let (code, out, _) =
        env.json(&["reseller", "transfer-credits", "--from", "1001", "--to", "1002", "--amount", "500", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["transferred"], true);
}

#[test]
fn agent_readme_command() {
    let mock = standard_mock();
    let env = Env::new(&mock);

    let (code, out, _) = env.json(&["agent-readme"]);
    assert_eq!(code, 0);
    assert_eq!(out["tool"], "loopia");
    assert_eq!(out["exitCodes"]["0"], "ok");
    assert_eq!(out["exitCodes"]["3"], "auth_required - stop, surface the remediation to a human");

    let out = env.run(&["agent-readme"]);
    let s = String::from_utf8(out.stdout).unwrap();
    assert!(s.contains("# loopia - agent operating manual"));
}

#[test]
fn error_exit_codes() {
    let mock = standard_mock();
    let env = Env::new(&mock);

    // No account specified -> code 7
    let (code, _, err) = env.json(&["domains", "list"]);
    assert_eq!(code, 7);
    assert_eq!(err["code"], "no_account");

    // Invalid account specified -> code 7
    let (code, _, err) = env.json(&["domains", "list", "-a", "nonexistent"]);
    assert_eq!(code, 7);
    assert_eq!(err["code"], "no_account");

    // Invalid arguments -> code 6
    let (code, _, err) = env.json(&["domains", "get"]);
    assert_eq!(code, 6);
    assert_eq!(err["code"], "invalid_input");
}
