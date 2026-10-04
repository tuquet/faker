@echo off
title Build Random User Generator EXE
cd /d "%~dp0"

echo ====================================================
echo   Build file .EXE voi Tauri (Sieu nhe, Native)
echo ====================================================

call npm run build:exe

echo.
echo ====================================================
echo [THANH CONG] File .exe da duoc build trong:
echo   src-tauri\target\release\
echo ====================================================
echo.
pause
