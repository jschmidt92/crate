#include "script_component.hpp"

["ace_arsenal_displayOpened", {
    disableSerialization;
    params ["_display"];
    _display displayAddEventHandler ["KeyDown", "_this select 3"];
    { (_display displayCtrl _x) ctrlShow false } forEach [1002, 1003, 1004, 1005, 1006];
}] call CFUNC(addEventHandler);
