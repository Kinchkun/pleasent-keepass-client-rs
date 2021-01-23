CREATE TABLE IF NOT EXISTS credentials (
                                           id TEXT PRIMARY KEY ,
                                           name TEXT NOT NULL ,
                                           username TEXT,
                                           notes TEXT,
                                           group_id TEXT,
                                           created TEXT NOT NULL ,
                                           modified TEXT NOT NULL,
                                           expires TEXT,
                                           synced TEXT
);
