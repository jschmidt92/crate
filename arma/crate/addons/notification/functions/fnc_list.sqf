#include "..\script_component.hpp"

/*
 * File: fnc_list.sqf
 * Author: IDSolutions
 * Date: 2026-06-13
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Retrieves all durable notifications for a player UID from the Rust extension.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Notifications, or an empty array on failure [ARRAY]
 *
 * Example:
 * [_uid] call forge_crate_notification_fnc_list;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { [] };

["notification:list", [_uid]] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith { [] };

private _notifications = fromJSON _result;
if !(_notifications isEqualType []) exitWith {
    ERROR_1("Notification list returned invalid payload: %1",_result);
    []
};

_notifications
