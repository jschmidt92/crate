#include "script_component.hpp"

[QGVAR(responseInitActor), {
    params [["_actor", createHashMap, [createHashMap]]];
    if (_actor isEqualTo createHashMap) exitWith {};
    GVAR(actorSnapshot) = _actor;
}] call CFUNC(addEventHandler);
