CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id          BIGSERIAL PRIMARY KEY,
    public_id   UUID                     NOT NULL DEFAULT uuid_generate_v4(),
    name        VARCHAR                  NOT NULL,
    create_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    update_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON users (public_id);

INSERT INTO users (name) VALUES ('admin');
INSERT INTO users (name) VALUES ('wojtek');