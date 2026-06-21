#include "script_component.hpp"

if !(hasInterface) exitWith {};

[CRPC(webui,response), {
    params ["_response"];
    [_response] call FUNC(send);
}] call CFUNC(addEventHandler);

[QEGVAR(bank,openRequested), {
    call FUNC(open);
}] call CFUNC(addEventHandler);
