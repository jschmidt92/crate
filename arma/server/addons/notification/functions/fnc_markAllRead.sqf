#include "..\script_component.hpp"

/*
 * Marks all unread notifications read for a player UID.
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { [] };

["notification:mark_all_read", [_uid]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { [] };

private _notifications = fromJSON _result;
if !(_notifications isEqualType []) exitWith {
    ERROR_1("Notification mark_all_read returned invalid payload: %1",_result);
    []
};

_notifications
