#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

FUNC(getFuelType) = {
    params [["_vehicle", objNull, [objNull]]];

    if (isNull _vehicle) exitWith {""};
    if (_vehicle isKindOf "Helicopter" || { _vehicle isKindOf "Plane" }) exitWith { "jeta1" };
    "regular"
};

[QGVAR(FuelStart), {
    params ["_source", "_target", "_unit"];
    private _plate = GETVAR(_target,EGVAR(garage,plate),"");
    private _fuelType = [_target] call FUNC(getFuelType);
    EXTCALL("fuel:started",[ARR_5(netID _source,netID _target,_unit,_plate,_fuelType)]);
}] call CFUNC(addEventHandler);

[QGVAR(FuelTick), {
    params ["_source", "_target", "_amount"];
    EXTCALL("fuel:tick",[ARR_3(netID _source,netID _target,_amount)]);
}] call CFUNC(addEventHandler);

[QGVAR(FuelStop), {
    params ["_source", "_target"];
    EXTCALL("fuel:stopped",[ARR_2(netID _source,netID _target)]);
}] call CFUNC(addEventHandler);

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_func", "_data"];

    if (_name != "crate:fuel") exitWith {};
    if (_func != "price") exitWith {};

    missionNamespace setVariable ["QGVAR(prices)", _data, true];
    ["forge_client_economy_prices", [_data]] call CBA_fnc_globalEvent;
}];
