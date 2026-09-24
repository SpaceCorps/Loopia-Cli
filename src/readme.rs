//! The manual an agent reads before its first call. Markdown by default so it can be pasted
//! into a system prompt or a CLAUDE.md; `--json` gives the same rules as data.

use crate::{obj, output};

pub fn print() {
    if output::json() {
        output::write(&obj! {
            "tool" => "loopia",
            "apiVersion" => API_VERSION,
            "rules" => RULES,
            "exitCodes" => obj! {
                "0" => "ok",
                "1" => "error - unclassified, report and stop",
                "2" => "network - retry once, then stop",
                "3" => "auth_required - stop, surface the remediation to a human",
                "4" => "not_found - do not retry",
                "5" => "rate_limited - back off before retrying",
                "6" => "invalid_input - fix the call",
                "7" => "no_account - run loopia accounts list",
            },
        });
        return;
    }
    println!("{README}");
}

pub const API_VERSION: &str = "1.0.0";

const RULES: &[&str] = &[
    "Pass --account <name> on commands or configure a default account with 'loopia account create <name> --default'.",
    "Run 'loopia accounts list' first if you do not know which accounts exist.",
    "On code auth_required, stop and surface the remediation string. Do not retry.",
    "Zone records are addressed by --domain and --subdomain (@ is the root apex domain).",
    "Updating nameservers requires at least two --nameserver arguments.",
    "Deletes are irreversible and take no confirmation. Verify details before removing.",
    "Use --json when parsing command output with jq or in agent tool loops.",
];

const README: &str = r#"# loopia - agent operating manual

A native CLI for the Loopia XML-RPC API: domains, subdomains, DNS zone records, billing,
invoices, and reseller operations. Results are YAML on stdout, errors are YAML on stderr,
and `--json` switches both to JSON.

## Accounts & Authentication

Credentials (username and password) can be stored securely in the native OS keystore
(macOS Keychain, Windows DPAPI, or Linux Secret Service / libsecret).

    loopia accounts list                       # list configured accounts
    loopia accounts add <name> --username <user> [--password <pass>] [--default]
    loopia login [<name>]                      # interactive login prompt
    loopia accounts test <name>                # verify credentials against Loopia API
    loopia accounts remove <name>              # remove account from keystore

You can also pass `--account <name>` (short `-a <name>`) or set `LOOPIA_ACCOUNT`. For CI/CD
or headless environments, `LOOPIA_API_USERNAME` and `LOOPIA_API_PASSWORD` are also supported.

## Domains

Manage domain names in the account:

    loopia domains list -a <account>
    loopia domains get --domain <domain> -a <account>
    loopia domains check --domain <domain> -a <account>
    loopia domains order --domain <domain> --accept-terms -a <account>
    loopia domains add --domain <domain> -a <account>
    loopia domains transfer --domain <domain> --auth-code <code> -a <account>
    loopia domains remove --domain <domain> [--deactivate] -a <account>
    loopia domains nameservers --domain <domain> --nameserver ns1.loopia.se --nameserver ns2.loopia.se -a <account>

## Subdomains

Manage subdomains attached to a domain:

    loopia subdomains list --domain <domain> -a <account>
    loopia subdomains add --domain <domain> --subdomain <subdomain> -a <account>
    loopia subdomains remove --domain <domain> --subdomain <subdomain> -a <account>

## DNS Zone Records

Manage DNS records (A, AAAA, CNAME, MX, TXT, NS, SRV, CAA):

    loopia records list --domain <domain> [--subdomain @] -a <account>
    loopia records add --domain <domain> [--subdomain @] --type A --rdata 192.0.2.1 [--ttl 3600] [--priority 0] -a <account>
    loopia records update --domain <domain> [--subdomain @] --record-id <id> --type A --rdata 192.0.2.2 -a <account>
    loopia records remove --domain <domain> [--subdomain @] --record-id <id> -a <account>

Subdomain defaults to `@` (the apex zone root).

## Billing & Invoices

Inspect PrePAID credits and unpaid invoices:

    loopia billing credits [--with-vat] -a <account>
    loopia billing unpaid-invoices [--with-vat] -a <account>
    loopia billing invoice --reference-no <ref> [--with-vat] -a <account>
    loopia billing pay-invoice --reference-no <ref> -a <account>

## Reseller Operations

Operations available for reseller accounts:

    loopia reseller customers -a <account>
    loopia reseller create-account --domain <domain> --account-type <type> --firstname <f> --lastname <l> ... -a <account>
    loopia reseller order-status --order-reference <ref> -a <account>
    loopia reseller transfer-credits --from <cust1> --to <cust2> --amount <amt> [--currency SEK] -a <account>

## Error Envelope

Failures print YAML (or JSON with `--json`) on stderr and exit with standard codes:

    0  ok
    1  error          unclassified error
    2  network        connection or transport failure - retry once
    3  auth_required  invalid username/password or token rejected
    4  not_found      resource not found
    5  rate_limited   rate limit hit - back off
    6  invalid_input  invalid command line argument or API rejection (BAD_INDATA)
    7  no_account     no account specified - run loopia accounts list
"#;
