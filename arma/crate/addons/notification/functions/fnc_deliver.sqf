#include "..\script_component.hpp"

/*
 * File: fnc_deliver.sqf
 * Author: IDSolutions
 * Date: 2026-06-13
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Sends unread notifications to a player's chat feed and optionally marks them as read.
 *
 * Arguments:
 * 0: [OBJECT] - Target player
 * 1: [BOOL] - Mark delivered notifications as read (default: false)
 *
 * Return Value:
 * Delivered notifications [ARRAY]
 *
 * Example:
 * [_player, true] call forge_crate_notification_fnc_deliver;
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
