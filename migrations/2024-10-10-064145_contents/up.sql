-- Your SQL goes here

CREATE TABLE IF NOT EXISTS  contents
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    episode_id INTEGER NOT NULL,
    uuid TEXT UNIQUE NOT NULL,
    index_no INTEGER NOT NULL,
    url TEXT NOT NULL,
    ads_url TEXT,
    content_type TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    bytes INTEGER NOT NULL,
    broken_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (episode_id) REFERENCES episode(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);