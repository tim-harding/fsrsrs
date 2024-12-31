use crate::Parameters;

use super::State;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn retrievability(&self, now: DateTime<Utc>) -> f64 {
        match self.state {
            State::New => 0.0,
            _ => {
                let elapsed_days = now.signed_duration_since(self.last_review).num_days();
                Parameters::forgetting_curve(elapsed_days as f64, self.stability)
            }
        }
    }
}
