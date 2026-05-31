@echo off
setlocal enabledelayedexpansion

set "ROOT_DIR=%CD%"
echo [build_all] FlowNet build starting from: !ROOT_DIR!
echo.
echo [build_all] [1/4] Checking Npcap SDK...
if not defined NPCAP_SDK_PATH (
  call local_build_config.bat
  if errorlevel 1 (
    echo [build_all] ERROR: Npcap SDK not found.
    echo [build_all] Please install from https://npcap.com/ then re-run.
    echo [build_all] Or set NPCAP_SDK_PATH manually:
    echo [build_all]   set NPCAP_SDK_PATH=C:\path\to\NpcapSDK
    pause
    exit /b 1
  )
) else (
  echo [build_all]   NPCAP_SDK_PATH=%NPCAP_SDK_PATH%
)
echo.
echo [build_all] [2/4] Checking prerequisites...

where npm >nul 2>&1
if errorlevel 1 (
  echo [build_all] ERROR: npm not found. Install Node.js from https://nodejs.org/
  pause
  exit /b 1
)
echo [build_all]   npm: found

where cargo >nul 2>&1
if errorlevel 1 (
  echo [build_all] ERROR: cargo not found. Install Rust from https://rustup.rs/
  pause
  exit /b 1
)
echo [build_all]   cargo: found
echo.
echo [build_all] [3/4] Building frontend (ui/)...

if not exist "!ROOT_DIR!\ui\package.json" (
  echo [build_all] ERROR: ui\package.json not found.
  echo [build_all] Make sure you are running this script from the project root.
  pause
  exit /b 1
)

cd /d "!ROOT_DIR!\ui"
if errorlevel 1 (
  echo [build_all] ERROR: Cannot change to ui\ directory.
  pause
  exit /b 1
)

echo [build_all]   npm install...
call npm install
if errorlevel 1 (
  echo [build_all] ERROR: npm install failed.
  pause
  exit /b 1
)

echo [build_all]   npm run build...
call npm run build
if errorlevel 1 (
  echo [build_all] ERROR: npm run build failed.
  pause
  exit /b 1
)

cd /d "!ROOT_DIR!"
echo.
echo [build_all]   Frontend built successfully.
echo.
echo [build_all] [4/4] Building Rust (cargo build --workspace --release)...

cargo build --workspace --release
if errorlevel 1 (
  echo [build_all] ERROR: cargo build failed.
  pause
  exit /b 1
)

echo.
echo [build_all] ===========================================================
echo [build_all]  Build complete!
echo [build_all]   Frontend: ui\dist\
echo [build_all]   Rust:     target\release\flownet.exe
echo [build_all]   Tauri:    src-tauri\target\release\
echo [build_all] ===========================================================
echo.

endlocal
