mod algo;

mod grade;
pub use grade::Grade;

mod fsrs;
pub use fsrs::review;

mod card;
pub use card::Card;

#[doc = include_str!("../README.md")]
mod readme {}

use chrono::{DateTime, Duration, Utc};

type Time = DateTime<Utc>;

const SEC_PER_DAY: f64 = 60.0 * 60.0 * 24.0;

fn to_days(duration: Duration) -> f64 {
    duration.num_seconds() as f64 / SEC_PER_DAY
}

fn from_days(days: f64) -> Duration {
    Duration::seconds((days * SEC_PER_DAY) as i64)
}
