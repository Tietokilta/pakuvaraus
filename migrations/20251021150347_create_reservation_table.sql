CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    reservation_name TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL
);