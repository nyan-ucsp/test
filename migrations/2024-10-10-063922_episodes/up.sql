-- Your SQL goes here

CREATE TABLE IF NOT EXISTS  episodes
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    album_id INTEGER NOT NULL,
    uuid TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    url TEXT,
    file_url TEXT,
    content_type TEXT,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    bytes INTEGER NOT NULL,
    broken_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (album_id) REFERENCES album(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);