# Build, Test, and Release

## Rust Validation

Run from the repository root:

```powershell
cargo fmt --check
cargo test --workspace
cargo build --release
```

The workspace tests cover domain services, event handlers, transport chunking, logging, persistence-related behavior, and feature workflows.

## WebUI Validation

Run from `webui`:

```powershell
npm install
npm run build
npm run build:arma
```

`build:arma` performs a production Vite build and rewrites Arma-compatible packaged assets into:

```text
arma/crate/addons/webui/ui/_site/
```

Arma cannot load normal Vite asset URLs from the packaged mod. The generated HTML uses `A3API.RequestFile` to retrieve CSS and JavaScript, injects them into the document, and includes an inline bootstrap loader that works before either asset arrives.

## SQF and PBO Validation

Run from `arma/crate`:

```powershell
hemtt check
hemtt build
```

The custom `JSDialog` control event may be reported as unknown by HEMTT because it is supplied by the target Arma browser integration. Treat other warnings and all errors as actionable.

## Complete Build

```powershell
Set-Location arma/crate
cmd /c tools\build.bat
```

Output:

```text
arma/crate/.hemttout/build/
  addons/*.pbo
  forge_crate_x64.dll
  mod metadata and licenses
```

## In-Game Smoke Tests

Cold-start persistence test:

1. Stop Arma/server completely.
2. Verify an existing actor record in SurrealDB.
3. Start the mission.
4. Connect once.
5. Confirm the persisted loadout is applied on the first connection.
6. Confirm logs report `surrealdb cache hydration completed`.

Bank WebUI test:

1. Approach an Eden object named `bank` or `bank_N`.
2. Use `Open Bank`.
3. Confirm the bootstrap loader appears instead of a blank frame.
4. Confirm balances load from the extension.
5. Exercise deposit, withdrawal, transfer, earnings submission, and PIN setup.
6. Confirm only the content pane scrolls and Close dismisses the display.

Locker test:

1. Open a `locker` terminal.
2. Move equipment between actor and proxy.
3. Close inventory.
4. Confirm actor save succeeds before locker commit.
5. Reconnect and verify neither side duplicated the transferred equipment.

## Release Safety

- Do not deploy a new PBO that expects extension commands absent from the installed DLL.
- Deploy the DLL and dependent PBOs together when a contract changes.
- Keep `webui/ui/_site` generated through `npm run build:arma`.
- Do not hand-edit generated WebUI assets.
- Review `@forge_crate/logs` and the Arma RPT after each cold-start integration test.
