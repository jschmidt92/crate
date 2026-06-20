#include "..\script_component.hpp"

/*
 * File: fnc_save.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Validates ownership and persists locker contents after actor-save confirmation.
 *
 * Arguments:
 * 0: Player [OBJECT]
 * 1: Server-created cargo proxy [OBJECT]
 * 2: Serialized locker profile [STRING]
 * 3: Player UID fallback for disconnect saves (optional) [STRING]
 *
 * Return Value:
 * Save succeeded [BOOL]
 *
 * Example:
 * [_player, _proxy, _lockerJson, _uid] call forge_crate_locker_fnc_save;
 */

params [
    ["_player", objNull, [objNull]],
    ["_proxy", objNull, [objNull]],
    ["_lockerJson", "", [""]],
    ["_uid", "", [""]]
];

if (isNull _player || {isNull _proxy} || {_lockerJson isEqualTo ""}) exitWith { false };

if (_uid isEqualTo "") then { _uid = getPlayerUID _player };
private _locker = fromJSON _lockerJson;
if (
    _uid isEqualTo "" ||
    {!(_locker isEqualType createHashMap)} ||
    {_proxy getVariable [QGVAR(ownerUid), ""] isNotEqualTo _uid} ||
    {_player getVariable [QGVAR(proxy), objNull] isNotEqualTo _proxy}
) exitWith { false };

_locker set ["uid", _uid];
["locker:commit", [toJSON _locker]] call EFUNC(extension,call) params ["_result", "_success"];

if !(_success) then {
    ERROR_2("Failed to save locker for %1: %2",_uid,_result);
};
[QGVAR(saveResult), [_success], _player] call CFUNC(targetEvent);
deleteVehicle _proxy;
_player setVariable [QGVAR(proxy), objNull];

_success
