//! Blocking HTTP client for the Loopia XML-RPC API.

use std::time::Duration;

use serde_json::Value;
use ureq::Agent;
use ureq::http::Response;

use crate::error::{Error, ErrorCode, Result};
use crate::xmlrpc;

pub const DEFAULT_ENDPOINT: &str = "https://api.loopia.se/RPCSERV";
const MAX_BODY: u64 = 64 * 1024 * 1024;

#[derive(Clone)]
pub struct Client {
    agent: Agent,
    endpoint: String,
    username: String,
    password: String,
    customer_number: Option<String>,
}

impl Client {
    pub fn new(username: &str, password: &str, customer_number: Option<&str>, endpoint: Option<&str>) -> Client {
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(100)))
            .timeout_connect(Some(Duration::from_secs(15)))
            .http_status_as_error(false)
            .user_agent(concat!("loopia-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .into();

        let endpoint = endpoint
            .map(str::to_string)
            .or_else(|| std::env::var("LOOPIA_API_ENDPOINT").ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string());

        Client {
            agent,
            endpoint,
            username: username.to_string(),
            password: password.to_string(),
            customer_number: customer_number.filter(|s| !s.trim().is_empty()).map(str::to_string),
        }
    }

    /// Calls a method that accepts the optional reseller customer number after the credentials.
    pub fn call(&self, method: &str, args: &[Value]) -> Result<Value> {
        let mut params = Vec::with_capacity(3 + args.len());
        params.push(Value::String(self.username.clone()));
        params.push(Value::String(self.password.clone()));
        if let Some(cust) = &self.customer_number {
            params.push(Value::String(cust.clone()));
        }
        params.extend_from_slice(args);
        self.invoke(method, &params)
    }

    /// Calls a method that only takes the credentials, with no customer number parameter.
    pub fn call_global(&self, method: &str, args: &[Value]) -> Result<Value> {
        let mut params = Vec::with_capacity(2 + args.len());
        params.push(Value::String(self.username.clone()));
        params.push(Value::String(self.password.clone()));
        params.extend_from_slice(args);
        self.invoke(method, &params)
    }

    fn invoke(&self, method: &str, params: &[Value]) -> Result<Value> {
        let request_xml = xmlrpc::build_request(method, params);

        let response = self
            .agent
            .post(&self.endpoint)
            .header("Content-Type", "text/xml; charset=utf-8")
            .header("Accept", "text/xml, application/xml")
            .send(request_xml.as_bytes())
            .map_err(transport_error)?;

        read_response(response)
    }
}

fn read_response(mut response: Response<ureq::Body>) -> Result<Value> {
    let status = response.status().as_u16();
    let bytes = response.body_mut().with_config().limit(MAX_BODY).read_to_vec().map_err(transport_error)?;

    let body = String::from_utf8_lossy(&bytes).to_string();

    if !(200..300).contains(&status) {
        return Err(status_error(status, &body));
    }

    xmlrpc::parse_response(&body)
}

fn transport_error(e: ureq::Error) -> Error {
    match e {
        ureq::Error::Timeout(_) => {
            Error::new(ErrorCode::Network, "The request to Loopia timed out.").fix("Retry once, then stop.")
        }
        other => Error::new(ErrorCode::Network, "Could not reach the Loopia XML-RPC API.")
            .detail(other.to_string())
            .fix("Check internet connection and verify the endpoint URL."),
    }
}

pub fn status_error(status: u16, body: &str) -> Error {
    let mut detail = format!("HTTP {status}");
    if !body.trim().is_empty() {
        detail.push_str(": ");
        detail.push_str(body.trim());
    }

    match status {
        401 => Error::new(ErrorCode::AuthRequired, "Loopia API authentication failed.")
            .detail(detail)
            .fix("Verify credentials with: loopia accounts test <account>"),
        403 => Error::new(ErrorCode::AuthRequired, "Access to Loopia API forbidden. Verify account permissions.")
            .detail(detail)
            .fix("Check that the user has API access enabled in Loopia customer zone."),
        404 => Error::new(ErrorCode::NotFound, "The requested endpoint does not exist.").detail(detail),
        429 => Error::new(ErrorCode::RateLimited, "Rate limited by the Loopia API.")
            .detail(detail)
            .fix("Back off before retrying."),
        s if s >= 500 => Error::new(ErrorCode::Network, "Loopia API returned a server error.")
            .detail(detail)
            .fix("Retry; if it persists Loopia platform may be undergoing maintenance."),
        _ => Error::new(ErrorCode::Error, "Loopia API request failed.").detail(detail),
    }
}
