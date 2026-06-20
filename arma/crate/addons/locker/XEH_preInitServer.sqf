#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

[QGVAR(initPlayer), {
    params [["_uid", "", [""]]];
    [_uid] call FUNC(init);
}] call CFUNC(addEventHandler);

[QGVAR(requestOpen), {
    params [
        ["_player", objNull, [objNull]],
        ["_terminal", objNull, [objNull]]
    ];
    [_player, _terminal] call FUNC(request);
}] call CFUNC(addEventHandler);

[QGVAR(save), {
    params [
        ["_player", objNull, [objNull]],
        ["_proxy", objNull, [objNull]],
        ["_lockerJson", "", [""]]
    ];
    [_player, _proxy, _lockerJson] call FUNC(save);
}] call CFUNC(addEventHandler);

[QEGVAR(actor,savedServer), {
    params [
        ["_player", objNull, [objNull]],
        ["_uid", "", [""]]
    ];
    if (isNull _player) exitWith {};
    private _proxy = _player getVariable [QGVAR(proxy), objNull];
    if (isNull _proxy) exitWith {};
    if (_uid isEqualTo "") then { _uid = getPlayerUID _player };

    private _items = [_proxy] call FUNC(contents);
    private _locker = createHashMapFromArray [
        ["uid", _uid],
        ["locker", createHashMapFromArray [["items", _items]]]
    ];
    [_player, _proxy, toJSON _locker, _uid] call FUNC(save);
}] call CFUNC(addEventHandler);

[QEGVAR(actor,saveFailedServer), {
    params [["_player", objNull, [objNull]]];
    if (isNull _player) exitWith {};
    private _proxy = _player getVariable [QGVAR(proxy), objNull];
    if !(isNull _proxy) then {
        deleteVehicle _proxy;
        _player setVariable [QGVAR(proxy), objNull];
    };
}] call CFUNC(addEventHandler);
