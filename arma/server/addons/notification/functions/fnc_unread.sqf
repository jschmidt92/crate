#include "..\script_component.hpp"

/*
 * Lists unread notifications for a player UID.
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { [] };

["notification:unread", [_uid]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { [] };

private _notifications = fromJSON _result;
if !(_notifications isEqualType []) exitWith {
    ERROR_1("Unread notification list returned invalid payload: %1",_result);
    []
};

_notifications
