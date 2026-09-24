//! Command dispatcher for Loopia CLI.

pub mod accounts;
pub mod billing;
pub mod domains;
pub mod login;
pub mod records;
pub mod reseller;
pub mod subdomains;

use crate::cli::Command;
use crate::error::Result;
use crate::readme;

pub fn run(command: Command) -> Result<()> {
    match command {
        Command::AgentReadme => {
            readme::print();
            Ok(())
        }
        Command::Login(args) => login::run(args),
        Command::Accounts(cmd) => accounts::run(cmd),
        Command::Domains(cmd) => domains::run(cmd),
        Command::Subdomains(cmd) => subdomains::run(cmd),
        Command::Records(cmd) => records::run(cmd),
        Command::Billing(cmd) => billing::run(cmd),
        Command::Invoices(cmd) => billing::run_invoices(cmd),
        Command::Reseller(cmd) => reseller::run(cmd),
    }
}
