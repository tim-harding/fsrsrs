use crate::Parameters;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card {
    pub reviewed_at: DateTime<Utc>,
    pub due: DateTime<Utc>,
    pub rating: Rating,
    pub state: State,
    pub stability: f64,
    pub difficulty: f64,
}

impl Card {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            reviewed_at: now,
            due: now,
            rating: Rating::Again,
            state: State::New,
            stability: 0.0,
            difficulty: 0.0,
        }
    }

    pub fn elapsed_days(&self, now: DateTime<Utc>) -> i64 {
        match self.state {
            State::New => 0,
            _ => now.signed_duration_since(self.reviewed_at).num_days(),
        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Review {
    pub rating: Rating,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub state: State,
    pub reviewed_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Schedule {
    pub card: Card,
    pub review: Review,
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
