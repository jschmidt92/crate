#include "script_component.hpp"

call FUNC(initState);

[QGVAR(beginInitActor), {
    ["LOADING", "actor initialization requested"] call FUNC(transition);
}] call CFUNC(addEventHandler);

[QGVAR(failedInitActor), {
    params [["_reason", "actor initialization failed", [""]]];
    ["FAILED", _reason] call FUNC(transition);
}] call CFUNC(addEventHandler);

[QGVAR(responseInitActor), {
    params [
        ["_actor", createHashMap, [createHashMap]],
        ["_created", false, [false]]
    ];
    if (_actor isEqualTo createHashMap) exitWith {
        ["FAILED", "actor response was empty"] call FUNC(transition);
    };
    if !(["APPLYING", "actor state received"] call FUNC(transition)) exitWith {};
    if !([player, _actor, _created] call FUNC(apply)) exitWith {
        ["FAILED", "actor state could not be applied"] call FUNC(transition);
    };
    GVAR(actorSnapshot) = _actor;
    ["READY", "actor state applied"] call FUNC(transition);
}] call CFUNC(addEventHandler);

[QGVAR(saveRequested), {
    params [
        ["_player", objNull, [objNull]],
        ["_requestId", "", [""]]
    ];
    if (isNull _player || {_requestId isEqualTo ""}) exitWith {};

    private _snapshot = [_player, false] call FUNC(capture);
    if (_snapshot isEqualTo createHashMap) exitWith {
        [QGVAR(saveResult), [_requestId, false]] call CFUNC(localEvent);
    };
    [QGVAR(save), [_player, _requestId, toJSON _snapshot]] call CFUNC(serverEvent);
}] call CFUNC(addEventHandler);
