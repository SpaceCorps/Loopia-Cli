# Loopia CLI

A command-line tool for the [Loopia](https://www.loopia.se) XML-RPC API. Manage your domains, subdomains, DNS zone records, and billing from the terminal.

## Installation

```bash
dotnet tool install -g Loopia.Console
```

## Authentication

Create an API user in the Loopia Customer Zone or Reseller Zone under **Account settings → LoopiaAPI**.

### Stored accounts (recommended)

Save each set of credentials once under a name, then pick one per command with `--account`:

```bash
loopia account create ivy      --username ivy@loopiaapi
loopia account create filestar --username filestar@loopiaapi
loopia account create bosma    --username bosma@loopiaapi --default

loopia domains list --account ivy
loopia records list --account filestar --domain example.com --subdomain @
loopia domains list -a bosma
```

Leaving out `--password` prompts for it without echoing it. Passwords are never written in
plain text: on Windows they are encrypted with DPAPI, bound to the current Windows user, and
on Linux and macOS with AES-GCM using a key file that only your account can read. The store
lives at `%APPDATA%\loopia\accounts.json` (`~/.config/loopia/accounts.json` on Linux and
macOS); `LOOPIA_CONFIG_DIR` moves it elsewhere.

Because the encryption is tied to your user account, the store is not portable — copy it to
another machine or user and the passwords will no longer decrypt.

```bash
loopia account list            # show the stored accounts, never their passwords
loopia account list --paths    # also show where they live and how they are encrypted
loopia account delete ivy      # remove one
```

When no `--account` is given, the CLI uses the account marked `--default`, or the only stored
account when there is just one. `LOOPIA_ACCOUNT` selects an account too.

### Environment variables

```bash
export LOOPIA_API_USERNAME=user@loopiaapi
export LOOPIA_API_PASSWORD=your-password
```

### Command-line options

```bash
loopia domains list --username user@loopiaapi --password your-password
```

Credentials are resolved in that order of precedence: explicit options first, then the
selected stored account, then the environment variables.

Resellers acting on a customer's account can supply a customer number via `--customer-number`,
`LOOPIA_CUSTOMER_NUMBER` or `loopia account create --customer-number`. The endpoint defaults to
`https://api.loopia.se/RPCSERV` and can be overridden with `--endpoint`, `LOOPIA_API_ENDPOINT`
or `loopia account create --endpoint`.

## Usage

```bash
loopia <command> [options]
```

### Commands

| Command | Description |
|---------|-------------|
| **Accounts** | |
| `account list` | List the stored accounts |
| `account create` | Store the credentials of a Loopia account under a name |
| `account delete` | Delete a stored account |
| **Domains** | |
| `domains list` | List all domain names in the account |
| `domains get` | Get billing and registration details for a domain |
| `domains check` | Check whether a domain name is available for registration |
| `domains order` | Register a new domain name |
| `domains add` | Add an already registered domain to the account |
| `domains transfer` | Transfer a domain to Loopia using an auth code |
| `domains remove` | Remove or deactivate a domain |
| `domains nameservers` | Set the name servers for a domain |
| **Subdomains** | |
| `subdomains list` | List the subdomains of a domain |
| `subdomains add` | Connect a subdomain to a domain |
| `subdomains remove` | Remove a subdomain |
| **Records** | |
| `records list` | List the zone records of a subdomain |
| `records add` | Add a zone record |
| `records update` | Update a zone record |
| `records remove` | Remove a zone record |
| **Billing** | |
| `billing credits` | Get the LoopiaPrePAID balance |
| `billing unpaid-invoices` | List unpaid invoices |
| `billing invoice` | Get a single invoice by reference number |
| `billing pay-invoice` | Pay an invoice using LoopiaPrePAID credits |
| **Reseller** | |
| `reseller customers` | List the customers connected to the reseller account |
| `reseller create-account` | Create a new Loopia account, optionally registering its domain |
| `reseller order-status` | Get the status of an account creation order |
| `reseller transfer-credits` | Transfer credits between two customer accounts |

### Examples

```bash
# List your domains
loopia domains list

# Check availability and register
loopia domains check --domain example.se
loopia domains order --domain example.se --accept-terms

# Point the apex at a server
loopia records add --domain example.se --type A --rdata 93.188.0.1

# Add mail routing on a subdomain
loopia records add --domain example.se --subdomain mail --type MX \
  --rdata mailcluster.loopia.se --priority 10

# Update a record found via 'records list'
loopia records update --domain example.se --record-id 12345 \
  --type A --rdata 93.188.0.2 --ttl 300

# Delegate a domain to external name servers
loopia domains nameservers --domain example.se \
  --nameserver ns1.example.net --nameserver ns2.example.net

# Check your prepaid balance and settle an invoice
loopia billing credits
loopia billing unpaid-invoices
loopia billing pay-invoice --reference-no 123456
```

> Zone records are always addressed by domain plus subdomain. `--subdomain` defaults to `@`, which is the domain itself.

## Output

All read commands output YAML, using the field names Loopia returns, for easy reading and scripting. Write commands print a confirmation and exit non-zero when Loopia reports a status other than `OK`.

`domains check` is the exception to that rule: it exits `0` when the domain is available and `1` when it is taken, so it can be used directly in a shell condition.

## Rate limits

Loopia allows up to 60 API calls per minute, of which at most 15 may be domain searches (`domains check`).

## License

MIT
