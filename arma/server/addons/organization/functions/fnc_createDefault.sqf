#include "..\script_component.hpp"

/*
 * File: fnc_createDefault.sqf
 * Author: IDSolutions
 * Date: 2026-06-12
 * Last Update: 2026-06-12
 * Public: No
 *
 * Description:
 * Ensures the default organization exists in the Rust extension with mission-configured starting state.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * Default organization [HASHMAP]
 *
 * Example:
 * [] call forge_server_organization_fnc_createDefault;
 */

private _defaultConfig = missionConfigFile >> "CfgMission" >> "DefaultOrganization";
private _startingBank = [_defaultConfig >> "startingBank", "0.00"] call EFUNC(common,getConfigMoney);

private _garageConfig = _defaultConfig >> "VirtualGarage";
private _virtualGarage = createHashMapFromArray [
    ["cars", getArray (_garageConfig >> "cars")],
    ["armor", getArray (_garageConfig >> "armor")],
    ["helis", getArray (_garageConfig >> "helis")],
    ["planes", getArray (_garageConfig >> "planes")],
    ["naval", getArray (_garageConfig >> "naval")],
    ["other", getArray (_garageConfig >> "other")]
];

private _lockerConfig = _defaultConfig >> "Locker";
private _virtualLocker = createHashMapFromArray [
    ["items", getArray (_lockerConfig >> "items")],
    ["weapons", getArray (_lockerConfig >> "weapons")],
    ["magazines", getArray (_lockerConfig >> "magazines")],
    ["backpacks", getArray (_lockerConfig >> "backpacks")]
];

["organization:create_default", [_startingBank, toJSON _virtualGarage, toJSON _virtualLocker]] call EFUNC(extension,extCall) params ["_result", "_success"];
if !(_success) exitWith { createHashMap };

private _organization = fromJSON _result;
if !(_organization isEqualType createHashMap) exitWith {
    ERROR_1("Default organization init returned invalid payload: %1",_result);
    createHashMap
};

_organization
