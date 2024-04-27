pub mod types;

use sqlx::{migrate::MigrateError, SqlitePool};
use squiggle::types::GameId;

use crate::types::{Game, Notification};

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("Database error {0}")]
    Database(#[from] sqlx::Error),
    #[error("Migration {0}")]
    Migration(#[from] MigrateError),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error {0}")]
    Database(#[from] sqlx::Error),
    #[error("Game couldn't be found {0}")]
    NoGameFound(GameId),
}

#[derive(Clone)]
pub struct Store {
    pool: SqlitePool,
}

impl Store {
    pub async fn new(url: &str) -> Result<Self, InitError> {
        let pool = SqlitePool::connect(url).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    #[tracing::instrument(skip(self), ret, err)]
    pub async fn upsert_game(&self, game: Game) -> Result<Game, Error> {
        let mut conn = self.pool.acquire().await?;

        let game: Game = sqlx::query_as(
            r"
            INSERT OR REPLACE INTO games (id, round, complete, home_team, away_team, home_score, away_score, timestr, year)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            ",
        )
        .bind(game.id)
        .bind(game.round)
        .bind(game.complete)
        .bind(game.home_team)
        .bind(game.away_team)
        .bind(game.home_score)
        .bind(game.away_score)
        .bind(game.timestr)
        .bind(game.year)
        .fetch_one(&mut *conn)
        .await?;

        Ok(game)
    }

    #[tracing::instrument(skip(self), ret, err)]
    pub async fn get_game_by_id(&self, id: GameId) -> Result<Option<Game>, Error> {
        let mut conn = self.pool.acquire().await?;

        let game: Option<Game> = sqlx::query_as(
            r"
            SELECT * FROM games WHERE id = ?
            ",
        )
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?;

        Ok(game)
    }

    pub async fn get_this_round_games(&self) -> Result<Vec<Game>, Error> {
        let mut conn = self.pool.acquire().await?;

        let games: Vec<Game> = sqlx::query_as(
            r"
            SELECT * FROM games
           ",
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(games)
    }

    #[tracing::instrument(skip(self), ret, err)]
    pub async fn game_has_notification(
        &self,
        game: GameId,
        notification: Notification,
    ) -> Result<bool, Error> {
        let mut conn = self.pool.acquire().await?;

        let rows = sqlx::query(
            r"
            SELECT notification FROM alerts WHERE id = ? and notification = ?
            ",
        )
        .bind(game)
        .bind(notification as u8)
        .fetch_all(&mut *conn)
        .await?;

        Ok(!rows.is_empty())
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn record_notification(
        &self,
        game: GameId,
        notification: Notification,
    ) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r"
            INSERT INTO alerts (id, notification)
            VALUES (?, ?)
            ",
        )
        .bind(game)
        .bind(notification as u8)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
