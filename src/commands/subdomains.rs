//! Subdomain management commands.

use serde_json::Value;

use crate::account::{self, ApiOptions};
use crate::cli::{ApiArgs, SubdomainsCommand};
use crate::client::Client;
use crate::error::Result;
use crate::xmlrpc::LoopiaStatus;
use crate::{obj, output};

fn get_client(api: &ApiArgs) -> Result<Client> {
    let resolved = account::resolve(ApiOptions {
        account: api.account.as_deref(),
        username: api.username.as_deref(),
        password: api.password.as_deref(),
        customer_number: api.customer_number.as_deref(),
        endpoint: api.endpoint.as_deref(),
    })?;
    Ok(resolved.client())
}

pub fn run(cmd: SubdomainsCommand) -> Result<()> {
    match cmd {
        SubdomainsCommand::List { domain, api } => list(&domain, &api),
        SubdomainsCommand::Add { domain, subdomain, api } => add(&domain, &subdomain, &api),
        SubdomainsCommand::Remove { domain, subdomain, api } => remove(&domain, &subdomain, &api),
    }
}

fn list(domain: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("getSubdomains", &[Value::String(domain.to_string())])?;
    output::write(&result);
    Ok(())
}

fn add(domain: &str, subdomain: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result =
        client.call("addSubdomain", &[Value::String(domain.to_string()), Value::String(subdomain.to_string())])?;
    LoopiaStatus::ensure_ok(&result, "addSubdomain")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "subdomain" => subdomain,
        "added" => true,
    });
    Ok(())
}

fn remove(domain: &str, subdomain: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result =
        client.call("removeSubdomain", &[Value::String(domain.to_string()), Value::String(subdomain.to_string())])?;
    LoopiaStatus::ensure_ok(&result, "removeSubdomain")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "subdomain" => subdomain,
        "removed" => true,
    });
    Ok(())
}
