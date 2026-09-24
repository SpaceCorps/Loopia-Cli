//! XML-RPC serialization and deserialization for LoopiaAPI.

use quick_xml::escape::unescape;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde_json::{Map, Number, Value};

use crate::error::{Error, ErrorCode, Result};

pub fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn serialize_value(v: &Value) -> String {
    match v {
        Value::Null => "<value><nil/></value>".to_string(),
        Value::Bool(b) => format!("<value><boolean>{}</boolean></value>", if *b { "1" } else { "0" }),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                format!("<value><int>{i}</int></value>")
            } else if let Some(f) = n.as_f64() {
                format!("<value><double>{f}</double></value>")
            } else {
                format!("<value><string>{n}</string></value>")
            }
        }
        Value::String(s) => format!("<value><string>{}</string></value>", escape_xml(s)),
        Value::Array(arr) => {
            let mut s = String::from("<value><array><data>");
            for item in arr {
                s.push_str(&serialize_value(item));
            }
            s.push_str("</data></array></value>");
            s
        }
        Value::Object(map) => {
            let mut s = String::from("<value><struct>");
            for (k, val) in map {
                s.push_str("<member>");
                s.push_str(&format!("<name>{}</name>", escape_xml(k)));
                s.push_str(&serialize_value(val));
                s.push_str("</member>");
            }
            s.push_str("</struct></value>");
            s
        }
    }
}

pub fn build_request(method_name: &str, params: &[Value]) -> String {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<methodCall>\n");
    xml.push_str(&format!("  <methodName>{}</methodName>\n", escape_xml(method_name)));
    xml.push_str("  <params>\n");
    for p in params {
        xml.push_str("    <param>\n      ");
        xml.push_str(&serialize_value(p));
        xml.push_str("\n    </param>\n");
    }
    xml.push_str("  </params>\n");
    xml.push_str("</methodCall>");
    xml
}

pub fn parse_response(xml: &str) -> Result<Value> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == "fault" => {
                let fault_val = parse_value_element(&mut reader)?;
                if let Value::Object(map) = fault_val {
                    let code = map
                        .get("faultCode")
                        .and_then(|v| v.as_i64().map(|i| i.to_string()).or_else(|| v.as_str().map(str::to_string)));
                    let msg = map.get("faultString").and_then(Value::as_str).unwrap_or("Unknown XML-RPC fault");
                    let mut err = match code.as_deref() {
                        Some("401") => Error::auth(msg),
                        Some("404") => Error::not_found(msg),
                        Some("429") => Error::rate_limited(msg),
                        _ => Error::other(format!("XML-RPC fault: {msg}")),
                    };
                    if let Some(c) = code {
                        err = err.detail(format!("faultCode: {c}"));
                    }
                    return Err(err);
                }
                return Err(Error::other("XML-RPC fault occurred."));
            }
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == "param" => {
                let val = parse_value_element(&mut reader)?;
                return Ok(val);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Error::other(format!("XML parse error: {e}"))),
            _ => {}
        }
        buf.clear();
    }

    Ok(Value::Null)
}

fn parse_value_element(reader: &mut Reader<&[u8]>) -> Result<Value> {
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == "value" => {
                return parse_inside_value(reader);
            }
            Ok(Event::Empty(ref e)) if e.local_name().as_ref() == "value" => {
                return Ok(Value::String(String::new()));
            }
            Ok(Event::Eof) => return Ok(Value::Null),
            Err(e) => return Err(Error::other(format!("XML error reading value: {e}"))),
            _ => {}
        }
        buf.clear();
    }
}

