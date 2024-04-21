use notifier::Notifier;
use squiggle::{
    event::types::Event,
    rest::{types::Game, Client},
    types::GameId,
};
use store::{
    types::{Game as DbGame, Notification},
    Store,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Squiggle API: {0}")]
    SquiggleApi(#[from] squiggle::rest::Error),
    #[error("Store: {0}")]
    Store(#[from] store::Error),
    #[error("Notifier: {0}")]
    Notifier(#[from] notifier::Error),
    #[error("Deser: {0}")]
    Deser(#[from] serde_json::Error),
}

pub struct Processor {
    store: Store,
    rest_client: Client,
    notifier: Notifier,
}

/// How complete the game needs to be before we send out close game alerts
const CLOSE_GAME_COMPLETION_THRESHOLD: u8 = 90;

/// The point difference between teams to consider the game as being close
const CLOSE_GAME_SCORE_THRESHOLD: i32 = 15;

#[tracing::instrument(ret)]
pub fn maybe_notification(game: &Game) -> Option<Notification> {
    if game.complete > CLOSE_GAME_COMPLETION_THRESHOLD {
        let close_game = i32::from(game.home_score) - i32::from(game.away_score);
        let close_game = close_game.abs() <= CLOSE_GAME_SCORE_THRESHOLD;
        close_game.then_some(Notification::CloseGame)
    } else {
        None
    }
}

#[tracing::instrument(ret)]
fn patch_game_with_event(mut game: Game, event: Event) -> Game {
    match event {
        Event::Score(score) => {
            game.away_score = score.score.away_score;
            game.home_score = score.score.home_score;
            game.complete = score.complete;
            game.timestr = Some(score.timestr);
        }
        Event::Game(_) => {
            // ignore for now as it's not that useful
        }
        Event::TimeStr(timestr) => {
            game.timestr = Some(timestr.timestr);
        }
        Event::Complete(complete) => {
            game.complete = complete.complete;
        }
        Event::Winner(winner) => {
            game.winner = Some(winner.winner);
        }
    }

    game
}

impl Processor {
    pub fn new(store: Store, rest_client: Client, notifier: Notifier) -> Self {
        Self {
            store,
            rest_client,
            notifier,
        }
    }
    #[tracing::instrument(skip(self), err)]
    pub async fn process_event(&self, event: Event) -> Result<(), Error> {
        let game_id = event.id();
        let db_game = self.get_or_insert_game(game_id).await?;

        let game = patch_game_with_event(Game::try_from(db_game)?, event);
        let maybe_notification = maybe_notification(&game);

        self.update_game(game).await?;

        // see if we should send a notification
        let Some(notification) = maybe_notification else {
            return Ok(());
        };

        // check if we've already sent a notification
        if self
            .store
            .game_has_notification(game_id, notification)
            .await?
        {
            return Ok(());
        }

        // mark the notification as sent
        self.store
            .record_notification(game_id, notification)
            .await?;

        self.notifier.notify(0)?;

        return Ok(());
    }

    #[tracing::instrument(skip(self), err)]
    async fn get_or_insert_game(&self, game_id: GameId) -> Result<DbGame, Error> {
        let game = self.store.get_game_by_id(game_id).await?;

        if let Some(game) = game {
            return Ok(game);
        }

        let game = self.rest_client.fetch_game(game_id).await?;
        let game = self.store.upsert_game(game.try_into()?).await?;
        Ok(game)
    }

    #[tracing::instrument(skip(self))]
    async fn update_game(&self, game: Game) -> Result<DbGame, Error> {
        Ok(self.store.upsert_game(game.try_into()?).await?)
    }
}
