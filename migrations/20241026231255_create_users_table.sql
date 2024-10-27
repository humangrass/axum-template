CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    username      VARCHAR(255)                          NOT NULL UNIQUE,
    email         VARCHAR(255)                          NOT NULL UNIQUE,
    status        VARCHAR(50)                           NOT NULL,
    password_hash TEXT                                  NOT NULL,
    created_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
