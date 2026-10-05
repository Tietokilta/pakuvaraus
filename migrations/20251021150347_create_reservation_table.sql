CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    full_name TEXT NOT NULL,
    email TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    internal BOOLEAN NOT NULL,
    approved BOOLEAN NOT NULL DEFAULT false,
    signed BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT start_before_end CHECK (start_time < end_time),
    CONSTRAINT block_overlapping_times
    EXCLUDE USING gist (
        tstzrange(start_time, end_time, '[)') WITH &&
    )
    WHERE (internal = true)
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

insert into users (username, password)
values ('test-user', '$argon2id$v=19$m=19456,t=2,p=1$SfdB+aDttM1CImYRHZz77A$GrbTi6iHIqIbV0rIg/Fy62mhdMvT/K75dfuVhafv/HE');
