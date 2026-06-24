# Architecture

Forge uses a layered, event-driven architecture with vertical feature slices at the application boundary.

## System Context

```mermaid
flowchart LR
    Player[Arma player] --> SQF[forge_crate SQF addons]
    SQF --> Extension[forge_crate_x64]
    Extension --> Domain[forge-lib]
    Extension --> Cache[Hot repositories]
    Cache --> Queue[Async write queue]
    Queue --> DB[(SurrealDB)]
    SQF --> Browser[Preact WebUI]
    Browser --> SQF

    classDef step fill:#18181b,stroke:#a57c34,color:#f4f4f5,stroke-width:1.5px
    classDef boundary fill:#1c1917,stroke:#d6a84f,color:#f4f4f5,stroke-width:2px
    classDef storage fill:#121214,stroke:#d6a84f,color:#f4f4f5,stroke-width:2px
    class Extension,Domain,Cache,Queue step
    class Player,SQF,Browser boundary
    class DB storage
    linkStyle default stroke:#a57c34,stroke-width:1.5px
```

The authoritative gameplay state lives in Rust repositories. SQF owns Arma-engine state and locality. The WebUI is a presentation client and never directly accesses the extension or database.

## Workspace Dependency Direction

```mermaid
flowchart TD
    WebUI[webui Preact application] --> SQF[Arma SQF bridge]
    SQF --> Server[forge-crate application]
    Server --> Lib[forge-lib domain]
    Server --> Persistence[SurrealDB adapter]
    Persistence --> Lib

    classDef step fill:#18181b,stroke:#a57c34,color:#f4f4f5,stroke-width:1.5px
    classDef boundary fill:#1c1917,stroke:#d6a84f,color:#f4f4f5,stroke-width:2px
    class WebUI,SQF boundary
    class Server,Lib,Persistence step
    linkStyle default stroke:#a57c34,stroke-width:1.5px
```

`forge-lib` has no dependency on Arma, arma-rs, Tokio, or SurrealDB.

## Rust Layers

### Domain Library

Location:

```text
lib/src/
```

Responsibilities:

- `models`: entities, snapshots, views, events, receipts, and money types.
- `repositories`: storage interfaces plus in-memory implementations.
- `services`: validation and domain mutation.
- `events`: event bus, handler, and publisher interfaces.
- `shared`: domain errors and validation helpers.

Services may use repository traits. They must not know about:

- SQF command names.
- JSON response envelopes.
- SurrealDB.
- Tokio.
- global server event-bus instances.

### Application and Extension Layer

Location:

```text
arma/crate/src/
```

Responsibilities:

- `lib.rs`: extension initialization and arma-rs command groups.
- top-level domain modules such as `actor.rs` and `bank.rs`: parse command inputs, invoke workflows, serialize results.
- `command.rs`: string-route dispatch used by chunked transport.
- `features/<domain>`: vertical application workflows.
- `events.rs`: central server event bus and cross-domain event handlers.
- `persistence`: cached repositories and SurrealDB adapter.
- `transport.rs`: request staging and response chunking.
- `log.rs`: asynchronous aggregate and domain logging.

## Command Flow

```mermaid
%%{init: {"theme":"base","themeVariables":{"background":"transparent","actorBkg":"#18181b","actorBorder":"#a57c34","actorTextColor":"#f4f4f5","signalColor":"#d6a84f","signalTextColor":"#f4f4f5","labelBoxBkgColor":"#18181b","labelBoxBorderColor":"#a57c34","labelTextColor":"#f4f4f5"}}}%%
sequenceDiagram
    participant SQF
    participant Command as Rust command module
    participant Feature as Feature workflow
    participant Service as forge-lib service
    participant Repo as Cached repository
    participant Queue as Persistence queue

    SQF->>Command: route + string arguments
    Command->>Feature: parsed input
    Feature->>Service: typed use case
    Service->>Repo: read or save
    Repo->>Queue: nonblocking enqueue
    Feature-->>Command: typed result
    Command-->>SQF: JSON or Error string
```

The normal gameplay response does not wait for SurrealDB.

## Repository Pattern

Server repository implementations wrap shared in-memory repositories:

```mermaid
flowchart LR
    Service --> Trait[Repository trait]
    Trait --> Cached[Cached repository]
    Cached --> Memory[In-memory repository]
    Cached --> Queue[Write queue]
    Queue --> Surreal[SurrealDB worker]

    classDef step fill:#18181b,stroke:#a57c34,color:#f4f4f5,stroke-width:1.5px
    classDef storage fill:#121214,stroke:#d6a84f,color:#f4f4f5,stroke-width:2px
    class Service,Trait,Cached,Memory,Queue step
    class Surreal storage
    linkStyle default stroke:#a57c34,stroke-width:1.5px
```

