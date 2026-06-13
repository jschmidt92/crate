#include "..\script_component.hpp"

/*
 * Sends unread notifications to a player's chat feed.
 */

params [["_player", objNull, [objNull]], ["_markRead", false, [false]]];

if (isNull _player) exitWith { [] };

private _uid = getPlayerUID _player;
if (_uid isEqualTo "") exitWith { [] };

private _notifications = [_uid] call FUNC(unread);

{
    private _id = _x getOrDefault ["id", ""];
    private _title = _x getOrDefault ["title", "Notification"];
    private _body = _x getOrDefault ["body", ""];

    [format ["%1: %2", _title, _body]] remoteExecCall ["systemChat", _player];

    if (_markRead && {_id isNotEqualTo ""}) then {
        [_uid, _id] call FUNC(markRead);
    };
} forEach _notifications;

_notifications
