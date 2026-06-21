# Forge Framework Documentation

Welcome to the Forge developer documentation. These guides are written for contributors, developers building on top of the framework, and for future wiki/documentation site migrations.

## Developer Guides

Start here:

- [Architecture Guide](architecture.md): Crate layout, runtime layers, command flow, and vertical-slice organization.
- [Feature Guide](features.md): Overview of current gameplay/server features (Actor, Bank, Commander, Garage/Locker, gameplay services, etc.) and codebase locations.
- [Custom UI/UX & Extensibility Guide](custom-ui-extensibility.md): Step-by-step developer guide on replacing the default UI, browser-SQF communication loop, and routing custom namespaces/CBA events.
- [Events, Audits & Notifications Guide](events.md): Domain events, the central event bus, durable audit records, and client notifications.
- [Adding Features Guide](adding-features.md): A practical step-by-step checklist for implementing new server-side feature slices.
- [Persistence Guide](persistence.md): SurrealDB persistence schemas, cache hydration, WebSocket reconnect, and transaction behavior.
- [Logging Guide](logging.md): Asynchronous non-blocking logging streams, file layouts, and path overrides.
