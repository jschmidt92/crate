#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes the player's owned locker, then continues the player initialization chain.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 * Return Value:
 * Locker profile [HASHMAP]
 *
 * Example:
 * [_uid] call forge_crate_locker_fnc_initPlayer;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { createHashMap };

["locker:init", [_uid]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _locker = fromJSON _result;
if !(_locker isEqualType createHashMap) exitWith {
    ERROR_1("Locker init returned invalid payload: %1",_result);
    createHashMap
};

private _player = [_uid] call EFUNC(common,getPlayerByUID);
if !(isNull _player) then {
    [CRPC(locker,responseInitLocker), [_locker], _player] call CFUNC(targetEvent);
};
[SRPC(v_locker,initPlayer), [_uid]] call CFUNC(localEvent);

_locker
