#include "..\script_component.hpp"

/*
 * File: fnc_enemySide.sqf
 * Author: IDSolutions
 * Date: 2026-06-18
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Resolves the enemy side from the ENEMY_SIDE or ENEMY_SIDE_STR mission variables.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Resolved enemy side, or sideUnknown when unavailable [SIDE]
 *
 * Example:
 * call forge_crate_commander_fnc_enemySide;
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
