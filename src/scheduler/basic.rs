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

    fn new_state(&self, rating: Rating) -> SchedulingInfo {
        let mut next = self.0.current;
        next.difficulty = self.0.parameters.init_difficulty(rating);
        next.stability = self.0.parameters.init_stability(rating);

        match rating {
            Again => {
                next.scheduled_days = 0;
                next.due = self.0.now + Duration::minutes(1);
                next.state = Learning;
            }
            Hard => {
                next.scheduled_days = 0;
                next.due = self.0.now + Duration::minutes(5);
                next.state = Learning;
            }
            Good => {
                next.scheduled_days = 0;
                next.due = self.0.now + Duration::minutes(10);
                next.state = Learning;
            }
            Easy => {
                let easy_interval = self
                    .0
                    .parameters
                    .next_interval(next.stability, next.elapsed_days);
                next.scheduled_days = easy_interval as i64;
                next.due = self.0.now + Duration::days(easy_interval as i64);
                next.state = Review;
            }
        };

        SchedulingInfo {
            card: next,
            review: self.0.current_review(rating),
        }
    }

    fn learning_state(&mut self, rating: Rating) -> SchedulingInfo {
        let mut next = self.0.current;
        let interval = self.0.current.elapsed_days;
        next.difficulty = self
            .0
            .parameters
            .next_difficulty(self.0.last.difficulty, rating);
        next.stability = self
            .0
            .parameters
            .short_term_stability(self.0.last.stability, rating);

        match rating {
            Again => {
                next.scheduled_days = 0;
                next.due = self.0.now + Duration::minutes(5);
                next.state = self.0.last.state;
            }
            Hard => {
                next.scheduled_days = 0;
                next.due = self.0.now + Duration::minutes(10);
                next.state = self.0.last.state;
            }
            Good => {
                let good_interval = self.0.parameters.next_interval(next.stability, interval);
                next.scheduled_days = good_interval as i64;
                next.due = self.0.now + Duration::days(good_interval as i64);
                next.state = Review;
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
                    .next_interval(next.stability, interval)
                    .max(good_interval + 1.0);
                next.scheduled_days = easy_interval as i64;
                next.due = self.0.now + Duration::days(easy_interval as i64);
                next.state = Review;
            }
        }

        SchedulingInfo {
            card: next,
            review: self.0.current_review(rating),
        }
    }

    fn review_state(&mut self, rating: Rating) -> SchedulingInfo {
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

        let item_again = SchedulingInfo {
            card: next_again,
            review: self.0.current_review(Again),
        };
        let item_hard = SchedulingInfo {
            card: next_hard,
            review: self.0.current_review(Hard),
        };
        let item_good = SchedulingInfo {
            card: next_good,
            review: self.0.current_review(Good),
        };
        let item_easy = SchedulingInfo {
            card: next_easy,
            review: self.0.current_review(Easy),
        };

        match rating {
            Again => item_again,
            Hard => item_hard,
            Good => item_good,
            Easy => item_easy,
        }
    }

    fn next_state(rating: Rating) -> State {
        match rating {
            Again => Relearning,
            Hard | Good | Easy => Review,
        }
    }

    pub fn review(&mut self, rating: Rating) -> SchedulingInfo {
        match self.0.last.state {
            New => self.new_state(rating),
            Learning | Relearning => self.learning_state(rating),
            Review => self.review_state(rating),
        }
    }
}
