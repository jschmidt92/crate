#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

[QGVAR(createDefault), {
    [] call FUNC(createDefault);
}] call CFUNC(addEventHandler);
