# Forge Framework Documentation

Welcome to the Forge developer documentation. These guides are written for contributors, developers building on top of the framework, and for future wiki/documentation site migrations.

## Developer Guides

Start here:

- [Architecture Guide](architecture.md): Crate layout, runtime layers, command flow, and vertical-slice organization.
- **Feature Guides**:
  - [Actor Feature](features/actor.md): Initialization sequence, position safety boundaries, disconnect logic, and client-side lifecycle synchronization.
  - [Bank Feature](features/bank.md): Player cash/account profiles, transaction batch mechanics, WebUI authority bridge, and payday prep.
  - [Commander Feature](features/commander.md): AI asset virtualization pipeline (3-state spawning), dynamic threat mapping, and objective route management.
  - [Garage & Locker Feature](features/garage-locker.md): Physical container networked inventory proxies, transactional actor-save guards, and virtual unlock catalogs.
  - [Gameplay Services](features/gameplay-services.md): Quote-charge workflows for Repair, Rearm, and Medical services.
  - [Organization Feature](features/organization.md): Membership rule engine, CEO disbandment fallback paths, and transactional payday plan execution.
  - [Persistence Status & Transport](features/persistence-transport.md): DB connection states, path definitions, and raw serialization/transmission channels.
  - [Refuel Feature](features/refuel.md): Fuel event hook session state tracker, price configuration mappings, and service receipts.
- [Custom UI/UX & Extensibility Guide](custom-ui-extensibility.md): Step-by-step developer guide on replacing the default UI, browser-SQF communication loop, and routing custom namespaces/CBA events.
- [Events, Audits & Notifications Guide](events.md): Domain events, the central event bus, durable audit records, and client notifications.
- [Adding Features Guide](adding-features.md): A practical step-by-step checklist for implementing new server-side feature slices.
- [Persistence Guide](persistence.md): SurrealDB persistence schemas, cache hydration, WebSocket reconnect, and transaction behavior.
- [Logging Guide](logging.md): Asynchronous non-blocking logging streams, file layouts, and path overrides.
