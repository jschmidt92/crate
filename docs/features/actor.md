# Actor Feature

Main files:
- `lib/src/models/actor.rs`
- `lib/src/services/actor.rs`
- `arma/crate/src/actor.rs`
- `arma/crate/src/features/actor/*`

## Mechanics

### Initialization
Actor initialization accepts an `ActorSnapshot` and returns the actor data plus a `created` flag. 
- If the actor is new, `ActorService::init_or_create` persists the mission-configured default loadout and publishes an `ActorCreated` domain event. SQF strips the local player before applying this default loadout.
- If the actor already exists, initialization treats the persisted actor as authoritative and does not save the temporary spawn snapshot over it. Client SQF restores the persisted loadout, ASL position, direction, rank, and stance.

### Position Restoration Protection
If position restoration leaves the player more than five meters above local ground while descending, SQF clears their velocity and moves them to a safe one-meter ATL position to prevent fall damage. Life-state metadata remains available to respawn and medical workflows rather than forcing a returning player dead or inventing an injury severity.

### Disconnect
Actor disconnect persists the live player snapshot and publishes `ActorDisconnected`. Bank, garage, locker, and virtual-storage cleanup then fan out through the central event bus.

### Client Synchronization State Machine
Client synchronization uses a small `createHashMap` state machine rather than a second repository. 
Its active path is `UNINITIALIZED` $\rightarrow$ `LOADING` $\rightarrow$ `APPLYING` $\rightarrow$ `READY`, with `FAILED` as the error state and retry source. All transitions go through the actor `transition` function, which rejects invalid changes, publishes `forge_crate_actor_lifecycleStateChanged`, and updates the player readiness variables. Server-side initialization guards prevent duplicate requests from racing multiple actor responses.

### CBA Settings Control
Actor CBA settings control position and loadout persistence independently. Both default to enabled and are globally synchronized. Disabling either setting stops restore and prevents disconnect snapshots from replacing the last persisted value. With loadout persistence disabled, the configured mission loadout is applied on every login.

## Code Organization (Vertical Slices)
Actor server workflows are organized as vertical slices:
- `init.rs`: Initialize or create actor.
- `lifecycle.rs`: Disconnect and delete actor.
- `query.rs`: Get actor by UID.

## Current Commands
- `actor:init`
- `actor:disconnect`
- `actor:get`
- `actor:delete`
