# Garage and Locker Feature

Main files:
- `lib/src/models/garage.rs`
- `lib/src/models/locker.rs`
- `lib/src/models/v_garage.rs`
- `lib/src/models/v_locker.rs`
- `lib/src/services/garage.rs`
- `lib/src/services/locker.rs`
- `lib/src/services/v_garage.rs`
- `lib/src/services/v_locker.rs`
- `arma/crate/src/garage.rs`
- `arma/crate/src/locker.rs`
- `arma/crate/src/v_garage.rs`
- `arma/crate/src/v_locker.rs`
- `arma/crate/src/features/garage/*`
- `arma/crate/src/features/locker/*`
- `arma/crate/src/features/v_garage/*`
- `arma/crate/src/features/v_locker/*`

## Mechanics

### Physical Locker Access & Networked Proxy
Physical locker access points are Eden-placed containers or objects with variable names `locker`, `locker_1`, `locker_2`, and so on. The locker addon discovers names through `locker_999` and adds an `Open Locker` action to each valid object. The placed object is only an access terminal, so the server clears and locks its ordinary shared cargo. 

Persisted player cargo is materialized in a hidden, server-created networked inventory proxy unique to that request, captured when the inventory closes, and saved through `locker:save` before the server deletes the proxy. Networked proxies are required because Arma does not support backpacks inside local-only containers during multiplayer. Multiple players can still use the same terminal concurrently because each request receives a separate proxy.

### Actor Synchronization Guard & Fail-Closed Transaction
Closing a locker publishes a correlated CBA actor-save request. The actor addon captures and persists its own post-transfer loadout through `actor:save`; only its success response allows the locker addon to normalize and persist its own proxy through `locker:commit`. This fail-closed ordering prevents the locker from accepting deposited equipment while the persisted actor still owns it. 

If actor persistence fails, the proxy remains available but is not reopened automatically. The player must explicitly use `Open Locker` again to retry, preventing recursive inventory-close loops. On disconnect, the actor addon's successful disconnect save emits a server-local event that allows locker persistence; failed actor persistence discards the temporary proxy without updating the locker.

### Locker Equipment Normalization
Locker persistence normalizes equipment into a classname-keyed commodity map:
- Weapon attachments are detached.
- Loaded primary and secondary muzzle magazines become magazine entries.
- Nested container contents are recursively flattened. Uniforms, vests, and backpacks therefore return empty, with their former contents available as loose locker cargo.
- Every commodity record includes an `ammo` field, which is zero for non-magazine entries. Magazine entries retain both object quantity and aggregate remaining ammunition; restoration redistributes those rounds into full magazines followed by a partial magazine.

### Virtual Garage & Locker Controls
The Virtual Garage and Virtual Locker addons each expose two globally synchronized CBA settings, all enabled by default:
- **Feature Setting**: Controls whether the virtual garage or arsenal is available at all.
- **Persistence Setting**: Independently controls whether player unlocks are loaded and saved through Rust; when persistence is disabled, mission defaults and organization unlocks remain available for the current session.

A disabled virtual module skips its profile and client snapshot without interrupting the physical garage/locker initialization chain.

### Virtual Locker (ACE Arsenal Integration)
Virtual Locker uses ACE Arsenal and adds `Open Virtual Arsenal` to the same Eden `locker*` terminals as physical storage. The client rebuilds a hidden local ACE Arsenal box whenever its merged unlock snapshot arrives. 

Closing this Forge-owned arsenal publishes the existing correlated actor-save request so the actor addon persists the resulting loadout through its own domain workflow. Opening the arsenal does not modify or save unlock records; unlock persistence remains owned by `v_locker` workflows.

## Code Organization
These server workflows use the same slice names:
- `lifecycle.rs`: Initialize and delete records.
- `query.rs`: Get records by player UID.
- `storage.rs`: Save full records.

Disconnect cleanup is internal to the `ActorDisconnected` event handlers and is not exposed as feature commands.

## Current Commands
- `garage:*`
- `locker:*`
- `v_garage:*`
- `v_locker:*`
