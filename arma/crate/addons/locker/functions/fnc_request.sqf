#include "..\script_component.hpp"

/*
 * File: fnc_request.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Validates a locker terminal request and returns the player's current locker profile.
 *
 * Arguments:
 * 0: Player [OBJECT]
 * 1: Eden locker terminal [OBJECT]
 *
 * Return Value:
 * Request accepted [BOOL]
 *
 * Example:
 * [_player, _terminal] call forge_crate_locker_fnc_request;
 */

params [
    ["_player", objNull, [objNull]],
    ["_terminal", objNull, [objNull]]
];

if (
    isNull _player ||
    {isNull _terminal} ||
    {!alive _player} ||
    {!(_terminal in GVAR(terminals))} ||
    {_player distance _terminal > 8}
) exitWith { false };

private _uid = getPlayerUID _player;
if (_uid isEqualTo "") exitWith { false };

["locker:get", [_uid]] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith {
    [QGVAR(saveResult), [false], _player] call CFUNC(targetEvent);
    false
};

private _locker = fromJSON _result;
if !(_locker isEqualType createHashMap) exitWith {
    ERROR_2("Locker request for %1 returned invalid payload: %2",_uid,_result);
    [QGVAR(saveResult), [false], _player] call CFUNC(targetEvent);
    false
};

private _previousProxy = _player getVariable [QGVAR(proxy), objNull];
if !(isNull _previousProxy) then {
    deleteVehicle _previousProxy;
};

private _proxy = createVehicle ["GroundWeaponHolder_Scripted", getPosATL _player, [], 0, "CAN_COLLIDE"];
_proxy allowDamage false;
_proxy hideObjectGlobal true;
_proxy setVariable [QGVAR(ownerUid), _uid, true];
_player setVariable [QGVAR(proxy), _proxy];

private _clientOwner = owner _player;
_proxy setOwner _clientOwner;
[{
    params ["_proxy", "_clientOwner"];
    owner _proxy isEqualTo _clientOwner
}, {
    params ["_proxy", "_clientOwner", "_locker", "_terminal", "_player"];
    [QGVAR(open), [_locker, _terminal, _proxy], _player] call CFUNC(targetEvent);
}, [_proxy, _clientOwner, _locker, _terminal, _player], 5, {
    params ["_proxy", "_clientOwner", "_locker", "_terminal", "_player"];
    ERROR_2("Locker proxy ownership transfer failed for owner %1 and proxy %2",_clientOwner,_proxy);
    deleteVehicle _proxy;
    _player setVariable [QGVAR(proxy), objNull];
    [QGVAR(saveResult), [false], _player] call CFUNC(targetEvent);
}] call CFUNC(waitUntilAndExecute);
true
