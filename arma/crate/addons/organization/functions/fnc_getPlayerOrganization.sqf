#include "..\script_component.hpp"

/*
 * File: fnc_getPlayerOrganization.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Gets the organization assigned to a player actor, falling back to the default organization.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 *
 * Return Value:
 * Organization [HASHMAP]
 *
 * Example:
 * [_uid] call forge_crate_organization_fnc_getPlayerOrganization;
 */

params [["_uid", "", [""]]];

if (_uid isEqualTo "") exitWith { createHashMap };

private _organizationId = "default";

["actor:get", [_uid]] call EFUNC(extension,extCall) params ["_actorResult", "_actorSuccess"];
if (_actorSuccess && {_actorResult isNotEqualTo "null"}) then {
    private _actor = fromJSON _actorResult;
    if (_actor isEqualType createHashMap) then {
        _organizationId = _actor getOrDefault ["organization", "default"];
    };
};

if (_organizationId isEqualTo "") then {
    _organizationId = "default";
};

["organization:get", [_organizationId]] call EFUNC(extension,extCall) params ["_organizationResult", "_organizationSuccess"];
if !(_organizationSuccess) exitWith { createHashMap };
if (_organizationResult isEqualTo "null") exitWith { createHashMap };

private _organization = fromJSON _organizationResult;
if !(_organization isEqualType createHashMap) exitWith {
    ERROR_1("Organization get returned invalid payload: %1",_organizationResult);
    createHashMap
};

_organization
