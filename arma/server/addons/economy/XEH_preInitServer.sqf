#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

FUNC(getFuelType) = {
    params [["_vehicle", objNull, [objNull]]];

    if (isNull _vehicle) exitWith {""};
    private _fuelType = switch (typeOf _vehicle) {
        case "Helicopter";
        case "Plane": { "jeta1" };
        default: { "regular" };
    };
    _fuelType
};

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_func", "_data"];

    if (_name != "crate:fuel") exitWith {};
    if (_func != "price") exitWith {};

    missionNamespace setVariable ["QGVAR(prices)", _data, true];
    ["forge_client_economy_prices", [_data]] call CBA_fnc_globalEvent;
}];
