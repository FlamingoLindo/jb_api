# Mailer

This directory contains the email-sending layer for the API. It builds SMTP messages, loads HTML templates, and sends outgoing mail for the features that need it.

The code here is focused on delivery only. It does not decide when an email should be sent. Instead, handlers call the mailer when they already know which email needs to go out.

## What It Does

The mailer supports three email flows:

- budget emails with a PDF attachment
- password reset emails with a token link
- database dump emails with an SQL attachment

## Files In This Directory

- `mailer.rs` contains the `Mailer` struct and the send functions.
- `mod.rs` exposes the mailer module to the rest of the crate.

## How It Works

The mailer reads SMTP configuration from environment variables:

- `SMTP_HOST`
- `SMTP_USERNAME`
- `SMTP_PASSWORD`

For each message, it:

1. loads the matching HTML template from `assets/emails/`
2. replaces the template placeholders with real values
3. builds the email with `lettre`
4. sends it through the SMTP relay

The templates used by this module are:

- `assets/emails/budget.html`
- `assets/emails/password.html`
- `assets/emails/dump.html`

## Example

```rust
use crate::mailer::mailer::Mailer;

Mailer::send_forgot_password(
 "user@example.com",
 "john",
 "reset-token-value",
)?;
```

This sends a password reset email to the given recipient using the password template and the token provided by the handler.

## Usage Pattern

Handlers call this module after they have completed the necessary business logic. For example, the password reset handler uses it after generating a token, and the budget handler uses it after producing the budget file.
