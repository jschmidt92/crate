#include "..\script_component.hpp"

/*
 * File: fnc_markAllRead.sqf
 * Author: IDSolutions
 * Date: 2026-06-13
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Marks every unread notification as read for a player UID.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Updated notifications, or an empty array on failure [ARRAY]
 *
 * Example:
 * [_uid] call forge_crate_notification_fnc_markAllRead;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { [] };

["notification:mark_all_read", [_uid]] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith { [] };

private _notifications = fromJSON _result;
if !(_notifications isEqualType []) exitWith {
    ERROR_1("Notification mark_all_read returned invalid payload: %1",_result);
    []
};

_notifications
