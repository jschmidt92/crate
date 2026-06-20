@ECHO off
setlocal
pushd "%~dp0.."
cargo build --release --manifest-path "%~dp0..\..\..\Cargo.toml" || goto :error
copy /Y "%~dp0..\..\..\target\release\forge_crate_x64.dll" "%~dp0..\forge_crate_x64.dll" >NUL || goto :error
hemtt check || goto :error
hemtt build || goto :error
popd
exit /B 0

:error
set code=%ERRORLEVEL%
popd
exit /B %code%
