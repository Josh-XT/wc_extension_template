# AGENTS.md - WorkConductor Extension Template Guide

Guidance for AI assistants and humans working in a WorkConductor Rust extension repo built from this template.

## What This Repo Is

This is a template for building WorkConductor Rust command extensions plus optional AGiXT Desktop UI bundles.

WorkConductor is the Rust backend replacement for AGiXT. Unlike AGiXT Python extension hubs, Rust command extensions are currently compiled into WorkConductor's `agixt-extensions` crate. Desktop UI bundles still use the familiar `desktop/<id>/manifest.json` and `desktop/<id>/main.js` shape and are served by WorkConductor's desktop extension endpoints.

## Primary Agent Task

When the user says something like **"Turn this WorkConductor extension template into an extension for {thing}"**, implement it end to end. Do not stop at advice.

Your job is to produce the right combination of:

- Rust command extension implementing WorkConductor's `Extension` trait.
- Optional persistent database/API changes in WorkConductor when the feature needs real server-side state.
- Optional desktop UI bundle that calls the Rust backend with `ctx.serverUrl` and `ctx.jwt`.
- Documentation and validation commands.

## Build Workflow

1. Extract requirements: domain, extension slug, command names, required settings, data model, auth needs, desktop UI needs, and persistence needs.
2. Rename mechanically first: `example_extension`, `ExampleExtension`, command names, manifest ID, JS registration ID, scope strings, labels, tests, and README examples.
3. Implement real command logic in Rust. Use clear helpers and return structured JSON values from `execute`.
4. Use explicit settings in `settings()` and consume them in `init()`.
5. Keep command metadata accurate. WorkConductor uses it for command seeding, argument coercion, and UI display.
6. If the feature needs persistent database state or custom HTTP routes, make the corresponding WorkConductor backend changes. Do not imply standalone Rust repos can dynamically register Axum routes at runtime.
7. Make the desktop UI match the backend. For command-backed screens, call `POST /v1/extensions/run`; for core WorkConductor APIs, call the real `/v1/...` endpoints.
8. Update README and tests.
9. Validate with formatting, Rust checks/tests, JSON parsing, and JavaScript syntax checks.

## Capability Matrix

| Need | Build This |
| --- | --- |
| Agent command calls a service | Rust `Extension` implementation with settings, client helper, commands, and structured errors. |
| Desktop page for the extension | `desktop/<id>/manifest.json` plus `desktop/<id>/main.js` registered with `window.AgixtRegisterExtension`. |
| User/company-owned persisted records | Add AGiXT-compatible tables and Axum endpoints in WorkConductor proper. Keep SQLite/Postgres parity. |
| Existing WorkConductor route integration | Use the existing `/v1/...` route from the desktop UI; do not duplicate logic in the UI. |
| External webhook or realtime flow | Add explicit authenticated Axum routes/WebSockets in WorkConductor proper. |

## Golden Rules

1. Extension slug is lowercase snake_case, for example `github_issues`.
2. Rust struct is PascalCase, for example `GitHubIssues`.
3. `Extension::name()` must return the slug exactly.
4. Initialize command metadata unconditionally; missing credentials should produce command errors, not hide commands.
5. Do not fake mutating commands. If the command says it created/updated/deleted something, it must actually do that.
6. Use safe Rust. Avoid `unsafe`.
7. Desktop UI gating is not authorization. The backend must enforce access.
8. Bump `desktop/<id>/manifest.json` `version` whenever `main.js` changes.
9. Keep the template honest about WorkConductor's current loading model: Rust extensions are compiled in.

## Integration Into WorkConductor

For a compiled extension, copy the finalized Rust module into:

```text
WorkConductor/agixt-rust/crates/agixt-extensions/src/<extension_slug>.rs
```

The implementation should live in `src/<extension_slug>.rs` and import traits from `crate::traits::{...}`. The template's `src/lib.rs` exists only as a local validation harness that re-exports WorkConductor's traits.

Then update:

- `crates/agixt-extensions/src/lib.rs` with `pub mod <extension_slug>;` and `pub use <extension_slug>::StructName;`.
- `crates/agixt-api/src/main.rs` built-in extension registration lists with `Arc::new(agixt_extensions::StructName::new())`.

If the desktop bundle should ship with WorkConductor or AGiXT Desktop, place it under an extension hub searched by WorkConductor, such as:

```text
<hub>/desktop/<extension_slug>/manifest.json
<hub>/desktop/<extension_slug>/main.js
```

## Local Validation

From this template repo:

```bash
cargo fmt --check
cargo test
node --check desktop/example_extension/main.js
python -m json.tool desktop/example_extension/manifest.json >/dev/null
```

After integrating into WorkConductor:

```bash
cd ../WorkConductor/agixt-rust
cargo fmt -p agixt-extensions -p agixt-api
cargo check -p agixt-extensions
cargo check -p agixt-api
```

Then run WorkConductor and verify:

- `GET /v1/extensions` lists the extension.
- `POST /v1/extensions/run` executes a command.
- `GET /v1/desktop/extensions?company_id=...&agent_id=...` includes the desktop UI when scopes allow it.

## Secrets

Never commit API keys, OAuth secrets, private URLs, customer data, `.env` files, or generated credentials. Use `settings()` metadata so WorkConductor can collect and store extension settings.
