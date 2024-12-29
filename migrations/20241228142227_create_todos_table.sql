-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    age INTEGER NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL
);