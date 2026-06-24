# Actor Feature

## Ownership

Main files:

- `lib/src/models/actor.rs`
- `lib/src/services/actor.rs`
- `arma/crate/src/actor.rs`
- `arma/crate/src/features/actor/`
- `arma/crate/addons/actor/`

## Initialization

```mermaid
%%{init: {"theme":"base","themeVariables":{"background":"transparent","actorBkg":"#18181b","actorBorder":"#a57c34","actorTextColor":"#f4f4f5","signalColor":"#d6a84f","signalTextColor":"#f4f4f5","labelBoxBkgColor":"#18181b","labelBoxBorderColor":"#a57c34","labelTextColor":"#f4f4f5"}}}%%
sequenceDiagram
    participant Server as Actor server SQF
    participant Persistence
    participant Rust as Actor feature
    participant Client as Actor client SQF

    Server->>Persistence: database_ready
    Persistence-->>Server: true after hydration
    Server->>Rust: actor:init(default snapshot)
    Rust-->>Server: actor + created flag
    Server-->>Client: responseInitActor
    Client->>Client: apply state
```

The readiness gate is critical on cold server starts. Initialization waits until existing actors have been hydrated into the cache.

New actor:

- mission default loadout is captured into the creation snapshot.
- Rust creates and saves the actor.
- `ActorCreated` is published.
- client SQF strips the unit and applies the configured loadout.

Existing actor:

- the persisted actor is authoritative.
- the temporary spawn snapshot does not overwrite it.
- loadout, position, direction, rank, and stance are restored according to settings.

## Client Lifecycle

```mermaid
flowchart TD
    Initialized["Client initialized"] --> Uninitialized["UNINITIALIZED"]
    Uninitialized --> InitRequested["Initialization requested"]
    InitRequested --> Loading["LOADING"]
    Loading --> ActorReceived["Actor state received"]
    ActorReceived --> Applying["APPLYING"]
    Applying --> Applied["State applied"]
    Applied --> Ready["READY"]
    Loading --> RequestFailed["Initialization request failed"]
    Applying --> ApplyFailed["Actor state apply failed"]
    RequestFailed --> Failed["FAILED"]
    ApplyFailed --> Failed
    Failed -.-> Retry["Retry initialization"]
    Retry -.-> Loading

    classDef step fill:#18181b,stroke:#a57c34,color:#f4f4f5,stroke-width:1.5px
    classDef success fill:#1c1917,stroke:#d6a84f,color:#f4f4f5,stroke-width:2px
    classDef failure fill:#1f1515,stroke:#b91c1c,color:#f4f4f5,stroke-width:2px
    class Initialized,Uninitialized,InitRequested,Loading,ActorReceived,Applying,Applied,Retry step
    class Ready success
    class RequestFailed,ApplyFailed,Failed failure
    linkStyle default stroke:#a57c34,stroke-width:1.5px
```

The lifecycle hashmap tracks synchronization only. Actor data remains in Rust repositories.

## Position Safety

After ASL restore:

- if the actor is more than five meters above local ATL ground.
- and vertical velocity is negative.
- velocity is cleared.
- the actor is moved to one meter ATL.

This prevents a restored airborne position from immediately causing fall damage.

## Persistence Settings

- `forge_crate_actor_persistPosition`
- `forge_crate_actor_persistLoadout`

Both default to enabled.

With loadout persistence disabled, the mission default loadout is applied on every initialization and live snapshots do not replace the persisted loadout.

## Save and Disconnect

`actor:save` persists a live snapshot without publishing disconnect.

`actor:disconnect`:

1. captures the live snapshot.
2. preserves fields disabled by CBA persistence settings.
3. saves the actor.
4. publishes `ActorDisconnected`.
5. lets bank and storage handlers clean up through the Rust event bus.

## Commands

- `actor:init`
- `actor:save`
- `actor:disconnect`
- `actor:get`
- `actor:delete`
