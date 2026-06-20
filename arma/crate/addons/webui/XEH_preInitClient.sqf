#include "script_component.hpp"

if !(hasInterface) exitWith {};

[CRPC(webui,bankResponse), {
    params ["_response"];
    [_response] call FUNC(sendToBrowser);
}] call CFUNC(addEventHandler);
