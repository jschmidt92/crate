#include "script_component.hpp"

GVAR(vGarageSnapshot) = createHashMap;

[QGVAR(responseInitVGarage), {
    params [["_garage", createHashMap, [createHashMap]]];
    if (_garage isEqualTo createHashMap) exitWith {};
    GVAR(vGarageSnapshot) = _garage;
}] call CFUNC(addEventHandler);
