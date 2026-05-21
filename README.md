# WorkConductor Extension Template

A starter template for building WorkConductor Rust extension hubs plus optional
AGiXT Desktop UI bundles.

This is the Rust counterpart to the AGiXT Python `extension_template` repo. The
important difference is that WorkConductor compiles Rust hub sources into the
backend image at build time. The hub repository owns the product code; the core
WorkConductor repository only provides the build-time installer.

## What's In The Box

```text
wc_extension_template/
├── rust/
│   ├── workconductor.toml                  # Build-time Rust hub manifest
│   └── extensions/
│       └── example_extension.rs            # Example Rust command extension
├── src/
│   └── lib.rs                              # Local validation harness
├── desktop/
│   └── example_extension/
│       ├── manifest.json                   # Desktop UI manifest
│       └── main.js                         # Desktop UI calling WorkConductor
├── .github/
│   └── prompts/
│       └── create-workconductor-extension.prompt.md
├── pricing.json                            # Marketplace/package metadata
├── Cargo.toml                              # Local compile/test harness
├── AGENTS.md                               # AI-assisted implementation guide
└── README.md
```

The Rust example in `rust/extensions/example_extension.rs` implements
WorkConductor's `Extension` trait and exposes:

- `Create Example Item`
- `List Example Items`
- `Get Example Item`
- `Update Example Item`
- `Delete Example Item`

The desktop example calls WorkConductor's generic command endpoint:

```http
POST /v1/extensions/run
```

## Loading Model

WorkConductor does not load Rust source files at runtime. Instead, the Docker
build copies hub-owned Rust sources into the Cargo workspace before compiling
the `agixt` binary.

The build installer reads:

```text
rust/workconductor.toml
rust/extensions/*.rs
rust/api/endpoints/*.rs
rust/crates/*
```

For command extensions, declare modules in `rust/workconductor.toml`:

```toml
[extensions]
modules = [
  { module = "example_extension", struct = "ExampleExtension" },
]
```

WorkConductor generates `hub_generated.rs` during the image build so those
commands are registered, seeded, and listed without storing product
implementations in the core repository.

## Local Development

Run validation from this repo:

```bash
cargo fmt --check
cargo test
node --check desktop/example_extension/main.js
python -m json.tool desktop/example_extension/manifest.json >/dev/null
python -m json.tool pricing.json >/dev/null
```

The local Cargo crate is only a harness. It defines a minimal `crate::traits`
module matching WorkConductor's extension trait surface, then compiles the real
hub source from `rust/extensions/example_extension.rs`.

## Building With WorkConductor

Clone or copy this hub into WorkConductor's ignored hub folder:

```text
WorkConductor/agixt-rust/extensions_hubs/example_extension/
```

Then build WorkConductor:

```bash
cd ../WorkConductor/agixt-rust
docker compose -f docker/docker-compose.yml build agixt-api
```

You can also pass the repository through WorkConductor's build-time hub pull:

```bash
export EXTENSIONS_HUB="owner/example-extension-repo"
export EXTENSIONS_HUB_BRANCH="main"
docker compose -f docker/docker-compose.yml build agixt-api
```

For private hubs, pass the GitHub token as the BuildKit secret supported by
WorkConductor's Dockerfile. Do not commit credentials.

## Marketplace And Company Visibility

`pricing.json` controls marketplace metadata and extension gating. Include every
display or slug form the backend may see:

```json
{
  "app_name": "Example Extension",
  "app_slug": "example-extension",
  "marketplace": {
    "listed": false,
    "included_extensions": [
      "Example Extension",
      "ExampleExtension",
      "example_extension"
    ]
  }
}
```

To make a hub visible only to one company, add `company_id` or `company_ids` at
the top level or inside `marketplace`:

```json
{
  "app_name": "Private Product Hub",
  "app_slug": "private-product-hub",
  "company_id": "1573d1a5-44de-479c-be14-c6281e9b697d"
}
```

When company restrictions are present, WorkConductor hides marketplace entries,
desktop UI bundles, extension settings, and commands from other companies.

## Desktop UI

Desktop bundles keep the existing AGiXT Desktop contract:

- `desktop/<id>/manifest.json`
- `desktop/<id>/main.js`
- `window.AgixtRegisterExtension(id, controller)`

Use `ctx.serverUrl` and `ctx.jwt` for authenticated calls. For command-backed
pages, prefer:

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

Bump `manifest.json` `version` whenever `main.js` changes so the desktop client
reloads it.

## Turning This Into A Real Extension

1. Pick a lowercase snake_case extension slug.
2. Rename `example_extension`, `ExampleExtension`, command names, desktop folder,
   manifest ID, JavaScript registration ID, scope strings, labels, tests, and
   README examples.
3. Update `rust/workconductor.toml` with the new module and struct.
4. Update `pricing.json` with `app_name`, `app_slug`, `included_extensions`, and
   optional `company_id`/`company_ids`.
5. Implement command logic in `rust/extensions/<extension_slug>.rs`.
6. Add explicit settings through `settings()` and read them in `init()` or from
   injected runtime args when appropriate.
7. If the feature needs custom WorkConductor API routes or database tables, add
   those changes to WorkConductor proper with SQLite/Postgres parity.
8. Update the desktop bundle if needed.
9. Validate locally, then build WorkConductor with the hub installed.

The included `.github/prompts/create-workconductor-extension.prompt.md` prompt is
designed for the workflow: "turn this template into a WorkConductor extension
for X."
