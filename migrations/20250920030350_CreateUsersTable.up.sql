-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    age INTEGER
);

ALTER TABLE users
    ADD COLUMN email VARCHAR NOT NULL,
    ADD COLUMN password VARCHAR NOT NULL;

ALTER TABLE users
    ADD CONSTRAINT email_is_unique UNIQUE (email);
