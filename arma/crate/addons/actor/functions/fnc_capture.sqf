#include "..\script_component.hpp"

/*
 * File: fnc_capture.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Builds an actor snapshot from a live Arma player object.
 *
 * Arguments:
 * 0: [OBJECT] - Player to snapshot
 * 1: [BOOL] - Use the mission-configured loadout instead of the live loadout (default: false)
 *
 * Return Value:
 * Actor snapshot [HASHMAP]
 *
 * Example:
 * [_player, false] call forge_crate_actor_fnc_capture;
 */

params [
    ["_player", objNull, [objNull]],
    ["_useConfiguredLoadout", false, [false]]
];

if (isNull _player) exitWith { createHashMap };

private _actorConfig = missionConfigFile >> "CfgMission" >> "Actor";
private _loadout = getUnitLoadout _player;
if (_useConfiguredLoadout || { !GVAR(persistLoadout) }) then {
    _loadout = getArray (_actorConfig >> "loadout");
};

createHashMapFromArray [
    ["uid", getPlayerUID _player],
    ["name", name _player],
    ["loadout", _loadout],
    ["persist_loadout", GVAR(persistLoadout)],
    ["position", getPosASL _player],
    ["persist_position", GVAR(persistPosition)],
    ["direction", getDir _player],
    ["stance", stance _player],
    ["rank", rank _player],
    ["life_state", lifeState _player],
    ["organization", "default"],
    ["holster", true]
]
