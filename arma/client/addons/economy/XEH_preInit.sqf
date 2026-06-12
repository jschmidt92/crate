#include "script_component.hpp"

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

// private _category = [QUOTE(MOD_NAME), LLSTRING(displayName)];

// #include "initSettings.inc.sqf"
// #include "initKeybinds.inc.sqf"

["ace_refuel_started", {
    params ["_source", "_target", "", "_unit"];
    [SRPC(economy,FuelStarted), [_source, _target, _unit]] call CFUNC(serverEvent);
}] call CFUNC(addEventHandler);

["ace_refuel_tick", {
    params ["_source", "_target", "_amount"];
    [SRPC(economy,FuelTick), [_source, _target, _amount]] call CFUNC(serverEvent);
}] call CFUNC(addEventHandler);

["ace_refuel_stopped", {
    params ["_source", "_target"];
    [SRPC(economy,FuelStop), [_source, _target]] call CFUNC(serverEvent);
}] call CFUNC(addEventHandler);
