#include "script_component.hpp"

call FUNC(register);

player addEventHandler ["InventoryClosed", {
    params ["_unit", "_container"];
    if (_container isEqualTo GVAR(activeProxy)) then {
        private _requestId = format ["%1:%2:%3", getPlayerUID _unit, diag_tickTime, floor random 1000000];
        GVAR(pendingTransfers) set [_requestId, _container];
        [QEGVAR(actor,saveRequested), [_unit, _requestId]] call CFUNC(localEvent);
    };
}];