fn parse_inside_value(reader: &mut Reader<&[u8]>) -> Result<Value> {
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.local_name().as_ref().to_string();
                match name.as_str() {
                    "string" => {
                        let text = read_text_until(reader, "string")?;
                        read_until_end(reader, "value")?;
                        return Ok(Value::String(text));
                    }
                    "int" | "i4" | "i8" => {
                        let text = read_text_until(reader, &name)?;
                        read_until_end(reader, "value")?;
                        let val = text.trim().parse::<i64>().map(Value::from).unwrap_or(Value::String(text));
                        return Ok(val);
                    }
                    "boolean" => {
                        let text = read_text_until(reader, "boolean")?;
                        read_until_end(reader, "value")?;
                        let val = text.trim() == "1" || text.trim().eq_ignore_ascii_case("true");
                        return Ok(Value::Bool(val));
                    }
                    "double" => {
                        let text = read_text_until(reader, "double")?;
                        read_until_end(reader, "value")?;
                        let val = text
                            .trim()
                            .parse::<f64>()
                            .ok()
                            .and_then(Number::from_f64)
                            .map(Value::Number)
                            .unwrap_or(Value::String(text));
                        return Ok(val);
                    }
                    "dateTime.iso8601" => {
                        let text = read_text_until(reader, "dateTime.iso8601")?;
                        read_until_end(reader, "value")?;
                        return Ok(Value::String(text));
                    }
                    "base64" => {
                        let text = read_text_until(reader, "base64")?;
                        read_until_end(reader, "value")?;
                        return Ok(Value::String(text));
                    }
                    "array" => {
                        let arr = parse_array(reader)?;
                        read_until_end(reader, "value")?;
                        return Ok(Value::Array(arr));
                    }
                    "struct" => {
                        let obj = parse_struct(reader)?;
                        read_until_end(reader, "value")?;
                        return Ok(Value::Object(obj));
                    }
                    _ => {
                        let text = read_text_until(reader, &name)?;
                        read_until_end(reader, "value")?;
                        return Ok(Value::String(text));
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = e.local_name();
                if name.as_ref() == "nil" {
                    read_until_end(reader, "value")?;
                    return Ok(Value::Null);
                }
                if name.as_ref() == "string" {
                    read_until_end(reader, "value")?;
                    return Ok(Value::String(String::new()));
                }
            }
            Ok(Event::Text(ref e)) => {
                let s = e.as_ref();
                let text = unescape(s).map(|c| c.into_owned()).unwrap_or_else(|_| s.to_string());
                if !text.trim().is_empty() {
                    read_until_end(reader, "value")?;
                    return Ok(Value::String(text));
                }
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == "value" => {
                return Ok(Value::String(String::new()));
            }
            Ok(Event::Eof) => return Ok(Value::Null),
            Err(e) => return Err(Error::other(format!("XML error in value: {e}"))),
            _ => {}
        }
        buf.clear();
    }
}

fn parse_array(reader: &mut Reader<&[u8]>) -> Result<Vec<Value>> {
    let mut items = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == "value" => {
                items.push(parse_inside_value(reader)?);
            }
            Ok(Event::Empty(ref e)) if e.local_name().as_ref() == "value" => {
                items.push(Value::String(String::new()));
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == "array" => {
                return Ok(items);
            }
            Ok(Event::Eof) => return Ok(items),
            Err(e) => return Err(Error::other(format!("XML error reading array: {e}"))),
            _ => {}
        }
        buf.clear();
    }
}

fn parse_struct(reader: &mut Reader<&[u8]>) -> Result<Map<String, Value>> {
    let mut map = Map::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == "member" => {
                let (k, v) = parse_member(reader)?;
                map.insert(k, v);
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == "struct" => {
                return Ok(map);
            }
            Ok(Event::Eof) => return Ok(map),
            Err(e) => return Err(Error::other(format!("XML error reading struct: {e}"))),
            _ => {}
        }
        buf.clear();
    }
}

fn parse_member(reader: &mut Reader<&[u8]>) -> Result<(String, Value)> {
    let mut key = String::new();
    let mut val = Value::Null;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == "name" => {
                key = read_text_until(reader, "name")?;
            }
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == "value" => {
                val = parse_inside_value(reader)?;
            }
            Ok(Event::Empty(ref e)) if e.local_name().as_ref() == "value" => {
                val = Value::String(String::new());
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == "member" => {
                return Ok((key, val));
            }
            Ok(Event::Eof) => return Ok((key, val)),
            Err(e) => return Err(Error::other(format!("XML error reading member: {e}"))),
            _ => {}
        }
        buf.clear();
    }
}

fn read_text_until(reader: &mut Reader<&[u8]>, tag: &str) -> Result<String> {
    let mut text = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(ref e)) => {
                let s = e.as_ref();
                if let Ok(unesc) = unescape(s) {
                    text.push_str(&unesc);
                } else {
                    text.push_str(s);
                }
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == tag => {
                return Ok(text);
            }
            Ok(Event::Eof) => return Ok(text),
            Err(e) => return Err(Error::other(format!("XML error reading text: {e}"))),
            _ => {}
        }
        buf.clear();
    }
}

fn read_until_end(reader: &mut Reader<&[u8]>, tag: &str) -> Result<()> {
    let mut depth = 0;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == tag => {
                depth += 1;
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == tag => {
                if depth == 0 {
                    return Ok(());
                }
                depth -= 1;
            }
            Ok(Event::Eof) => return Ok(()),
            Err(e) => return Err(Error::other(format!("XML error reading until end of tag: {e}"))),
            _ => {}
        }
        buf.clear();
    }
}

