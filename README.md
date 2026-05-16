# WorkConductor Extension Template

A starter template for building WorkConductor Rust command extensions plus optional AGiXT Desktop UI bundles.

This is the Rust counterpart to the AGiXT Python `extension_template` repo. It keeps the same AI-assisted workflow, but the example code targets the WorkConductor Rust backend instead of Python FastAPI extension hubs.

## What's In The Box

```text
wc_extension_template/
├── src/
│   ├── lib.rs                              # Local validation harness
│   └── example_extension.rs                # Example Rust command extension
├── desktop/
│   └── example_extension/
│       ├── manifest.json                   # Desktop UI manifest
│       └── main.js                         # Desktop UI calling WorkConductor
├── .github/
│   └── prompts/
│       └── create-workconductor-extension.prompt.md
├── Cargo.toml                              # Local compile/test harness
├── AGENTS.md                               # AI-assisted implementation guide
└── README.md
```

The Rust example in `src/example_extension.rs` implements WorkConductor's `Extension` trait and exposes these commands:

- `Create Example Item`
- `List Example Items`
- `Get Example Item`
- `Update Example Item`
- `Delete Example Item`

The desktop example calls WorkConductor's generic command endpoint:

```http
POST /v1/extensions/run
```

That makes the example backend path Rust-native rather than relying on a Python extension router.

## Important Loading Model

AGiXT Python can load `.py` files from an `EXTENSIONS_HUB` at runtime. WorkConductor currently keeps Rust command extensions compiled into the `agixt-extensions` crate. This template is therefore a source template and validation harness:

1. Generate or edit the Rust extension here.
2. Copy the finalized module into WorkConductor.
3. Register it in WorkConductor's extension lists.
4. Keep the optional desktop bundle in a normal extension hub path.

Native Rust shared-library plugin loading is intentionally disabled in WorkConductor's safe loader right now.

## Local Development

This repo assumes it lives next to the local `WorkConductor` checkout:

```text
xtsys/
├── WorkConductor/
└── wc_extension_template/
```

The `Cargo.toml` uses a path dependency on:

```text
../WorkConductor/agixt-rust/crates/agixt-extensions
```

If your checkout differs, update that path.

Run validation:

```bash
cargo fmt --check
cargo test
node --check desktop/example_extension/main.js
python -m json.tool desktop/example_extension/manifest.json >/dev/null
```

## Integrating A Rust Extension Into WorkConductor

After replacing the example with a real extension, copy the Rust module into:

```text
WorkConductor/agixt-rust/crates/agixt-extensions/src/<extension_slug>.rs
```

The file uses `crate::traits::{...}` so it can be copied into WorkConductor unchanged. The template crate's `src/lib.rs` only re-exports those traits locally for validation.

Then update:

```rust
// WorkConductor/agixt-rust/crates/agixt-extensions/src/lib.rs
pub mod <extension_slug>;
pub use <extension_slug>::YourStructName;
```

And register it anywhere WorkConductor seeds built-in extensions:

```rust
// WorkConductor/agixt-rust/crates/agixt-api/src/main.rs
Arc::new(agixt_extensions::YourStructName::new()),
```

Then validate WorkConductor:

```bash
cd ../WorkConductor/agixt-rust
cargo fmt -p agixt-extensions -p agixt-api
cargo check -p agixt-extensions
cargo check -p agixt-api
```

## Desktop UI

Desktop bundles keep the existing AGiXT Desktop contract:

- `desktop/<id>/manifest.json`
- `desktop/<id>/main.js`
- `window.AgixtRegisterExtension(id, controller)`

Use `ctx.serverUrl` and `ctx.jwt` for every authenticated call. For command-backed pages, prefer:

```js
await fetch(new URL('/v1/extensions/run', ctx.serverUrl), {
  method: 'POST',
  headers: {
    Authorization: 'Bearer ' + ctx.jwt,
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    command_name: 'List Example Items',
    command_args: {},
  }),
});
```

Bump `manifest.json` `version` whenever `main.js` changes so the desktop client reloads it.

## Permissions And Scopes

WorkConductor/AGiXT generate these base scopes for an extension:

- `ext:<extension_name>:read`
- `ext:<extension_name>:execute`
- `ext:<extension_name>:configure`

Command names can also produce feature scopes such as `ext:example_extension:create:write` depending on the command metadata and server seeding logic. The desktop `requires.company_scope` gate controls visibility only. Backend commands and endpoints must still enforce real authorization.

## Turning This Into A Real Extension

1. Pick a lowercase snake_case extension slug.
2. Rename `ExampleExtension`, `example_extension`, command names, tests, desktop folder, manifest ID, JS registration ID, and visible labels.
3. Replace `ExampleItem` with the real domain model.
4. Add explicit settings through `settings()` and consume them in `init()`.
5. Implement command logic in Rust and return structured JSON.
6. Add WorkConductor API/database changes if the feature needs persistent records or custom routes.
7. Update the desktop bundle if needed.
8. Update this README with real setup and validation steps.

The included `.github/prompts/create-workconductor-extension.prompt.md` prompt is designed for the workflow: "turn this template into a WorkConductor extension for X."
