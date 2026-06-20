#include "..\script_component.hpp"

/*
 * File: fnc_unread.sqf
 * Author: IDSolutions
 * Date: 2026-06-13
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Retrieves unread durable notifications for a player UID from the Rust extension.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Unread notifications, or an empty array on failure [ARRAY]
 *
 * Example:
 * [_uid] call forge_crate_notification_fnc_unread;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { [] };

["notification:unread", [_uid]] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith { [] };

private _notifications = fromJSON _result;
if !(_notifications isEqualType []) exitWith {
    ERROR_1("Unread notification list returned invalid payload: %1",_result);
    []
};

_notifications
