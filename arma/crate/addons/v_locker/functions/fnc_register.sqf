#include "..\script_component.hpp"

/*
 * File: fnc_register.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Adds virtual arsenal access to the Eden locker terminals discovered by the physical locker addon.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Registered terminal count [NUMBER]
 *
 * Example:
 * call forge_crate_v_locker_fnc_register;
 */

private _terminals = GETEGVAR(locker,terminals,[]);
{
    private _actionId = _x getVariable [QGVAR(actionId), -1];
    if (_actionId < 0) then {
        _actionId = _x addAction [
            LLSTRING(Open),
            {
                call FUNC(open);
            },
            nil,
            1.4,
            true,
            true,
            "",
            "alive _this && {_this distance _target <= 4}",
            4
        ];
        _x setVariable [QGVAR(actionId), _actionId];
    };
} forEach _terminals;

INFO_1("Registered virtual arsenal on %1 locker terminal(s)",count _terminals);
count _terminals