pub struct LoopiaStatus;

impl LoopiaStatus {
    pub const OK: &'static str = "OK";

    pub fn read(result: &Value) -> String {
        match result {
            Value::String(s) => s.clone(),
            Value::Array(arr) if arr.len() == 1 => Self::read(&arr[0]),
            Value::Null => "UNKNOWN_ERROR".into(),
            _ => result.to_string(),
        }
    }

    pub fn ensure_ok(result: &Value, operation: &str) -> Result<()> {
        let status = Self::read(result);
        if status.eq_ignore_ascii_case(Self::OK) {
            return Ok(());
        }

        let err = match status.as_str() {
            "AUTH_ERROR" => Error::new(ErrorCode::AuthRequired, format!("{operation} failed: authentication error."))
                .detail("Invalid LoopiaAPI username or password.")
                .fix("Verify credentials with: loopia accounts test <account>"),
            "BAD_INDATA" => {
                Error::new(ErrorCode::InvalidInput, format!("{operation} failed: bad input parameters (BAD_INDATA)."))
                    .detail("The parameters supplied to the API call were rejected as malformed or invalid.")
                    .fix("Check command options and arguments with --help.")
            }
            "DOMAIN_OCCUPIED" => Error::new(
                ErrorCode::InvalidInput,
                format!("{operation} failed: domain is occupied (DOMAIN_OCCUPIED)."),
            ),
            "DOMAIN_ALREADY_EXISTS" => Error::new(
                ErrorCode::InvalidInput,
                format!("{operation} failed: domain already exists in account (DOMAIN_ALREADY_EXISTS)."),
            ),
            "RATE_LIMITED" | "TOO_MANY_REQUESTS" => {
                Error::new(ErrorCode::RateLimited, format!("{operation} failed: rate limited by Loopia API."))
                    .fix("Back off before retrying.")
            }
            _ => Error::new(ErrorCode::Error, format!("{operation} failed with status: {status}")),
        };

        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a & b < c > 'd' \"e\""), "a &amp; b &lt; c &gt; &apos;d&apos; &quot;e&quot;");
    }

    #[test]
    fn test_serialize_value() {
        assert_eq!(serialize_value(&json!("hello")), "<value><string>hello</string></value>");
        assert_eq!(serialize_value(&json!(123)), "<value><int>123</int></value>");
        assert_eq!(serialize_value(&json!(true)), "<value><boolean>1</boolean></value>");
        assert_eq!(serialize_value(&json!(false)), "<value><boolean>0</boolean></value>");
        assert_eq!(serialize_value(&json!(null)), "<value><nil/></value>");
    }

    #[test]
    fn test_parse_response_string() {
        let xml = "<?xml version=\"1.0\"?><methodResponse><params><param><value><string>OK</string></value></param></params></methodResponse>";
        let val = parse_response(xml).unwrap();
        assert_eq!(val, json!("OK"));
        assert_eq!(LoopiaStatus::read(&val), "OK");
        assert!(LoopiaStatus::ensure_ok(&val, "test").is_ok());
    }

    #[test]
    fn test_parse_response_array_and_struct() {
        let xml = r#"<?xml version="1.0"?>
<methodResponse>
  <params>
    <param>
      <value>
        <array>
          <data>
            <value>
              <struct>
                <member>
                  <name>domain</name>
                  <value><string>example.com</string></value>
                </member>
                <member>
                  <name>paid</name>
                  <value><boolean>1</boolean></value>
                </member>
              </struct>
            </value>
          </data>
        </array>
      </value>
    </param>
  </params>
</methodResponse>"#;
        let val = parse_response(xml).unwrap();
        assert_eq!(val, json!([{"domain": "example.com", "paid": true}]));
    }

    #[test]
    fn test_parse_fault() {
        let xml = r#"<?xml version="1.0"?>
<methodResponse>
  <fault>
    <value>
      <struct>
        <member>
          <name>faultCode</name>
          <value><int>401</int></value>
        </member>
        <member>
          <name>faultString</name>
          <value><string>Bad credentials</string></value>
        </member>
      </struct>
    </value>
  </fault>
</methodResponse>"#;
        let err = parse_response(xml).unwrap_err();
        assert_eq!(err.code, ErrorCode::AuthRequired);
    }
}
