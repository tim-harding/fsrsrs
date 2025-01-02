use crate::{Card, Parameters};
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
        current.reviewed_at = now;

        Self {
            parameters,
            previous: card,
            current,
            now,
        }
    }
}
