//! Login command for interactive or scripted credential storage.

use crate::cli::LoginArgs;
use crate::commands::accounts;
use crate::error::Result;

pub fn run(args: LoginArgs) -> Result<()> {
    accounts::add(
        &args.name,
        args.username.as_deref(),
        args.password.as_deref(),
        args.password_stdin,
        args.customer_number.as_deref(),
        args.endpoint.as_deref(),
        args.default,
        args.force,
    )
}
