@echo off
title Random User Generator Web App
cd /d "%~dp0"

echo ====================================================
echo   Random User Generator - Local Suite
echo ====================================================

if not exist "node_modules\" (
    echo [INFO] Dang cai dat thu vien lan dau...
    call npm install
)

echo [INFO] Dang khoi dong Web Server tai http://localhost:3500 ...
start "" http://localhost:3500
node src/server.js

pause
