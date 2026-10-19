# Pakuvaraus

## Developing

1. Create an .env file. You can use [.env.example](./.env.example) file as a reference:

```sh
cp .env.example .env
```

The following parameters are required: `DATABASE_URL`, `EMAIL_ENABLED`, `APPROVER_EMAIL`.

2. Start the Postgres database

You can use docker compose ([docker-compose.yml](./docker-compose.yml)):

```sh
docker compose up
```

Or run docker directly:
```
docker run --name postgres -e POSTGRES_PASSWORD=password123 -p 5432:5432 -d postgres
```

3. Run the migrations

This will connect to the database set in `DATABASE_URL`. At build time, sqlx will validate the queries against the database.

```sh
# install sqlx-cli
cargo install sqlx-cli --no-default-features --features native-tls,postgres

sqlx database create
sqlx migrate run
```

4. Start the application

```sh
cargo run
```

The web UI is available at http://127.0.0.1:3000/login

Default username: `test-user`, default password: `test123`

## API

Example using powershell

```powershell
$body = @{
    full_name = "Test name 1"
    address = "Konemiehentie 2"
    email = "test@example.com"
    phone_number = "040 1234567"
    hetu = "010100+002H"
    start_time = "2025-10-20T10:00:00.000000Z"
    end_time = "2025-10-20T12:00:00.000000Z"
    internal = $true
    used_outside_finland = $false
}

Invoke-RestMethod -Uri http://127.0.0.1:3000/reservations -Method Post -ContentType "application/json" -Body ($body | ConvertTo-Json)

Invoke-RestMethod -Uri http://127.0.0.1:3000/reservations -Method Get
```

Example with `curl`:

```sh
curl 'http://127.0.0.1:3000/reservations' \
    -H 'Content-Type: application/json' \
    -d '{
        "full_name": "Test name 1",
        "address": "Konemiehentie 2",
        "email": "test@example.com",
        "phone_number": "040 1234567",
        "hetu": "010100+002H",
        "start_time": "2025-10-20T10:00:00.000000Z",
        "end_time": "2025-10-20T12:00:00.000000Z",
        "internal": true,
        "used_outside_finland": false
    }'
```
Invalid input is rejected with `422 Unprocessable Entity` and a plain-text message naming the problem.
```sh
curl 'http://127.0.0.1:3000/reservations'
```
