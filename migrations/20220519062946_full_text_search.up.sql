CREATE VIRTUAL TABLE posts_fts USING fts5(
    sub,
    com,
    content=posts
);

CREATE TRIGGER posts_fts_insert AFTER INSERT ON posts
BEGIN
    INSERT INTO posts_fts (rowid, sub, com) VALUES (new.rowid, new.sub, new.com);
END;

CREATE TRIGGER posts_fts_delete AFTER DELETE ON posts
BEGIN
    INSERT INTO posts_fts (posts_fts, rowid, sub, com) VALUES ('delete', old.rowid, old.sub, old.com);
END;

CREATE TRIGGER posts_fts_update AFTER UPDATE ON posts
BEGIN
    INSERT INTO posts_fts (posts_fts, rowid, sub, com) VALUES ('delete', old.rowid, old.sub, old.com);
    INSERT INTO posts_fts (rowid, sub, com) VALUES (new.rowid, new.sub, new.com);
END;
