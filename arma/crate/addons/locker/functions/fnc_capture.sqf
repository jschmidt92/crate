#include "..\script_component.hpp"

/*
 * File: fnc_capture.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Captures normalized locker cargo after the actor module confirms its own save.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Captured locker profile [HASHMAP]
 *
 * Example:
 * call forge_crate_locker_fnc_capture;
 */

private _proxy = GVAR(activeProxy);
if (isNull _proxy) exitWith { createHashMap };

private _items = [_proxy] call FUNC(contents);

private _profile = GVAR(activeProfile);
_profile set ["uid", getPlayerUID player];
_profile set ["locker", createHashMapFromArray [["items", _items]]];

[QGVAR(save), [player, _proxy, toJSON _profile]] call CFUNC(serverEvent);
GVAR(activeProxy) = objNull;
GVAR(activeProfile) = createHashMap;
GVAR(lockerSnapshot) = _profile;

_profile
