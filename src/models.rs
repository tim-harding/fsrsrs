use crate::Parameters;
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card {
    pub reviewed_at: DateTime<Utc>,
    pub interval: Duration,
    pub rating: Rating,
    pub state: State,
    pub stability: f64,
    pub difficulty: f64,
}

impl Card {
    pub fn new() -> Self {
        Self {
            reviewed_at: Utc::now(),
            interval: Duration::zero(),
            rating: Rating::Again,
            state: State::New,
            stability: 0.0,
            difficulty: 0.0,
        }
    }

    pub fn due(&self) -> DateTime<Utc> {
        self.reviewed_at + self.interval
    }

    pub fn elapsed(&self, now: DateTime<Utc>) -> Duration {
        match self.state {
            State::New => Duration::zero(),
            _ => now.signed_duration_since(self.reviewed_at),
        }
    }

    pub fn elapsed_days(&self, now: DateTime<Utc>) -> i64 {
        self.elapsed(now).num_days()
    }

    pub fn retrievability(&self, parameters: &Parameters, now: DateTime<Utc>) -> f64 {
        parameters.forgetting_curve(self.elapsed_days(now) as f64, self.stability)
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rating {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum State {
    #[default]
    New = 0,
    Learning = 1,
    Reviewing = 2,
    Relearning = 3,
}
