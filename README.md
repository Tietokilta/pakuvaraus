# Pakuvaraus

## Test locally

create .env with DATABASE_URL variable set, which includes the password.
    DATABASE_URL=postgres://postgres:password123@localhost:5432/postgres

run postgres, for example with docker
    docker run --name postgres -e POSTGRES_PASSWORD=password123 -p 5432:5432 -d postgres

set up the database
    sqlx database create
    sqlx migrate run