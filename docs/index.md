---
title: "Loopia CLI"
description: "A blazing fast native command-line tool and agent interface for the Loopia XML-RPC API. Built in Rust 2024 for developers and autonomous AI agents."
author: "SpaceCorps"
date: "2026-09-24"
canonical: "https://spacecorps.github.io/Loopia-Cli/index.md"
---

# Loopia CLI

A blazing fast native command-line tool and agent interface for the Loopia XML-RPC API. Built in Rust 2024 for developers and autonomous AI agents.

## Quickstart

```bash
# Authenticate interactively via username and password prompt
loopia login

# Or non-interactively in headless CI/CD environments
echo "$LOOPIA_API_PASSWORD" | loopia accounts add main --username "$LOOPIA_API_USERNAME" --password-stdin
```

## Features

- **Blazing Fast Native Rust**: 1–3 ms startup time with zero runtime dependencies.
- **AI Agent Native**: Structured JSON output (`--json`) and standardized error envelopes.
- **Secure Keystore Integration**: Password storage in native macOS Keychain, Windows DPAPI, and Linux Secret Service.
- **Full Domain & DNS Control**: Complete management of domains, subdomains, nameservers, and zone records (A, AAAA, CNAME, MX, TXT, NS, SRV, CAA).
- **Billing & Reseller Ready**: Query credits, inspect unpaid invoices, pay invoices, and manage sub-accounts.

## When to Use This CLI

Use the `loopia` CLI whenever you need to:
- Manage domain registrations, transfers, and DNS servers.
- Automate DNS zone records for SSL/TLS validation and web infrastructure.
- Inspect LoopiaPrePAID credits and settle invoices programmatically.
- Automate reseller customer provisioning.
- Integrate Loopia capabilities into LLMs or autonomous agents.

## Documentation Links

- [llms.txt](https://spacecorps.github.io/Loopia-Cli/llms.txt)
- [Full Agent Manual](https://spacecorps.github.io/Loopia-Cli/llms-full.txt)
- [Pricing](https://spacecorps.github.io/Loopia-Cli/pricing.md)
- [Authentication Guide](https://spacecorps.github.io/Loopia-Cli/auth.md)
- [GitHub Repository](https://github.com/SpaceCorps/Loopia-Cli)
