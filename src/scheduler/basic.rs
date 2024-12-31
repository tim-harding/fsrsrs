use super::base::Base;
use crate::{
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
        let mut card = self.0.current;
        card.difficulty = self.0.parameters.init_difficulty(rating);
        card.stability = self.0.parameters.init_stability(rating);

        let (days, due, state) = match rating {
            Again => (0, Duration::minutes(1), Learning),
            Hard => (0, Duration::minutes(5), Learning),
            Good => (0, Duration::minutes(10), Learning),
            Easy => {
                let easy_interval = self
                    .0
                    .parameters
                    .next_interval(card.stability, card.elapsed_days)
                    as i64;
                (easy_interval, Duration::days(easy_interval), Review)
            }
        };

        card.scheduled_days = days;
        card.due = self.0.now + due;
        card.state = state;
        card
    }

    fn review_learning(&mut self, rating: Rating) -> Card {
        let mut card = self.0.current;
        let interval = self.0.current.elapsed_days;
        card.difficulty = self
            .0
            .parameters
            .next_difficulty(self.0.last.difficulty, rating);
        card.stability = self
            .0
            .parameters
            .short_term_stability(self.0.last.stability, rating);

        let (days, due, state) = match rating {
            Again => (0, Duration::minutes(5), self.0.last.state),
            Hard => (0, Duration::minutes(10), self.0.last.state),
            Good => {
                let good_interval =
                    self.0.parameters.next_interval(card.stability, interval) as i64;
                (good_interval, Duration::days(good_interval), Review)
            }
            Easy => {
                let good_stability = self
                    .0
                    .parameters
                    .short_term_stability(self.0.last.stability, Good);
                let good_interval = self.0.parameters.next_interval(good_stability, interval);
                let easy_interval = self
                    .0
                    .parameters
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
        let next = self.0.current;
        let interval = self.0.current.elapsed_days;
        let stability = self.0.last.stability;
        let difficulty = self.0.last.difficulty;
        let retrievability = self.0.last.retrievability(&self.0.parameters, self.0.now);

        let mut next_again = next;
        let mut next_hard = next;
        let mut next_good = next;
        let mut next_easy = next;

        next_again.difficulty = self.0.parameters.next_difficulty(difficulty, Again);
        next_hard.difficulty = self.0.parameters.next_difficulty(difficulty, Hard);
        next_good.difficulty = self.0.parameters.next_difficulty(difficulty, Good);
        next_easy.difficulty = self.0.parameters.next_difficulty(difficulty, Easy);

        next_again.stability =
            self.0
                .parameters
                .next_stability(difficulty, stability, retrievability, Again);
        next_hard.stability =
            self.0
                .parameters
                .next_stability(difficulty, stability, retrievability, Hard);
        next_good.stability =
            self.0
                .parameters
                .next_stability(difficulty, stability, retrievability, Good);
        next_easy.stability =
            self.0
                .parameters
                .next_stability(difficulty, stability, retrievability, Easy);

        let mut hard_interval = self
            .0
            .parameters
            .next_interval(next_hard.stability, interval);
        let mut good_interval = self
            .0
            .parameters
            .next_interval(next_good.stability, interval);
        hard_interval = hard_interval.min(good_interval);
        good_interval = good_interval.max(hard_interval + 1.0);
        let easy_interval = self
            .0
            .parameters
            .next_interval(next_easy.stability, interval)
            .max(good_interval + 1.0);

        next_again.scheduled_days = 0;
        next_again.due = self.0.now + Duration::minutes(5);

        next_hard.scheduled_days = hard_interval as i64;
        next_hard.due = self.0.now + Duration::days(hard_interval as i64);

        next_good.scheduled_days = good_interval as i64;
        next_good.due = self.0.now + Duration::days(good_interval as i64);

        next_easy.scheduled_days = easy_interval as i64;
        next_easy.due = self.0.now + Duration::days(easy_interval as i64);

        next_again.state = Self::next_state(Again);
        next_hard.state = Self::next_state(Hard);
        next_good.state = Self::next_state(Good);
        next_easy.state = Self::next_state(Easy);
        next_again.lapses += 1;

        match rating {
            Again => next_again,
            Hard => next_hard,
            Good => next_good,
            Easy => next_easy,
        }
    }

    fn next_state(rating: Rating) -> State {
        match rating {
            Again => Relearning,
            Hard | Good | Easy => Review,
        }
    }
}
