/// Common types
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

pub type GameId = u32;

#[derive(Deserialize_repr, Serialize, PartialEq, Debug, sqlx::Type)]
#[repr(u8)]
pub enum Team {
    Adelaide = 1,
    Brisbane = 2,
    Carlton = 3,
    Collingwood = 4,
    Essendon = 5,
    Fremantle = 6,
    Geelong = 7,
    GoldCoast = 8,
    GreaterWesternSydney = 9,
    Hawthorn = 10,
    Melbourne = 11,
    NorthMelbourne = 12,
    PortAdelaide = 13,
    Richmond = 14,
    StKilda = 15,
    Sydney = 16,
    WestCoast = 17,
    WesternBulldogs = 18,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TimeStr {
    #[serde(rename = "1/4 Time")]
    EndOfFirstQuarter,
    #[serde(rename = "2/4 Time")]
    EndOfSecondQuarter,
    #[serde(rename = "3/4 Time")]
    EndOfThirdQuarter,
    #[serde(rename = "Full Time")]
    EndOfGame,
    #[serde(untagged)]
    Other(String),
}
