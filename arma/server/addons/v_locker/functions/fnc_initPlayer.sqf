#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes the player's virtual locker unlocks, merges organization unlocks for the client payload, and completes the player initialization chain.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 * Return Value:
 * Virtual locker profile [HASHMAP]
 *
 * Example:
 * [_uid] call forge_server_v_locker_fnc_initPlayer;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { createHashMap };

private _lockerConfig = missionConfigFile >> "CfgMission" >> "Actor" >> "Locker";
private _unlocks = createHashMapFromArray [
    ["items", getArray (_lockerConfig >> "items")],
    ["weapons", getArray (_lockerConfig >> "weapons")],
    ["magazines", getArray (_lockerConfig >> "magazines")],
    ["backpacks", getArray (_lockerConfig >> "backpacks")]
];

["v_locker:init", [_uid, toJSON _unlocks]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _virtualLocker = fromJSON _result;
if !(_virtualLocker isEqualType createHashMap) exitWith {
    ERROR_1("Virtual locker init returned invalid payload: %1",_result);
    createHashMap
};

private _organization = [_uid] call EFUNC(organization,getPlayerOrganization);
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

private _player = [_uid] call EFUNC(common,getPlayerByUID);
if !(isNull _player) then {
    [CRPC(v_locker,responseInitVLocker), [_virtualLocker], _player] call CFUNC(targetEvent);
};

_virtualLocker
