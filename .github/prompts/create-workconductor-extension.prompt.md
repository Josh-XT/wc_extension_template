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

- [src/example_extension.rs](../../src/example_extension.rs)
- [src/lib.rs](../../src/lib.rs)
- [desktop/example_extension/manifest.json](../../desktop/example_extension/manifest.json)
- [desktop/example_extension/main.js](../../desktop/example_extension/main.js)
- [README.md](../../README.md)

Build requirements:

1. Choose a lowercase snake_case extension slug and a PascalCase Rust struct name. Rename `example_extension`, `ExampleExtension`, command names, manifest ID, JavaScript registration ID, scopes, visible labels, and tests consistently.
2. Decide whether this is a command-only connector, a database-backed WorkConductor feature, a desktop UI feature, or a hybrid. Keep only the capabilities that fit the request.
3. Implement WorkConductor commands with WorkConductor's `Extension` trait. Use `CommandMetadata`, `ArgumentMetadata`, and `SettingMetadata` so WorkConductor can seed command metadata and settings.
4. Keep command implementations deterministic and backend-authoritative. If a command mutates state, use safe interior mutability or a real database-backed service layer; do not fake success.
5. If the extension needs persistent AGiXT database tables or custom REST endpoints, add those changes to WorkConductor proper (`agixt-api` and the AGiXT-compatible schema) rather than pretending a standalone Rust hub can register Axum routes dynamically.
6. If a desktop UI is useful, update `desktop/<id>/manifest.json` and `desktop/<id>/main.js`. The UI should call WorkConductor with `ctx.serverUrl` and `ctx.jwt`, commonly through `POST /v1/extensions/run` for command-backed screens.
7. Enforce permissions on the backend. Desktop manifest `requires.company_scope` controls visibility only.
8. Update README with settings, commands, desktop behavior, integration steps, and validation instructions.

Validation checklist before finishing:

- No stale `example_extension`, `ExampleExtension`, placeholder command names, or placeholder UI text remains unless intentionally documented.
- `cargo fmt --check` passes.
- `cargo test` or `cargo check` passes for the template crate.
- `desktop/<id>/manifest.json` parses as JSON.
- `node --check desktop/<id>/main.js` passes when Node is available.
- If WorkConductor is available, copy/register the module, run the backend, verify `/v1/extensions` lists it, execute at least one command through `/v1/extensions/run`, and verify the desktop manifest loads.

Final response should include what was built, files changed, assumptions made, and validation performed.
