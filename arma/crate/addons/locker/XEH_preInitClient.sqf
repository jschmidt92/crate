#include "script_component.hpp"

GVAR(activeProfile) = createHashMap;
GVAR(activeProxy) = objNull;
GVAR(lockerSnapshot) = createHashMap;
GVAR(pendingTransfers) = createHashMap;

[QGVAR(responseInitLocker), {
    params [["_locker", createHashMap, [createHashMap]]];
    if (_locker isEqualTo createHashMap) exitWith {};
    GVAR(lockerSnapshot) = _locker;
}] call CFUNC(addEventHandler);

[QGVAR(open), {
    params [
        ["_locker", createHashMap, [createHashMap]],
        ["_terminal", objNull, [objNull]],
        ["_proxy", objNull, [objNull]]
    ];
    [_locker, _terminal, _proxy] call FUNC(open);
}] call CFUNC(addEventHandler);

[QGVAR(saveResult), {
    params [["_success", false, [false]]];
    if !(_success) then {
        systemChat LLSTRING(SaveFailed);
    };
}] call CFUNC(addEventHandler);

[QEGVAR(actor,saveResult), {
    params [
        ["_requestId", "", [""]],
        ["_success", false, [false]]
    ];
    private _proxy = GVAR(pendingTransfers) getOrDefault [_requestId, objNull];
    GVAR(pendingTransfers) deleteAt _requestId;
    if (isNull _proxy || {_proxy isNotEqualTo GVAR(activeProxy)}) exitWith {};

    if (_success) then {
        call FUNC(capture);
    } else {
        systemChat LLSTRING(SaveRetry);
    };
}] call CFUNC(addEventHandler);
