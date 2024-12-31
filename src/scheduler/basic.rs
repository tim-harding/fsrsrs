use super::base::Base;
use crate::{
    cards::Cards,
    Card, Parameters,
    Rating::{self, *},
    SchedulingInfo,
    State::{self, *},
};
use chrono::{DateTime, Duration, Utc};

pub struct Basic(Base);

impl Basic {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Self {
        Self(Base::new(parameters, card, now))
    }

    pub fn review(&mut self, rating: Rating) -> SchedulingInfo {
        let card = match self.0.last.state {
            New => self.review_new(rating),
            Learning | Relearning => self.review_learning(rating),
            Review => self.review_reviewing(rating),
        };
        SchedulingInfo {
            card,
            review: self.0.current_review(rating),
        }
    }

    fn review_new(&self, rating: Rating) -> Card {
        let p = &self.0.parameters;
        let mut card = self.0.current;
        card.difficulty = p.init_difficulty(rating);
        card.stability = p.init_stability(rating);

        let (days, due, state) = match rating {
            Again => (0, Duration::minutes(1), Learning),
            Hard => (0, Duration::minutes(5), Learning),
            Good => (0, Duration::minutes(10), Learning),
            Easy => {
                let easy_interval = p.next_interval(card.stability, card.elapsed_days) as i64;
                (easy_interval, Duration::days(easy_interval), Review)
            }
        };

        card.scheduled_days = days;
        card.due = self.0.now + due;
        card.state = state;
        card
    }

    fn review_learning(&mut self, rating: Rating) -> Card {
        let p = &self.0.parameters;
        let mut card = self.0.current;
        let interval = self.0.current.elapsed_days;
        card.difficulty = p.next_difficulty(self.0.last.difficulty, rating);
        card.stability = p.short_term_stability(self.0.last.stability, rating);

        let (days, due, state) = match rating {
            Again => (0, Duration::minutes(5), self.0.last.state),
            Hard => (0, Duration::minutes(10), self.0.last.state),
            Good => {
                let good_interval = p.next_interval(card.stability, interval) as i64;
                (good_interval, Duration::days(good_interval), Review)
            }
            Easy => {
                let good_stability = p.short_term_stability(self.0.last.stability, Good);
                let good_interval = p.next_interval(good_stability, interval);
                let easy_interval = p
                    .next_interval(card.stability, interval)
                    .max(good_interval + 1.0) as i64;
                (easy_interval, Duration::days(easy_interval), Review)
            }
        };

        card.scheduled_days = days;
        card.due = self.0.now + due;
        card.state = state;
        card
    }

    fn review_reviewing(&mut self, rating: Rating) -> Card {
        let p = &self.0.parameters;
        let interval = self.0.current.elapsed_days;
        let stability = self.0.last.stability;
        let difficulty = self.0.last.difficulty;
        let retrievability = self.0.last.retrievability(p, self.0.now);

        let mut cards = Cards::new(self.0.current);
        cards.update(|(rating, card)| {
            card.difficulty = p.next_difficulty(difficulty, rating);
            card.stability = p.next_stability(difficulty, stability, retrievability, rating);
        });

        let [hard_interval, good_interval, easy_interval] = self.review_intervals(
            cards.hard.stability,
            cards.good.stability,
            cards.easy.stability,
            interval,
        );

        let mut card = cards.get(rating);

        let (days, due, lapses) = match rating {
            Again => (0, Duration::minutes(5), 1),
            Hard => (hard_interval, Duration::days(hard_interval), 0),
            Good => (good_interval, Duration::days(good_interval), 0),
            Easy => (easy_interval, Duration::days(easy_interval), 0),
        };

        card.scheduled_days = days;
        card.due = self.0.now + due;
        card.lapses += lapses;
        card.state = Self::next_state(rating);
        card
    }

    fn review_intervals(
        &self,
        hard_stability: f64,
        good_stability: f64,
        easy_stability: f64,
        interval: i64,
    ) -> [i64; 3] {
        let p = &self.0.parameters;
        let hard_interval = p.next_interval(hard_stability, interval);
        let good_interval = p.next_interval(good_stability, interval);
        let hard_interval = hard_interval.min(good_interval);
        let good_interval = good_interval.max(hard_interval + 1.0);
        let easy_interval = p
            .next_interval(easy_stability, interval)
            .max(good_interval + 1.0);
        [
            hard_interval as i64,
            good_interval as i64,
            easy_interval as i64,
        ]
    }

    fn next_state(rating: Rating) -> State {
        match rating {
            Again => Relearning,
            Hard | Good | Easy => Review,
        }
    }
}
