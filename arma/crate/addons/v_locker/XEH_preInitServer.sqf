#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

[QGVAR(initPlayer), {
    params [["_uid", "", [""]]];
    [_uid] call FUNC(init);
}] call CFUNC(addEventHandler);
