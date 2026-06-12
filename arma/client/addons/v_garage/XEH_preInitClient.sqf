#include "script_component.hpp"

[QGVAR(responseInitVGarage), {
    params [["_garage", createHashMap, [createHashMap]]];
    if (_garage isEqualTo createHashMap) exitWith {};
    GVAR(profile) = _garage;
}] call CFUNC(addEventHandler);
