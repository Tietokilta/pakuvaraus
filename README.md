# Pakuvaraus

## Test locally

create .env with DATABASE_URL variable set, which includes the password.
    DATABASE_URL=postgres://postgres:password123@localhost:5432/postgres

run postgres, for example with docker
    docker run --name postgres -e POSTGRES_PASSWORD=password123 -p 5432:5432 -d postgres

set up the database, install sqlx-cli to run the commands
    sqlx database create
    sqlx migrate run

add and get reservations in powershell:

$body = @{
    reservation_name = "Test name 1"
    email = "test@example.com"
    start_time = "2025-10-20T10:00:00.000000Z"
    end_time = "2025-10-20T12:00:00.000000Z"
}

Invoke-RestMethod -Uri http://127.0.0.1:3000/reservations -Method Post -ContentType "application/json" -Body ($body | ConvertTo-Json)

Invoke-RestMethod -Uri http://127.0.0.1:3000/reservations -Method Get

in browser open
    http://127.0.0.1:3000/login

sqlx can check queries at compile time, but this requires a postgres instance to be running, if such is not available, set sqlx to offline mode