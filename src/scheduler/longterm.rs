use super::base::Base;
use crate::{
    Card, Parameters,
    Rating::{self, *},
    Schedule,
    State::*,
};
use chrono::{DateTime, Duration, Utc};

pub struct Longterm(Base);

impl Longterm {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Self {
        Self(Base::new(parameters, card, now))
    }

    pub fn review(&mut self, rating: Rating) -> Schedule {
        match self.0.last.state {
            New => self.review_new(rating),
            Learning | Relearning => self.review_learning(rating),
            Reviewing => self.review_reviewing(rating),
        }
    }

    fn review_new(&mut self, rating: Rating) -> Schedule {
        let next = self.0.current;
        self.0.current.scheduled_days = 0;
        self.0.current.elapsed_days = 0;

        let mut next_again = next;
        let mut next_hard = next;
        let mut next_good = next;
        let mut next_easy = next;

        self.init_difficulty_stability(
            &mut next_again,
            &mut next_hard,
            &mut next_good,
            &mut next_easy,
        );
        self.next_interval(
            &mut next_again,
            &mut next_hard,
            &mut next_good,
            &mut next_easy,
            0,
        );
        self.next_state(
            &mut next_again,
            &mut next_hard,
            &mut next_good,
            &mut next_easy,
        );

        let item_again = Schedule {
            card: next_again,
            review: self.0.current_review(Again),
        };
        let item_hard = Schedule {
            card: next_hard,
            review: self.0.current_review(Hard),
        };
        let item_good = Schedule {
            card: next_good,
            review: self.0.current_review(Good),
        };
        let item_easy = Schedule {
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

    fn review_learning(&mut self, rating: Rating) -> Schedule {
        self.review_reviewing(rating)
    }

    fn review_reviewing(&mut self, rating: Rating) -> Schedule {
        let next = self.0.current;
        let interval = self.0.current.elapsed_days;
        let stability = self.0.last.stability;
        let difficulty = self.0.last.difficulty;
        let retrievability = self.0.last.retrievability(&self.0.parameters, self.0.now);

        let mut next_again = next;
        let mut next_hard = next;
        let mut next_good = next;
        let mut next_easy = next;

        self.next_difficulty_stability(
            &mut next_again,
            &mut next_hard,
            &mut next_good,
            &mut next_easy,
            difficulty,
            stability,
            retrievability,
        );
        self.next_interval(
            &mut next_again,
            &mut next_hard,
            &mut next_good,
            &mut next_easy,
            interval,
        );
        self.next_state(
            &mut next_again,
            &mut next_hard,
            &mut next_good,
            &mut next_easy,
        );
        next_again.lapses += 1;

        let item_again = Schedule {
            card: next_again,
            review: self.0.current_review(Again),
        };
        let item_hard = Schedule {
            card: next_hard,
            review: self.0.current_review(Hard),
        };
        let item_good = Schedule {
            card: next_good,
            review: self.0.current_review(Good),
        };
        let item_easy = Schedule {
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

    fn init_difficulty_stability(
        &self,
        next_again: &mut Card,
        next_hard: &mut Card,
        next_good: &mut Card,
        next_easy: &mut Card,
    ) {
        next_again.difficulty = self.0.parameters.init_difficulty(Again);
        next_again.stability = self.0.parameters.init_stability(Again);

        next_hard.difficulty = self.0.parameters.init_difficulty(Hard);
        next_hard.stability = self.0.parameters.init_stability(Hard);

        next_good.difficulty = self.0.parameters.init_difficulty(Good);
        next_good.stability = self.0.parameters.init_stability(Good);

        next_easy.difficulty = self.0.parameters.init_difficulty(Easy);
        next_easy.stability = self.0.parameters.init_stability(Easy);
    }

    #[allow(clippy::too_many_arguments)]
    fn next_difficulty_stability(
        &self,
        next_again: &mut Card,
        next_hard: &mut Card,
        next_good: &mut Card,
        next_easy: &mut Card,
        difficulty: f64,
        stability: f64,
        retrievability: f64,
    ) {
        next_again.difficulty = self.0.parameters.next_difficulty(difficulty, Again);
        next_again.stability =
            self.0
                .parameters
                .next_forget_stability(difficulty, stability, retrievability);

        next_hard.difficulty = self.0.parameters.next_difficulty(difficulty, Hard);
        next_hard.stability =
            self.0
                .parameters
                .next_recall_stability(difficulty, stability, retrievability, Hard);

        next_good.difficulty = self.0.parameters.next_difficulty(difficulty, Good);
        next_good.stability =
            self.0
                .parameters
                .next_recall_stability(difficulty, stability, retrievability, Good);

        next_easy.difficulty = self.0.parameters.next_difficulty(difficulty, Easy);
        next_easy.stability =
            self.0
                .parameters
                .next_recall_stability(difficulty, stability, retrievability, Easy);
    }

    fn next_interval(
        &self,
        next_again: &mut Card,
        next_hard: &mut Card,
        next_good: &mut Card,
        next_easy: &mut Card,
        elapsed_days: i64,
    ) {
        let mut again_interval = self
            .0
            .parameters
            .next_interval(next_again.stability, elapsed_days);
        let mut hard_interval = self
            .0
            .parameters
            .next_interval(next_hard.stability, elapsed_days);
        let mut good_interval = self
            .0
            .parameters
            .next_interval(next_good.stability, elapsed_days);
        let mut easy_interval = self
            .0
            .parameters
            .next_interval(next_easy.stability, elapsed_days);

        again_interval = again_interval.min(hard_interval);
        hard_interval = hard_interval.max(again_interval + 1.0);
        good_interval = good_interval.max(hard_interval + 1.0);
        easy_interval = easy_interval.max(good_interval + 1.0);

        next_again.scheduled_days = again_interval as i64;
        next_again.due = self.0.now + Duration::days(again_interval as i64);

        next_hard.scheduled_days = hard_interval as i64;
        next_hard.due = self.0.now + Duration::days(hard_interval as i64);

        next_good.scheduled_days = good_interval as i64;
        next_good.due = self.0.now + Duration::days(good_interval as i64);

        next_easy.scheduled_days = easy_interval as i64;
        next_easy.due = self.0.now + Duration::days(easy_interval as i64);
    }

    fn next_state(
        &self,
        next_again: &mut Card,
        next_hard: &mut Card,
        next_good: &mut Card,
        next_easy: &mut Card,
    ) {
        next_again.state = Reviewing;
        next_hard.state = Reviewing;
        next_good.state = Reviewing;
        next_easy.state = Reviewing;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::Card,
        parameters::Parameters,
        scheduler::longterm::Longterm,
        testing::{string_to_utc, RoundFloat, TEST_RATINGS, WEIGHTS},
    };

    #[test]
    fn longterm() {
        let params = Parameters {
            w: WEIGHTS,
            enable_short_term: false,
            ..Default::default()
        };

        let mut card = Card::new();
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let mut interval_history = vec![];
        let mut stability_history = vec![];
        let mut difficulty_history = vec![];

        for rating in TEST_RATINGS.into_iter() {
            let next = {
                let mut scheduler = Longterm::new(params.clone(), card, now);
                let schedule = scheduler.review(rating);
                schedule.card
            };

            card = {
                let mut scheduler = Longterm::new(params.clone(), card, now);
                let schedule = scheduler.review(rating);
                schedule.card
            };

            assert_eq!(card, next);

            interval_history.push(card.scheduled_days);
            stability_history.push(card.stability.round_float(4));
            difficulty_history.push(card.difficulty.round_float(4));
            now = card.due;
        }

        let expected_interval = [3, 13, 48, 155, 445, 1158, 17, 3, 9, 27, 74, 190, 457];
        let expected_stability = [
            3.0412, 13.0913, 48.1585, 154.9373, 445.0556, 1158.0778, 16.6306, 2.9888, 9.4633,
            26.9474, 73.9723, 189.7037, 457.4379,
        ];
        let expected_difficulty = [
            4.4909, 4.2666, 4.0575, 3.8624, 3.6804, 3.5108, 5.219, 6.8122, 6.4314, 6.0763, 5.7452,
            5.4363, 5.1483,
        ];

        assert_eq!(interval_history, expected_interval);
        assert_eq!(stability_history, expected_stability);
        assert_eq!(difficulty_history, expected_difficulty);
    }
}
