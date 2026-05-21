---
description: "Use when: turning this WorkConductor extension template into a concrete Rust extension and optional desktop UI."
name: "Create WorkConductor Extension From Template"
argument-hint: "Describe the extension to build, e.g. contacts app, GitHub issues connector, MSP asset dashboard"
agent: "agent"
---

Turn this WorkConductor extension template into a production-quality Rust extension for:

`{{input}}`

Follow [AGENTS.md](../../AGENTS.md) exactly. Treat this as an implementation task, not a planning-only task.

Before editing, inspect the current template files:

- [workconductor.toml](../../workconductor.toml)
- [example_extension.rs](../../example_extension.rs)
- [src/lib.rs](../../src/lib.rs)
- [ui/manifest.json](../../ui/manifest.json)
- [ui/main.js](../../ui/main.js)
- [pricing.json](../../pricing.json)
- [README.md](../../README.md)

Build requirements:

1. Choose a lowercase snake_case extension slug and a PascalCase Rust struct name. Rename `example_extension`, `ExampleExtension`, command names, manifest ID, JavaScript registration ID, scopes, visible labels, and tests consistently.
2. Decide whether this is a command-only connector, a database-backed WorkConductor feature, a desktop UI feature, or a hybrid. Keep only the capabilities that fit the request.
3. Implement WorkConductor commands in `<extension_slug>.rs` at the repository root, or `extensions/<extension_slug>.rs` for larger hubs, with WorkConductor's `Extension` trait. Use `CommandMetadata`, `ArgumentMetadata`, and `SettingMetadata` so WorkConductor can seed command metadata and settings.
4. Keep command implementations deterministic and backend-authoritative. If a command mutates state, use safe interior mutability or a real database-backed service layer; do not fake success.
5. Update `workconductor.toml` so the build-time installer can register the module and struct.
6. Update `pricing.json` with `app_name`, `app_slug`, `marketplace.included_extensions`, and optional company restrictions.
7. If the extension needs persistent AGiXT database tables or custom REST endpoints, add those changes to WorkConductor proper (`agixt-api` and the AGiXT-compatible schema) rather than pretending a standalone Rust hub can register Axum routes dynamically.
8. If a desktop UI is useful, update `ui/manifest.json` and `ui/main.js`. The UI should call WorkConductor with `ctx.serverUrl` and `ctx.jwt`, commonly through `POST /v1/extensions/run` for command-backed screens.
9. Enforce permissions on the backend. Desktop manifest `requires.company_scope` controls visibility only.
10. Update README with settings, commands, desktop behavior, integration steps, and validation instructions.

Validation checklist before finishing:

- No stale `example_extension`, `ExampleExtension`, placeholder command names, or placeholder UI text remains unless intentionally documented.
- `cargo fmt --check` passes.
- `cargo test` or `cargo check` passes for the template crate.
- `ui/manifest.json` parses as JSON.
- `pricing.json` parses as JSON.
- `node --check ui/main.js` passes when Node is available.
- If WorkConductor is available, install the hub into `WorkConductor/agixt-rust/extensions_hubs/`, build the backend image, verify `/v1/extensions` lists it, execute at least one command through `/v1/extensions/run`, and verify the desktop manifest loads.

Final response should include what was built, files changed, assumptions made, and validation performed.
