//! Clap CLI definition for Loopia CLI.

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "loopia",
    version,
    about = "CLI for the Loopia XML-RPC API - manage domains, subdomains, DNS records, billing, and reseller accounts",
    after_help = "An LLM agent should start with: loopia agent-readme",
    propagate_version = true,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Print raw JSON instead of YAML, for scripting
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Clone, Default)]
pub struct ApiArgs {
    /// Use the credentials of a stored account (or set LOOPIA_ACCOUNT env var)
    #[arg(short = 'a', long, value_name = "NAME")]
    pub account: Option<String>,

    /// LoopiaAPI username, e.g. user@loopiaapi (or set LOOPIA_API_USERNAME env var)
    #[arg(long, value_name = "USERNAME")]
    pub username: Option<String>,

    /// LoopiaAPI password (or set LOOPIA_API_PASSWORD env var)
    #[arg(long, value_name = "PASSWORD")]
    pub password: Option<String>,

    /// Customer number, resellers only (or set LOOPIA_CUSTOMER_NUMBER env var)
    #[arg(long, value_name = "NUMBER")]
    pub customer_number: Option<String>,

    /// XML-RPC endpoint URL (or set LOOPIA_API_ENDPOINT env var)
    #[arg(long, value_name = "URL")]
    pub endpoint: Option<String>,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Command {
    /// Print the operating manual for an LLM agent driving this CLI
    AgentReadme,

    /// Log in to a Loopia account and store credentials
    Login(LoginArgs),

    /// Manage stored credentials for Loopia accounts
    #[command(subcommand, alias = "account")]
    Accounts(AccountsCommand),

    /// Manage domain names
    #[command(subcommand)]
    Domains(DomainsCommand),

    /// Manage subdomains
    #[command(subcommand)]
    Subdomains(SubdomainsCommand),

    /// Manage DNS zone records
    #[command(subcommand)]
    Records(RecordsCommand),

    /// Inspect credits and invoices
    #[command(subcommand)]
    Billing(BillingCommand),

    /// Manage invoices
    #[command(subcommand)]
    Invoices(InvoicesCommand),

    /// Reseller-only operations
    #[command(subcommand)]
    Reseller(ResellerCommand),
}

// -----------------------------------------------------------------------------
// login & accounts
// -----------------------------------------------------------------------------

#[derive(Args, Clone)]
pub struct LoginArgs {
    /// Name you will pass to --account (default: "default")
    #[arg(value_name = "NAME", default_value = "default")]
    pub name: String,

    /// LoopiaAPI username, e.g. user@loopiaapi
    #[arg(long, value_name = "USERNAME")]
    pub username: Option<String>,

    /// LoopiaAPI password (prompted securely if omitted)
    #[arg(long, value_name = "PASSWORD", conflicts_with = "password_stdin")]
    pub password: Option<String>,

    /// Read password from stdin
    #[arg(long)]
    pub password_stdin: bool,

    /// Customer number, resellers only
    #[arg(long, value_name = "NUMBER")]
    pub customer_number: Option<String>,

    /// XML-RPC endpoint, when it differs from the Loopia default
    #[arg(long, value_name = "URL")]
    pub endpoint: Option<String>,

    /// Use this account when no --account is given
    #[arg(long)]
    pub default: bool,

    /// Overwrite an account of the same name without asking
    #[arg(long)]
    pub force: bool,
}

#[derive(Subcommand)]
pub enum AccountsCommand {
    /// List the stored accounts
    List {
        /// Show where the accounts are stored and how they are encrypted
        #[arg(long)]
        paths: bool,
    },

    /// Store credentials of a Loopia account under a name
    #[command(alias = "create")]
    Add {
        /// Name you will pass to --account, e.g. work
        name: String,

        /// LoopiaAPI username, e.g. user@loopiaapi
        #[arg(long, value_name = "USERNAME")]
        username: Option<String>,

        /// LoopiaAPI password (prompted securely if omitted)
        #[arg(long, value_name = "PASSWORD", conflicts_with = "password_stdin")]
        password: Option<String>,

        /// Read password from stdin
        #[arg(long)]
        password_stdin: bool,

        /// Customer number, resellers only
        #[arg(long, value_name = "NUMBER")]
        customer_number: Option<String>,

        /// XML-RPC endpoint, when it differs from the Loopia default
        #[arg(long, value_name = "URL")]
        endpoint: Option<String>,

        /// Use this account when no --account is given
        #[arg(long)]
        default: bool,

        /// Overwrite an account of the same name without asking
        #[arg(long)]
        force: bool,
    },

    /// Delete a stored account
    #[command(alias = "delete")]
    Remove {
        /// Name of the stored account to delete
        name: String,

        /// Delete without asking for confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Test whether a stored account's credentials work with Loopia API
    Test {
        /// Name of the stored account to test
        name: String,
    },
}

// -----------------------------------------------------------------------------
// domains
// -----------------------------------------------------------------------------

#[derive(Subcommand)]
pub enum DomainsCommand {
    /// List all domain names in the account
    List {
        #[command(flatten)]
        api: ApiArgs,
    },

