CREATE TABLE songs_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id TEXT NOT NULL,
    day INTEGER NOT NULL,
    month INTEGER NOT NULL
);

INSERT INTO songs_new (file_id, day, month)
SELECT fileID, day, month
FROM songs;

DROP TABLE songs;

ALTER TABLE songs_new RENAME TO songs;
