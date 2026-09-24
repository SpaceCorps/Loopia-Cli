//! Account management commands.

use std::io::{self, BufRead};

use serde_json::Value;

use crate::account::{self, ApiOptions};
use crate::cli::AccountsCommand;
use crate::config::{self, AccountConfig};
use crate::error::{Error, Result};
use crate::secrets;
use crate::{obj, output};

pub fn run(cmd: AccountsCommand) -> Result<()> {
    match cmd {
        AccountsCommand::List { paths } => list(paths),
        AccountsCommand::Add {
            name,
            username,
            password,
            password_stdin,
            customer_number,
            endpoint,
            default,
            force,
        } => add(
            &name,
            username.as_deref(),
            password.as_deref(),
            password_stdin,
            customer_number.as_deref(),
            endpoint.as_deref(),
            default,
            force,
        ),
        AccountsCommand::Remove { name, yes } => remove(&name, yes),
        AccountsCommand::Test { name } => test(&name),
    }
}

pub fn list(show_paths: bool) -> Result<()> {
    let config = config::load()?;
    let store_backend = secrets::store()?.name();

    if show_paths {
        output::write(&obj! {
            "configPath" => config::config_path().display().to_string(),
            "keystore" => store_backend,
        });
        return Ok(());
    }

    let default_name = config.effective_default();
    let mut list = Vec::new();

    for (name, acct) in config.sorted() {
        let is_default = default_name.as_deref() == Some(name.as_str());
        list.push(obj! {
            "account" => name,
            "username" => &acct.username,
            "customerNumber" => acct.customer_number.as_deref().unwrap_or(""),
            "endpoint" => acct.endpoint.as_deref().unwrap_or(crate::client::DEFAULT_ENDPOINT),
            "isDefault" => is_default,
            "addedAt" => &acct.added_at,
        });
    }

    output::write(&Value::Array(list));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn add(
    name: &str,
    username: Option<&str>,
    password: Option<&str>,
    password_stdin: bool,
    customer_number: Option<&str>,
    endpoint: Option<&str>,
    is_default: bool,
    force: bool,
) -> Result<()> {
    let _lock = config::lock()?;
    let mut config = config::load()?;

    if config.find(name).is_some() && !force {
        return Err(Error::invalid(format!("Account '{name}' already exists. Pass --force to overwrite.")));
    }

    let username = match username {
        Some(u) if !u.trim().is_empty() => u.trim().to_string(),
        _ => {
            eprint!("LoopiaAPI username: ");
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            let trimmed = line.trim().to_string();
            if trimmed.is_empty() {
                return Err(Error::invalid("Username cannot be empty."));
            }
            trimmed
        }
    };

    let password = match password {
        Some(p) if !p.trim().is_empty() => p.trim().to_string(),
        _ if password_stdin => {
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line)?;
            let trimmed = line.trim().to_string();
            if trimmed.is_empty() {
                return Err(Error::invalid("Password from stdin cannot be empty."));
            }
            trimmed
        }
        _ => {
            let pass = rpassword::prompt_password("LoopiaAPI password: ")
                .map_err(|e| Error::other(format!("Failed to read password: {e}")))?;
            let trimmed = pass.trim().to_string();
            if trimmed.is_empty() {
                return Err(Error::invalid("Password cannot be empty."));
            }
            trimmed
        }
    };

    // Store password securely in keystore
    secrets::store()?.set(&secrets::account_key(name), &password)?;

    config.accounts.insert(
        name.to_string(),
        AccountConfig {
            username: username.clone(),
            customer_number: customer_number.map(str::to_string),
            endpoint: endpoint.map(str::to_string),
            added_at: config::now_utc(),
        },
    );

    if is_default || config.accounts.len() == 1 {
        config.default_account = Some(name.to_string());
    }

    config::save(&config)?;

    output::write(&obj! {
        "status" => "ok",
        "account" => name,
        "username" => username,
        "isDefault" => config.default_account.as_deref() == Some(name),
    });

    Ok(())
}

fn remove(name: &str, yes: bool) -> Result<()> {
    let _lock = config::lock()?;
    let mut config = config::load()?;

    let stored_name = match config.find(name) {
        Some((n, _)) => n.clone(),
        None => return Err(Error::not_found(format!("No account named '{name}'."))),
    };

    if !yes {
        eprint!("Delete account '{stored_name}'? (y/N): ");
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if !line.trim().eq_ignore_ascii_case("y") && !line.trim().eq_ignore_ascii_case("yes") {
            return Err(Error::invalid("Cancelled by user."));
        }
    }

    config.accounts.shift_remove(&stored_name);
    if config.default_account.as_deref() == Some(&stored_name) {
        config.default_account = None;
    }

    let _ = secrets::store()?.delete(&secrets::account_key(&stored_name));
    config::save(&config)?;

    output::write(&obj! {
        "status" => "ok",
        "account" => stored_name,
        "deleted" => true,
    });

    Ok(())
}

fn test(name: &str) -> Result<()> {
    let resolved = account::resolve(ApiOptions {
        account: Some(name),
        username: None,
        password: None,
        customer_number: None,
        endpoint: None,
    })?;

    let client = resolved.client();
    let res = client.call("getDomains", &[])?;

    output::write(&obj! {
        "status" => "ok",
        "account" => name,
        "valid" => true,
        "domainCount" => res.as_array().map(Vec::len).unwrap_or(0),
    });

    Ok(())
}
