use serde::{Deserialize, Serialize};

use crate::types::{GameId, Team, TimeStr};

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: GameId,
    pub round: u16,
    #[serde(rename(deserialize = "hteamid"))]
    pub home_team: Team,
    #[serde(rename(deserialize = "ateamid"))]
    pub away_team: Team,
    pub complete: u8,
    #[serde(rename(deserialize = "winnerteamid"))]
    pub winner: Option<Team>,
    #[serde(rename(deserialize = "hscore"))]
    pub home_score: u16,
    #[serde(rename(deserialize = "ascore"))]
    pub away_score: u16,
    pub timestr: Option<TimeStr>,
    pub year: u16,
    pub date: String,
    pub tz: String,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_completed_game() {
        let game: Game = serde_json::from_str(r#"{"hgoals":11,"id":35740,"agoals":12,"unixtime":1712979900,"date":"2024-04-13 13:45:00","abehinds":7,"complete":100,"timestr":"Full Time","localtime":"2024-04-13 13:45:00","hbehinds":14,"tz":"+10:00","ascore":79,"winner":"Greater Western Sydney","hscore":80,"venue":"Manuka Oval","updated":"2024-04-13 16:29:08","year":2024,"winnerteamid":9,"is_grand_final":0,"hteam":"Greater Western Sydney","is_final":0,"ateamid":15,"ateam":"St Kilda","roundname":"Round 5","hteamid":9,"round":5}"#).expect("Should deser");

        assert_eq!(game.id, 35740);
        assert_eq!(game.round, 5);
        assert_eq!(game.home_team, Team::GreaterWesternSydney);
        assert_eq!(game.away_team, Team::StKilda);
        assert_eq!(game.winner, Some(Team::GreaterWesternSydney));
        assert_eq!(game.complete, 100);
        assert_eq!(game.away_score, 79);
        assert_eq!(game.home_score, 80);
    }

    #[test]
    fn test_not_started_game() {
        let game: Game = serde_json::from_str(r#"{"ateam":"Western Bulldogs","roundname":"Round 7","hteamid":6,"round":7,"is_grand_final":0,"hteam":"Fremantle","winnerteamid":null,"ateamid":18,"is_final":0,"venue":"Perth Stadium","hscore":0,"winner":null,"year":2024,"updated":"2023-11-17 11:12:57","ascore":0,"tz":"+10:00","complete":0,"localtime":"2024-04-27 17:30:00","timestr":null,"hbehinds":null,"abehinds":null,"unixtime":1714210200,"agoals":null,"date":"2024-04-27 19:30:00","hgoals":null,"id":35760}"#).expect("Couldn't deser");

        assert_eq!(game.id, 35760);
        assert_eq!(game.round, 7);
        assert_eq!(game.home_team, Team::Fremantle);
        assert_eq!(game.away_team, Team::WesternBulldogs);
        assert_eq!(game.winner, None);
        assert_eq!(game.complete, 0);
        assert_eq!(game.away_score, 0);
        assert_eq!(game.home_score, 0);
    }
}
