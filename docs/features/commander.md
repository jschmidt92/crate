# Commander Feature

Main files:
- `arma/crate/addons/commander/config.cpp`
- `arma/crate/addons/commander/CfgCommander.hpp`
- `arma/crate/addons/commander/functions/fnc_create.sqf`
- `arma/crate/addons/commander/functions/fnc_reassign.sqf`
- `arma/crate/addons/commander/functions/fnc_objectives.sqf`
- `arma/crate/addons/commander/functions/fnc_enemySide.sqf`

## Mechanics

The Commander module is an advanced dynamic AI system that manages AI asset spawning, objective prioritization, threat assessment, and sector patrols. It implements a performance-optimized virtualization state machine to keep server CPU overhead low in multiplayer environments.

### 1. Threat Assessment & Objective Selection
- **Threat Calculation (`computeThreat`)**: Monitors players' 2D distance to objectives and player density on the server, generating a dynamic threat level between `0.0` (peaceful) and `1.0` (high threat).
- **Objective Prioritization (`updateObjectivePriority`)**: Assigns dynamic priority values to objectives based on player proximity.
- **Strategic Selection (`selectObjective`)**: Targets the highest-priority active sector to route offensive assets. Fallback positions automatically scan `allMapMarkers` starting with `"obj_"` and check `surfaceIsWater` to prevent objective placement in water.

### 2. Multi-State Virtualization Pipeline
To conserve server CPU cycles, AI groups are not immediately materialized as active units. Instead, they transition through an optimized lifecycle:
- **State A: Fully Virtualized (En Route)**: Groups exist only as lightweight data structures (HashMaps) storing type, side, size, spawn position, destination, creation time, and travel speed. Their map coordinate is mathematically interpolated over time as they "march" towards the objective via `interpolateGroupPos`.
- **State B: Leader-Only Patrol (At Sector, players > 2000m)**: When a virtual infantry or support group reaches its target sector, it spawns *only the team leader* (group size 1). The leader immediately initiates a random CBA patrol around the sector, providing organic presence with minimal performance cost.
- **State C: Fully Materialized (Players within 2000m)**: If a player enters the 2000m virtualization boundary of the group's current position (either en route or at the sector), the full squad (and support vehicles) is spawned/reinforced around the leader in formation, preserving the squad's configured size.
- **Hysteresis & Dematerialization**: If players retreat beyond 2500m, active squads are stripped back down to leader-only status (if at a sector) or dematerialized fully back to virtual data (if still en route), avoiding rapid spawn/despawn cycles at boundary margins.
- **Armor Group Exclusion**: Armor groups are excluded from leader-only spawning (always spawning crew + vehicle together when players approach) to maintain vehicle simulation integrity.

### 3. Sector Defenses and Patrols
- Active groups arriving within 200m of their target objective automatically transition into defensive patrol patterns.
- Waypoints are cleared and managed using CBA patrol tasks with configured, type-specific radii:
  - **Infantry**: 200m patrol radius.
  - **Support**: 300m patrol radius.
  - **Armor**: 400m patrol radius.

### 4. API & Lifecycle Methods
All API calls are invoked as hashMap methods on the global commander instance (`forge_commander_service`):
- **`start`**: Begins loops for threat calculations, target prioritization, spawner management, virtualization, and group maintenance.
- **`stop`**: Pauses all loops. Spawned AI groups remain alive on the map but the commander stops managing them or spawning new ones.
- **`destroy`**: Pauses loops, deletes all managed groups and vehicles safely from the simulation, and resets the commander's state.
