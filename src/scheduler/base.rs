use crate::{models::State::*, Card, Parameters, Rating, Review};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Base {
    pub parameters: Parameters,
    pub last: Card,
    pub current: Card,
    pub now: DateTime<Utc>,
}

impl Base {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Self {
        let mut current_card: Card = card;
        current_card.elapsed_days = match card.state {
            New => 0,
            _ => (now - card.last_review).num_days(),
        };
        current_card.last_review = now;
        current_card.reps += 1;
        let mut scheduler = Self {
            parameters,
            last: card,
            current: current_card,
            now,
        };
        scheduler.init_seed();
        scheduler
    }

    pub const fn build_log(&self, rating: Rating) -> Review {
        Review {
            rating,
            state: self.current.state,
            elapsed_days: self.current.elapsed_days,
            scheduled_days: self.current.scheduled_days,
            reviewed_date: self.now,
        }
    }

    // TODO: Unconditionally clobbers user-provided seed
    fn init_seed(&mut self) {
        let time = self.now.timestamp_millis();
        let reps = self.current.reps;
        let mul = self.current.difficulty * self.current.stability;
        self.parameters.seed = format!("{}_{}_{}", time, reps, mul);
    }
}
