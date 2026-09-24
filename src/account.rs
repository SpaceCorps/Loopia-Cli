//! Account resolution for Loopia API calls.

use crate::client::Client;
use crate::config::{self, Config};
use crate::error::{Error, ErrorCode, Result};
use crate::secrets;

pub struct Resolved {
    #[allow(dead_code)]
    pub name: String,
    pub username: String,
    pub password: String,
    pub customer_number: Option<String>,
    pub endpoint: Option<String>,
}

impl Resolved {
    pub fn client(&self) -> Client {
        Client::new(&self.username, &self.password, self.customer_number.as_deref(), self.endpoint.as_deref())
    }
}

pub struct ApiOptions<'a> {
    pub account: Option<&'a str>,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
    pub customer_number: Option<&'a str>,
    pub endpoint: Option<&'a str>,
}

pub fn resolve(opts: ApiOptions<'_>) -> Result<Resolved> {
    // If username and password are provided explicitly via CLI options, use them directly
    if let (Some(u), Some(p)) = (opts.username, opts.password) {
        return Ok(Resolved {
            name: "direct".into(),
            username: u.to_string(),
            password: p.to_string(),
            customer_number: opts.customer_number.map(str::to_string),
            endpoint: opts.endpoint.map(str::to_string),
        });
    }

    let config = config::load()?;

    // Determine account name from flag, env, or default
    let env_account = std::env::var("LOOPIA_ACCOUNT").ok().filter(|s| !s.trim().is_empty());
    let default_account = config.effective_default();
    let requested =
        opts.account.filter(|s| !s.trim().is_empty()).or(env_account.as_deref()).or(default_account.as_deref());

    if let Some(req_name) = requested {
        if let Some((stored_name, acct)) = config.find(req_name) {
            let password = if let Some(p) = opts.password {
                p.to_string()
            } else if let Some(key_pass) = secrets::store()?.get(&secrets::account_key(stored_name))? {
                key_pass
            } else if let Ok(env_p) = std::env::var("LOOPIA_API_PASSWORD") {
                env_p
            } else {
                return Err(Error::new(
                    ErrorCode::AuthRequired,
                    format!("Account '{stored_name}' has no stored password."),
                )
                .detail("The account entry exists in config, but no password was found in the keystore.")
                .fix(format!("loopia accounts add {stored_name} --password <pass> --force")));
            };

            let username = opts.username.map(str::to_string).unwrap_or_else(|| acct.username.clone());
            let customer_number = opts
                .customer_number
                .map(str::to_string)
                .or_else(|| acct.customer_number.clone())
                .or_else(|| std::env::var("LOOPIA_CUSTOMER_NUMBER").ok());
            let endpoint = opts
                .endpoint
                .map(str::to_string)
                .or_else(|| acct.endpoint.clone())
                .or_else(|| std::env::var("LOOPIA_API_ENDPOINT").ok());

            return Ok(Resolved { name: stored_name.clone(), username, password, customer_number, endpoint });
        }

        return Err(Error::new(ErrorCode::NoAccount, format!("No account named '{req_name}'."))
            .detail(describe(&config))
            .fix("loopia accounts list"));
    }

    // Direct environment variables fallback
    if let (Ok(u), Ok(p)) = (std::env::var("LOOPIA_API_USERNAME"), std::env::var("LOOPIA_API_PASSWORD"))
        && !u.trim().is_empty()
        && !p.trim().is_empty()
    {
        return Ok(Resolved {
            name: "env".into(),
            username: u,
            password: p,
            customer_number: opts
                .customer_number
                .map(str::to_string)
                .or_else(|| std::env::var("LOOPIA_CUSTOMER_NUMBER").ok()),
            endpoint: opts.endpoint.map(str::to_string).or_else(|| std::env::var("LOOPIA_API_ENDPOINT").ok()),
        });
    }

    Err(Error::new(ErrorCode::NoAccount, "No account specified. Pass --account <name> or configure one.")
        .detail(describe(&config))
        .fix("loopia accounts list"))
}

fn describe(config: &Config) -> String {
    if config.accounts.is_empty() {
        return "No accounts configured yet. Run 'loopia accounts add <name> --username <user>'.".into();
    }
    let list: Vec<String> = config.sorted().into_iter().map(|(k, v)| format!("{k} ({})", v.username)).collect();
    format!("Configured accounts: {}", list.join(", "))
}
