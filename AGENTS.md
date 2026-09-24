# AGENTS.md

Notes for whoever extends or maintains this codebase next.

`loopia` is a high-performance Rust CLI over the Loopia XML-RPC API (`https://api.loopia.se/RPCSERV`), built to be driven by humans and autonomous LLM agents. It replaces the legacy .NET global tool `Loopia.Console` by Niels Bosma and maintains backward compatibility with legacy configuration paths while introducing native OS vaults, sub-3ms startup, and agent discovery standards.

For the manual the *agent* reads, run `loopia agent-readme` (or `loopia agent-readme --json`) - that text lives in `src/readme.rs` and is the tool's machine interface. This file is for developers and agents editing the source code.

## Developer Commands

```bash
cargo build --release              # target/release/loopia
cargo test --locked                # unit tests + tests/cli.rs against an in-process mock XML-RPC server
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --check
cargo install --path . --locked    # install on system PATH
```

Always use a throwaway config directory when testing so you never touch real credentials:

```bash
export LOOPIA_CONFIG_DIR=$(mktemp -d) LOOPIA_SECRET_STORE=plaintext LOOPIA_ALLOW_PLAINTEXT_STORE=1
```

| Variable | Effect |
| --- | --- |
| `LOOPIA_CONFIG_DIR` | Overrides the configuration and secrets storage directory |
| `LOOPIA_SECRET_STORE` | Forces a specific credential vault backend: `keychain`, `libsecret`, `dpapi`, `plaintext` |
| `LOOPIA_ALLOW_PLAINTEXT_STORE=1` | Permits the plaintext fallback store when no OS keystore exists (e.g. headless CI) |
| `LOOPIA_API_URL` | Overrides the XML-RPC endpoint URL — how `tests/cli.rs` points at its in-process mock |
| `LOOPIA_ACCOUNT` | Specifies default account to resolve if `-a` is not given |
| `LOOPIA_USERNAME` / `LOOPIA_PASSWORD` | Direct credential overrides (useful in isolated CI containers) |
| `LOOPIA_CUSTOMER_NUMBER` | Default reseller customer number |

## Source Layout

```
src/
  main.rs          arg parsing, --json pre-scan, clap error envelopes
  cli.rs           clap derive command tree, argument definitions, aliases, and help text
  commands/
    mod.rs         command dispatcher
    accounts.rs    accounts add|list|test|remove|default
    login.rs       interactive and stdin credential login
    domains.rs     domains list|get|check
    subdomains.rs  subdomains list|add|remove
    records.rs     records list|get|add|update|remove
    billing.rs     billing invoices|invoice|pay-invoice
    reseller.rs    reseller customers|check-domain|order-status|transfer-credits
  client.rs        blocking XML-RPC HTTP client (ureq + rustls), status code -> ErrorCode mapping
  xmlrpc.rs        event-driven XML-RPC serializer & deserializer (quick-xml 0.42), LoopiaStatus parsing
  error.rs         ErrorCode (= exit code 1..7) and Error { code, message, remediation }
  output.rs        YAML default, JSON with --json, structured error envelope, obj! macro
  account.rs       Account resolution pipeline: flags -> env -> default -> keystore
  config.rs        config.yaml persistence, atomic writes (0600), legacy accounts.json fallback
  secrets.rs       macOS Keychain (/usr/bin/security), Linux secret-tool, Windows DPAPI, plaintext fallback
  readme.rs        built-in loopia agent-readme documentation generator
tests/cli.rs       end-to-end integration tests using in-process TCP mock XML-RPC server
```

## Architecture Decisions

**Blocking HTTP with ureq, zero async overhead.** CLI invocations execute single-shot RPC operations. Tokio and async runtimes introduce unnecessary compilation weight, binary bloat, and runtime initialization cost. `ureq 3.4` with `rustls` provides instant startup (<3ms).

**Pure event-driven XML-RPC engine.** Rather than pulling in heavy generic XML-RPC libraries, `src/xmlrpc.rs` implements a clean, robust, RFC-compliant XML-RPC serializer and streaming deserializer on top of `quick-xml 0.42`. It transparently maps XML-RPC `<struct>`, `<array>`, `<string>`, `<int>`, `<boolean>`, and `<double>` values into `serde_json::Value`.

**LoopiaStatus semantic validation.** Loopia XML-RPC responses frequently return string status codes (e.g. `OK`, `AUTH_ERROR`, `BAD_INDATA`, `UNKNOWN_ERROR`, `INSUFFICIENT_FUNDS`). The XML-RPC layer automatically converts these into structured `Error` objects with semantic `ErrorCode` values and clear remediations.

**Keystore integration via CLI utilities.** Like the canonical SpaceCorps blueprint, the macOS Keychain is accessed via `/usr/bin/security` to prevent code-signing popups on rebuilds. Linux utilizes `secret-tool` (Secret Service API), and Windows utilizes DPAPI (`CryptProtectData`/`CryptUnprotectData`).

## Invariants

1. **Exit codes are strictly mapped to the `code:` field.**
   - `0`: Success (`ok`)
   - `1`: General failure (`error`)
   - `2`: Network / HTTP failure (`network`)
   - `3`: Authentication failure (`auth_required`)
   - `4`: Not found (`not_found`)
   - `5`: Rate limited (`rate_limited`)
   - `6`: Invalid argument / schema failure (`invalid_input`)
   - `7`: Account not found (`no_account`)
2. **Standardized error envelopes on stderr.** Whether an error originates from clap arg parsing, keystore lookup, HTTP transport, or Loopia XML-RPC status codes, stderr always receives a structured envelope:
   ```json
   {
     "code": "auth_required",
     "message": "Authentication failed for user user@example.com",
     "remediation": "Verify your credentials or re-run 'loopia login'."
   }
   ```
3. **Multi-account priority order:**
   `--username / --password` &rarr; `--account <name>` &rarr; `LOOPIA_ACCOUNT` &rarr; config default account &rarr; single account fallback &rarr; `LOOPIA_USERNAME / LOOPIA_PASSWORD`.

## Releasing

CI (`.github/workflows/ci.yml`) runs `cargo fmt`, `cargo clippy`, and `cargo test` across macOS, Ubuntu, and Windows. Releases are triggered by publishing a GitHub tag or release:

```bash
git tag v1.0.0
git push origin v1.0.0
```

The workflow automatically builds optimized standalone binaries for `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, and `x86_64-pc-windows-msvc`, and attaches them to the release.
