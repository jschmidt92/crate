#include "script_component.hpp"

if !(isServer) exitWith {};

[SRPC(webui,bankRequest), {
    params ["_player", "_requestId", "_event", "_data"];
    [_player, _requestId, _event, _data] call FUNC(handleBankRequest);
}] call CFUNC(addEventHandler);

[SRPC(webui,refreshBank), {
    params ["_player"];
    [_player] call FUNC(refreshBank);
}] call CFUNC(addEventHandler);
