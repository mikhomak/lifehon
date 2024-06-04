CREATE TABLE "l_site_configuration"
(
    id                        SERIAL PRIMARY KEY,
    allow_site_comments       BOOL NOT NULL default true,
    allow_posting             BOOL not null default true,
    allow_registration        BOOL not null default true,
    allow_login               BOOL not null default true,
    allow_exp                 BOOL not null default true,
    allow_hobby_communication BOOL not null default true
);

INSERT INTO l_site_configuration
VALUES (1, true, true, true, true, true, true);

CREATE TABLE "l_hobby"
(
    name       VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT 'NOW'::timestamptz,
    enabled    BOOL        NOT NULL DEFAULT TRUE,
    PRIMARY KEY (name)
);

INSERT INTO l_hobby
VALUES ('habits');

CREATE TABLE "l_user"
(
    id             UUID         NOT NULL        DEFAULT gen_random_uuid(),
    name           VARCHAR(50)  NOT NULL UNIQUE,
    display_name   VARCHAR(50)  NOT NULL UNIQUE,
    email          VARCHAR(254) NOT NULL UNIQUE,
    created_at     TIMESTAMPTZ  NOT NULL        DEFAULT 'NOW'::timestamptz,
    password       VARCHAR(50)  NOT NULL,
    login_enabled  bool         NOT NULL        DEFAULT TRUE,
    consent        bool         NOT NULL        DEFAULT FALSE,
    public_profile bool         NOT NULL        DEFAULT TRUE,
    exp            BIGINT       NOT NULL        DEFAULT 0,
    PRIMARY KEY (name)
);

CREATE TABLE "rel_user2hobby"
(
    hobby_name VARCHAR(50) NOT NULL REFERENCES "l_hobby" (name),
    user_name  VARCHAR(50) NOT NULL REFERENCES "l_user" (name),
    PRIMARY KEY (hobby_name, user_name)
);

CREATE TABLE "l_task"
(
    id          UUID         NOT NULL DEFAULT gen_random_uuid(),
    user_name   VARCHAR(254) NOT NULL REFERENCES "l_user" (name),
    hobby       VARCHAR(50)  NOT NULL REFERENCES "l_hobby" (name),
    external_id VARCHAR(254) NOT NULL,
    name        VARCHAR(50)  NOT NULL,
    description text,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT 'NOW'::timestamptz,
    finished_at TIMESTAMPTZ  NOT NULL DEFAULT 'NOW'::timestamptz,
    given_exp   BIGINT       NOT NULL DEFAULT 0,
    public      bool         NOT NULL DEFAULT FALSE,
    PRIMARY KEY (user_name, external_id)
);

CREATE TABLE "l_post"
(
    id             UUID        NOT NULL DEFAULT gen_random_uuid(),
    title          VARCHAR(50) NOT NULL,
    text           text,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT 'NOW'::timestamptz,
    likes          bigint      NOT NULL DEFAULT 0,
    rating         float8      NOT NULL DEFAULT 0,
    user_name      VARCHAR(50)        NOT NULL REFERENCES "l_user" (name),
    allow_comments BOOL        NOT NULL default true,
    allow_likes    BOOL        NOT NULL default true,
    PRIMARY KEY (id)
);
