use crate::{to_days, Duration, Grade, Time};

/// The state of FSRS after a review
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
    fn elapsed(&self, now: Time) -> Duration {
        now.signed_duration_since(self.when)
    }

    /// Amount of time in days since the last review
    pub(crate) fn elapsed_days(&self, now: Time) -> f64 {
        to_days(self.elapsed(now))
    }
}
