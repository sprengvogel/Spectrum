-- Your SQL goes here
CREATE TABLE fileentries (
    id INTEGER NOT NULL PRIMARY KEY,
    filename TEXT NOT NULL,
    file_last_modified TIMESTAMP
)