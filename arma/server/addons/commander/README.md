# Commander Addon (`forge_server_commander`)

An advanced, object-oriented, performance-optimized dynamic AI commander system for multiplayer servers in Arma 3. The Commander service dynamically manages AI asset spawning, objective prioritization, threat assessment, and sector patrols, while utilizing an aggressive virtualization state machine to minimize server CPU overhead.

---

## Features

### 1. Dynamic Threat Assessment & Objective Selection

- **Threat Calculation (`computeThreat`)**: Monitors players' 2D distance to objectives and player density on the server. Produces a threat level between `0.0` (peaceful) and `1.0` (high combat threat).
- **Objective Prioritization (`updateObjectivePriority`)**: Assigns priority scores to objectives based on player proximity.
- **Strategic Selection (`selectObjective`)**: Targets the highest-priority sector to direct offensive assets.

### 2. Performance-Optimized Virtualization

To conserve server CPU cycles, AI groups are not immediately materialized as active units. Instead, they transition through a multi-state virtualization pipeline:

- **State A: Fully Virtualized (En Route)**: Groups exist only as lightweight data structures (HashMaps) storing type, side, size, spawn position, destination, creation time, and travel speed. Their map coordinate is mathematically interpolated over time as they "march" towards the objective.
- **State B: Leader-Only Patrol (At Sector, players > 2000m)**: When a virtual infantry or support group reaches its target sector, it materializes *only* the team leader (group size 1). The leader immediately clears all move waypoints and initiates a random CBA patrol around the sector, providing organic presence with minimal performance cost.
- **State C: Fully Materialized (Players within 2000m)**: If a player enters the 2000m virtualization boundary of the group's current position (either en route or at the sector), the full squad (and support vehicles) is spawned/reinforced around the leader in formation.
- **Hysteresis & Dematerialization**: If players retreat beyond 2500m, active squads are stripped back down to leader-only status (if at a sector) or dematerialized fully back to virtual data (if still en route), avoiding rapid spawn/despawn cycles at boundary margins.

### 3. Sector Defenses and Patrols

- Active groups arriving within 200m of their target objective automatically transition into defensive patrol patterns.
- Waypoints are managed using CBA patrol tasks with configured, type-specific radii:
  - **Infantry**: 200m patrol radius.
  - **Support**: 300m patrol radius.
  - **Armor**: 400m patrol radius.

---

## Configuration (`CfgCommander`)

The Commander is configured via `CfgCommander` defined in the addon's config or overridden inside the `missionConfigFile`.

```cpp
class CfgCommander {
    defaults[] = {
        {"reassignCadence", 20},                 // Time (s) between objective reassignments
        {"spawnConsiderCadence", 45},            // Time (s) between spawning considerations
        {"threatRecalcCadence", 10},             // Time (s) between threat updates
        {"maxInfantryGroups", 6},                // Limit for concurrent infantry groups
        {"maxArmorGroups", 2},                   // Limit for concurrent armor groups
        {"maxSupportGroups", 2},                 // Limit for concurrent support groups
        {"objectiveRecalcDistance", 2000},       // Distance to trigger objective recalculation
        {"objectiveMarkerFallbackRadius", 1500}, // Radius for locating fallback objectives
        {"spawnMinDistance", 800},               // Min distance from sector to spawn assets
        {"spawnMaxDistance", 2200},              // Max distance from sector to spawn assets
        {"minReassignInterval", 60},             // Minimum time (s) before a group can be redirected
        {"infantryGroupSize", 8},                // Size of infantry squads
        {"armorCrewGroupSize", 4},               // Size of armor crews
        {"supportGroupSize", 6},                 // Size of support teams
        {"highThreatThreshold", 0.65},           // Threat level to spawn Armor instead of Infantry
        {"medThreatThreshold", 0.35},            // Threat level to spawn Support/Infantry mix
        {"virtualizationDistance", 2000},        // Distance (m) to materialize units near players
        {"virtualizationHysteresis", 500},       // Buffer distance (m) before dematerialization
        {"virtualizationCheckCadence", 3},       // Proximity check loop cadence (s)
        {"patrolRadiusInfantry", 200},           // Patrol radius (m) for Infantry groups
        {"patrolRadiusArmor", 400},              // Patrol radius (m) for Armor vehicles
        {"patrolRadiusSupport", 300},            // Patrol radius (m) for Support groups
        {"arrivalDistance", 200},                // Distance (m) to sector that triggers patrol mode
        {"infantryTravelSpeed", 8},              // Movement speed (m/s) for virtual infantry
        {"vehicleTravelSpeed", 20}               // Movement speed (m/s) for virtual armor
    };
};
```

---

## External Dependencies

- **CBA (Community Base Addons)**: 
  - Scheduled loop management (`CBA_fnc_waitAndExecute`, `CBA_fnc_waitUntilAndExecute`).
  - Patrol waypoints (`CBA_fnc_taskPatrol`).
- **Faction Unit Mapper (`CfgFactionUnitMap`)**:
  - Resolves infantry, armor, and support classnames using `configFile >> "CfgFactionUnitMap" >> ENEMY_FACTION_STR` variables set in the mission namespace.
- **Mission Setup (`forge_server_task`)**:
  - Starts the service using the event hook `GETMVAR(EGVAR(task,missionSetup_settingsApplied))` to ensure configuration tables are fully resolved before spawning.

---

## API & Initialization

### Initialization

The Commander is created during server `postInit` using:

```sqf
private _overrides = createHashMapFromArray (getArray (missionConfigFile >> "CfgMission" >> "Commander"));
[_overrides] call forge_commander_fnc_createCommander;
```

### Lifecycle Methods

All API calls are invoked as hashMap methods on the global commander instance (`forge_commander_service`):

- **`start`**: Begins loops for threat calculations, target prioritization, spawner management, virtualization, and group maintenance.
  
  ```sqf
  forge_commander_service call ["start", []];
  ```

- **`stop`**: Pauses all loops. Spawned AI groups remain alive on the map but the commander stops managing them or spawning new ones.
  
  ```sqf
  forge_commander_service call ["stop", []];
  ```

- **`destroy`**: Pauses loops, deletes all managed groups and vehicles safely from the simulation, and resets the commander's state.
  
  ```sqf
  forge_commander_service call ["destroy", []];
  ```
