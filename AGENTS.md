# AGENTS.md - AGiXT v2 Extension Template Guide

Guidance for AI assistants and humans working in an AGiXT v2 Rust extension repo built from this template.

## What This Repo Is

This is a template for building AGiXT v2 Rust extension hubs plus optional AGiXT Desktop UI bundles.

AGiXT v2 is the Rust backend replacement for AGiXT Python, currently developed under the WorkConductor codename. Rust hub sources live in the product hub repository and are copied into AGiXT v2's Cargo workspace at image build time. Desktop UI bundles live in `ui/` and are served by AGiXT v2's desktop extension endpoints.

## Primary Agent Task

When the user says something like **"Turn this WorkConductor extension template into an extension for {thing}"**, implement it end to end. Do not stop at advice.

Your job is to produce the right combination of:

- Rust command extension at the repository root implementing AGiXT v2's `Extension` trait, for example `example_extension.rs`, or under `extensions/` for larger multi-extension hubs.
- `extension.toml` build manifest listing every Rust module and struct exported by the hub.
- `pricing.json` marketplace metadata with `included_extensions` and optional `company_id`/`company_ids`.
- Optional persistent database/API changes in AGiXT v2 when the feature needs real server-side state.
- Optional desktop UI bundle under `ui/` that calls the Rust backend with `ctx.serverUrl` and `ctx.jwt`.
- Documentation and validation commands.

## Build Workflow

1. Extract requirements: domain, extension slug, command names, required settings, data model, auth needs, desktop UI needs, and persistence needs.
2. Rename mechanically first: `example_extension`, `ExampleExtension`, command names, manifest ID, JS registration ID, scope strings, labels, tests, and README examples.
3. Implement real command logic in Rust. Use clear helpers and return structured JSON values from `execute`.
4. Use explicit settings in `settings()` and consume them in `init()`.
5. Keep command metadata accurate. AGiXT v2 uses it for command seeding, argument coercion, and UI display.
6. If the feature needs persistent database state or custom HTTP routes, make the corresponding AGiXT v2 backend changes. Hub Rust command modules are build-time installed; they do not dynamically register Axum routes at runtime.
7. Make the desktop UI match the backend. For command-backed screens, call `POST /v1/extensions/run`; for core AGiXT v2 APIs, call the real `/v1/...` endpoints.
8. Update README and tests.
9. Validate with formatting, Rust checks/tests, JSON parsing, and JavaScript syntax checks.

## Capability Matrix

| Need | Build This |
| --- | --- |
| Agent command calls a service | Rust `Extension` implementation in a root-level `<extension_slug>.rs` file, or `extensions/<extension_slug>.rs` for larger hubs, with settings, client helper, commands, and structured errors. |
| Desktop page for the extension | `ui/manifest.json` plus `ui/main.js` registered with `window.AgixtRegisterExtension`. |
| User/company-owned persisted records | Add AGiXT-compatible tables and Axum endpoints in AGiXT v2 proper. Keep SQLite/Postgres parity. |
| Existing AGiXT v2 route integration | Use the existing `/v1/...` route from the desktop UI; do not duplicate logic in the UI. |
| External webhook or realtime flow | Add explicit authenticated Axum routes/WebSockets in AGiXT v2 proper. |

## Golden Rules

1. Extension slug is lowercase snake_case, for example `github_issues`.
2. Rust struct is PascalCase, for example `GitHubIssues`.
3. `Extension::name()` must return the slug exactly.
4. Initialize command metadata unconditionally; missing credentials should produce command errors, not hide commands.
5. Do not fake mutating commands. If the command says it created/updated/deleted something, it must actually do that.
6. Use safe Rust. Avoid `unsafe`.
7. Desktop UI gating is not authorization. The backend must enforce access.
8. Bump `ui/manifest.json` `version` whenever `main.js` changes.
9. Keep the template honest about AGiXT v2's current loading model: Rust hubs are installed at image build time and compiled into the backend binary.

## Integration Into AGiXT v2

For a small compiled extension hub, keep the finalized Rust module at the repository root:

```text
<extension_slug>.rs
```

For larger hubs, use:

```text
extensions/<extension_slug>.rs
```

The implementation should import traits from `crate::traits::{...}` so it compiles both in this local harness and after AGiXT v2 copies it into `agixt-extensions`.

Declare the module and struct in:

```text
extension.toml
```

`src/lib.rs` is only a local compile/test adapter. It is not the AGiXT v2 extension entry point and should only mirror enough of AGiXT v2's trait surface to validate the real root-level module.

Then install the hub into AGiXT v2 by cloning/copying it under:

```text
WorkConductor/agixt-rust/extensions_hubs/<hub_name>/
```

or by passing it through AGiXT v2's `EXTENSIONS_HUB` build arg. AGiXT v2's Docker builder copies Rust sources and generates hub registration automatically.

## Local Validation

From this template repo:

```bash
cargo fmt --check
cargo test
node --check ui/main.js
python -m json.tool ui/manifest.json >/dev/null
python -m json.tool pricing.json >/dev/null
```

After integrating into AGiXT v2:

```bash
cd ../WorkConductor/agixt-rust
docker compose -f docker/docker-compose.yml build agixt-api
```

Then run WorkConductor and verify:

- `GET /v1/extensions` lists the extension.
- `POST /v1/extensions/run` executes a command.
- `GET /v1/desktop/extensions?company_id=...&agent_id=...` includes the desktop UI when scopes allow it.

## Secrets

Never commit API keys, OAuth secrets, private URLs, customer data, `.env` files, or generated credentials. Use `settings()` metadata so AGiXT v2 can collect and store extension settings.
