#include "..\script_component.hpp"

/*
 * File: fnc_init.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Initializes the player's bank profile, then continues the player initialization chain.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Bank profile [HASHMAP]
 *
 * Example:
 * [_uid] call forge_crate_bank_fnc_init;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { createHashMap };

private _actorConfig = missionConfigFile >> "CfgMission" >> "Actor";
private _startingCash = [_actorConfig >> "startingCash", "0.00"] call EFUNC(common,configMoney);
private _startingBank = [_actorConfig >> "startingBank", "0.00"] call EFUNC(common,configMoney);

["bank:init", [_uid, _startingCash, _startingBank]] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _bank = fromJSON _result;
if !(_bank isEqualType createHashMap) exitWith {
    ERROR_1("Bank init returned invalid payload: %1",_result);
    createHashMap
};

private _player = [_uid] call EFUNC(common,findPlayer);
if !(isNull _player) then {
    [CRPC(bank,responseInitBank), [_bank], _player] call CFUNC(targetEvent);
    [_player, true] call EFUNC(notification,deliver);
};
[SRPC(garage,initPlayer), [_uid]] call CFUNC(localEvent);

_bank
