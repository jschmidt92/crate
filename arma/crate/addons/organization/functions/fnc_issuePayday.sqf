#include "..\script_component.hpp"

/*
 * File: fnc_issuePayday.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Issues an organization-funded payday to selected members.
 *
 * Arguments:
 * 0: [STRING] - Issuer player UID
 * 1: [STRING] - Organization ID
 * 2: [STRING] - Payday amount per recipient, or empty string to use mission config
 *
 * Return Value:
 * Payday result [HASHMAP]
 *
 * Example:
 * [_uid, "default", "100.00"] call forge_crate_organization_fnc_issuePayday;
 */

params [
    ["_uid", "", [""]],
    ["_organization", "default", [""]],
    ["_amount", "", [""]]
];

if (_uid isEqualTo "") exitWith { createHashMap };
if (_organization isEqualTo "") then { _organization = "default"; };
if (_amount isEqualTo "") then {
    _amount = [missionConfigFile >> "CfgMission" >> "DefaultOrganization" >> "paydayAmount", "0.00"] call EFUNC(common,configMoney);
};

private _isDefaultCeo = [_uid] call FUNC(isDefaultCeo);
[
    "organization:issue_payday",
    [_uid, _organization, _amount, str _isDefaultCeo]
] call EFUNC(extension,call) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _payday = fromJSON _result;
if !(_payday isEqualType createHashMap) exitWith {
    ERROR_1("Organization payday returned invalid payload: %1",_result);
    createHashMap
};

_payday
