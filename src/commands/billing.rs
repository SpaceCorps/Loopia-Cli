//! Billing and invoices commands.

use serde_json::Value;

use crate::account::{self, ApiOptions};
use crate::cli::{ApiArgs, BillingCommand, InvoicesCommand};
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

pub fn run(cmd: BillingCommand) -> Result<()> {
    match cmd {
        BillingCommand::Credits { with_vat, api } => credits(with_vat, &api),
        BillingCommand::UnpaidInvoices { with_vat, api } => unpaid_invoices(with_vat, &api),
        BillingCommand::Invoice { reference_no, with_vat, api } => invoice(&reference_no, with_vat, &api),
        BillingCommand::PayInvoice { reference_no, api } => pay_invoice(&reference_no, &api),
    }
}

pub fn run_invoices(cmd: InvoicesCommand) -> Result<()> {
    match cmd {
        InvoicesCommand::List { with_vat, api } => unpaid_invoices(with_vat, &api),
        InvoicesCommand::Get { reference_no, with_vat, api } => invoice(&reference_no, with_vat, &api),
        InvoicesCommand::Pay { reference_no, api } => pay_invoice(&reference_no, &api),
    }
}

fn credits(with_vat: bool, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("getCreditsAmount", &[Value::Bool(with_vat)])?;
    output::write(&result);
    Ok(())
}

fn unpaid_invoices(with_vat: bool, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("getUnpaidInvoices", &[Value::Bool(with_vat)])?;
    output::write(&result);
    Ok(())
}

fn invoice(reference_no: &str, with_vat: bool, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("getInvoice", &[Value::String(reference_no.to_string()), Value::Bool(with_vat)])?;
    output::write(&result);
    Ok(())
}

fn pay_invoice(reference_no: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call("payInvoiceUsingCredits", &[Value::String(reference_no.to_string())])?;
    LoopiaStatus::ensure_ok(&result, "payInvoiceUsingCredits")?;
    output::write(&obj! {
        "status" => "ok",
        "reference_no" => reference_no,
        "paid" => true,
    });
    Ok(())
}
