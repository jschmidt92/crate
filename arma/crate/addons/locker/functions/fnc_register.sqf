#include "..\script_component.hpp"

/*
 * File: fnc_register.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Discovers Eden-placed locker terminals named locker or locker_N and registers client interactions.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Registered locker terminals [ARRAY]
 *
 * Example:
 * call forge_crate_locker_fnc_register;
 */

private _terminals = [];
private _names = ["locker"];
for "_index" from 1 to 999 do {
    _names pushBack format ["locker_%1", _index];
};

{
    private _terminal = missionNamespace getVariable [_x, objNull];
    if (!isNull _terminal && {_terminals pushBackUnique _terminal >= 0}) then {
        if (isServer) then {
            clearWeaponCargoGlobal _terminal;
            clearItemCargoGlobal _terminal;
            clearMagazineCargoGlobal _terminal;
            clearBackpackCargoGlobal _terminal;
            _terminal lockInventory true;
        };

        if (hasInterface) then {
            private _actionId = _terminal getVariable [QGVAR(actionId), -1];
            if (_actionId < 0) then {
                _actionId = _terminal addAction [
                    LLSTRING(Open),
                    {
                        params ["_target", "_caller"];
                        if !(isNull GVAR(activeProxy)) exitWith {
                            GVAR(activeProxy) setPosATL (getPosATL _caller);
                            _caller action ["Gear", GVAR(activeProxy)];
                        };
                        [QGVAR(requestOpen), [_caller, _target]] call CFUNC(serverEvent);
                    },
                    nil,
                    1.5,
                    true,
                    true,
                    "",
                    "alive _this && {_this distance _target <= 4}",
                    4
                ];
                _terminal setVariable [QGVAR(actionId), _actionId];
            };
        };
    };
} forEach _names;

GVAR(terminals) = _terminals;
INFO_1("Registered %1 Eden locker terminal(s)",count _terminals);

_terminals
