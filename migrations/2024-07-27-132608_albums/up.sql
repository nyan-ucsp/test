-- Your SQL goes here

CREATE TABLE IF NOT EXISTS albums
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    uuid TEXT UNIQUE NOT NULL,
    category_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0,
    images TEXT NOT NULL,
    tags TEXT,
    enable BOOLEAN NOT NULL DEFAULT 1,
    min_age INTEGER NOT NULL DEFAULT 0,
    url TEXT NOT NULL,
    content_type TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    bytes INTEGER NOT NULL,
    released_at TIMESTAMP,
    broken_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES category(id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);