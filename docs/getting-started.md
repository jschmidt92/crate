# Getting Started

## Prerequisites

- Rust toolchain compatible with edition 2024.
- Node.js and npm for the Preact WebUI.
- HEMTT for SQF validation and PBO packaging.
- Arma 3 with CBA A3.
- ACE Arsenal when the Virtual Locker addon is enabled.
- Optional SurrealDB server when durable persistence is enabled.

## Workspace Components

The root Cargo workspace contains:

```text
lib/          package forge-lib
arma/crate/   package forge-crate, output forge_crate_x64.dll
```

The WebUI is a separate npm package:

```text
webui/
```

## First Build

Build and test Rust:

```powershell
cargo fmt --check
cargo test --workspace
cargo build --release
```

Build and package the browser application:

```powershell
Set-Location webui
npm install
npm run build:arma
```

Build the complete Arma mod:

```powershell
Set-Location arma/crate
cmd /c tools\build.bat
```

`tools/build.bat`:

1. Builds the release extension from the root workspace.
2. Copies `target/release/forge_crate_x64.dll` into `arma/crate`.
3. Runs `hemtt check`.
4. Runs `hemtt build`.

The packaged build is written beneath:

```text
arma/crate/.hemttout/build/
```

## Persistence Setup

Copy:

```text
arma/crate/config.example.toml
```

to:

```text
arma/crate/config.toml
```

Set `database.enabled = true`, then configure the SurrealDB endpoint, namespace, database, and credentials. The extension creates its schemaless tables automatically.

The default config path is:

```text
@forge_crate/config.toml
```

Override it with:

```powershell
$env:FORGE_SERVER_CONFIG="G:\forge\arma\crate\config.toml"
```

See [Configuration](configuration.md) and [Persistence](persistence.md).

## Development WebUI

Run the local Vite server:

```powershell
Set-Location webui
npm run dev
```

The browser version can render without Arma, but authoritative bank requests will report that the Arma bridge is unavailable. Use this mode for layout and interaction work, then run `npm run build:arma` before an in-game test.

## Minimal Mission Setup

- Define `CfgMission >> Actor` values for starting balances and default loadout.
- Place bank access objects named `bank`, `bank_1`, `bank_2`, and so on.
- Place locker access objects named `locker`, `locker_1`, `locker_2`, and so on.
- Enable or disable Actor, Virtual Garage, and Virtual Locker persistence through CBA settings.

## First Diagnostics

Useful extension commands:

- `version`
- `status`
- `database_status`
- `database_ready`
- `config_path`
- `log_path`

Extension logs are written under `@forge_crate/logs` by default. See [Logging](logging.md).
