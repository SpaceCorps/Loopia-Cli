//! DNS zone record management commands.

use serde_json::Value;

use crate::account::{self, ApiOptions};
use crate::cli::{ApiArgs, RecordsCommand};
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

fn zone_record(record_type: &str, rdata: &str, ttl: u32, priority: u32, record_id: u32) -> Value {
    obj! {
        "type" => record_type,
        "ttl" => ttl,
        "priority" => priority,
        "rdata" => rdata,
        "record_id" => record_id,
    }
}

pub fn run(cmd: RecordsCommand) -> Result<()> {
    match cmd {
        RecordsCommand::List { domain, subdomain, api } => list(&domain, &subdomain, &api),
        RecordsCommand::Add { domain, subdomain, record_type, rdata, ttl, priority, api } => {
            add(&domain, &subdomain, &record_type, &rdata, ttl, priority, &api)
        }
        RecordsCommand::Update { domain, subdomain, record_id, record_type, rdata, ttl, priority, api } => {
            update(&domain, &subdomain, record_id, &record_type, &rdata, ttl, priority, &api)
        }
        RecordsCommand::Remove { domain, subdomain, record_id, api } => remove(&domain, &subdomain, record_id, &api),
    }
}

fn list(domain: &str, subdomain: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result =
        client.call("getZoneRecords", &[Value::String(domain.to_string()), Value::String(subdomain.to_string())])?;
    output::write(&result);
    Ok(())
}

fn add(
    domain: &str,
    subdomain: &str,
    record_type: &str,
    rdata: &str,
    ttl: u32,
    priority: u32,
    api: &ApiArgs,
) -> Result<()> {
    let client = get_client(api)?;
    let record = zone_record(record_type, rdata, ttl, priority, 0);
    let result = client
        .call("addZoneRecord", &[Value::String(domain.to_string()), Value::String(subdomain.to_string()), record])?;
    LoopiaStatus::ensure_ok(&result, "addZoneRecord")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "subdomain" => subdomain,
        "type" => record_type,
        "rdata" => rdata,
        "added" => true,
    });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn update(
    domain: &str,
    subdomain: &str,
    record_id: u32,
    record_type: &str,
    rdata: &str,
    ttl: u32,
    priority: u32,
    api: &ApiArgs,
) -> Result<()> {
    let client = get_client(api)?;
    let record = zone_record(record_type, rdata, ttl, priority, record_id);
    let result = client
        .call("updateZoneRecord", &[Value::String(domain.to_string()), Value::String(subdomain.to_string()), record])?;
    LoopiaStatus::ensure_ok(&result, "updateZoneRecord")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "subdomain" => subdomain,
        "record_id" => record_id,
        "updated" => true,
    });
    Ok(())
}

fn remove(domain: &str, subdomain: &str, record_id: u32, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call(
        "removeZoneRecord",
        &[Value::String(domain.to_string()), Value::String(subdomain.to_string()), Value::Number(record_id.into())],
    )?;
    LoopiaStatus::ensure_ok(&result, "removeZoneRecord")?;
    output::write(&obj! {
        "status" => "ok",
        "domain" => domain,
        "subdomain" => subdomain,
        "record_id" => record_id,
        "removed" => true,
    });
    Ok(())
}
