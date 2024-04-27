pub mod types;

use reqwest::header::{HeaderValue, USER_AGENT};
use serde::Deserialize;
use tracing::error;
use types::Game;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No games")]
    MissingGame,
    #[error("Request: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Deserialization: {0}")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("Couldn't parse user-agent as header: {0}")]
    BadUserAgent(String),
}

#[derive(Debug, Deserialize)]
struct GamesResponse {
    games: Vec<Game>,
}

pub struct Client {
    client: reqwest::Client,
    user_agent: HeaderValue,
}

impl Client {
    pub fn new(user_agent: &str) -> Result<Self, InitError> {
        let client = reqwest::Client::new();
        let user_agent = HeaderValue::from_str(user_agent)
            .map_err(|_err| InitError::BadUserAgent(user_agent.to_string()))?;
        Ok(Self { client, user_agent })
    }

    #[tracing::instrument(skip(self), ret, err)]
    pub async fn fetch_game(&self, game_id: u32) -> Result<Game, Error> {
        let filter = format!("games;game={game_id}");
        let mut games_response = self.fetch(filter).await?;
        let game = games_response.games.pop().ok_or(Error::MissingGame)?;
        Ok(game)
    }

    #[tracing::instrument(skip(self), ret, err)]
    pub async fn fetch_games(&self, round: u16, year: u16) -> Result<Vec<Game>, Error> {
        let filter = format!("games;year={year};round={round}");
        let games_response = self.fetch(filter).await?;
        Ok(games_response.games)
    }

    #[tracing::instrument(skip(self), ret, err)]
    async fn fetch(&self, filter: String) -> Result<GamesResponse, Error> {
        let url = format!("https://api.squiggle.com.au/?q={filter}");
        let resp = self
            .client
            .get(url)
            .header(USER_AGENT, &self.user_agent)
            .send()
            .await?;
        let text = resp.text().await?;

        let games_response: GamesResponse = serde_json::from_str(&text).inspect_err(
            |err| error!(payload = text, error = ?err, "Couldn't deserialize games response"),
        )?;
        Ok(games_response)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Team;

    #[test]
    fn test_games_response() {
        let resp: GamesResponse = serde_json::from_str(
            r#"{
  "games": [
    {
      "ateam": "Western Bulldogs",
      "roundname": "Round 7",
      "hteamid": 6,
      "round": 7,
      "is_grand_final": 0,
      "hteam": "Fremantle",
      "winnerteamid": null,
      "ateamid": 18,
      "is_final": 0,
      "venue": "Perth Stadium",
      "hscore": 0,
      "winner": null,
      "year": 2024,
      "updated": "2023-11-17 11:12:57",
      "ascore": 0,
      "tz": "+10:00",
      "complete": 0,
      "localtime": "2024-04-27 17:30:00",
      "timestr": null,
      "hbehinds": null,
      "abehinds": null,
      "unixtime": 1714210200,
      "agoals": null,
      "date": "2024-04-27 19:30:00",
      "hgoals": null,
      "id": 35760
    }
  ]
}"#,
        )
        .expect("Couldn't deser");

        assert_eq!(resp.games.len(), 1)
    }

    #[tokio::test]
    async fn test_fetch_game() {
        let client = Client::new("sam.vr.lewis@gmail.com").expect("client");
        let game = client.fetch_game(35740).await.expect("game");

        assert_eq!(game.id, 35740);
        assert_eq!(game.round, 5);
        assert_eq!(game.home_team, Team::GreaterWesternSydney);
        assert_eq!(game.away_team, Team::StKilda);
        assert_eq!(game.winner, Some(Team::GreaterWesternSydney));
        assert_eq!(game.complete, 100);
        assert_eq!(game.away_score, 79);
        assert_eq!(game.home_score, 80);
    }
}
