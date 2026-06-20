[
    QGVAR(persistPosition),
    "CHECKBOX",
    [LLSTRING(setting_persistPosition), LLSTRING(setting_persistPosition_tooltip)],
    ["Forge", LLSTRING(displayName)],
    true,
    1
] call CFUNC(addSetting);

[
    QGVAR(persistLoadout),
    "CHECKBOX",
    [LLSTRING(setting_persistLoadout), LLSTRING(setting_persistLoadout_tooltip)],
    ["Forge", LLSTRING(displayName)],
    true,
    1
] call CFUNC(addSetting);
