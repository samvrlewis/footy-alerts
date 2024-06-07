CREATE TABLE IF NOT EXISTS games
(
    id          INTEGER PRIMARY KEY NOT NULL,
    round       INTEGER NOT NULL,
    complete    INTEGER NOT NULL,
    home_team   INTEGER NOT NULL,
    away_team   INTEGER NOT NULL,
    home_score  INTEGER NOT NULL,
    away_score  INTEGER NOT NULL,
    timestr     TEXT,
    year        INTEGER NOT NULL,
    date        TEXT,
    tz          TEXT
);

CREATE TABLE IF NOT EXISTS alerts
(
    id  INTEGER NOT NULL,
    notification INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS subscriptions
(
    team            INTEGER,
    close_games     INTEGER NOT NULL,
    final_scores    INTEGER NOT NULL,
    quarter_scores  INTEGER NOT NULL,
    endpoint        TEXT PRIMARY KEY NOT NULL,
    p256dh          TEXT NOT NULL,
    auth            TEXT NOT NULL
);
