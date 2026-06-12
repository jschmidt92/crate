#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes the player's bank profile, then continues the player initialization chain.
 *
 * Arguments:
 * 0: [OBJECT] - Player to initialize
 *
 * Return Value:
 * Bank profile [HASHMAP]
 *
 * Example:
 * [_player] call forge_server_bank_fnc_initPlayer;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };

private _actorConfig = missionConfigFile >> "CfgForgeMission" >> "Actor";
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

["bank:init", [getPlayerUID _player, _startingCash, _startingBank]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _bank = fromJSON _result;
if !(_bank isEqualType createHashMap) exitWith {
    ERROR_1("Bank init returned invalid payload: %1",_result);
    createHashMap
};

[SRPC(garage,initPlayer), [_player]] call CFUNC(localEvent);

_bank