Special multi-record workflows use `WriteOp::Batch`, which is applied inside a SurrealDB transaction.

## Event Backbone

Feature workflows publish completed facts through `EventPublisher`.

```mermaid
flowchart LR
    Workflow --> Publisher[ServerEventPublisher]
    Publisher --> Bus[EventBus]
    Bus --> Durable[DurableEventBackend]
    Bus --> Bank[Bank lifecycle handler]
    Bus --> Garage[Garage lifecycle handler]
    Bus --> Locker[Locker lifecycle handler]

    classDef step fill:#18181b,stroke:#a57c34,color:#f4f4f5,stroke-width:1.5px
    classDef backbone fill:#2a2113,stroke:#d6a84f,color:#f4f4f5,stroke-width:2px
    class Workflow,Publisher,Durable,Bank,Garage,Locker step
    class Bus backbone
    linkStyle default stroke:#a57c34,stroke-width:1.5px
```

The event bus is synchronous in memory. Handlers queue durable work rather than waiting for database I/O.

Current event-driven cross-domain use cases include:

- `ActorCreated` provisioning downstream profiles.
- `ActorDisconnected` cleanup for bank and storage domains.
- `LockerTransferCommitted` audit persistence.
- organization lifecycle audits and notifications.

## SQF Layer

Location:

```text
arma/crate/addons/
```

Each addon owns:

- `config.cpp` and `CfgEventHandlers.hpp`.
- `XEH_PREP.hpp` compiled function registration.
- `XEH_preInit*.sqf` event/settings registration.
- `XEH_postInit*.sqf` runtime startup.
- domain functions.

SQF coordination uses CBA events. A module owns its snapshot and persistence call; another module may request that action but should not construct or mutate the first module's payload.

Example: locker close requests an actor save and only commits the locker after receiving the correlated actor result.

## Actor Cold-Start Gate

Actor initialization must not query repositories before SurrealDB hydration completes.

```mermaid
%%{init: {"theme":"base","themeVariables":{"background":"transparent","actorBkg":"#18181b","actorBorder":"#a57c34","actorTextColor":"#f4f4f5","signalColor":"#d6a84f","signalTextColor":"#f4f4f5","labelBoxBkgColor":"#18181b","labelBoxBorderColor":"#a57c34","labelTextColor":"#f4f4f5"}}}%%
sequenceDiagram
    participant Player
    participant SQF as Actor SQF
    participant Extension
    participant DB as Persistence worker

    Player->>SQF: joins mission
    SQF->>Extension: database_ready
    Extension-->>SQF: false
    DB->>DB: connect, define tables, hydrate caches
    DB->>Extension: ready=true
    SQF->>Extension: database_ready
    Extension-->>SQF: true
    SQF->>Extension: actor:init
    Extension-->>SQF: persisted actor or newly created actor
```

The SQF poll uses CBA scheduling every 250 ms and does not block the game thread.

## WebUI Boundary

The Preact UI is loaded in `CT_WEBBROWSER`. Requests cross four boundaries:

```mermaid
%%{init: {"theme":"base","themeVariables":{"background":"transparent","actorBkg":"#18181b","actorBorder":"#a57c34","actorTextColor":"#f4f4f5","signalColor":"#d6a84f","signalTextColor":"#f4f4f5","labelBoxBkgColor":"#18181b","labelBoxBorderColor":"#a57c34","labelTextColor":"#f4f4f5"}}}%%
sequenceDiagram
    participant UI as Browser
    participant Client as Client SQF
    participant Server as Server SQF
    participant Rust

    UI->>Client: A3API.SendAlert(JSON)
    Client->>Server: CBA server event
    Server->>Rust: extension command
    Rust-->>Server: JSON result
    Server-->>Client: targeted CBA event
    Client-->>UI: ExecJS forgeHostReceive(response)
```

See [WebUI and Browser Bridge](webui.md).

## Design Rules

- Keep command functions thin.
- Keep business rules in services.
- Keep Arma locality and engine operations in SQF.
- Keep workflow orchestration in feature slices.
- Keep persistence adapters out of the domain library.
- Publish events after successful mutation.
- Use transactions for multi-record money movement.
- Treat generated WebUI assets as build output.
- Keep persistent identifiers and money serialized in stable view models.
