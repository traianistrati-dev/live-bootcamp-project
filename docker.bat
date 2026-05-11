@echo off

set ENV_FILE=.\auth-service\.env

if not exist "%ENV_FILE%" (
    echo Error: .env file not found!
    exit /b 1
)

for /f "usebackq tokens=*" %%i in ("%ENV_FILE%") do (
    set %%i
)

docker compose build
docker compose up
