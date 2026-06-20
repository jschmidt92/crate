#include "script_component.hpp"

GVAR(vLockerSnapshot) = createHashMap;

[QGVAR(responseInitVLocker), {
    params [["_locker", createHashMap, [createHashMap]]];
    if (_locker isEqualTo createHashMap) exitWith {};
    GVAR(vLockerSnapshot) = _locker;
}] call CFUNC(addEventHandler);
