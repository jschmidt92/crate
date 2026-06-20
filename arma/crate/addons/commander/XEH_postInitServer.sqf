#include "script_component.hpp"

if !(isMultiplayer) exitWith {};
if (isNil QGVAR(service)) then {
    private _overrides = createHashMapFromArray (getArray (missionConfigFile >> "CfgMission" >> "Commander"));
    [_overrides] call FUNC(createCommander);
};

[{
    !isNil QGVAR(service) && { (GETMVAR(EGVAR(task,missionSetup_settingsApplied),objNull)) isNotEqualTo objNull }
}, {
    GVAR(service) call ["start", []];
}] call CFUNC(waitUntilAndExecute);
