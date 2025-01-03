use crate::{Duration, Parameters, Time};

/// The state of FSRS after a review
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card {
    /// When the card was last reviewed
    pub reviewed_at: Time,
    /// Time interval from the last review for the next review
    pub interval: Duration,
    /// Difficulty rating of the review
    pub grade: Grade,
    /// FSRS memory stability after the review
    pub stability: f64,
    /// FSRS memory difficulty after the review
    pub difficulty: f64,
}

impl Card {
    /// When the card is due for another review
    pub fn due(&self) -> Time {
        self.reviewed_at + self.interval
    }

    /// Amount of time passed since the last review
    pub fn elapsed(&self, now: Time) -> Duration {
        now.signed_duration_since(self.reviewed_at)
    }

    /// Amount of time in days since the last review
    pub fn elapsed_days(&self, now: Time) -> i64 {
        self.elapsed(now).num_days()
    }

    /// FSRS memory retrievability after the review
    pub fn retrievability(&self, parameters: &Parameters, now: Time) -> f64 {
        parameters.forgetting_curve(self.elapsed_days(now) as f64, self.stability)
    }
}

/// Difficulty classification of a review
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Grade {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}
