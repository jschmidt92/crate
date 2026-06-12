#include "..\script_component.hpp"

/*
 * File: fnc_addPlayerMember.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Registers a player UID as a member of an organization.
 *
 * Arguments:
 * 0: [STRING] - Player UID
 * 1: [STRING] - Organization ID
 *
 * Return Value:
 * Organization [HASHMAP]
 *
 * Example:
 * [_uid, "default"] call forge_server_organization_fnc_addPlayerMember;
 */

params [["_uid", "", [""]], ["_organization", "default", [""]]];

if (_uid isEqualTo "") exitWith { createHashMap };
if (_organization isEqualTo "") then {
    _organization = "default";
};

["organization:add_member", [_organization, _uid]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _organizationPayload = fromJSON _result;
if !(_organizationPayload isEqualType createHashMap) exitWith {
    ERROR_1("Organization add member returned invalid payload: %1",_result);
    createHashMap
};

_organizationPayload
