mod algo;
pub use algo::Grade;

mod fsrs;
pub use fsrs::review;

mod card;
pub use card::Card;

#[doc = include_str!("../README.md")]
mod readme {}

pub type Time = chrono::DateTime<chrono::Utc>;
pub use chrono::Duration;