    /// Get billing and registration details for a domain
    Get {
        /// The domain name to look up (ACE-coded for IDN domains)
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Check whether a domain name is available for registration
    Check {
        /// The domain name to check (ACE-coded for IDN domains)
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Register a new domain name
    Order {
        /// The domain name to register (ACE-coded for IDN domains)
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// Confirm that the account owner has accepted registration terms and conditions
        #[arg(long)]
        accept_terms: bool,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Add an already registered domain to the account
    Add {
        /// An already registered domain name to add to the account
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Transfer a domain to Loopia using an auth code
    Transfer {
        /// The domain name to transfer to Loopia
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// The auth code from current registrar
        #[arg(long, value_name = "CODE")]
        auth_code: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Remove or deactivate a domain
    Remove {
        /// The domain name to remove
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// Deactivate now and remove on due date instead of removing immediately
        #[arg(long)]
        deactivate: bool,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Set the name servers for a domain
    Nameservers {
        /// The domain name to update
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// A name server; repeat the option, at least twice
        #[arg(long = "nameserver", value_name = "HOST")]
        nameservers: Vec<String>,

        #[command(flatten)]
        api: ApiArgs,
    },
}

// -----------------------------------------------------------------------------
// subdomains
// -----------------------------------------------------------------------------

#[derive(Subcommand)]
pub enum SubdomainsCommand {
    /// List the subdomains of a domain
    List {
        /// The domain name whose subdomains to list
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Connect a subdomain to a domain
    Add {
        /// The domain name to attach the subdomain to
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// The subdomain to add, e.g. www
        #[arg(long, value_name = "SUBDOMAIN")]
        subdomain: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Remove a subdomain
    Remove {
        /// The domain name containing the subdomain
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// The subdomain to remove
        #[arg(long, value_name = "SUBDOMAIN")]
        subdomain: String,

        #[command(flatten)]
        api: ApiArgs,
    },
}

// -----------------------------------------------------------------------------
// records
// -----------------------------------------------------------------------------

#[derive(Subcommand)]
pub enum RecordsCommand {
    /// List the zone records of a subdomain
    List {
        /// The domain name holding the zone
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// The subdomain, or @ for the domain itself (default: @)
        #[arg(long, value_name = "SUBDOMAIN", default_value = "@")]
        subdomain: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Add a zone record
    Add {
        /// The domain name holding the zone
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// The subdomain, or @ for the domain itself (default: @)
        #[arg(long, value_name = "SUBDOMAIN", default_value = "@")]
        subdomain: String,

        /// Record type, e.g. A, AAAA, CNAME, MX, TXT, NS, SRV, CAA
        #[arg(long = "type", value_name = "TYPE")]
        record_type: String,

        /// Record data, e.g. an IP address or a host name
        #[arg(long, value_name = "RDATA")]
        rdata: String,

        /// Time to live in seconds (default: 3600)
        #[arg(long, default_value_t = 3600)]
        ttl: u32,

        /// Priority, used by MX and SRV records (default: 0)
        #[arg(long, default_value_t = 0)]
        priority: u32,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Update a zone record
    Update {
        /// The domain name holding the zone
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// The subdomain, or @ for the domain itself (default: @)
        #[arg(long, value_name = "SUBDOMAIN", default_value = "@")]
        subdomain: String,

        /// The ID of the record to update, as returned by 'records list'
        #[arg(long, value_name = "ID")]
        record_id: u32,

        /// Record type, e.g. A, AAAA, CNAME, MX, TXT, NS, SRV, CAA
        #[arg(long = "type", value_name = "TYPE")]
        record_type: String,

        /// Record data, e.g. an IP address or a host name
        #[arg(long, value_name = "RDATA")]
        rdata: String,

        /// Time to live in seconds (default: 3600)
        #[arg(long, default_value_t = 3600)]
        ttl: u32,

        /// Priority, used by MX and SRV records (default: 0)
        #[arg(long, default_value_t = 0)]
        priority: u32,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Remove a zone record
    Remove {
        /// The domain name holding the zone
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// The subdomain, or @ for the domain itself (default: @)
        #[arg(long, value_name = "SUBDOMAIN", default_value = "@")]
        subdomain: String,

        /// The ID of the record to remove, as returned by 'records list'
        #[arg(long, value_name = "ID")]
        record_id: u32,

        #[command(flatten)]
        api: ApiArgs,
    },
}

// -----------------------------------------------------------------------------
// billing & invoices
// -----------------------------------------------------------------------------

#[derive(Subcommand)]
pub enum BillingCommand {
    /// Get the LoopiaPrePAID balance
    Credits {
        /// Include VAT in the returned amount
        #[arg(long)]
        with_vat: bool,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// List unpaid invoices
    UnpaidInvoices {
        /// Include VAT in the returned amounts
        #[arg(long)]
        with_vat: bool,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Get a single invoice by reference number
    Invoice {
        /// The invoice reference number
        #[arg(long, value_name = "REFERENCE")]
        reference_no: String,

        /// Include VAT in the returned amounts
        #[arg(long)]
        with_vat: bool,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Pay an invoice using LoopiaPrePAID credits
    PayInvoice {
        /// The reference number of the invoice to pay
        #[arg(long, value_name = "REFERENCE")]
        reference_no: String,

        #[command(flatten)]
        api: ApiArgs,
    },
}

#[derive(Subcommand)]
pub enum InvoicesCommand {
    /// List unpaid invoices
    List {
        /// Include VAT in the returned amounts
        #[arg(long)]
        with_vat: bool,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Get a single invoice by reference number
    Get {
        /// The invoice reference number
        #[arg(long, value_name = "REFERENCE")]
        reference_no: String,

        /// Include VAT in the returned amounts
        #[arg(long)]
        with_vat: bool,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Pay an invoice using LoopiaPrePAID credits
    Pay {
        /// The reference number of the invoice to pay
        #[arg(long, value_name = "REFERENCE")]
        reference_no: String,

        #[command(flatten)]
        api: ApiArgs,
    },
}

// -----------------------------------------------------------------------------
// reseller
// -----------------------------------------------------------------------------

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum ResellerCommand {
    /// List the customers connected to the reseller account
    Customers {
        #[command(flatten)]
        api: ApiArgs,
    },

    /// Create a new Loopia account, optionally registering its domain
    CreateAccount {
        /// Domain name used as the account username (ACE-coded for IDN domains)
        #[arg(long, value_name = "DOMAIN")]
        domain: String,

        /// LOOPIADOMAIN, LOOPIADNS, EMAIL_PRIVATE, STARTER, HOSTING_PRIVATE, HOSTING_BUSINESS or HOSTING_BUSINESS_PLUS
        #[arg(long, value_name = "TYPE")]
        account_type: String,

        /// NO_CONFIG, PARKING, HOSTING_UNIX, HOSTING_AUTOBAHN or HOSTING_WINDOWS (default: NO_CONFIG)
        #[arg(long, value_name = "CONFIG", default_value = "NO_CONFIG")]
        domain_configuration: String,

        /// Register the domain as part of creating the account
        #[arg(long)]
        buy_domain: bool,

        /// The reseller handles billing instead of the customer being invoiced directly
        #[arg(long)]
        billing_contact_reseller: bool,

        /// The reseller is the technical contact instead of the customer
        #[arg(long)]
        tech_contact_reseller: bool,

        /// Confirm that the end user has accepted the relevant agreements
        #[arg(long)]
        accept_terms: bool,

        /// Owner contact first name
        #[arg(long, value_name = "NAME")]
        firstname: String,

        /// Owner contact last name
        #[arg(long, value_name = "NAME")]
        lastname: String,

        /// Owner contact company, empty for private individuals
        #[arg(long, value_name = "COMPANY", default_value = "")]
        company: String,

        /// Owner contact street address
        #[arg(long, value_name = "STREET")]
        street: String,

        /// Owner contact street address, second line
        #[arg(long, value_name = "STREET", default_value = "")]
        street2: String,

        /// Owner contact postal code
        #[arg(long, value_name = "ZIP")]
        zip: String,

        /// Owner contact city
        #[arg(long, value_name = "CITY")]
        city: String,

        /// Owner contact country as a two-letter ISO code, e.g. SE
        #[arg(long, value_name = "ISO2")]
        country: String,

        /// Owner contact organisation or personal identity number
        #[arg(long, value_name = "ORGNO", default_value = "")]
        orgno: String,

        /// NORID personal identifier, required only for .no domains held by private individuals
        #[arg(long, value_name = "PID", default_value = "")]
        norid_pid: String,

        /// Owner contact phone number
        #[arg(long, value_name = "PHONE", default_value = "")]
        phone: String,

        /// Owner contact mobile number
        #[arg(long, value_name = "CELL", default_value = "")]
        cell: String,

        /// Owner contact fax number
        #[arg(long, value_name = "FAX", default_value = "")]
        fax: String,

        /// Owner contact email address
        #[arg(long, value_name = "EMAIL")]
        email: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Get the status of an account creation order
    OrderStatus {
        /// The order reference returned by 'reseller create-account'
        #[arg(long, value_name = "REFERENCE")]
        order_reference: String,

        #[command(flatten)]
        api: ApiArgs,
    },

    /// Transfer credits between two customer accounts
    TransferCredits {
        /// Customer number of the sending account
        #[arg(long, value_name = "CUSTOMER_NUMBER")]
        from: String,

        /// Customer number of the receiving account
        #[arg(long, value_name = "CUSTOMER_NUMBER")]
        to: String,

        /// The amount to transfer
        #[arg(long, value_name = "AMOUNT")]
        amount: f64,

        /// Three-letter ISO 4217 currency code, SEK or NOK (default: SEK)
        #[arg(long, value_name = "CURRENCY", default_value = "SEK")]
        currency: String,

        #[command(flatten)]
        api: ApiArgs,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn command_tree_is_valid() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }
}
