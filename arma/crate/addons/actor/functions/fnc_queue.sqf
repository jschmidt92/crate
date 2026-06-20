#include "..\script_component.hpp"

/*
 * File: fnc_queue.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Defers actor initialization until persistent repositories have finished hydrating.
 *
 * Arguments:
 * 0: [OBJECT] - Player to initialize
 * 1: [BOOL] - Whether this call is a scheduled readiness poll (default: false)
 *
 * Return Value:
 * Initialization queued [BOOL]
 *
 * Example:
 * [_player] call forge_crate_actor_fnc_queue;
 */

params [
    ["_player", objNull, [objNull]],
    ["_polling", false, [false]]
];

if (isNull _player || { GETVAR(_player,GVAR(initialized),false) }) exitWith { false };
if (!_polling && { GETVAR(_player,GVAR(initQueued),false) }) exitWith { true };

SETVAR(_player,GVAR(initQueued),true);

["database_ready", []] call EFUNC(extension,call) params ["_result", "_success"];
if (_success && { toLowerANSI _result isEqualTo "true" }) exitWith {
    SETVAR(_player,GVAR(initQueued),false);
    [_player] call FUNC(init);
    true
};

[{
    params ["_player"];
    [_player, true] call FUNC(queue);
}, [_player], 0.25] call CFUNC(waitAndExecute);

true
