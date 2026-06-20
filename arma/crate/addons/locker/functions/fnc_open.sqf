#include "..\script_component.hpp"

/*
 * File: fnc_open.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Materializes a locker profile in a player-specific networked cargo proxy and opens the inventory display.
 *
 * Arguments:
 * 0: Locker profile [HASHMAP]
 * 1: Eden locker terminal [OBJECT]
 * 2: Server-created cargo proxy [OBJECT]
 *
 * Return Value:
 * Cargo proxy [OBJECT]
 *
 * Example:
 * [_locker, _terminal, _proxy] call forge_crate_locker_fnc_open;
 */

params [
    ["_locker", createHashMap, [createHashMap]],
    ["_terminal", objNull, [objNull]],
    ["_proxy", objNull, [objNull]]
];

if (!hasInterface || {isNull _terminal} || {isNull _proxy} || {_locker isEqualTo createHashMap}) exitWith { objNull };
if !(local _proxy) exitWith {
    ERROR_1("Locker proxy is not local to the requesting client: %1",_proxy);
    systemChat LLSTRING(SaveFailed);
    objNull
};
if !(isNull GVAR(activeProxy)) exitWith {
    systemChat LLSTRING(AlreadyOpen);
    objNull
};

private _items = (_locker getOrDefault ["locker", createHashMap]) getOrDefault ["items", createHashMap];
if (_items isEqualType createHashMap) then {
    {
        private _item = _y;
        private _classname = _item getOrDefault ["classname", ""];
        private _amount = _item getOrDefault ["amount", 0];
        if (_classname isNotEqualTo "" && {_amount > 0}) then {
            switch (_item getOrDefault ["category", "items"]) do {
                case "weapons": { _proxy addWeaponCargoGlobal [_classname, _amount] };
                case "magazines": {
                    if !("ammo" in _item) then {
                        ERROR_1("Locker magazine record is missing mandatory ammo: %1",_classname);
                    } else {
                        private _ammoRemaining = _item get "ammo";
                        private _capacity = getNumber (configFile >> "CfgMagazines" >> _classname >> "count");
                        for "_index" from 1 to _amount do {
                            private _magazineAmmo = if (_capacity > 0) then {
                                _ammoRemaining min _capacity
                            } else {
                                0
                            };
                            _proxy addMagazineAmmoCargo [_classname, 1, _magazineAmmo];
                            _ammoRemaining = (_ammoRemaining - _magazineAmmo) max 0;
                        };
                    };
                };
                case "backpacks": { _proxy addBackpackCargoGlobal [_classname, _amount] };
                default { _proxy addItemCargoGlobal [_classname, _amount] };
            };
        };
    } forEach _items;
};

GVAR(activeProfile) = _locker;
GVAR(activeProxy) = _proxy;

[{
    params ["_proxy"];
    if (!isNull _proxy && {_proxy isEqualTo GVAR(activeProxy)}) then {
        player action ["Gear", _proxy];
    };
}, [_proxy]] call CFUNC(execNextFrame);

_proxy
