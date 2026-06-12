#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes the player's virtual locker unlocks and completes the player initialization chain.
 *
 * Arguments:
 * 0: [OBJECT] - Player to initialize
 * Return Value:
 * Virtual locker profile [HASHMAP]
 *
 * Example:
 * [_player] call forge_server_v_locker_fnc_initPlayer;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };

private _lockerConfig = missionConfigFile >> "CfgForgeMission" >> "Actor" >> "Locker";
private _unlocks = createHashMapFromArray [
    ["items", getArray (_lockerConfig >> "items")],
    ["weapons", getArray (_lockerConfig >> "weapons")],
    ["magazines", getArray (_lockerConfig >> "magazines")],
    ["backpacks", getArray (_lockerConfig >> "backpacks")]
];

["v_locker:init", [getPlayerUID _player, toJSON _unlocks]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _virtualLocker = fromJSON _result;
if !(_virtualLocker isEqualType createHashMap) exitWith {
    ERROR_1("Virtual locker init returned invalid payload: %1",_result);
    createHashMap
};

_virtualLocker
