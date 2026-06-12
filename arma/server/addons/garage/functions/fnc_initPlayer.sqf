#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes the player's owned garage, then continues the player initialization chain.
 *
 * Arguments:
 * 0: [OBJECT] - Player to initialize
 * Return Value:
 * Garage profile [HASHMAP]
 *
 * Example:
 * [_player] call forge_server_garage_fnc_initPlayer;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };

["garage:init", [getPlayerUID _player]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _garage = fromJSON _result;
if !(_garage isEqualType createHashMap) exitWith {
    ERROR_1("Garage init returned invalid payload: %1",_result);
    createHashMap
};

[SRPC(v_garage,initPlayer), [_player]] call CFUNC(localEvent);

_garage
