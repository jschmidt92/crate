#include "script_component.hpp"

[QGVAR(responseInitLocker), {
    params [["_locker", createHashMap, [createHashMap]]];
    if (_locker isEqualTo createHashMap) exitWith {};
    GVAR(lockerSnapshot) = _locker;
}] call CFUNC(addEventHandler);
