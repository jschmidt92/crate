#include "..\script_component.hpp"

/*
 * File: fnc_register.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Discovers Eden-placed bank terminals named bank or bank_N and registers client interactions.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Registered bank terminals [ARRAY]
 *
 * Example:
 * call forge_crate_bank_fnc_register;
 */

if !(hasInterface) exitWith { [] };

private _terminals = [];
private _names = ["bank"];
for "_index" from 1 to 999 do {
    _names pushBack format ["bank_%1", _index];
};

{
    private _terminal = missionNamespace getVariable [_x, objNull];
    if (!isNull _terminal && {_terminals pushBackUnique _terminal >= 0}) then {
        private _actionId = _terminal getVariable [QGVAR(actionId), -1];
        if (_actionId < 0) then {
            _actionId = _terminal addAction [
                LLSTRING(Open),
                {
                    [QGVAR(openRequested), [player]] call CFUNC(localEvent);
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
} forEach _names;

GVAR(terminals) = _terminals;
INFO_1("Registered %1 Eden bank terminal(s)",count _terminals);

_terminals
