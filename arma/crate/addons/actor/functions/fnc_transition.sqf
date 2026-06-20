#include "..\script_component.hpp"

/*
 * File: fnc_transition.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Performs a validated actor synchronization lifecycle transition.
 *
 * Arguments:
 * 0: [STRING] - Target lifecycle state
 * 1: [STRING] - Reason for the transition (default: "")
 *
 * Return Value:
 * Transition accepted [BOOL]
 *
 * Example:
 * ["LOADING", "actor initialization requested"] call forge_crate_actor_fnc_transition;
 */

params [
    ["_next", "", [""]],
    ["_reason", "", [""]]
];

if (isNil QGVAR(lifecycle)) then { call FUNC(initState); };

private _current = GVAR(lifecycle) getOrDefault ["state", "UNINITIALIZED"];
if (_next isEqualTo _current) exitWith { true };

private _allowed = switch (_current) do {
    case "UNINITIALIZED": { ["LOADING"] };
    case "LOADING": { ["APPLYING", "FAILED"] };
    case "APPLYING": { ["READY", "FAILED"] };
    case "FAILED": { ["LOADING"] };
    default { [] };
};
if !(_next in _allowed) exitWith {
    WARNING_3("Rejected actor lifecycle transition %1 -> %2 (%3)",_current,_next,_reason);
    false
};

GVAR(lifecycle) set ["previous", _current];
GVAR(lifecycle) set ["state", _next];
GVAR(lifecycle) set ["reason", _reason];
GVAR(lifecycle) set ["changedAt", diag_tickTime];

if (!isNull player) then {
    player setVariable [QGVAR(lifecycleState), _next, true];
    player setVariable [QGVAR(isLoaded), _next isEqualTo "READY", true];
};

[QGVAR(lifecycleStateChanged), [_current, _next, _reason]] call CFUNC(localEvent);
true
