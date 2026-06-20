#include "..\script_component.hpp"

/*
 * File: fnc_open.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Opens the player's synchronized ACE Virtual Arsenal.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Arsenal opened [BOOL]
 *
 * Example:
 * call forge_crate_v_locker_fnc_open;
 */

if (!hasInterface || {!GVAR(enabled)}) exitWith { false };
if (isNull GVAR(arsenalBox)) exitWith {
    systemChat LLSTRING(NotReady);
    false
};

private _unlocks = GVAR(vLockerSnapshot) getOrDefault ["unlocks", createHashMap];
private _hasUnlocks = ["items", "weapons", "magazines", "backpacks"] findIf {
    (_unlocks getOrDefault [_x, []]) isNotEqualTo []
} >= 0;
if !(_hasUnlocks) exitWith {
    systemChat LLSTRING(NoUnlocks);
    false
};

GVAR(arsenalOpen) = true;
[GVAR(arsenalBox), player] call AFUNC(arsenal,openBox);
true
