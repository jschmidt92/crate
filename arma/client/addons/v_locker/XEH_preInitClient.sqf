#include "script_component.hpp"

[QGVAR(responseInitVLocker), {
    params [["_locker", createHashMap, [createHashMap]]];
    if (_locker isEqualTo createHashMap) exitWith {};
    GVAR(vLockerSnapshot) = _locker;
}] call CFUNC(addEventHandler);
