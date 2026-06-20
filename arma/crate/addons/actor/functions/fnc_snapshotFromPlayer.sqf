#include "..\script_component.hpp"

/*
 * File: fnc_snapshotFromPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Builds an actor snapshot from a live Arma player object.
 *
 * Arguments:
 * 0: [OBJECT] - Player to snapshot
 *
 * Return Value:
 * Actor snapshot [HASHMAP]
 *
 * Example:
 * [_player] call forge_crate_actor_fnc_snapshotFromPlayer;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };

private _actorConfig = missionConfigFile >> "CfgMission" >> "Actor";
private _loadout = getUnitLoadout _player;
private _configuredLoadout = getArray (_actorConfig >> "loadout");
if (_configuredLoadout isNotEqualTo []) then {
    _loadout = _configuredLoadout;
};

createHashMapFromArray [
    ["uid", getPlayerUID _player],
    ["name", name _player],
    ["loadout", _loadout],
    ["position", getPosASL _player],
    ["direction", getDir _player],
    ["stance", stance _player],
    ["rank", rank _player],
    ["life_state", lifeState _player],
    ["organization", "default"],
    ["holster", true]
]
