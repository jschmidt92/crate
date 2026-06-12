#include "..\script_component.hpp"

/*
 * File: fnc_disconnectPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Persists actor disconnect state while the player object is still available, then starts the module disconnect chain.
 *
 * Arguments:
 * 0: [OBJECT] - Disconnecting player
 * 1: [STRING] - Player UID fallback
 *
 * Return Value:
 * Actor disconnect payload [HASHMAP]
 *
 * Example:
 * [_player, _uid] call forge_server_actor_fnc_disconnectPlayer;
 */

params [["_player", objNull, [objNull]], ["_uid", "", [""]]];

if (_uid isEqualTo "" && {!isNull _player}) then {
    _uid = getPlayerUID _player;
};
if (_uid isEqualTo "") exitWith { createHashMap };

private _result = "";
private _success = false;
if !(isNull _player) then {
    private _snapshot = [_player] call FUNC(snapshotFromPlayer);
    if (_snapshot isNotEqualTo createHashMap) then {
        ["actor:disconnect", [toJSON _snapshot]] call EFUNC(extension,extCall) params ["_disconnectResult", "_disconnectSuccess"];
        _result = _disconnectResult;
        _success = _disconnectSuccess;
    };
};

if !(_success) then {
    ["actor:disconnect_uid", [_uid]] call EFUNC(extension,extCall) params ["_disconnectResult", "_disconnectSuccess"];
    _result = _disconnectResult;
    _success = _disconnectSuccess;
};

if !(_success) exitWith { createHashMap };

[SRPC(bank,disconnectPlayer), [_uid]] call CFUNC(localEvent);

if (_result isEqualTo "OK") exitWith { createHashMap };

private _actor = fromJSON _result;
if (_actor isEqualType createHashMap) exitWith { _actor };

createHashMap
