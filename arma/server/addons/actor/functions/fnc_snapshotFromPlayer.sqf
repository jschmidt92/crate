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
 * [_player] call forge_server_actor_fnc_snapshotFromPlayer;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };

private _actorConfig = missionConfigFile >> "CfgForgeMission" >> "Actor";
private _loadout = getUnitLoadout _player;
private _configuredLoadout = getArray (_actorConfig >> "loadout");
if (_configuredLoadout isNotEqualTo []) then {
    _loadout = _configuredLoadout;
};

private _organization = "default";
if (isText (_actorConfig >> "organization")) then {
    _organization = getText (_actorConfig >> "organization");
};
if (_organization isEqualTo "") then {
    _organization = "default";
};

private _startingCash = "0.00";
if (isText (_actorConfig >> "startingCash")) then {
    _startingCash = getText (_actorConfig >> "startingCash");
};
if (isNumber (_actorConfig >> "startingCash")) then {
    _startingCash = str getNumber (_actorConfig >> "startingCash");
};

private _startingBank = "0.00";
if (isText (_actorConfig >> "startingBank")) then {
    _startingBank = getText (_actorConfig >> "startingBank");
};
if (isNumber (_actorConfig >> "startingBank")) then {
    _startingBank = str getNumber (_actorConfig >> "startingBank");
};

private _garageConfig = _actorConfig >> "VirtualGarage";
private _virtualGarage = createHashMapFromArray [
    ["cars", getArray (_garageConfig >> "cars")],
    ["armor", getArray (_garageConfig >> "armor")],
    ["helis", getArray (_garageConfig >> "helis")],
    ["planes", getArray (_garageConfig >> "planes")],
    ["naval", getArray (_garageConfig >> "naval")],
    ["other", getArray (_garageConfig >> "other")]
];

private _lockerConfig = _actorConfig >> "Locker";
private _virtualArsenal = createHashMapFromArray [
    ["items", getArray (_lockerConfig >> "items")],
    ["weapons", getArray (_lockerConfig >> "weapons")],
    ["magazines", getArray (_lockerConfig >> "magazines")],
    ["backpacks", getArray (_lockerConfig >> "backpacks")]
];

private _starting = createHashMapFromArray [
    ["cash", _startingCash],
    ["bank", _startingBank],
    ["virtual_arsenal", _virtualArsenal],
    ["virtual_garage", _virtualGarage]
];

createHashMapFromArray [
    ["uid", getPlayerUID _player],
    ["name", name _player],
    ["loadout", _loadout],
    ["position", getPosASL _player],
    ["direction", getDir _player],
    ["stance", stance _player],
    ["rank", rank _player],
    ["life_state", lifeState _player],
    ["organization", _organization],
    ["holster", true],
    ["starting", _starting]
]
