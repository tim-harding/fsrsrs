use crate::Parameters;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum State {
    #[default]
    New = 0,
    Learning = 1,
    Review = 2,
    Relearning = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[derive(Debug, Clone)]
pub struct SchedulingInfo {
    pub card: Card,
    pub review_log: ReviewLog,
}

pub type RecordLog = HashMap<Rating, SchedulingInfo>;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReviewLog {
    pub rating: Rating,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub state: State,
    pub reviewed_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Card {
    pub due: DateTime<Utc>,
    pub stability: f64,
    pub difficulty: f64,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub reps: i32,
    pub lapses: i32,
    pub state: State,
    pub last_review: DateTime<Utc>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            due: Utc::now(),
            last_review: Utc::now(),
            ..Default::default()
        }
    }

    pub fn get_retrievability(&self, now: DateTime<Utc>) -> f64 {
        match self.state {
            State::New => 0.0,
            _ => {
                let elapsed_days = now.signed_duration_since(self.last_review).num_days();
                Parameters::forgetting_curve(elapsed_days as f64, self.stability)
            }
        }
    }
}
