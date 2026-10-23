@echo off
rem Builds Flickdeck for Windows and copies the program to dist\Flickdeck\Flickdeck.exe.
cd /d "%~dp0\..\.."
cargo build --release -p flickdeck
if errorlevel 1 exit /b 1
if not exist dist\Flickdeck mkdir dist\Flickdeck
copy /Y target\release\flickdeck.exe dist\Flickdeck\Flickdeck.exe
