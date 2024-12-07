-- Your SQL goes here

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR NOT NULL,
    target VARCHAR NOT NULL,
    raised VARCHAR NOT NULL DEFAULT '0 SOL',
    min_donation DOUBLE PRECISION NOT NULL DEFAULT 0.001,
    wallet VARCHAR NOT NULL
);
