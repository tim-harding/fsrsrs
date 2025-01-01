mod base;
mod basic;
mod longterm;

use crate::{Card, Parameters, Rating, Review, Schedule};
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

    pub fn next_card(&self, rating: Rating) -> Card {
        match &self.0 {
            Inner::Basic(basic) => basic.next_card(rating),
            Inner::Longterm(longterm) => longterm.next_card(rating),
        }
    }

    pub fn current_review(&self, rating: Rating) -> Review {
        match &self.0 {
            Inner::Basic(basic) => basic.current_review(rating),
            Inner::Longterm(longterm) => longterm.current_review(rating),
        }
    }

    pub fn schedule(&self, rating: Rating) -> Schedule {
        Schedule {
            card: self.next_card(rating),
            review: self.current_review(rating),
        }
    }
}

enum Inner {
    Basic(Basic),
    Longterm(Longterm),
}
