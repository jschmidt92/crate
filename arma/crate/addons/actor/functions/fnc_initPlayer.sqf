#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes an actor in the Rust extension from a live player snapshot and sends the initialized actor to the client.
 *
 * Arguments:
 * 0: [OBJECT] - Player to initialize
 *
 * Return Value:
 * Initialized actor [HASHMAP]
 *
 * Example:
 * [_player] call forge_crate_actor_fnc_initPlayer;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };

private _snapshot = [_player] call FUNC(snapshotFromPlayer);
if (_snapshot isEqualTo createHashMap) exitWith { createHashMap };

["actor:init", [toJSON _snapshot]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _actor = fromJSON _result;
if !(_actor isEqualType createHashMap) exitWith {
    ERROR_1("Actor init returned invalid payload: %1",_result);
    createHashMap
};

[CRPC(actor,responseInitActor), [_actor], _player] call CFUNC(targetEvent);
[
    getPlayerUID _player,
    _actor getOrDefault ["organization", "default"]
] call EFUNC(organization,addPlayerMember);
[SRPC(bank,initPlayer), [getPlayerUID _player]] call CFUNC(localEvent);

_actor
