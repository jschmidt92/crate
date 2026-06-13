#include "..\script_component.hpp"

/*
 * Marks one notification read for a player UID.
 */

params [["_uid", "", [""]], ["_id", "", [""]]];

if (_uid isEqualTo "" || {_id isEqualTo ""}) exitWith { createHashMap };

["notification:mark_read", [_uid, _id]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _notification = fromJSON _result;
if !(_notification isEqualType createHashMap) exitWith {
    ERROR_1("Notification mark_read returned invalid payload: %1",_result);
    createHashMap
};

_notification
