#include "script_component.hpp"

if !(hasInterface) exitWith {};

[CRPC(webui,bankResponse), {
    params ["_response"];
    [_response] call FUNC(send);
}] call CFUNC(addEventHandler);
