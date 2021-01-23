DROP TABLE if exists credentials;
DROP TABLE if exists folders;
CREATE TABLE IF NOT EXISTS credentials (
                                           id TEXT PRIMARY KEY ,
                                           name TEXT NOT NULL ,
                                           username TEXT,
                                           notes TEXT,
                                           group_id TEXT,
                                           created TEXT NOT NULL ,
                                           modified TEXT NOT NULL,
                                           expires TEXT,
                                           synced TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS folders (
    id TEXT PRIMARY KEY,
    parent_id TEXT NOT NULL ,
    name TEXT NOT NULL ,
    created TEXT NOT NULL ,
    modified TEXT NOT NULL,
    expires TEXT,
    synced TEXT NOT NULL
);
