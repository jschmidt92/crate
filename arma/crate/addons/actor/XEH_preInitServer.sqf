#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

[QGVAR(initPlayer), {
    params [["_player", objNull, [objNull]]];
    [_player] call FUNC(queue);
}] call CFUNC(addEventHandler);

[QGVAR(disconnectPlayer), {
    params [["_player", objNull, [objNull]], ["_uid", "", [""]]];
    private _actor = [_player, _uid] call FUNC(disconnect);
    if (_actor isNotEqualTo createHashMap) then {
        [QGVAR(savedServer), [_player, _uid]] call CFUNC(localEvent);
    } else {
        [QGVAR(saveFailedServer), [_player, _uid]] call CFUNC(localEvent);
    };
}] call CFUNC(addEventHandler);

[QGVAR(save), {
    params [
        ["_player", objNull, [objNull]],
        ["_requestId", "", [""]],
        ["_snapshotJson", "", [""]]
    ];
    if (isNull _player || {_requestId isEqualTo ""} || {_snapshotJson isEqualTo ""}) exitWith {};

    private _snapshot = fromJSON _snapshotJson;
    private _uid = getPlayerUID _player;
    if (_uid isEqualTo "" || {!(_snapshot isEqualType createHashMap)}) exitWith {
        [QGVAR(saveResult), [_requestId, false], _player] call CFUNC(targetEvent);
    };
    _snapshot set ["uid", _uid];

    ["actor:save", [toJSON _snapshot]] call EFUNC(extension,call) params ["_result", "_success"];
    if !(_success) then {
        ERROR_2("Failed to save actor for %1: %2",_uid,_result);
    };
    [QGVAR(saveResult), [_requestId, _success], _player] call CFUNC(targetEvent);
}] call CFUNC(addEventHandler);
