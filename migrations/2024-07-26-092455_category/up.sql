-- Your SQL goes here
CREATE TABLE IF NOT EXISTS  category
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

INSERT INTO category (id, name) VALUES (1, 'manhwa');