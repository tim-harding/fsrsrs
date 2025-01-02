use crate::{Card, Parameters, Rating, Review, State::*};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Base {
    pub now: DateTime<Utc>,
    pub parameters: Parameters,
    pub previous: Card,
    pub current: Card,
}

impl Base {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Self {
        let mut current = card;
        current.elapsed_days = match card.state {
            New => 0,
            _ => (now - card.last_review).num_days(),
        };
        current.last_review = now;
        current.reps += 1;

        Self {
            parameters,
            previous: card,
            current,
            now,
        }
    }

    pub fn current_review(self, rating: Rating, card: Card) -> Review {
        let Self {
            now: reviewed_date,
            current:
                Card {
                    state,
                    elapsed_days,
                    ..
                },
            ..
        } = self;

        Review {
            rating,
            state,
            elapsed_days,
            scheduled_days: (card.due - self.now).num_days(),
            reviewed_date,
        }
    }
}
