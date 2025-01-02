mod long_term;
mod short_term;

use crate::{Card, Parameters, Rating};
use chrono::{DateTime, Utc};
use long_term::LongTerm;
use short_term::ShortTerm;

pub struct Scheduler(Inner);

impl Scheduler {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Scheduler {
        Self(if parameters.enable_short_term {
            Inner::Basic(ShortTerm::new(parameters, card, now))
        } else {
            Inner::Longterm(LongTerm::new(parameters, card, now))
        })
    }

    pub fn next_card(&self, rating: Rating) -> Card {
        match &self.0 {
            Inner::Basic(basic) => basic.next_card(rating),
            Inner::Longterm(longterm) => longterm.next_card(rating),
        }
    }
}

enum Inner {
    Basic(ShortTerm),
    Longterm(LongTerm),
}
