#include "script_component.hpp"

GVAR(vLockerSnapshot) = createHashMap;
GVAR(arsenalBox) = objNull;
GVAR(arsenalOpen) = false;
GVAR(pendingActorSaves) = createHashMap;

[QGVAR(responseInitVLocker), {
    params [["_locker", createHashMap, [createHashMap]]];
    if (_locker isEqualTo createHashMap) exitWith {};
    GVAR(vLockerSnapshot) = _locker;
    [_locker] call FUNC(sync);
}] call CFUNC(addEventHandler);

["ace_arsenal_displayClosed", {
    if !(GVAR(arsenalOpen)) exitWith {};
    GVAR(arsenalOpen) = false;

    private _requestId = format ["arsenal:%1:%2:%3", getPlayerUID player, diag_tickTime, floor random 1000000];
    GVAR(pendingActorSaves) set [_requestId, true];
    [QEGVAR(actor,saveRequested), [player, _requestId]] call CFUNC(localEvent);
}] call CFUNC(addEventHandler);

[QEGVAR(actor,saveResult), {
    params [
        ["_requestId", "", [""]],
        ["_success", false, [false]]
    ];
    if !(_requestId in GVAR(pendingActorSaves)) exitWith {};
    GVAR(pendingActorSaves) deleteAt _requestId;
    if !(_success) then {
        systemChat LLSTRING(ActorSaveFailed);
    };
}] call CFUNC(addEventHandler);
