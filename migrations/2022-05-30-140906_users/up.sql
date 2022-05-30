-- Your SQL goes here
CREATE TABLE users
(
    id           SERIAL PRIMARY KEY,
    username     VARCHAR NOT NULL,
    email        VARCHAR NOT NULL,
    date_created TIMESTAMP not null default CURRENT_TIMESTAMP
)
