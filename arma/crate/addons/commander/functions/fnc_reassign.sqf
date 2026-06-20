#include "..\script_component.hpp"

/*
 * File: fnc_reassign.sqf
 * Author: IDSolutions
 * Date: 2026-06-18
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Reassigns active and virtual Commander groups to a target sector and updates combat behavior.
 *
 * Arguments:
 * 0: [HASHMAP OBJECT] - Commander service object
 * 1: [ARRAY] - Target sector position
 *
 * Return Value:
 * None
 *
 * Example:
 * [_commander, _sectorPosition] call forge_crate_commander_fnc_reassign;
 */

params [["_self", createHashMap, [createHashMap]], ["_sectorPos", [0,0,0], [[]]]];

private _groups = _self getOrDefault ["activeGroups", []];
{
    if (isNull _x || { ({ alive _x } count units _x) isEqualTo 0 }) then { continue };

    _x setBehaviourStrong "AWARE";
    _x setSpeedMode "NORMAL";
    _x setCombatMode "RED";
    _x move _sectorPos;
    _x setVariable [QGVAR(sectorTarget), _sectorPos];
    _x setVariable [QGVAR(patrolAssigned), false];
} forEach _groups;

private _virtualGroups = _self getOrDefault ["virtualGroups", []];
{
    private _currentPos = _self call ["interpolateGroupPos", [_x]];
    _x set ["spawnPos", _currentPos];
    _x set ["sectorPos", _sectorPos];
    _x set ["createdAt", time];
} forEach _virtualGroups;
