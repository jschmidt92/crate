#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes the player's virtual garage unlocks, then continues the player initialization chain.
 *
 * Arguments:
 * 0: [OBJECT] - Player to initialize
 * Return Value:
 * Virtual garage profile [HASHMAP]
 *
 * Example:
 * [_player] call forge_server_v_garage_fnc_initPlayer;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };

private _garageConfig = missionConfigFile >> "CfgForgeMission" >> "Actor" >> "VirtualGarage";
private _unlocks = createHashMapFromArray [
    ["cars", getArray (_garageConfig >> "cars")],
    ["armor", getArray (_garageConfig >> "armor")],
    ["helis", getArray (_garageConfig >> "helis")],
    ["planes", getArray (_garageConfig >> "planes")],
    ["naval", getArray (_garageConfig >> "naval")],
    ["other", getArray (_garageConfig >> "other")]
];

["v_garage:init", [getPlayerUID _player, toJSON _unlocks]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _virtualGarage = fromJSON _result;
if !(_virtualGarage isEqualType createHashMap) exitWith {
    ERROR_1("Virtual garage init returned invalid payload: %1",_result);
    createHashMap
};

[CRPC(v_garage,responseInitVGarage), [_virtualGarage], _player] call CFUNC(targetEvent);
[SRPC(locker,initPlayer), [_player]] call CFUNC(localEvent);

_virtualGarage
