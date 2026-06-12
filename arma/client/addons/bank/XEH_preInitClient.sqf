#include "script_component.hpp"

[QGVAR(responseInitBank), {
    params [["_bank", createHashMap, [createHashMap]]];
    if (_bank isEqualTo createHashMap) exitWith {};
    GVAR(profile) = _bank;
}] call CFUNC(addEventHandler);
