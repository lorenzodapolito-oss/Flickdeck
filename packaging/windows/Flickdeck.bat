@echo off
rem Builds Flickdeck.it for Windows and copies the program to dist\Flickdeck.it\Flickdeck.it.exe.
cd /d "%~dp0\..\.."
cargo build --release -p flickdeck
if errorlevel 1 exit /b 1
if not exist dist\Flickdeck.it mkdir dist\Flickdeck.it
copy /Y target\release\flickdeck.exe dist\Flickdeck.it\Flickdeck.it.exe
