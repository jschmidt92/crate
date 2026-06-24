# Forge Developer Documentation

Forge is an Arma 3 mission framework composed of:

- SQF addons packaged as the `forge_crate` Arma mod.
- A native Rust extension named `forge_crate_x64`.
- A reusable Rust domain library named `forge-lib`.
- A Preact application hosted by Arma's `CT_WEBBROWSER`.
- Optional SurrealDB persistence with asynchronous cache-backed writes.

These documents describe the code that currently exists. Examples that describe recommended future integrations are explicitly labeled as examples.

## Start Here

1. [Getting Started](getting-started.md)
2. [Architecture](architecture.md)
3. [Configuration](configuration.md)
4. [Build and Test](testing-and-build.md)
5. [Adding Features](adding-features.md)

## Runtime References

- [SQF Addons and Lifecycle](sqf-addons.md)
- [Extension Command Reference](command-reference.md)
- [Events, Audit, and Notifications](events.md)
- [Persistence](persistence.md)
- [Logging](logging.md)
- [WebUI and Browser Bridge](webui.md)
- [Custom UI Extensibility](custom-ui-extensibility.md)

## Feature Guides

- [Feature Inventory](features.md)
- [Actor](features/actor.md)
- [Bank](features/bank.md)
- [Commander](features/commander.md)
- [Garage and Locker](features/garage-locker.md)
- [Gameplay Services](features/gameplay-services.md)
- [Organization](features/organization.md)
- [Persistence and Transport](features/persistence-transport.md)
- [Refuel](features/refuel.md)

## Repository Map

```text
forge/
  arma/crate/            Arma mod and native extension crate
    addons/              SQF addons and packaged WebUI assets
    src/                 Rust application/runtime layer
    tools/build.bat      Release DLL + HEMTT build pipeline
  lib/                   Shared Rust domain library
  webui/                 Preact/Vite source application
  docs/                  Developer documentation
```

## Architectural Rules

- Domain rules belong in `forge-lib` services.
- Arma-facing parsing belongs in thin extension command modules or SQF handlers.
- Server workflows belong in `arma/crate/src/features`.
- SurrealDB code belongs under `arma/crate/src/persistence`.
- Cross-domain reactions use the Rust `EventBus`.
- SQF modules coordinate through CBA events without sharing domain payload ownership.
- Player money movement goes through `BankService`.
- The browser never directly calls the server extension; it uses the client/server SQF bridge.

## Documentation Maintenance

When a change adds or alters a command, event, table, CBA setting, terminal convention, WebUI route, or persistence rule, update the corresponding root document in the same change.
