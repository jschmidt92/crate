@ECHO off
setlocal
pushd "%~dp0.."
cargo build --release --manifest-path "%~dp0..\..\..\Cargo.toml" || goto :error
copy /Y "%~dp0..\..\..\target\release\forge_crate_x64.dll" "%~dp0..\forge_crate_x64.dll" >NUL || goto :error
hemtt script update_build.rhai
hemtt script update_patch.rhai
hemtt release
set code=%ERRORLEVEL%
popd
exit /B %code%

:error
set code=%ERRORLEVEL%
popd
exit /B %code%
