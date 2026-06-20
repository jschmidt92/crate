#include "..\script_component.hpp"

/*
 * File: fnc_sync.sqf
 * Author: IDSolutions
 * Date: 2026-06-20
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Rebuilds the client-local ACE Arsenal box from merged player, mission, and organization unlocks.
 *
 * Arguments:
 * 0: Virtual locker profile [HASHMAP]
 *
 * Return Value:
 * Arsenal box [OBJECT]
 *
 * Example:
 * [_virtualLocker] call forge_crate_v_locker_fnc_sync;
 */

params [["_virtualLocker", createHashMap, [createHashMap]]];
if (!hasInterface || {!GVAR(enabled)} || {_virtualLocker isEqualTo createHashMap}) exitWith { objNull };

if !(isNull GVAR(arsenalBox)) then {
    [GVAR(arsenalBox), true] call AFUNC(arsenal,removeVirtualItems);
    deleteVehicle GVAR(arsenalBox);
};

private _box = "ReammoBox_F" createVehicleLocal [0, 0, -1000];
_box allowDamage false;
_box hideObject true;

private _unlocks = _virtualLocker getOrDefault ["unlocks", createHashMap];
private _items = [];
{
    {
        if (_x isEqualType "" && {_x isNotEqualTo ""}) then {
            _items pushBackUnique _x;
        };
    } forEach (_unlocks getOrDefault [_x, []]);
} forEach ["items", "weapons", "magazines", "backpacks"];

[_box, _items] call AFUNC(arsenal,initBox);
GVAR(arsenalBox) = _box;
INFO_1("Virtual arsenal synchronized with %1 unlock(s)",count _items);

_box
