mod base;
mod basic;
mod longterm;

use crate::{Card, Parameters, Rating, Review, SchedulingInfo};
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

    pub fn next_card(&mut self, rating: Rating) -> Card {
        match &mut self.0 {
            Inner::Basic(basic) => basic.next_card(rating),
            Inner::Longterm(longterm) => longterm.review(rating).card,
        }
    }

    pub fn current_review(&mut self, rating: Rating) -> Review {
        match &mut self.0 {
            Inner::Basic(basic) => basic.current_review(rating),
            Inner::Longterm(longterm) => longterm.review(rating).review,
        }
    }

    // TODO: Take &self
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
