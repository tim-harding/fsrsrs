mod base;
mod basic;
mod longterm;

use crate::{Card, Parameters, Rating, RecordLog, SchedulingInfo};
use basic::Basic;
use chrono::{DateTime, Utc};
use longterm::Longterm;

pub struct Scheduler(Inner);

impl Scheduler {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Scheduler {
        Self(if parameters.enable_short_term {
            Inner::Basic(Basic::new(parameters, card, now))
        } else {
            Inner::Longterm(Longterm::new(parameters, card, now))
        })
    }

    pub fn preview(&mut self) -> RecordLog {
        Rating::iter_variants()
            .map(|rating| (rating, self.review(rating)))
            .collect()
    }

    pub fn review(&mut self, rating: Rating) -> SchedulingInfo {
        match &mut self.0 {
            Inner::Basic(basic) => basic.review(rating),
            Inner::Longterm(longterm) => longterm.review(rating),
        }
    }
}

enum Inner {
    Basic(Basic),
    Longterm(Longterm),
}
