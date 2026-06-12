#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

[QGVAR(createDefault), []] call CFUNC(localEvent);
