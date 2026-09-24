# Loopia CLI

[![Release](https://img.shields.io/github/v/release/SpaceCorps/Loopia-Cli?color=blue&label=version)](https://github.com/SpaceCorps/Loopia-Cli/releases/latest)
[![CI](https://github.com/SpaceCorps/Loopia-Cli/actions/workflows/ci.yml/badge.svg)](https://github.com/SpaceCorps/Loopia-Cli/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-online-success)](https://spacecorps.github.io/Loopia-Cli/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A blazing fast, native command-line tool and autonomous agent interface for the [Loopia](https://www.loopia.se) XML-RPC API (`https://api.loopia.se/RPCSERV`). Built in Rust 2024 for modern developers, DevOps automation, and autonomous AI coding workflows.

---

## Highlights

- ⚡ **Sub-3ms Startup**: Compiled as a native static binary with zero runtime dependencies. Executes in ~1–3 ms (unlike legacy .NET/runtime-bound tools).
- 🔐 **OS Keystore Integration**: `loopia login` prompts securely and stores API passwords in native OS vaults (macOS Keychain, Linux Secret Service / Keyutils, Windows DPAPI).
- 🌐 **Complete XML-RPC Surface**: Full management of domains, subdomains, DNS zone records (A, AAAA, CNAME, TXT, MX, etc.), invoices/billing, and reseller operations.
- 🤖 **AI Agent Native**: Machine-readable `--json` output, standardized error envelopes with stable exit codes (0–7), and built-in `loopia agent-readme`.
- 🏢 **Multi-Account & Reseller First**: Switch accounts with `-a <name>` or `LOOPIA_ACCOUNT`, or manage sub-clients using `--customer-number`.

---

## Installation

### Using Cargo

```bash
cargo install --git https://github.com/SpaceCorps/Loopia-Cli --locked
```

### Pre-built Standalone Binaries

Download standalone binary archives directly from the [GitHub Releases](https://github.com/SpaceCorps/Loopia-Cli/releases/latest) page:

| Platform | Architecture | Binary Package |
|:---|:---|:---|
| **macOS** | Apple Silicon (`aarch64`) | [`loopia-v1.0.0-aarch64-apple-darwin.tar.gz`](https://github.com/SpaceCorps/Loopia-Cli/releases/download/v1.0.0/loopia-v1.0.0-aarch64-apple-darwin.tar.gz) |
| **macOS** | Intel (`x86_64`) | [`loopia-v1.0.0-x86_64-apple-darwin.tar.gz`](https://github.com/SpaceCorps/Loopia-Cli/releases/download/v1.0.0/loopia-v1.0.0-x86_64-apple-darwin.tar.gz) |
| **Linux** | x86_64 (musl static) | [`loopia-v1.0.0-x86_64-unknown-linux-musl.tar.gz`](https://github.com/SpaceCorps/Loopia-Cli/releases/download/v1.0.0/loopia-v1.0.0-x86_64-unknown-linux-musl.tar.gz) |
| **Windows**| x64 (MSVC) | [`loopia-v1.0.0-x86_64-pc-windows-msvc.zip`](https://github.com/SpaceCorps/Loopia-Cli/releases/download/v1.0.0/loopia-v1.0.0-x86_64-pc-windows-msvc.zip) |

---

## Quickstart

### 1. Authenticate

Authenticate interactively to save your API credentials to the OS vault:

```bash
# Interactive login (saved under default account)
loopia login -u user@example.com

# Login to a named account
loopia login production -u prod@example.com

# Headless / CI pipeline login (reads password from stdin without shell history trace)
echo "$LOOPIA_PASSWORD" | loopia login ci -u user@example.com --password-stdin
```

### 2. Inspect Domains & Subdomains

```bash
# List all registered domains in your account
loopia domains list

# Check domain availability
loopia domains check -d example.com

# List subdomains for a domain
loopia subdomains list -d example.com

# Add a new subdomain
loopia subdomains add -d example.com -s api
```

### 3. Manage DNS Zone Records

```bash
# List all DNS records for a subdomain
loopia records list -d example.com -s www

# Add an A record
loopia records add -d example.com -s @ --type A --value 192.0.2.1 --ttl 3600

# Add a TXT verification record
loopia records add -d example.com -s @ --type TXT --value "v=spf1 include:_spf.loopia.se ~all" --ttl 3600

# Update an existing record
loopia records update -d example.com -s @ --record-id 12345 --type A --value 192.0.2.2 --ttl 300

# Remove a record
loopia records remove -d example.com -s @ --record-id 12345
```

### 4. Billing & Invoices

```bash
# List all unpaid invoices
loopia billing invoices

# Inspect an invoice
loopia billing invoice --reference 12345678

# Pay an invoice using account credit balance
loopia billing pay-invoice --reference 12345678
```

---

## Command Reference

Every command that accesses the API accepts `--account <name>` (short `-a <name>`). Reseller commands accept `--customer-number <id>`.

### Authentication & Accounts

| Command | Description |
|:---|:---|
| `loopia login [name]` | Authenticate with username and password; stores password in OS vault |
| `loopia accounts list [--check]` | List configured accounts (pass `--check` to verify credentials against API) |
| `loopia accounts add <name>` | Manually register an account and store password in OS vault |
| `loopia accounts remove <name>` | Remove an account and purge its credentials from the local vault |
| `loopia accounts default <name>` | Set the active default account |

### Domains & Subdomains

| Command | Description |
|:---|:---|
| `loopia domains list` | List all registered domains |
| `loopia domains get -d <domain>` | Get detailed status and expiration for a domain |
| `loopia domains check -d <domain>` | Check if a domain is available for registration |
| `loopia subdomains list -d <domain>` | List subdomains for a domain |
| `loopia subdomains add -d <domain> -s <subdomain>` | Create a new subdomain |
| `loopia subdomains remove -d <domain> -s <subdomain>` | Delete a subdomain and its DNS records |

### DNS Zone Records

| Command | Description |
|:---|:---|
| `loopia records list -d <domain> -s <subdomain>` | List all DNS records for a subdomain |
| `loopia records get -d <domain> -s <subdomain> -r <id>` | Fetch a specific DNS record by record ID |
| `loopia records add -d <domain> -s <subdomain> --type <t> --value <v> [--ttl <s>] [--priority <p>]` | Add a DNS record (A, AAAA, CNAME, TXT, MX, SRV, etc.) |
| `loopia records update -d <domain> -s <subdomain> -r <id> --type <t> --value <v> [--ttl <s>] [--priority <p>]` | Update an existing DNS record |
| `loopia records remove -d <domain> -s <subdomain> -r <id>` | Delete a DNS record |

### Billing & Invoices

| Command | Description |
|:---|:---|
| `loopia billing invoices` | List all unpaid invoices |
| `loopia billing invoice --reference <ref>` | Get details and line items for an invoice |
| `loopia billing pay-invoice --reference <ref>` | Pay an unpaid invoice using prepaid account credits |

### Reseller Operations

| Command | Description |
|:---|:---|
| `loopia reseller customers` | List all sub-customers managed by this reseller account |
| `loopia reseller check-domain -d <domain>` | Check domain availability globally across TLDs |
| `loopia reseller order-status --order-id <id>` | Check fulfillment and provisioning status of an order |
| `loopia reseller transfer-credits --to-customer <id> --amount <n> --currency <c>` | Transfer credits between accounts |

### Agent & Discovery

| Command | Description |
|:---|:---|
| `loopia agent-readme` | Output comprehensive Markdown agent guidance manual |
| `loopia agent-readme --json` | Output machine-readable JSON agent schema, tools, and error codes |

---

## Output Formats & AI Agent Readiness

Commands format stdout as clean YAML by default. Pass `--json` when parsing outputs with `jq`, Python, or LLM tool-calling loops:

```bash
# Extract domain names with jq
loopia domains list --json | jq -r '.[].domain'
```

### Machine-Readable Error Envelopes

All errors are output to `stderr` as structured JSON/YAML envelopes with stable exit codes:

```json
{
  "code": "auth_required",
  "message": "Authentication failed for user user@example.com (AUTH_ERROR)",
  "remediation": "Verify your credentials or re-run 'loopia login'."
}
```

| Exit Code | Error Symbol | Handling Directive |
|:---|:---|:---|
| `0` | `ok` | Command completed successfully |
| `1` | `error` | General failure or API returned an error status |
| `2` | `network` | Network connectivity or HTTP timeout failure; retry with exponential backoff |
| `3` | `auth_required` | Unauthenticated or invalid credentials; surface remediation to user |
| `4` | `not_found` | Resource (domain, subdomain, record, invoice) does not exist; do not retry |
| `5` | `rate_limited` | API rate limit reached; back off before retrying |
| `6` | `invalid_input` | Parameter validation failed; fix input parameters before retrying |
| `7` | `no_account` | Requested account not found in configuration or keystore |

### Agent Manuals

Inspect built-in agent guides directly from the CLI:

```bash
loopia agent-readme          # Human-readable markdown guide
loopia agent-readme --json   # Machine-readable rules and schemas
```

For web-based LLMs and crawlers, refer to [llms.txt](https://spacecorps.github.io/Loopia-Cli/llms.txt) and [llms-full.txt](https://spacecorps.github.io/Loopia-Cli/llms-full.txt).

---

## Configuration & Environment Variables

| Variable | Description | Default |
|:---|:---|:---|
| `LOOPIA_CONFIG_DIR` | Custom directory path for `config.yaml` | `~/.config/loopia` (or OS equivalent) |
| `LOOPIA_SECRET_STORE` | Force specific credential store: `keychain`, `libsecret`, `dpapi`, `plaintext` | Auto-detected |
| `LOOPIA_ALLOW_PLAINTEXT_STORE` | Set to `1` to allow a chmod 0600 file store on headless Linux without DBus | `0` |
| `LOOPIA_API_URL` | Override the XML-RPC endpoint URL (useful for mock testing) | `https://api.loopia.se/RPCSERV` |
| `LOOPIA_ACCOUNT` | Default account name to use if `-a` is not specified | `default` |
| `LOOPIA_USERNAME` | Direct username override (bypasses keystore) | None |
| `LOOPIA_PASSWORD` | Direct password override (bypasses keystore) | None |
| `LOOPIA_CUSTOMER_NUMBER` | Default reseller customer number | None |

---

## Contributing & License

Contributions are welcome! Please submit issues and pull requests to [SpaceCorps/Loopia-Cli](https://github.com/SpaceCorps/Loopia-Cli).

Licensed under the [MIT License](LICENSE).
