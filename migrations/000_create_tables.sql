CREATE TABLE if not exists files (
    id              INTEGER PRIMARY KEY NOT NULL,
    location_id     INTEGER NOT NULL, 
    filename        TEXT NOT NULL, 
    title           TEXT NOT NULL,
    description     TEXT NOT NULL,
    owner_id        INTEGER NOT NULL
);

CREATE TABLE if not exists locations (
    id              INTEGER PRIMARY KEY NOT NULL, 
    label           TEXT NOT NULL, 
    lat             REAL NOT NULL, 
    lon             REAL NOT NULL,
    kind            TEXT NOT NULL,
    owner_id        INTEGER NOT NULL
);

CREATE TABLE if not exists users (
    id                  INTEGER PRIMARY KEY NOT NULL, 
    username            TEXT NOT NULL, 
    password            TEXT NOT NULL,
    salt                TEXT NOT NULL,
    email               TEXT NOT NULL,
    email_verified      INTEGER NOT NULL,
    totp_secret         TEXT NOT NULL,
    totp_verified       INTEGER NOT NULL,
    registered_date     REAL NOT NULL,
    last_login_date     REAL NOT NULL,
    last_active_date    REAL NOT NULL,
    group_id            INTEGER NOT NULL
);

CREATE TABLE if not exists user_groups (
    id              INTEGER PRIMARY KEY NOT NULL, 
    group_name      TEXT NOT NULL, 
    permissions     TEXT NOT NULL
);

CREATE TABLE if not exists comments (
    id              INTEGER PRIMARY KEY NOT NULL,
    comment         TEXT NOT NULL, 
    location_id     INTEGER NOT NULL, 
    file_id         INTEGER NOT NULL, 
    owner_id        INTEGER NOT NULL,
    reply_to_id     INTEGER NOT NULL,
    posted_date     REAL NOT NULL,
    last_edit_date  REAL NOT NULL
);