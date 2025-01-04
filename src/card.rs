use crate::{
    algo::{self, Grade},
    Duration, Time,
};

const SEC_PER_DAY: f64 = 60.0 * 60.0 * 24.0;

/// The state of FSRS after a review
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card {
    /// When the card was last reviewed
    pub when: Time,
    /// Time interval from the last review for the next review
    pub due: Time,
    /// Difficulty rating of the review
    pub grade: Grade,
    /// FSRS memory stability after the review
    pub stability: f64,
    /// FSRS memory difficulty after the review
    pub difficulty: f64,
}

impl Card {
    /// Amount of time passed since the last review
    pub fn elapsed(&self, now: Time) -> Duration {
        now.signed_duration_since(self.when)
    }

    /// Amount of time in days since the last review
    pub fn elapsed_days(&self, now: Time) -> f64 {
        self.elapsed(now).num_seconds() as f64 / SEC_PER_DAY
    }

    /// FSRS memory retrievability after the review
    pub fn retrievability(&self, now: Time) -> f64 {
        algo::retrievability(self.elapsed_days(now), self.stability)
    }
}
