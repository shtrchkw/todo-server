-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(250) NOT NULL, -- argon hash
    created_at TIMESTAMP NOT NULL
);