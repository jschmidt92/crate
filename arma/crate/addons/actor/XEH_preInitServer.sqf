#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

[QGVAR(initPlayer), {
    params [["_player", objNull, [objNull]]];
    [_player] call FUNC(initPlayer);
}] call CFUNC(addEventHandler);

[QGVAR(disconnectPlayer), {
    params [["_player", objNull, [objNull]], ["_uid", "", [""]]];
    [_player, _uid] call FUNC(disconnectPlayer);
}] call CFUNC(addEventHandler);
