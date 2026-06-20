#include "..\script_component.hpp"

/*
 * File: fnc_init.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-20
 * Public: No
 *
 * Description:
 * Initializes the player's virtual locker unlocks, merges organization unlocks for the client payload, and completes the player initialization chain.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Virtual locker profile [HASHMAP]
 *
 * Example:
 * [_uid] call forge_crate_v_locker_fnc_init;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { createHashMap };
if !(GVAR(enabled)) exitWith { createHashMap };

private _lockerConfig = missionConfigFile >> "CfgMission" >> "Actor" >> "Locker";
private _unlocks = createHashMapFromArray [
    ["items", getArray (_lockerConfig >> "items")],
    ["weapons", getArray (_lockerConfig >> "weapons")],
    ["magazines", getArray (_lockerConfig >> "magazines")],
    ["backpacks", getArray (_lockerConfig >> "backpacks")]
];

private _virtualLocker = if (GVAR(persistenceEnabled)) then {
    ["v_locker:init", [_uid, toJSON _unlocks]] call EFUNC(extension,call) params ["_result", "_success"];
    if !(_success) exitWith { createHashMap };
    fromJSON _result
} else {
    createHashMapFromArray [
        ["uid", _uid],
        ["unlocks", _unlocks]
    ]
};
if (!(_virtualLocker isEqualType createHashMap) || { _virtualLocker isEqualTo createHashMap }) exitWith {
    ERROR("Virtual locker init returned an invalid payload");
    createHashMap
};

private _organization = [_uid] call EFUNC(organization,getForPlayer);
if (_organization isEqualType createHashMap && {_organization isNotEqualTo createHashMap}) then {
    private _playerUnlocks = _virtualLocker getOrDefault ["unlocks", createHashMap];
    private _organizationUnlocks = _organization getOrDefault ["virtual_locker", createHashMap];

    if (_playerUnlocks isEqualType createHashMap && {_organizationUnlocks isEqualType createHashMap}) then {
        {
            private _category = _x;
            private _organizationClassnames = _organizationUnlocks getOrDefault [_category, []];
            private _classNames = (_playerUnlocks getOrDefault [_category, []]) + [];
            {
                _classNames pushBackUnique _x;
            } forEach _organizationClassnames;
            _playerUnlocks set [_category, _classNames];
        } forEach ["items", "weapons", "magazines", "backpacks"];

        _virtualLocker set ["organization", _organization getOrDefault ["id", "default"]];
        _virtualLocker set ["organization_unlocks", _organizationUnlocks];
    };
};

private _player = [_uid] call EFUNC(common,findPlayer);
if !(isNull _player) then {
    [CRPC(v_locker,responseInitVLocker), [_virtualLocker], _player] call CFUNC(targetEvent);
};

_virtualLocker
