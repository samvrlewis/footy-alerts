use squiggle::types::{GameId, Team, TimeStr};

#[derive(Debug, sqlx::FromRow)]
pub struct Game {
    pub id: GameId,
    pub round: u16,
    pub complete: u8,
    pub home_team: Team,
    pub away_team: Team,
    pub home_score: u16,
    pub away_score: u16,
    pub timestr: String,
    pub year: u16,
    pub date: String,
    pub tz: String,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Notification {
    EndOfFirstQuarter,
    EndOfSecondQuarter,
    EndOfThirdQuarter,
    EndOfGame,
    CloseGame,
}

impl TryFrom<squiggle::rest::types::Game> for Game {
    type Error = serde_json::Error;

    fn try_from(value: squiggle::rest::types::Game) -> Result<Self, Self::Error> {
        let time_str = value
            .timestr
            .unwrap_or_else(|| TimeStr::Other("Not started".to_string()));

        let time_str = serde_json::to_string(&time_str)?;

        Ok(Self {
            id: value.id,
            round: value.round,
            complete: value.complete,
            home_team: value.home_team,
            away_team: value.away_team,
            home_score: value.home_score,
            away_score: value.away_score,
            timestr: time_str,
            year: value.year,
            date: value.date,
            tz: value.tz,
        })
    }
}

impl TryFrom<Game> for squiggle::rest::types::Game {
    type Error = serde_json::Error;

    fn try_from(value: Game) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            round: value.round,
            home_team: value.home_team,
            away_team: value.away_team,
            complete: value.complete,
            winner: None,
            home_score: value.home_score,
            away_score: value.away_score,
            timestr: serde_json::from_str(&value.timestr)?,
            year: value.year,
            date: value.date,
            tz: value.tz,
        })
    }
}
