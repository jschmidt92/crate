#include "..\script_component.hpp"

/*
 * File: fnc_reassignGroups.sqf
 * Description: Moves active enemy groups to direct them towards the primary target objective.
 *              Sets combat behavior so groups engage hostiles en route.
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
