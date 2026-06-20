#include "..\script_component.hpp"

/*
 * File: fnc_initPlayer.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Initializes the player's virtual garage unlocks, merges organization unlocks for the client payload, then continues the player initialization chain.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 * Return Value:
 * Virtual garage profile [HASHMAP]
 *
 * Example:
 * [_uid] call forge_crate_v_garage_fnc_initPlayer;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { createHashMap };

private _garageConfig = missionConfigFile >> "CfgMission" >> "Actor" >> "VirtualGarage";
private _unlocks = createHashMapFromArray [
    ["cars", getArray (_garageConfig >> "cars")],
    ["armor", getArray (_garageConfig >> "armor")],
    ["helis", getArray (_garageConfig >> "helis")],
    ["planes", getArray (_garageConfig >> "planes")],
    ["naval", getArray (_garageConfig >> "naval")],
    ["other", getArray (_garageConfig >> "other")]
];

["v_garage:init", [_uid, toJSON _unlocks]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _virtualGarage = fromJSON _result;
if !(_virtualGarage isEqualType createHashMap) exitWith {
    ERROR_1("Virtual garage init returned invalid payload: %1",_result);
    createHashMap
};

private _organization = [_uid] call EFUNC(organization,getPlayerOrganization);
if (_organization isEqualType createHashMap && { _organization isNotEqualTo createHashMap }) then {
    private _playerUnlocks = _virtualGarage getOrDefault ["unlocks", createHashMap];
    private _organizationUnlocks = _organization getOrDefault ["virtual_garage", createHashMap];

    if (_playerUnlocks isEqualType createHashMap && { _organizationUnlocks isEqualType createHashMap }) then {
        {
            private _category = _x;
            private _organizationClassnames = _organizationUnlocks getOrDefault [_category, []];
            private _classNames = (_playerUnlocks getOrDefault [_category, []]) + [];
            {
                _classNames pushBackUnique _x;
            } forEach _organizationClassnames;
            _playerUnlocks set [_category, _classNames];
        } forEach ["cars", "armor", "helis", "planes", "naval", "other"];

        _virtualGarage set ["organization", _organization getOrDefault ["id", "default"]];
        _virtualGarage set ["organization_unlocks", _organizationUnlocks];
    };
};

private _player = [_uid] call EFUNC(common,getPlayerByUID);
if !(isNull _player) then {
    [CRPC(v_garage,responseInitVGarage), [_virtualGarage], _player] call CFUNC(targetEvent);
};
[SRPC(locker,initPlayer), [_uid]] call CFUNC(localEvent);

_virtualGarage
