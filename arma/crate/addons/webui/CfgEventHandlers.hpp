class Extended_PreStart_EventHandlers {
    class ADDON {
        init = QUOTE(call COMPILE_SCRIPT(XEH_preStart));
    };
};

class Extended_PreInit_EventHandlers {
    class ADDON {
        init = QUOTE(call COMPILE_SCRIPT(XEH_preInit));
        clientInit = QUOTE(call COMPILE_SCRIPT(XEH_preInitClient));
        serverInit = QUOTE(call COMPILE_SCRIPT(XEH_preInitServer));
    };
};
