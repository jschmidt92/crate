#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

addMissionEventHandler ["PlayerConnected", {
    params [
        ["_id", 0, [0]],
        ["_uid", "", [""]],
        ["_name", "", [""]],
        ["_jip", false, [false]],
        ["_owner", 0, [0]],
        ["_idStr", "", [""]]
    ];

    [{
        params ["_uid"];
        !isNull ([_uid] call EFUNC(common,getPlayerByUID))
    }, {
        params ["_uid"];
        private _player = [_uid] call EFUNC(common,getPlayerByUID);
        [QGVAR(initPlayer), [_player]] call CFUNC(localEvent);
    }, [_uid]] call CFUNC(waitUntilAndExecute);
}];

{
    [QGVAR(initPlayer), [_x]] call CFUNC(localEvent);
} forEach allPlayers;
