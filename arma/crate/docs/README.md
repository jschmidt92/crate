# Forge Rust Server Docs

These docs describe the Rust side of the Forge Arma server extension. They are written for contributors and future wiki migration.

Start here:

- [Architecture](architecture.md): crate layout, runtime layers, command flow, and vertical-slice organization.
- [Feature Guide](features.md): current gameplay/server features and where their code lives.
- [Events](events.md): domain events, the central event bus, durable audit records, and notifications.
- [Adding Features](adding-features.md): practical checklist for adding a new server feature.
- [Persistence](persistence.md): SurrealDB persistence, hydration, queued writes, and transaction behavior.
- [Logging](logging.md): asynchronous domain logs, file locations, and path overrides.
