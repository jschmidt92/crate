#define FORGE_WEBUI_IDD 78000
#define FORGE_WEBUI_BROWSER_IDC 78001

class RscText;
class Forge_WebUI_Display {
    idd = FORGE_WEBUI_IDD;
    movingEnable = 0;
    enableSimulation = 1;
    onLoad = QUOTE(uiNamespace setVariable [ARR_2(QQGVAR(display),_this select 0)]);
    onUnload = QUOTE(uiNamespace setVariable [ARR_2(QQGVAR(display),displayNull)]);

    class ControlsBackground {
        class Background: RscText {
            idc = -1;
            x = "safeZoneX";
            y = "safeZoneY";
            w = "safeZoneW";
            h = "safeZoneH";
            colorBackground[] = {0.02, 0.03, 0.03, 0.96};
        };
    };

    class Controls {
        class Browser: RscText {
            type = 106;
            idc = FORGE_WEBUI_BROWSER_IDC;
            x = "safeZoneXAbs";
            y = "safeZoneY";
            w = "safeZoneWAbs";
            h = "safeZoneH";
            colorBackground[] = {0, 0, 0, 0};
        };
    };
};
