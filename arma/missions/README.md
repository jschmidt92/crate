# Forge Mission Config

Copy `forge_mission_config.hpp` into the mission folder and include it from `description.ext`:

```cpp
#include "forge_mission_config.hpp"
```

`CfgMission >> Actor` controls first-login actor defaults. Money values should stay quoted strings so Arma does not round large or precise values before Rust receives them. New players start in the framework default organization.

The default organization is a framework convention. Its id is `default`, and its limited CEO slot is the single placed playable unit with variable name `ceo`.

`CfgMission >> DefaultOrganization` controls default org starting state. Money values should stay quoted strings. Use `startingBank` for org funds, `paydayAmount` for the default org payday amount, `Locker` for default organization virtual arsenal unlocks, and `VirtualGarage` for default organization virtual garage unlocks.

`CfgMission >> Services` controls service charges. Money values should stay quoted strings. Refuel uses per-liter prices, repair uses a full-repair price multiplied by vehicle damage, rearm uses a per-unit price, and medical uses fixed respawn/full-heal prices. Set any value to `"0.00"` for a no-cost service.

Virtual arsenal unlocks use the mission config label `Locker`, matching the previous framework shape. Use category arrays under `CfgMission >> Actor >> Locker`, for example `weapons[] = {"hgun_P07_F"};`.

Virtual garage unlocks are category arrays under `CfgMission >> Actor >> VirtualGarage`, for example `cars[] = {"B_Quadbike_01_F"};`.
