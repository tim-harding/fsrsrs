use crate::{models::Card, parameters::Parameters, scheduler::Scheduler};
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
        // TODO: Avoid cloning this each time
        Scheduler::new(self.parameters.clone(), card, now)
    }
}
