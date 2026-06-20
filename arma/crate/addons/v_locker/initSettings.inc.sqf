[
    QGVAR(enabled),
    "CHECKBOX",
    [LLSTRING(setting_enabled), LLSTRING(setting_enabled_tooltip)],
    ["Forge", LLSTRING(displayName)],
    true,
    1
] call CFUNC(addSetting);

[
    QGVAR(persistenceEnabled),
    "CHECKBOX",
    [LLSTRING(setting_persistenceEnabled), LLSTRING(setting_persistenceEnabled_tooltip)],
    ["Forge", LLSTRING(displayName)],
    true,
    1
] call CFUNC(addSetting);
