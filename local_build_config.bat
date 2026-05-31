@echo off
setlocal enabledelayedexpansion

if /I "%1"=="--persist" set PERSIST=1

echo [local_build_config] Scanning for Npcap SDK...

set SDK_ROOT=
set SEARCH_PATHS=^
  "%ProgramFiles%\Npcap SDK"^
  "%ProgramFiles%\Npcap"^
  "%ProgramFiles(x86)%\Npcap SDK"^
  "%ProgramFiles(x86)%\Npcap"^
  "%UserProfile%\Npcap SDK"^
  "C:\NpcapSDK"

for %%p in (%SEARCH_PATHS%) do (
  if exist "%%~p\Lib\x64\wpcap.lib" (
    if exist "%%~p\Lib\x64\Packet.lib" (
      set SDK_ROOT=%%~p
      echo [local_build_config]   FOUND: %%~p
      goto :found
    )
  ) else (
    if exist "%%~p" (
      echo [local_build_config]   EXISTS: %%~p ^(no Lib\x64\wpcap.lib^)
    )
  )
)

echo [local_build_config]   NOT FOUND in any standard location.
echo [local_build_config] ^>
echo [local_build_config] ^> Download the Npcap SDK from:
echo [local_build_config] ^>   https://npcap.com/
echo [local_build_config] ^>
echo [local_build_config] ^> After installing, re-run this script or set:
echo [local_build_config] ^>   set NPCAP_SDK_PATH=C:\path\to\NpcapSDK

endlocal & (
  if defined PERSIST (
    echo [local_build_config] Persisting NPCAP_SDK_PATH is skipped (SDK not found).
  )
)
exit /b 1

:found
endlocal & set SDK_ROOT=%SDK_ROOT% & (
  set "NPCAP_SDK_PATH=%SDK_ROOT%"
  if defined PERSIST (
    setx NPCAP_SDK_PATH "%SDK_ROOT%" >nul
    echo [local_build_config] NPCAP_SDK_PATH permanently set to: %SDK_ROOT%
  ) else (
    echo [local_build_config] NPCAP_SDK_PATH=%SDK_ROOT%
  )
  echo [local_build_config] Done.
)
exit /b 0
