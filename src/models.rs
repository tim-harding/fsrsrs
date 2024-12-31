use crate::Parameters;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card {
    pub last_review: DateTime<Utc>,
    pub due: DateTime<Utc>,
    pub state: State,
    pub stability: f64,
    pub difficulty: f64,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub reps: i32,
    pub lapses: i32,
}

impl Card {
    pub fn new() -> Self {
        Self {
            due: Utc::now(),
            last_review: Utc::now(),
            ..Default::default()
        }
    }

    pub fn elapsed_days(&self, now: DateTime<Utc>) -> i64 {
        match self.state {
            State::New => 0,
            _ => now.signed_duration_since(self.last_review).num_days(),
        }
    }

    pub fn retrievability(&self, parameters: &Parameters, now: DateTime<Utc>) -> f64 {
        parameters.forgetting_curve(self.elapsed_days(now) as f64, self.stability)
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

impl Rating {
    pub fn iter_variants() -> impl Iterator<Item = Self> {
        [Rating::Again, Rating::Hard, Rating::Good, Rating::Easy].into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReviewLog {
    pub rating: Rating,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub state: State,
    pub reviewed_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SchedulingInfo {
    pub card: Card,
    pub review_log: ReviewLog,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum State {
    #[default]
    New = 0,
    Learning = 1,
    Review = 2,
    Relearning = 3,
}
