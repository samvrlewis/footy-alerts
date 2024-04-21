use serde::Deserialize;

use crate::types::{GameId, Team, TimeStr};

#[derive(Debug, Deserialize)]
pub enum Side {
    #[serde(rename = "ateam")]
    Away,
    #[serde(rename = "hteam")]
    Home,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Score {
    #[serde(rename = "hscore")]
    pub home_score: u16,
    #[serde(rename = "ascore")]
    pub away_score: u16,
}

#[derive(Debug, Deserialize)]
pub struct ScoreEvent {
    #[serde(rename = "gameid")]
    pub game_id: GameId,
    #[serde(rename = "type")]
    pub score_type: String,
    pub complete: u8,
    pub score: Score,
    pub timestr: TimeStr,
}

#[derive(Debug, Deserialize)]
pub struct GameEvent {
    pub id: GameId,
    pub round: u16,
    #[serde(rename = "hteam")]
    pub home_team: Team,
    #[serde(rename = "ateam")]
    pub away_team: Team,
    pub complete: u8,
    pub winner: Option<Team>,
    #[serde(rename = "hscore")]
    pub home_score: u16,
    #[serde(rename = "ascore")]
    pub away_score: u16,
    #[serde(rename = "hgoals")]
    pub home_goals: u16,
    #[serde(rename = "hbehinds")]
    pub home_behinds: u16,
    #[serde(rename = "agoals")]
    pub away_goals: u16,
    #[serde(rename = "abehinds")]
    pub away_behinds: u16,
    pub timestr: TimeStr,
}

#[derive(Debug, Deserialize)]
pub struct TimeStrEvent {
    #[serde(rename = "gameid")]
    pub game_id: GameId,
    pub timestr: TimeStr,
}

#[derive(Debug, Deserialize)]
pub struct CompleteEvent {
    #[serde(rename = "gameid")]
    pub game_id: GameId,
    pub complete: u8,
}

#[derive(Debug, Deserialize)]
pub struct WinnerEvent {
    #[serde(rename = "gameid")]
    pub game_id: GameId,
    pub winner: Team,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Event {
    /// Sent when a score occurs
    Score(ScoreEvent),
    /// Sent at the end of the game
    Game(GameEvent),
    /// Sent periodically with time updates
    TimeStr(TimeStrEvent),
    /// Sent periodically with an indication of the percentage of game played
    Complete(CompleteEvent),
    /// Sent at the end of the game to indicate the winner
    Winner(WinnerEvent),
}

impl Event {
    #[must_use]
    pub fn id(&self) -> GameId {
        match self {
            Event::Score(score) => score.game_id,
            Event::Game(game) => game.id,
            Event::TimeStr(time) => time.game_id,
            Event::Complete(complete) => complete.game_id,
            Event::Winner(winner) => winner.game_id,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_game() {
        let event: Event = serde_json::from_str(r#"{ "id":8706, "year":2022, "round":5, "hteam":10, "ateam":7, "date":"2022-04-18T05:20:00.000Z", "tz":"+10:00", "complete":75, "winner":null, "hscore":64, "ascore":76, "hgoals":10, "hbehinds":4, "agoals":11, "abehinds":10, "venue":"M.C.G.", "timestr":"3/4 Time", "updated":"2022-04-18T07:23:03.000Z", "is_final":0, "is_grand_final":0 }"#).unwrap();

        let Event::Game(game) = event else {
            panic!("Not a game")
        };

        assert_eq!(game.id, 8706);
        assert_eq!(game.home_team, Team::Hawthorn);
        assert_eq!(game.away_team, Team::Geelong);
        assert_eq!(game.complete, 75);
        assert_eq!(game.winner, None);
        assert_eq!(game.home_score, 64);
        assert_eq!(game.away_score, 76);
        assert_eq!(game.home_goals, 10);
        assert_eq!(game.home_behinds, 4);
        assert_eq!(game.away_goals, 11);
        assert_eq!(game.away_behinds, 10);
        assert_eq!(game.timestr, TimeStr::EndOfThirdQuarter);
    }

    #[test]
    fn test_score() {
        let event: Event = serde_json::from_str(r#"{ "gameid":8706, "type":"behind", "side":"ateam", "team":7, "complete":78, "timestr":"Q4  4:36", "score":{ "hscore":64, "hgoals":10, "hbehinds":4, "ascore":77, "agoals":11, "abehinds":11 } }"#).unwrap();

        let Event::Score(score) = event else {
            panic!("Not a score");
        };

        assert_eq!(score.game_id, 8706);
        assert_eq!(score.score_type, "behind");
        assert_eq!(score.complete, 78);
        assert_eq!(score.timestr, TimeStr::Other("Q4  4:36".to_string()));
        assert_eq!(
            score.score,
            Score {
                home_score: 64,
                away_score: 77,
            }
        );
    }
}
