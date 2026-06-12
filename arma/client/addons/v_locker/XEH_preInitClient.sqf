#include "script_component.hpp"

[QGVAR(responseInitVLocker), {
    params [["_locker", createHashMap, [createHashMap]]];
    if (_locker isEqualTo createHashMap) exitWith {};
    GVAR(profile) = _locker;
}] call CFUNC(addEventHandler);
