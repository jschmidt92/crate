#include "..\script_component.hpp"

/*
 * File: fnc_createCommander.sqf
 * Description: Constructor for the Commander service object using createHashMapObject.
 *              Manages AI force spawning, threat assessment, and objective targeting.
 */

params [["_config", createHashMap, [createHashMap]]];

GVAR(base_service) = createHashMapFromArray [
    ["#type", "Commander"],

    // Default Attributes
    ["running", false],
    ["lastReassignAt", 0],
    ["lastSpawnConsiderAt", 0],
    ["config", createHashMap],
    ["objectives", []],
    ["activeGroups", []],
    ["virtualGroups", []],
    ["objectivePriority", createHashMap],
    ["lastThreat", 0],

    ["infantryClass", ""],
    ["armorClass", ""],
    ["supportClass", ""],
    ["crewClass", ""],

    // --- Utility Methods ---

    ["ensureObjectives", {
        params ["_self"];

        private _objectives = _self get "objectives";
        if (_objectives isNotEqualTo []) exitWith { _objectives };

        _objectives = [] call FUNC(getObjectivesFallback);
        _self set ["objectives", _objectives];
        _objectives
    }],

    // --- Lifecycle Methods ---

    ["start", {
        params ["_self"];

        if (_self get "running") exitWith { false };
        _self set ["running", true];

        _self call ["resolveFactionBlueprints", []];
        _self call ["ensureObjectives", []];
        _self set ["activeGroups", []];
        _self set ["virtualGroups", []];
        _self set ["objectivePriority", createHashMap];
        _self set ["lastThreat", 0];

        [_self get "threatLoop", [_self], 0] call CFUNC(waitAndExecute);
        [_self get "battleLoop", [_self], 0] call CFUNC(waitAndExecute);
        [_self get "spawnerLoop", [_self], 0] call CFUNC(waitAndExecute);
        [_self get "virtualizationLoop", [_self], 0] call CFUNC(waitAndExecute);
        [_self get "groupMaintenanceLoop", [_self], 0] call CFUNC(waitAndExecute);

        true;
    }],
    ["stop", {
        params ["_self"];
        _self set ["running", false];
        true;
    }],
    ["destroy", {
        params ["_self"];
        _self set ["running", false];

        private _groups = _self get "activeGroups";
        {
            if (isNull _x) then { continue };
            private _vehicles = [];
            { _vehicles pushBackUnique (vehicle _x); } forEach (units _x);
            { deleteVehicle _x; } forEach (units _x);
            { if (_x isNotEqualTo (vehicle _x)) then { deleteVehicle _x; }; } forEach _vehicles;
            deleteGroup _x;
        } forEach _groups;

        _self set ["activeGroups", []];
        _self set ["virtualGroups", []];
        _self set ["objectives", []];
        _self set ["objectivePriority", createHashMap];
        true;
    }],

    // --- Core Loops ---

    ["threatLoop", {
        params ["_self"];

        if !(_self get "running") exitWith {};

        private _t = _self call ["computeThreat", []];
        _self set ["lastThreat", _t];

        private _cadence = (_self get "config") getOrDefault ["threatRecalcCadence", 10];
        [_self get "threatLoop", [_self], _cadence] call CFUNC(waitAndExecute);
    }],
    ["battleLoop", {
        params ["_self"];

        if !(_self get "running") exitWith {};

        private _cfg = _self get "config";
        private _reassignCadence = _cfg getOrDefault ["reassignCadence", 20];

        _self call ["updateObjectivePriority", []];
        private _target = _self call ["selectObjective", []];

        if (time - (_self get "lastReassignAt") > (_cfg getOrDefault ["minReassignInterval", 60])) then {
            [_self, _target] call FUNC(reassignGroups);
            _self set ["lastReassignAt", time];
        };

        [_self get "battleLoop", [_self], _reassignCadence] call CFUNC(waitAndExecute);
    }],
    ["spawnerLoop", {
        params ["_self"];

        if !(_self get "running") exitWith {};

        private _cfg = _self get "config";
        private _now = time;

        // Guard: cadence not yet elapsed
        if ((_now - (_self get "lastSpawnConsiderAt")) < (_cfg getOrDefault ["spawnConsiderCadence", 45])) exitWith {
            [_self get "spawnerLoop", [_self], 1] call CFUNC(waitAndExecute);
        };

        _self set ["lastSpawnConsiderAt", _now];

        // Guard: no valid enemy side
        private _enemySide = [] call FUNC(findEnemySide);
        if (_enemySide isEqualTo sideUnknown) exitWith {
            [_self get "spawnerLoop", [_self], 5] call CFUNC(waitAndExecute);
        };

        _self call ["ensureObjectives", []];

        private _target = _self call ["selectObjective", []];
        private _threat = _self get "lastThreat";

        // Filter to living groups
        private _active = _self get "activeGroups";
        private _living = _active select {
            private _g = _x;
            !isNull _g && { { alive _x } count (units _g) > 0 }
        };

        // Tally group types
        private _maxInf = _cfg getOrDefault ["maxInfantryGroups", 6];
        private _maxArmor = _cfg getOrDefault ["maxArmorGroups", 2];
        private _maxSup = _cfg getOrDefault ["maxSupportGroups", 2];

        private _infCount = 0;
        private _armorCount = 0;
        private _supCount = 0;

        {
            private _leader = leader _x;
            private _veh = vehicle _leader;

            if (isNull _veh || { _veh isEqualTo _leader }) then {
                _infCount = _infCount + 1;
                continue;
            };

            private _vehType = typeOf _veh;
            if (_vehType isKindOf "APC" || { _vehType isKindOf "Tank" }) then {
                _armorCount = _armorCount + 1;
            } else {
                _supCount = _supCount + 1;
            };
        } forEach _living;

        // Tally virtual groups
        private _virtualGroups = _self getOrDefault ["virtualGroups", []];
        {
            private _type = _x get "type";
            if (_type isEqualTo "INFANTRY") then { _infCount = _infCount + 1; };
            if (_type isEqualTo "ARMOR") then { _armorCount = _armorCount + 1; };
            if (_type isEqualTo "SUPPORT") then { _supCount = _supCount + 1; };
        } forEach _virtualGroups;

        // Build list of asset types with available slots
        private _availableTypes = [];
        if (_armorCount < _maxArmor) then { _availableTypes pushBack "ARMOR"; };
        if (_infCount < _maxInf) then { _availableTypes pushBack "INFANTRY"; };
        if (_supCount < _maxSup) then { _availableTypes pushBack "SUPPORT"; };
        if (_availableTypes isNotEqualTo []) then {
            private _preferred = _self call ["pickAssetType", [_threat]];
            private _type = if (_preferred in _availableTypes) then { _preferred } else { _availableTypes select 0 };
            private _g = _self call ["trySpawnAsset", [_enemySide, _target, _type]];
            if !(isNull _g) then { _living pushBackUnique _g; };
        };

        _self set ["activeGroups", _living];
        [_self get "spawnerLoop", [_self], 2] call CFUNC(waitAndExecute);
    }],

    // --- Configuration Resolution ---

    ["resolveFactionBlueprints", {
        params ["_self"];

        private _factionStr = GETMVAR(ENEMY_FACTION_STR,"");
        if (_factionStr isEqualTo "") exitWith {};

        private _pathRoot = configFile >> "CfgFactionUnitMap";
        if !(isClass _pathRoot) exitWith {};

        private _factionCfg = _pathRoot >> _factionStr;
        if !(isClass _factionCfg) exitWith {};

        {
            _x params ["_category", "_attr"];
            if !(isClass (_factionCfg >> _category)) then { continue };

            private _classes = getArray (_factionCfg >> _category >> "classes");
            private _class = if (_classes isNotEqualTo []) then {
                selectRandom _classes
            } else {
                getText (_factionCfg >> _category >> "class")
            };

            if (_class isNotEqualTo "") then { _self set [_attr, _class]; };
        } forEach [
            ["infantry", "infantryClass"],
            ["armor", "armorClass"],
            ["support", "supportClass"]
        ];
    }],

    // --- Threat & Objective Assessment ---

    ["pickAssetType", {
        params ["_self", "_threat"];

        private _cfg = _self get "config";
        if (_threat >= (_cfg getOrDefault ["highThreatThreshold", 0.65])) exitWith { "ARMOR" };
        if (_threat >= (_cfg getOrDefault ["medThreatThreshold", 0.35])) exitWith {
            if (random 1 < 0.6) exitWith { "INFANTRY" };
            "SUPPORT"
        };

        "INFANTRY"
    }],
    ["computeThreat", {
        params ["_self"];

        private _objectives = _self call ["ensureObjectives", []];
        private _players = allPlayers select { isPlayer _x && {alive _x} };
        private _playerCount = count _players;
        private _minDist = 1e9;

        {
            private _player = _x;
            {
                private _d = _player distance2D _x;
                if (_d < _minDist) then { _minDist = _d };
            } forEach _objectives;
        } forEach _players;

        if (_minDist isEqualTo 1e9) then { _minDist = 5000 };

        private _proximityScore = 1 - CLAMP(_minDist / 5000,0,1);
        private _densityScore = CLAMP(_playerCount / 10,0,1);

        (0.65 * _proximityScore) + (0.35 * _densityScore)
    }],
    ["updateObjectivePriority", {
        params ["_self"];

        private _objectives = _self get "objectives";
        if (_objectives isEqualTo []) exitWith {};

        private _players = allPlayers select { isPlayer _x && {alive _x} };
        private _priority = createHashMap;

        {
            private _pos = _x;
            private _minDist = 1e9;

            {
                private _d = _x distance2D _pos;
                if (_d < _minDist) then { _minDist = _d };
            } forEach _players;

            if (_minDist isEqualTo 1e9) then { _minDist = 5000 };

            private _score = 1 - CLAMP(_minDist / 5000,0,1);
            private _key = format ["%1_%2", round (_pos select 0), round (_pos select 1)];
            _priority set [_key, _score];
        } forEach _objectives;

        _self set ["objectivePriority", _priority];
    }],
    ["selectObjective", {
        params ["_self"];

        private _objectives = _self get "objectives";
        if (_objectives isEqualTo []) exitWith {[0,0,0]};

        private _priority = _self get "objectivePriority";
        private _best = [];
        private _bestScore = -1;

        {
            private _pos = _x;
            private _key = format ["%1_%2", round (_pos select 0), round (_pos select 1)];
            private _score = _priority getOrDefault [_key, 0];
            if (_score > _bestScore) then {
                _bestScore = _score;
                _best = _pos;
            };
        } forEach _objectives;

        if (_best isEqualTo []) then { selectRandom _objectives } else { _best };
    }],

    // --- Spawning ---

    ["trySpawnAsset", {
        params ["_self", "_enemySide", "_objectivePos", "_type"];

        if (_enemySide isEqualTo sideUnknown) exitWith { objNull };
        if (isNil "_objectivePos") exitWith { objNull };

        private _cfg = _self get "config";
        private _spawnMin = _cfg getOrDefault ["spawnMinDistance", 800];
        private _spawnMax = _cfg getOrDefault ["spawnMaxDistance", 2200];

        private _spawnDir = random 360;
        private _spawnDist = (_spawnMin + random ((_spawnMax - _spawnMin) max 1));

        private _spawnPos = [
            (_objectivePos select 0) + (_spawnDist * (sin _spawnDir)),
            (_objectivePos select 1) + (_spawnDist * (cos _spawnDir)),
            0
        ];

        private _size = 8;
        switch (_type) do {
            case "INFANTRY": {
                _size = _cfg getOrDefault ["infantryGroupSize", 8];
            };
            case "ARMOR": {
                _size = _cfg getOrDefault ["armorCrewGroupSize", 4];
            };
            default {
                _size = _cfg getOrDefault ["supportGroupSize", 6];
            };
        };

        private _speed = 8;
        if (_type isEqualTo "ARMOR") then {
            _speed = _cfg getOrDefault ["vehicleTravelSpeed", 20];
        } else {
            _speed = _cfg getOrDefault ["infantryTravelSpeed", 8];
        };

        private _vGroup = createHashMapFromArray [
            ["type", _type],
            ["side", _enemySide],
            ["spawnPos", _spawnPos],
            ["sectorPos", _objectivePos],
            ["size", _size],
            ["createdAt", time],
            ["speed", _speed]
        ];

        private _vGroups = _self getOrDefault ["virtualGroups", []];
        _vGroups pushBack _vGroup;
        _self set ["virtualGroups", _vGroups];

        objNull
    }],
    ["virtualizationLoop", {
        params ["_self"];

        if !(_self get "running") exitWith {};

        private _cfg = _self get "config";
        private _distThreshold = _cfg getOrDefault ["virtualizationDistance", 2000];
        private _hysteresis = _cfg getOrDefault ["virtualizationHysteresis", 500];
        private _dematerializeThreshold = _distThreshold + _hysteresis;
        private _cadence = _cfg getOrDefault ["virtualizationCheckCadence", 3];

        private _players = allPlayers select { isPlayer _x && {alive _x} };

        // 1. Check virtual groups for materialization
        private _vGroups = +(_self getOrDefault ["virtualGroups", []]);
        {
            private _interpolatedPos = _self call ["interpolateGroupPos", [_x]];
            private _minDist = 1e9;
            {
                private _d = _x distance2D _interpolatedPos;
                if (_d < _minDist) then { _minDist = _d };
            } forEach _players;

            if (_minDist <= _distThreshold) then {
                _self call ["materializeGroup", [_x, false]];
            } else {
                private _sectorPos = _x get "sectorPos";
                private _distToSector = _interpolatedPos distance2D _sectorPos;
                private _type = _x get "type";
                if (_distToSector <= 50 && { _type isNotEqualTo "ARMOR" }) then {
                    _self call ["materializeGroup", [_x, true]];
                };
            };
        } forEach _vGroups;

        // 2. Check active groups for dematerialization/leader-only reduction
        private _active = +(_self getOrDefault ["activeGroups", []]);
        {
            private _group = _x;
            if (isNull _group) then { continue };

            private _units = units _group select { alive _x };
            if (count _units isEqualTo 0) then { continue };

            private _leader = leader _group;
            if (behaviour _leader isEqualTo "COMBAT") then { continue };

            private _currentPos = getPosATL _leader;
            private _minDist = 1e9;
            {
                private _d = _x distance2D _currentPos;
                if (_d < _minDist) then { _minDist = _d };
            } forEach _players;

            private _isLeaderOnly = _group getVariable [QGVAR(leaderOnly), false];

            if (_players isNotEqualTo [] && { _minDist <= _distThreshold }) then {
                if (_isLeaderOnly) then { _self call ["materializeSquad", [_group]]; };
            } else {
                if (_players isEqualTo [] || { _minDist > _dematerializeThreshold }) then {
                    private _sectorTarget = _group getVariable [QGVAR(sectorTarget), [0,0,0]];
                    private _type = _group getVariable [QGVAR(groupType), "INFANTRY"];
                    private _distToSector = _currentPos distance2D _sectorTarget;

                    if (_distToSector <= 200 && { _type isNotEqualTo "ARMOR" }) then {
                        if (!_isLeaderOnly) then {
                            private _squad = _units - [_leader];
                            private _vehicles = [];
                            { _vehicles pushBackUnique (vehicle _x); } forEach _squad;
                            { deleteVehicle _x; } forEach _squad;
                            { if (_x isNotEqualTo (vehicle _x) && { _x isNotEqualTo (vehicle _leader) }) then { deleteVehicle _x; }; } forEach _vehicles;

                            private _origSize = _group getVariable [QGVAR(originalSize), count _units];
                            _group setVariable [QGVAR(squadSizeRemaining), (_origSize - 1) max 0];
                            _group setVariable [QGVAR(leaderOnly), true];
                        };
                    } else {
                        _self call ["dematerializeGroup", [_group]];
                    };
                };
            };
        } forEach _active;

        [_self get "virtualizationLoop", [_self], _cadence] call CFUNC(waitAndExecute);
    }],
    ["interpolateGroupPos", {
        params ["_self", "_vGroup"];

        private _spawnPos = _vGroup get "spawnPos";
        private _sectorPos = _vGroup get "sectorPos";
        private _createdAt = _vGroup get "createdAt";
        private _speed = _vGroup get "speed";

        private _elapsed = (time - _createdAt) max 0;
        private _distTraveled = _elapsed * _speed;
        private _totalDist = _spawnPos distance2D _sectorPos;

        if (_totalDist <= 0) exitWith { _sectorPos };

        private _ratio = (_distTraveled / _totalDist) min 1;
        private _x = (_spawnPos select 0) + (((_sectorPos select 0) - (_spawnPos select 0)) * _ratio);
        private _y = (_spawnPos select 1) + (((_sectorPos select 1) - (_spawnPos select 1)) * _ratio);

        [_x, _y, 0]
    }],
    ["materializeGroup", {
        params ["_self", "_vGroup", ["_leaderOnly", false, [true]]];

        private _type = _vGroup get "type";
        private _side = _vGroup get "side";
        private _sectorPos = _vGroup get "sectorPos";
        private _size = _vGroup get "size";

        private _spawnSize = [_size, 1] select _leaderOnly;
        private _interpolatedPos = _self call ["interpolateGroupPos", [_vGroup]];

        private _group = objNull;
        switch (_type) do {
            case "INFANTRY": {
                _group = _self call ["spawnInfantryGroup", [_side, _interpolatedPos, _sectorPos, _spawnSize]];
            };
            case "ARMOR": {
                _group = _self call ["spawnArmorGroup", [_side, _interpolatedPos, _sectorPos, _size]];
            };
            default {
                _group = _self call ["spawnSupportGroup", [_side, _interpolatedPos, _sectorPos, _spawnSize]];
            };
        };

        if !(isNull _group) then {
            _group setVariable [QGVAR(groupType), _type];
            _group setVariable [QGVAR(sectorTarget), _sectorPos];
            _group setVariable [QGVAR(patrolAssigned), false];
            _group setVariable [QGVAR(leaderOnly), _leaderOnly];
            _group setVariable [QGVAR(squadSizeRemaining), if (_leaderOnly) then { _size - 1 } else { 0 }];
            _group setVariable [QGVAR(originalSize), _size];

            private _active = _self get "activeGroups";
            _active pushBackUnique _group;
            _self set ["activeGroups", _active];
        };

        private _vGroups = _self getOrDefault ["virtualGroups", []];
        _vGroups = _vGroups - [_vGroup];
        _self set ["virtualGroups", _vGroups];

        _group
    }],
    ["materializeSquad", {
        params ["_self", "_group"];

        if (isNull _group) exitWith {};

        private _type = _group getVariable [QGVAR(groupType), "INFANTRY"];
        private _squadSize = _group getVariable [QGVAR(squadSizeRemaining), 0];
        if (_squadSize <= 0) exitWith {};

        private _leader = leader _group;
        if (isNull _leader || { !alive _leader }) exitWith {};

        private _leaderPos = getPosATL _leader;

        private _unitClass = _self get "infantryClass";
        if (_unitClass isNotEqualTo "") then {
            for "_i" from 1 to _squadSize do {
                private _u = _group createUnit [_unitClass, _leaderPos, [], 10, "NONE"];
                _u setUnitPos "AUTO";
            };
        };

        if (_type isEqualTo "SUPPORT") then {
            private _supportClass = _self get "supportClass";
            private _hasVeh = false;
            {
                if (_x isKindOf "Car" || { _x isKindOf "Tank" || { _x isKindOf "StaticWeapon" } }) exitWith { _hasVeh = true; };
            } forEach (assignedVehicles _group + [vehicle _leader]);

            if (_supportClass isNotEqualTo "" && { !_hasVeh }) then {
                private _support = createVehicle [_supportClass, _leaderPos, [], 10, "NONE"];
                _support setFuel 1;
                _group addVehicle _support;
            };
        };

        _group setVariable [QGVAR(leaderOnly), false];
        _group setVariable [QGVAR(squadSizeRemaining), 0];
    }],
    ["dematerializeGroup", {
        params ["_self", "_group"];

        if (isNull _group) exitWith {};

        private _type = _group getVariable [QGVAR(groupType), "INFANTRY"];
        private _sectorTarget = _group getVariable [QGVAR(sectorTarget), [0,0,0]];
        private _units = units _group select { alive _x };
        private _size = count _units;
        if (_size isEqualTo 0) exitWith {
            private _active = _self get "activeGroups";
            _active = _active - [_group];
            _self set ["activeGroups", _active];
        };

        private _origSize = _group getVariable [QGVAR(originalSize), _size];
        private _side = side _group;

        private _sumPos = [0,0,0];
        { _sumPos = _sumPos vectorAdd (getPosATL _x); } forEach _units;

        private _currentPos = _sumPos vectorMultiply (1 / _size);
        private _vehicles = [];
        { _vehicles pushBackUnique (vehicle _x); } forEach _units;
        { deleteVehicle _x; } forEach _units;
        { if (_x isNotEqualTo (vehicle _x)) then { deleteVehicle _x; }; } forEach _vehicles;
        deleteGroup _group;

        private _cfg = _self get "config";
        private _speed = 8;
        if (_type isEqualTo "ARMOR") then {
            _speed = _cfg getOrDefault ["vehicleTravelSpeed", 20];
        } else {
            _speed = _cfg getOrDefault ["infantryTravelSpeed", 8];
        };

        private _vGroup = createHashMapFromArray [
            ["type", _type],
            ["side", _side],
            ["spawnPos", _currentPos],
            ["sectorPos", _sectorTarget],
            ["size", _origSize],
            ["createdAt", time],
            ["speed", _speed]
        ];

        private _vGroups = _self getOrDefault ["virtualGroups", []];
        _vGroups pushBack _vGroup;
        _self set ["virtualGroups", _vGroups];

        private _active = _self get "activeGroups";
        _active = _active - [_group];
        _self set ["activeGroups", _active];
    }],
    ["groupMaintenanceLoop", {
        params ["_self"];

        if !(_self get "running") exitWith {};

        private _cfg = _self get "config";
        private _arrivalDistance = _cfg getOrDefault ["arrivalDistance", 200];
        private _patrolInf = _cfg getOrDefault ["patrolRadiusInfantry", 200];
        private _patrolArm = _cfg getOrDefault ["patrolRadiusArmor", 400];
        private _patrolSup = _cfg getOrDefault ["patrolRadiusSupport", 300];

        private _active = _self getOrDefault ["activeGroups", []];
        {
            private _group = _x;
            if (isNull _group) then { continue };

            if (_group getVariable [QGVAR(patrolAssigned), false]) then { continue };

            private _leader = leader _group;
            if (isNull _leader || { !alive _leader }) then { continue };

            private _sectorPos = _group getVariable [QGVAR(sectorTarget), [0,0,0]];
            if (_sectorPos isEqualTo [0,0,0]) then { continue };

            private _dist = _leader distance2D _sectorPos;
            if (_dist <= _arrivalDistance) then {
                private _type = _group getVariable [QGVAR(groupType), "INFANTRY"];
                private _radius = _patrolInf;
                if (_type isEqualTo "ARMOR") then { _radius = _patrolArm; };
                if (_type isEqualTo "SUPPORT") then { _radius = _patrolSup; };

                while { waypoints _group isNotEqualTo [] } do {
                    deleteWaypoint [_group, 0];
                };

                [_group, _sectorPos, _radius] call CBA_fnc_taskPatrol;
                _group setVariable [QGVAR(patrolAssigned), true];
            };
        } forEach _active;

        [_self get "groupMaintenanceLoop", [_self], 5] call CFUNC(waitAndExecute);
    }],
    ["spawnInfantryGroup", {
        params ["_self", "_enemySide", "_spawnPos", "_sectorPos", "_groupSize"];

        private _class = _self get "infantryClass";
        if (_class isEqualTo "") exitWith { objNull };

        private _group = createGroup _enemySide;

        for "_i" from 1 to _groupSize do {
            private _u = _group createUnit [_class, _spawnPos, [], 0, "NONE"];
            _u setUnitPos "AUTO";
        };

        _group setFormDir (random 360);
        _group setBehaviourStrong "AWARE";
        _group setSpeedMode "NORMAL";
        _group setCombatMode "RED";
        _group move _sectorPos;
        _group;
    }],
    ["spawnArmorGroup", {
        params ["_self", "_enemySide", "_spawnPos", "_sectorPos", "_crewSize"];

        private _vehicleClass = _self get "armorClass";
        if (_vehicleClass isEqualTo "") exitWith { objNull };

        private _veh = createVehicle [_vehicleClass, _spawnPos, [], 0, "NONE"];
        _veh setFuel 1;
        _veh setPosATL _spawnPos;
        _veh setVehicleLock "LOCKED";

        private _group = createGroup _enemySide;
        private _crewClass = _self get "crewClass";
        if (_crewClass isEqualTo "") then { _crewClass = _self get "infantryClass"; };
        if (_crewClass isNotEqualTo "") then {
            for "_i" from 1 to _crewSize do {
                private _crew = _group createUnit [_crewClass, _spawnPos, [], 0, "NONE"];
                _crew moveInAny _veh;
            };
        };

        _group addVehicle _veh;
        _group setBehaviourStrong "AWARE";
        _group setCombatMode "RED";
        _veh doMove _sectorPos;
        _group;
    }],
    ["spawnSupportGroup", {
        params ["_self", "_enemySide", "_spawnPos", "_sectorPos", "_groupSize"];

        private _unitClass = _self get "infantryClass";
        if (_unitClass isEqualTo "") exitWith { objNull };

        private _group = createGroup _enemySide;

        for "_i" from 1 to _groupSize do {
            private _u = _group createUnit [_unitClass, _spawnPos, [], 0, "FORM"];
            _u setUnitPos "AUTO";
        };

        // Attach support vehicle if configured
        private _supportClass = _self get "supportClass";
        if (_supportClass isNotEqualTo "" && { _groupSize > 1 }) then {
            private _support = createVehicle [_supportClass, _spawnPos, [], 0, "NONE"];

            _support setFuel 1;
            _group addVehicle _support;
        };

        _group setBehaviourStrong "AWARE";
        _group setSpeedMode "NORMAL";
        _group setCombatMode "RED";
        _group move _sectorPos;
        _group;
    }]
];

// Load addon defaults from CfgCommander, then merge mission overrides
private _defaultConfig = createHashMapFromArray (getArray (configFile >> "CfgCommander" >> "defaults"));
{ _defaultConfig set [_x, _y]; } forEach _config;

// Instantiate the object
GVAR(service) = createHashMapObject [GVAR(base_service), []];
GVAR(service) set ["config", _defaultConfig];
GVAR(service)
