#include "..\script_component.hpp"

/*
 * File: fnc_markRead.sqf
 * Author: IDSolutions
 * Date: 2026-06-13
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Marks one durable notification as read for a player UID.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 * 1: [STRING] - Notification ID
 *
 * Return Value:
 * Updated notification, or an empty hashmap on failure [HASHMAP]
 *
 * Example:
 * [_uid, _notificationId] call forge_crate_notification_fnc_markRead;
 */

params [["_uid", "", [""]], ["_id", "", [""]]];

if (_uid isEqualTo "" || {_id isEqualTo ""}) exitWith { createHashMap };

["notification:mark_read", [_uid, _id]] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _notification = fromJSON _result;
if !(_notification isEqualType createHashMap) exitWith {
    ERROR_1("Notification mark_read returned invalid payload: %1",_result);
    createHashMap
};

_notification
