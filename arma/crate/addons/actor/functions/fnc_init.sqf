#include "..\script_component.hpp"

/*
 * File: fnc_init.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-20
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
 * [_player] call forge_crate_actor_fnc_init;
 */

params [["_player", objNull, [objNull]]];

if (isNull _player) exitWith { createHashMap };
if (GETVAR(_player,GVAR(initialized),false)) exitWith { createHashMap };
if (GETVAR(_player,GVAR(initInProgress),false)) exitWith { createHashMap };

SETVAR(_player,GVAR(initInProgress),true);
[CRPC(actor,beginInitActor), [], _player] call CFUNC(targetEvent);

private _fail = {
    params [["_reason", "actor initialization failed", [""]]];
    SETVAR(_player,GVAR(initInProgress),false);
    [CRPC(actor,failedInitActor), [_reason], _player] call CFUNC(targetEvent);
};

private _snapshot = [_player, true] call FUNC(capture);
if (_snapshot isEqualTo createHashMap) exitWith {
    ["failed to build actor snapshot"] call _fail;
    createHashMap
};

["actor:init", [toJSON _snapshot]] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith {
    [_result] call _fail;
    createHashMap
};

private _initResult = fromJSON _result;
if !(_initResult isEqualType createHashMap) exitWith {
    ERROR_1("Actor init returned invalid payload: %1",_result);
    ["actor init returned an invalid payload"] call _fail;
    createHashMap
};

private _actor = _initResult getOrDefault ["actor", createHashMap];
private _created = _initResult getOrDefault ["created", false];
if (_actor isEqualTo createHashMap) exitWith {
    ERROR_1("Actor init returned no actor: %1",_result);
    ["actor init returned no actor"] call _fail;
    createHashMap
};

SETVAR(_player,GVAR(initInProgress),false);
SETVAR(_player,GVAR(initialized),true);
[CRPC(actor,responseInitActor), [_actor, _created], _player] call CFUNC(targetEvent);
[
    getPlayerUID _player,
    _actor getOrDefault ["organization", "default"]
] call EFUNC(organization,addMember);
[SRPC(bank,initPlayer), [getPlayerUID _player]] call CFUNC(localEvent);

_actor
