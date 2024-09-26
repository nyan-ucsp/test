-- Your SQL goes here

CREATE TABLE IF NOT EXISTS  episode (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    album_id INTEGER NOT NULL,
    title TEXT,
    uuid TEXT,
    url TEXT,
    broken_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (album_id) REFERENCES album(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);