/// Common types
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

pub type GameId = u32;

#[derive(
    Deserialize_repr, Serialize, PartialEq, Debug, sqlx::Type, Clone, strum_macros::Display,
)]
#[repr(u8)]
pub enum Team {
    Adelaide = 1,
    Brisbane = 2,
    Carlton = 3,
    Collingwood = 4,
    Essendon = 5,
    Fremantle = 6,
    Geelong = 7,
    #[serde(rename(serialize = "Gold Coast"))]
    #[strum(serialize = "Gold Coast")]
    GoldCoast = 8,
    #[serde(rename(serialize = "GWS"))]
    #[strum(serialize = "GWS")]
    GreaterWesternSydney = 9,
    Hawthorn = 10,
    Melbourne = 11,
    #[serde(rename(serialize = "North Melbourne"))]
    #[strum(serialize = "North Melbourne")]
    NorthMelbourne = 12,
    #[serde(rename(serialize = "Port Adelaide"))]
    #[strum(serialize = "Port Adelaide")]
    PortAdelaide = 13,
    Richmond = 14,
    #[serde(rename(serialize = "St Kilda"))]
    #[strum(serialize = "St Kilda")]
    StKilda = 15,
    Sydney = 16,
    #[serde(rename(serialize = "West Coast"))]
    #[strum(serialize = "West Coast")]
    WestCoast = 17,
    #[serde(rename(serialize = "Western Bulldogs"))]
    #[strum(serialize = "Western Bulldogs")]
    WesternBulldogs = 18,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, strum_macros::Display)]
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
    #[strum(to_string = "{0}")]
    Other(String),
}
