use crate::{Card, Parameters};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Base {
    pub now: DateTime<Utc>,
    pub parameters: Parameters,
    pub card: Card,
}

impl Base {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Self {
        Self {
            parameters,
            card,
            now,
        }
    }
}
