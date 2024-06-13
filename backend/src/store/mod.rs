pub mod types;

use sqlx::{migrate::MigrateError, SqlitePool};
use squiggle::types::{GameId, Team};
use types::{Game, Notification, Subscription};

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
            INSERT OR REPLACE INTO games (id, round, complete, home_team, away_team, home_score, away_score, timestr, year, date, tz)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
            .bind(game.date)
            .bind(game.tz)
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

    #[tracing::instrument(skip(self), ret, err)]
    pub async fn get_this_round_games(&self) -> Result<Vec<Game>, Error> {
        let mut conn = self.pool.acquire().await?;

        let games: Vec<Game> = sqlx::query_as(
            r"
            SELECT *
            FROM games
            WHERE year = (SELECT MAX(year) FROM games)
              AND round = (SELECT MAX(round) FROM games WHERE year = (SELECT MAX(year) FROM games));
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

    #[tracing::instrument(skip(self), err)]
    pub async fn add_subscription(&self, subscription: Subscription) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r"
            INSERT OR REPLACE INTO subscriptions (team, close_games, final_scores,
                            quarter_scores, endpoint, p256dh, auth)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(subscription.team)
        .bind(subscription.close_games)
        .bind(subscription.final_scores)
        .bind(subscription.quarter_scores)
        .bind(subscription.endpoint)
        .bind(subscription.p256dh)
        .bind(subscription.auth)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_subscription_for_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Option<Subscription>, Error> {
        let mut conn = self.pool.acquire().await?;

        let subscription: Option<Subscription> = sqlx::query_as(
            r"
            SELECT * FROM subscriptions where endpoint = ?
           ",
        )
        .bind(endpoint)
        .fetch_optional(&mut *conn)
        .await?;

        Ok(subscription)
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_subscriptions_for_notification(
        &self,
        home_team: Team,
        away_team: Team,
        notification: Notification,
    ) -> Result<Vec<Subscription>, Error> {
        let mut conn = self.pool.acquire().await?;

        let subscriptions: Vec<Subscription> = sqlx::query_as(
            r"
            SELECT * FROM subscriptions
            WHERE (team = ? OR team = ? OR team IS NULL) AND (close_games = ? OR final_scores = ? OR quarter_scores = ?) AND (active = 1)
           ",
        )
            .bind(home_team)
            .bind(away_team)
            .bind(notification.is_close_game_notification())
            .bind(notification.is_full_game_notification())
            .bind(notification.is_quarter_notification())
            .fetch_all(&mut *conn)
            .await?;

        Ok(subscriptions)
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn delete_subscription(&self, endpoint: &str) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r"
            UPDATE subscriptions
            SET active = 0
            WHERE endpoint = ?
            ",
        )
        .bind(endpoint)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
