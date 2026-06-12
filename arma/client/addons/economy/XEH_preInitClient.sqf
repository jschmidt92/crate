#include "script_component.hpp"

if !(hasInterface) exitWith {};

["ace_refuel_started", {
    params ["_source", "_target", "", "_unit"];
    [SRPC(economy,FuelStart), [_source, _target, _unit]] call CFUNC(serverEvent);
}] call CFUNC(addEventHandler);

["ace_refuel_tick", {
    params ["_source", "_target", "_amount"];
    [SRPC(economy,FuelTick), [_source, _target, _amount]] call CFUNC(serverEvent);
}] call CFUNC(addEventHandler);

["ace_refuel_stopped", {
    params ["_source", "_target"];
    [SRPC(economy,FuelStop), [_source, _target]] call CFUNC(serverEvent);
}] call CFUNC(addEventHandler);

[QGVAR(prices), {
    params ["_price"];

    private _regular = parseNumber _price;
    private _avgas = _regular * 1.6;
    private _jetA1 = _regular * 2.0;
    if !(player diarySubjectExists QEGVAR(common,diary)) then {
        player createDiarySubject [
            QEGVAR(common,diary),
            "Forge Dynamics"
        ];
    };

    GVAR(diaryRecord) = player createDiaryRecord [
        QEGVAR(common,diary), [
            "Fuel Prices",
            format ["The fuel prices in this region are:<br />Regular: $%1 per liter<br />Avgas: $%2 per liter<br />Jet A-1: $%3 per liter",
                _regular,
                _avgas,
                _jetA1
            ]
        ]
    ];
}] call CBA_fnc_addPerFrameHandler;
