//! Reseller operations commands.

use serde_json::{Number, Value};

use crate::account::{self, ApiOptions};
use crate::cli::{ApiArgs, ResellerCommand};
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

pub fn run(cmd: ResellerCommand) -> Result<()> {
    match cmd {
        ResellerCommand::Customers { api } => customers(&api),
        ResellerCommand::CreateAccount {
            domain,
            account_type,
            domain_configuration,
            buy_domain,
            billing_contact_reseller,
            tech_contact_reseller,
            accept_terms,
            firstname,
            lastname,
            company,
            street,
            street2,
            zip,
            city,
            country,
            orgno,
            norid_pid,
            phone,
            cell,
            fax,
            email,
            api,
        } => create_account(
            &domain,
            &account_type,
            &domain_configuration,
            buy_domain,
            billing_contact_reseller,
            tech_contact_reseller,
            accept_terms,
            &firstname,
            &lastname,
            &company,
            &street,
            &street2,
            &zip,
            &city,
            &country,
            &orgno,
            &norid_pid,
            &phone,
            &cell,
            &fax,
            &email,
            &api,
        ),
        ResellerCommand::OrderStatus { order_reference, api } => order_status(&order_reference, &api),
        ResellerCommand::TransferCredits { from, to, amount, currency, api } => {
            transfer_credits(&from, &to, amount, &currency, &api)
        }
    }
}

fn customers(api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call_global("getCustomers", &[])?;
    output::write(&result);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn create_account(
    domain: &str,
    account_type: &str,
    domain_configuration: &str,
    buy_domain: bool,
    billing_contact_reseller: bool,
    tech_contact_reseller: bool,
    accept_terms: bool,
    firstname: &str,
    lastname: &str,
    company: &str,
    street: &str,
    street2: &str,
    zip: &str,
    city: &str,
    country: &str,
    orgno: &str,
    norid_pid: &str,
    phone: &str,
    cell: &str,
    fax: &str,
    email: &str,
    api: &ApiArgs,
) -> Result<()> {
    if !accept_terms {
        return Err(Error::invalid(
            "Loopia requires the end user to have accepted the agreements; pass --accept-terms.",
        ));
    }

    let client = get_client(api)?;
    let contact = obj! {
        "firstname" => firstname,
        "lastname" => lastname,
        "company" => company,
        "street" => street,
        "street2" => street2,
        "zip" => zip,
        "city" => city,
        "country_iso2" => country,
        "orgno" => orgno,
        "norid_pid" => norid_pid,
        "phone" => phone,
        "cell" => cell,
        "fax" => fax,
        "email" => email,
    };

    let result = client.call_global(
        "createNewAccount",
        &[
            Value::String(domain.to_string()),
            contact,
            Value::Bool(billing_contact_reseller),
            Value::Bool(tech_contact_reseller),
            Value::Bool(buy_domain),
            Value::String(domain_configuration.to_string()),
            Value::String(account_type.to_string()),
            Value::Bool(true),
        ],
    )?;

    output::write(&result);
    Ok(())
}

fn order_status(order_reference: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let result = client.call_global("getOrderStatus", &[Value::String(order_reference.to_string())])?;
    output::write(&result);
    Ok(())
}

fn transfer_credits(from: &str, to: &str, amount: f64, currency: &str, api: &ApiArgs) -> Result<()> {
    let client = get_client(api)?;
    let amount_val = Number::from_f64(amount).map(Value::Number).unwrap_or_else(|| Value::String(amount.to_string()));

    let result = client.call_global(
        "transferCreditsByCurrency",
        &[
            Value::String(from.to_string()),
            Value::String(to.to_string()),
            amount_val,
            Value::String(currency.to_string()),
        ],
    )?;
    LoopiaStatus::ensure_ok(&result, "transferCreditsByCurrency")?;
    output::write(&obj! {
        "status" => "ok",
        "from" => from,
        "to" => to,
        "amount" => amount,
        "currency" => currency,
        "transferred" => true,
    });
    Ok(())
}
