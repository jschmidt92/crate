#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

[QGVAR(initPlayer), {
    params [["_uid", "", [""]]];
    [_uid] call FUNC(initPlayer);
}] call CFUNC(addEventHandler);

[QGVAR(disconnectPlayer), {
    params [["_uid", "", [""]]];
    [_uid] call FUNC(disconnectPlayer);
}] call CFUNC(addEventHandler);
