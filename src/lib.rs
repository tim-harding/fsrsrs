mod algo;
pub use algo::Grade;

mod fsrs;
pub use fsrs::review;

mod card;
pub use card::Card;

#[doc = include_str!("../README.md")]
mod readme {}

use chrono::{DateTime, Duration, Utc};

pub type Time = DateTime<Utc>;

/// Get the current [Time]
pub fn now() -> Time {
    Utc::now()
}

const SEC_PER_DAY: f64 = 60.0 * 60.0 * 24.0;

fn to_days(duration: Duration) -> f64 {
    duration.num_seconds() as f64 / SEC_PER_DAY
}

fn from_days(days: f64) -> Duration {
    Duration::seconds((days * SEC_PER_DAY) as i64)
}
