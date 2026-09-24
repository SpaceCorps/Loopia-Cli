---
title: "Authentication Guide"
description: "Authentication methods, credential storage, and error handling for developers and AI agents using the Loopia CLI."
author: "SpaceCorps"
date: "2026-09-24"
---

# Authentication Guide for Loopia CLI

This document outlines authentication methods, credential storage, and error handling for developers and AI agents using the Loopia CLI.

## Overview
The Loopia CLI interfaces directly with the Loopia XML-RPC API (`https://api.loopia.se/RPCSERV`). Authentication requires a dedicated LoopiaAPI username (typically in the format `user@loopiaapi`) and password configured in your Loopia customer zone. Credentials can be stored securely in the host operating system's native keychain or supplied directly via environment variables.

## Prerequisites
- A Loopia account ([loopia.se](https://www.loopia.se))
- LoopiaAPI credentials created under Kundzon -> Kontoinställningar -> LoopiaAPI
- Loopia CLI installed on your machine (`cargo install --git https://github.com/SpaceCorps/Loopia-Cli --locked`)

## Authentication Flow

### Interactive Login (`loopia login`)
The recommended flow for local developer machines:
```bash
loopia login [account_name]
```
1. You are prompted for your LoopiaAPI username (e.g. `user@loopiaapi`).
2. You are securely prompted for your LoopiaAPI password (input characters are masked).
3. The password is saved into the native OS keystore under `account:{name}`.
4. Non-secret account metadata is recorded in `config.yaml`.

### Non-Interactive / Headless Login
For headless CI/CD environments, Docker containers, or autonomous agent runners:
```bash
echo "$LOOPIA_API_PASSWORD" | loopia accounts add [name] --username "$LOOPIA_API_USERNAME" --password-stdin
```
Or pass the password directly as a CLI flag:
```bash
loopia accounts add [name] --username "$LOOPIA_API_USERNAME" --password "$LOOPIA_API_PASSWORD"
```

## Environment Variables
The CLI checks the environment for credentials when no keystore account is specified:
- `LOOPIA_API_USERNAME`: Fallback LoopiaAPI username.
- `LOOPIA_API_PASSWORD`: Fallback LoopiaAPI password.
- `LOOPIA_ACCOUNT`: Default account name to use for operations when `--account` is omitted.
- `LOOPIA_CUSTOMER_NUMBER`: Optional reseller customer number.
- `LOOPIA_API_ENDPOINT`: Custom XML-RPC endpoint URL (defaults to `https://api.loopia.se/RPCSERV`).

## Multi-Account Management
Switch or verify accounts using:
```bash
loopia accounts list
loopia accounts test [account_name]
```

## Error Handling
When authentication fails, commands exit with non-zero exit codes and output standardized JSON/YAML error payloads:
- `auth_required` (Exit code 3): Invalid username or password, or token rejected by API.
- `no_account` (Exit code 7): Specified account does not exist in keystore or no account specified.
- `invalid_input` (Exit code 6): Arguments or parameters rejected (e.g., BAD_INDATA).
- `rate_limited` (Exit code 5): Loopia API rate limit reached.

## Security Best Practices
1. **Never Commit Passwords**: Keep `.env` or plaintext password files out of version control.
2. **Use OS Keystore**: The CLI automatically utilizes macOS Keychain, Windows DPAPI, or Linux Secret Service.
3. **Dedicated API Users**: In Loopia Kundzon, create dedicated API users with scoped access rights rather than reusing main account passwords.
4. **Machine Verification**: When writing agent automation scripts, always pass `--json` to reliably capture error codes.
