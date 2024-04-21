CREATE TABLE IF NOT EXISTS games
(
    id          INTEGER PRIMARY KEY NOT NULL,
    round       INTEGER NOT NULL,
    complete    INTEGER NOT NULL,
    home_team   INTEGER NOT NULL,
    away_team   INTEGER NOT NULL,
    home_score  INTEGER NOT NULL,
    away_score  INTEGER NOT NULL,
    timestr     TEXT
);

CREATE TABLE IF NOT EXISTS alerts
(
    id  INTEGER NOT NULL,
    notification INTEGER NOT NULL
);

