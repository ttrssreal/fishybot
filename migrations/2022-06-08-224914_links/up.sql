CREATE TABLE links (
    id          INTEGER   PRIMARY KEY AUTOINCREMENT
                          NOT NULL
                          UNIQUE,
    discord_tag CHAR (32) NOT NULL
                          UNIQUE,
    ign         CHAR (16) UNIQUE
                          NOT NULL
);
