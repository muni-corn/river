CREATE TABLE IF NOT EXISTS "public.task" (
    "id"                SERIAL      NOT NULL    PRIMARY KEY
,   "owner"             INT         NOT NULL    REFERENCES "public.user" (id)
,   "name"              TEXT        NOT NULL
,   "percent_complete"  REAL                                                     -- from 0 to 1
,   "minutes_spent"     INT
,   "was_completed_at"  TIMESTAMPTZ
,   "creation_date"     TIMESTAMPTZ NOT NULL
,   "private"           BOOLEAN
);

CREATE TABLE IF NOT EXISTS "public.user" (
    "id"                SERIAL                  PRIMARY KEY
,   "email"             TEXT        NOT NULL    UNIQUE
,   "first_name"        TEXT        NOT NULL
,   "last_name"         TEXT
,   "display_name"      TEXT
,   "current_task"      INT                     REFERENCES "public.task" (id)    -- null if idle
,   "away_reason"       TEXT                                                     -- null if working
,   "creation_date"     TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS "public.auth" (
    "id"                SERIAL      NOT NULL    PRIMARY KEY
,   "user"              INT         NOT NULL    REFERENCES "public.user" (id)
,   "hash"              TEXT        NOT NULL     
);

CREATE TABLE IF NOT EXISTS "public.task_owners" ( -- created to remove circular foreign keys between task and user
    "id"                SERIAL      NOT NULL    PRIMARY KEY
,   "owner"             INT         NOT NULL    REFERENCES "public.user" (id)
,   "task"              INT         NOT NULL    REFERENCES "public.task" (id)
);

CREATE TABLE IF NOT EXISTS "public.team" (
    "id"                SERIAL      NOT NULL    PRIMARY KEY
,   "creator"           INT         NOT NULL    REFERENCES "public.user" (id)
,   "name"              TEXT        NOT NULL
);

CREATE TABLE IF NOT EXISTS "public.team_members" (
    "id"                SERIAL      NOT NULL    PRIMARY KEY
,   "team"              INT         NOT NULL    REFERENCES "public.team" (id)
,   "member"            INT         NOT NULL    REFERENCES "public.user" (id)
);

CREATE TABLE IF NOT EXISTS "public.history" (
    "id"                SERIAL      NOT NULL    PRIMARY KEY
,   "user"              INT         NOT NULL    REFERENCES "public.user" (id)
,   "action"            TEXT        NOT NULL
,   "time"              TIMESTAMPTZ NOT NULL
,   "private"           BOOLEAN     NOT NULL
,   "related_task"      INT                     REFERENCES "public.task" (id)
);

CREATE SEQUENCE IF NOT EXISTS uuid_seq AS int START WITH 1000;
