//! Domain management commands.

use serde_json::Value;

use crate::account::{self, ApiOptions};
use crate::cli::{ApiArgs, DomainsCommand};
use crate::client::Client;
use crate::error::{Error, Result};
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

pub fn run(cmd: DomainsCommand) -> Result<()> {
    match cmd {
        DomainsCommand::List { api } => list(&api),
        DomainsCommand::Get { domain, api } => get(&domain, &api),
        DomainsCommand::Check { domain, api } => check(&domain, &api),
        DomainsCommand::Order { domain, accept_terms, api } => order(&domain, accept_terms, &api),
        DomainsCommand::Add { domain, api } => add(&domain, &api),
        DomainsCommand::Transfer { domain, auth_code, api } => transfer(&domain, &auth_code, &api),
        DomainsCommand::Remove { domain, deactivate, api } => remove(&domain, deactivate, &api),
        DomainsCommand::Nameservers { domain, nameservers, api } => update_nameservers(&domain, &nameservers, &api),
    }
}

fn list(api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("getDomains", &[])?;
    output::write(&result);
    Ok(())
}

fn get(domain: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("getDomain", &[Value::String(domain.to_string())])?;
    output::write(&result);
    Ok(())
}

fn check(domain: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let res = client.call_global("domainIsFree", &[Value::String(domain.to_string())])?;
    let status = LoopiaStatus::read(&res);

    if status.eq_ignore_ascii_case(LoopiaStatus::OK) {
        output::write(&obj! {
            "domain" => domain,
            "available" => true,
            "status" => "OK",
        });
        Ok(())
    } else {
        output::write(&obj! {
            "domain" => domain,
            "available" => false,
            "status" => &status,
        });
        Err(Error::invalid(format!("{domain} is not available ({status}).")).detail(status))
    }
}

fn order(domain: &str, accept_terms: bool, api: &ApiArgs) -> Result<()> {
    if !accept_terms {
        return Err(Error::invalid("Loopia rejects orders without accepted terms; pass --accept-terms."));
    }
    let client = get_client(api)?;
    let result = client.call("orderDomain", &[Value::String(domain.to_string()), Value::Bool(true)])?;
    LoopiaStatus::ensure_ok(&result, "orderDomain")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "ordered" => true,
    });
    Ok(())
}

fn add(domain: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("addDomain", &[Value::String(domain.to_string())])?;
    LoopiaStatus::ensure_ok(&result, "addDomain")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "added" => true,
    });
    Ok(())
}

fn transfer(domain: &str, auth_code: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result =
        client.call("transferDomain", &[Value::String(domain.to_string()), Value::String(auth_code.to_string())])?;
    LoopiaStatus::ensure_ok(&result, "transferDomain")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "transferStarted" => true,
    });
    Ok(())
}

fn remove(domain: &str, deactivate: bool, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("removeDomain", &[Value::String(domain.to_string()), Value::Bool(deactivate)])?;
    LoopiaStatus::ensure_ok(&result, "removeDomain")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "deactivated" => deactivate,
        "removed" => !deactivate,
    });
    Ok(())
}

fn update_nameservers(domain: &str, nameservers: &[String], api: &ApiArgs) -> Result<()> {
    if nameservers.len() < 2 {
        return Err(Error::invalid("At least two --nameserver values are required."));
    }
    let client = get_client(api)?;
    let ns_values: Vec<Value> = nameservers.iter().map(|s| Value::String(s.clone())).collect();
    let result = client.call("updateDNSServers", &[Value::String(domain.to_string()), Value::Array(ns_values)])?;
    LoopiaStatus::ensure_ok(&result, "updateDNSServers")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "nameservers" => nameservers,
    });
    Ok(())
}
