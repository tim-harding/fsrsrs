use crate::{
    models::{Card, Rating, RecordLog, SchedulingInfo},
    parameters::Parameters,
    Scheduler,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Default, Clone)]
pub struct Fsrs {
    parameters: Parameters,
}

impl Fsrs {
    pub const fn new(parameters: Parameters) -> Self {
        Self { parameters }
    }

    pub fn scheduler(&self, card: Card, now: DateTime<Utc>) -> Scheduler {
        Scheduler::new(self.parameters.clone(), card, now)
    }

    pub fn repeat(&self, card: Card, now: DateTime<Utc>) -> RecordLog {
        self.scheduler(card, now).preview()
    }

    pub fn next(&self, card: Card, now: DateTime<Utc>, rating: Rating) -> SchedulingInfo {
        self.scheduler(card, now).review(rating)
    }
}
