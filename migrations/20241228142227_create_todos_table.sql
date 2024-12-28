-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
    id VARCHAR(255) PRIMARY KEY,
    text TEXT NOT NULL,
    completed BOOLEAN NOT NULL
);