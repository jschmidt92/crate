# Rust Server Architecture

Forge is a Rust workspace with two main crates:

- `forge-lib`: shared domain models, repository traits, services, events, and errors.
- `forge-crate`: the Arma extension crate built as `forge_crate_x64`, with command routing, runtime wiring, persistence, and server-specific feature workflows.

The server crate depends on `forge-lib`; `forge-lib` does not depend on the server.

## High-Level Flow

Most extension calls follow this path:

```mermaid
flowchart LR
    Arma[Arma command] --> Route[Route command]
    Route --> Command[Command module]
    Command --> Feature[Feature workflow]
    Feature --> Service[forge-lib service]
    Service --> Repository[Repository trait]
    Repository --> Response[Return response]

    Feature --> Event[Optional domain event]
    Event --> Bus[Central EventBus]
    Bus --> Durable[DurableEventBackend]
```

The command module should stay thin. It should parse command arguments, call the appropriate workflow, serialize the result, and log failures.

Extension callbacks travel in the other direction, from Rust/native extension code back into SQF. The SQF side registers one raw `ExtensionCallback` bridge in `arma/crate/addons/main/XEH_preInitServer.sqf`, then routes Forge callback namespaces to feature-owned CBA events:

```mermaid
flowchart LR
    Extension[ExtensionCallback] --> Main[main callback bridge]
    Main --> Parse[fromJSON payload]
    Parse --> Event[CBA local event]
    Event --> Feature[feature addon handler]
    Feature --> Client[optional client event/UI]
```

Feature addons subscribe to events such as `forge_crate_refuel_price` instead of adding their own raw `ExtensionCallback` handlers.

## Shared Library

`lib/src/models`

Domain data structures and serializable views live here. Examples:

- `actor.rs`: actor snapshots and actor state.
- `bank.rs`: money, bank profiles, accounts, and bank transactions.
- `organization.rs`: organizations, members, invites, payday plans, and transfer result models.
- `domain_event.rs`: the central `DomainEvent` enum.
- `actor_event.rs` and `organization_event.rs`: domain-specific event payloads.
- `notification.rs`: durable notification and audit record models.

`lib/src/repositories`

Repository traits define storage boundaries. In-memory implementations are used for tests and as hot caches in the server.

`lib/src/services`

Services hold domain behavior and validation. They know about repository traits, but they do not know about SurrealDB, Arma command routing, or the server event bus.

`lib/src/events`

The event system defines:

- `DomainEventHandler`: something that reacts to a `DomainEvent`.
- `EventBus`: dispatches events to handlers.
- `EventPublisher`: an interface used by feature workflows so they do not depend on a global bus directly.

## Server Crate

`arma/crate/src/lib.rs`

Initializes logging, config, persistence, the central event bus, and arma-rs command groups.

`arma/crate/src/command.rs`

String route dispatcher used by the transport layer.

`arma/crate/src/events.rs`

Owns the server-level event bus:

```mermaid
flowchart LR
    Publisher[ServerEventPublisher] --> Bus[EventBus]
    Bus --> Durable[DurableEventBackend]
    Bus --> Disconnect[Actor disconnect handlers]
    Durable --> EventRows[(domain_event rows)]
    Durable --> AuditRows[(audit rows)]
    Durable --> Notifications[(notification rows)]
```

This is the application event backbone. Feature workflows publish events through `EventPublisher`, and handlers react through the central bus.

Actor disconnect is initiated once by the actor SQF addon. After the actor snapshot is saved, the actor feature publishes `actor.disconnected`; independent bank, garage, virtual garage, locker, and virtual locker handlers perform their cleanup through this bus.

`arma/crate/src/features`

Feature slices live here:

```text
features/actor/
  init.rs
  lifecycle.rs
  query.rs
```

```text
features/bank/
  account.rs
  lifecycle.rs
```

```text
features/refuel/
features/rearm/
features/repair/
features/medical/
  mod.rs
```

```text
features/garage/
features/locker/
features/v_garage/
features/v_locker/
  lifecycle.rs
  query.rs
  storage.rs
```

```text
features/organization/
  create.rs
  invite.rs
  membership.rs
  payday.rs
  query.rs
  mod.rs
```

Each slice owns workflow orchestration for a related use case group.

`arma/crate/src/persistence`

Persistence-specific code:

- `repository.rs`: cached repository implementations.
- `service.rs`: background persistence worker.
- `surreal.rs`: SurrealDB connection, hydration, and write application.
- `model.rs`: queued write operation types and metrics.
- `payday.rs`: transactional multi-record payday persistence.
- `durable_events.rs`: event handler that writes domain events, audit records, and notifications.

## Design Rules

- Keep core rules in `forge-lib` services.
- Keep server workflow orchestration in feature modules.
- Keep command modules thin.
- Use repository traits in services instead of direct persistence calls.
- Route player bank-account money movement through `BankService`.
- For paid gameplay services, calculate service rules in the service module and charge through `BankService`.
- Publish domain events through `EventPublisher`, not by directly calling persistence.
- Put SurrealDB-specific logic under `arma/crate/src/persistence`.

## Paid Service Flow

```mermaid
sequenceDiagram
    participant SQF as Arma/SQF
    participant Cmd as Server command
    participant Feature as Feature slice
    participant Service as Service module
    participant Bank as BankService
    participant Repo as BankRepository

    SQF->>Cmd: repair/rearm/refuel/heal request
    Cmd->>Feature: parsed input
    Feature->>Service: complete service
    Service->>Service: validate and price
    Service->>Bank: withdraw_from_account
    Bank->>Repo: save updated bank profile
    Service-->>Feature: ServiceReceipt
    Feature-->>Cmd: ServiceReceipt
    Cmd-->>SQF: JSON receipt or Error
```

## Vertical Slice Direction

The project is moving toward a hybrid vertical-slice structure:

- Shared domain models, services, repository traits, and errors remain in `forge-lib`.
- Server workflows move into `arma/crate/src/features/<feature>/<slice>.rs`.
- Command modules expose the Arma surface and delegate to feature workflows.

This keeps shared business rules reusable while making feature work easier to locate.
