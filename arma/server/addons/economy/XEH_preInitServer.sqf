#include "script_component.hpp"

if !(isMultiplayer) exitWith {};

FUNC(getFuelType) = {
    params [["_vehicle", objNull, [objNull]]];

    if (isNull _vehicle) exitWith {""};
    if (_vehicle isKindOf "Helicopter" || { _vehicle isKindOf "Plane" }) exitWith { "jeta1" };
    "regular"
};

FUNC(getServiceAmount) = {
    params [["_service", "", [""]], ["_property", "", [""]], ["_default", "0.00", [""]]];

    [missionConfigFile >> "CfgForgeMission" >> "Services" >> _service >> _property, _default] call EFUNC(common,getConfigMoney)
};

FUNC(getRefuelPrice) = {
    params [["_fuelType", "regular", [""]]];

    if (_fuelType == "jeta1") exitWith {
        ["Refuel", "jeta1PricePerLiter", "1.80"] call FUNC(getServiceAmount)
    };
    ["Refuel", "regularPricePerLiter", "1.00"] call FUNC(getServiceAmount)
};

[QGVAR(FuelStart), {
    params ["_source", "_target", "_unit"];

    private _plate = GETVAR(_target,EGVAR(garage,plate),"");
    private _fuelType = [_target] call FUNC(getFuelType);
    private _pricePerLiter = [_fuelType] call FUNC(getRefuelPrice);
    EXTCALL("refuel:started",[netId _source, netId _target, getPlayerUID _unit, _plate, _fuelType, _pricePerLiter]);
}] call CFUNC(addEventHandler);

[QGVAR(FuelTick), {
    params ["_source", "_target", "_amount"];
    EXTCALL("refuel:tick",[ARR_3(netId _source,netId _target,_amount)]);
}] call CFUNC(addEventHandler);

[QGVAR(FuelStop), {
    params ["_source", "_target"];
    EXTCALL("refuel:stopped",[ARR_2(netId _source,netId _target)]);
}] call CFUNC(addEventHandler);

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_func", "_data"];

    if (_name != "crate:refuel") exitWith {};
    if (_func != "price") exitWith {};

    SETMPVAR(GVAR(prices),_data);
    ["forge_client_economy_prices", [_data]] call CBA_fnc_globalEvent;
}];
