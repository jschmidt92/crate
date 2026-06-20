# Commander Module — Complete Walkthrough

## Overview

Four implementation cycles were performed on the Commander module, resulting in code improvements, configuration alignment, module fixes, performance virtualization, sector patrolling, and finally leader-only patrol virtualization.

---

## Round 1 — Code Review (fnc_create.sqf)

13 issues fixed in [fnc_create.sqf](file:///g:/forge/arma/crate/addons/commander/functions/fnc_create.sqf):
- **Critical:** Config loading pipeline fixed (was silently discarding mission overrides)
- **Config format:** Converted [CfgCommander.hpp](file:///g:/forge/arma/crate/addons/commander/CfgCommander.hpp) and mission config to array-of-arrays format
- **Logic:** Fixed threat distance measurement, selectObjective type safety, spawn slot enforcement, setPosASL→setPosATL, hardcoded BLUFOR class, group leak
- **DRY:** Extracted `ensureObjectives` method, unified player filtering, forEach-based faction resolution, single source of truth for config defaults

---

## Round 2 — Module Review (all files)

14 additional issues fixed across 5 files:

### [XEH_postInitServer.sqf](file:///g:/forge/arma/crate/addons/commander/XEH_postInitServer.sqf) — Fix #1

Added `!isNil QGVAR(service)` guard to the `waitUntilAndExecute` condition. Previously, if `create` failed, the `start` callback would crash on a nil reference.

```diff
 [{
-    (GETMVAR(EGVAR(task,...),objNull)) isNotEqualTo objNull
+    !isNil QGVAR(service) && { (GETMVAR(EGVAR(task,...),objNull)) isNotEqualTo objNull }
 }, {
```

---

### [fnc_reassign.sqf](file:///g:/forge/arma/crate/addons/commander/functions/fnc_reassign.sqf) — Fixes #3, #4

- Flattened to guard clause (`continue` instead of wrapping body in `if/then`)
- Added combat behavior: `AWARE`, `NORMAL` speed, `RED` combat mode — groups now engage hostiles while moving to objectives

---

### [fnc_enemySide.sqf](file:///g:/forge/arma/crate/addons/commander/functions/fnc_enemySide.sqf) — Fixes #5, #6

- Added missing side aliases: `"OPFOR"` → `east`, `"INDEPENDENT"` / `"RESISTANCE"` → `resistance`
- Default now returns `sideUnknown` instead of silently returning `resistance` for invalid inputs
- Documented the external dependency on `ENEMY_SIDE` / `ENEMY_SIDE_STR` / `ENEMY_FACTION_STR` missionNamespace variables

---

### [fnc_objectives.sqf](file:///g:/forge/arma/crate/addons/commander/functions/fnc_objectives.sqf) — Fixes #7, #8, #13

- Now scans `allMapMarkers` for markers prefixed with `"obj_"` if no well-known names match
- Fallback positions checked with `surfaceIsWater` to avoid placing objectives in the ocean
- Added a 4th fallback candidate (map center) and absolute last-resort if all positions are water
- Fixed `!=` → `isNotEqualTo` for style consistency

---

### [fnc_create.sqf](file:///g:/forge/arma/crate/addons/commander/functions/fnc_create.sqf) — Fixes #9, #10, #11, #12, #14

**New `destroy` method** (#11) — Full cleanup of spawned groups:
```sqf
["destroy", {
    // Stops loops, deletes all units + vehicles, clears state
}]
```
`stop` pauses the Commander (groups remain alive). `destroy` fully cleans up.

**`spawnSupportGroup` rewrite** (#9, _#12):
- Support vehicle now properly linked via `_group addVehicle`
- Removed hardcoded `"B_UAV_01_F"` BLUFOR fallback — spawns infantry-only if no `supportClass` configured
- Added combat behavior (`AWARE`, `NORMAL`, `RED`)

**`spawnArmorGroup`** (#10):
- Crew now spawned at `_spawnPos` instead of `[0,0,0]` — prevents stranded units if `moveInAny` fails
- Added combat behavior (`AWARE`, `RED`)

**`spawnInfantryGroup`**:
- Added combat behavior (`AWARE`, `NORMAL`, `RED`) for consistency

**`updateObjectivePriority`** (#14):
- Normalization distance unified from `/6000` to `/5000` to match `computeThreat`

---

## Round 3 — Group Virtualization & Sector Patrol Behavior

Implemented virtualization to run groups as lightweight data structures until a player gets within 2000m, and added CBA patrol behavior once they arrive at their objectives.

- **Virtual Group Structure:** Saved as lightweight HashMaps containing type, side, spawnPos, sectorPos, crew size, createdAt, and travel speed.
- **Position Interpolation:** Added `interpolateGroupPos` which calculates the current travel position on the map using elapsed time and speed. Groups materialize at this interpolated position rather than teleporting.
- **Hysteresis Buffer:** Materializes when any player is within 2000m; dematerializes back to data when all players move beyond 2500m (prevents rapid spawn/despawn cycles).
- **Proximity Loop:** A 3-second cycle checks player distances to virtual positions (materializing) and active units (dematerializing).
- **Limit Integrity:** Counts both active and virtual groups towards the Commander's max infantry, armor, and support group caps to ensure spawn limits are always respected.
- **Reassignment Support:** When objectives are redirected, virtual groups automatically update their destinations and begin moving from their current interpolated coordinates rather than resetting.
- **Sector Patrols:** A 5-second maintenance loop checks if active groups are within 200m of their target sector. Clears existing waypoints and registers `CBA_fnc_taskPatrol` with radii: Infantry (200m), Support (300m), Armor (400m).

---

## Round 4 — Leader-Only Patrol Virtualization

Implemented leader-only patrol behavior for infantry and support groups when they reach their destination while players are still far away.

- **Leader-Only Spawn**: When a virtual group reaches its target sector and no players are within 2000m, it spawns *only the team leader* (group size 1). The leader immediately begins executing the CBA patrol task around the objective.
- **Full Materialization**: As soon as a player gets within 2000m of the active leader, the rest of the squad (and support vehicles, if applicable) is spawned in formation around the leader to reinforce the group.
- **Dynamic Dematerialization**: If players retreat beyond 2500m of a group at the sector, the squad is deleted (reclaiming server CPU resources) and the group is returned to leader-only status with the leader continuing their patrol.
- **Original Size Preservation**: The group remembers its original size during all leader-only and full materialization transitions, ensuring squad sizes are never degraded.
- **Armor Group Exclusion**: Armor groups are excluded from leader-only spawning (always spawning crew + vehicle together when players approach) to maintain vehicle simulation integrity.

---

## Verification

- `hemtt build` passes cleanly with **0 warnings, 0 errors**.
