#include "..\script_component.hpp"

/*
 * File: fnc_findEnemySide.sqf
 * Description: Determines the side of the enemy forces using missionNamespace variables.
 *              Reads ENEMY_SIDE (side) or ENEMY_SIDE_STR (string) from missionNamespace.
 *              These variables must be set by the mission or another addon before the
 *              Commander starts.
 */

private _side = GETMVAR(ENEMY_SIDE,sideUnknown);
if (_side isNotEqualTo sideUnknown) exitWith { _side };

private _sideStr = GETMVAR(ENEMY_SIDE_STR,"");
if (_sideStr isEqualTo "") exitWith { sideUnknown };

switch (toUpperANSI _sideStr) do {
    case "EAST";
    case "OPFOR": { east };
    case "GUER";
    case "INDEPENDENT";
    case "RESISTANCE": { resistance };
    default { sideUnknown };
}
