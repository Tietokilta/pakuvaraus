CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    reservation_name TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    CONSTRAINT block_overlapping_times
    EXCLUDE USING gist (
        tstzrange(start_time, end_time) WITH &&
    )
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

insert into users (id, username, password)
values (1, 'test-user', 'password');