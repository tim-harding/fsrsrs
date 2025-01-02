use super::base::Base;
use crate::{
    cards::Cards,
    Card, Parameters,
    Rating::{self, *},
    Review,
    State::{self, *},
};
use chrono::{DateTime, Duration, Utc};

pub struct Basic(Base);

impl Basic {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Self {
        Self(Base::new(parameters, card, now))
    }

    pub fn next_card(&self, rating: Rating) -> Card {
        match self.0.previous.state {
            New => self.review_new(rating),
            Learning | Relearning => self.review_learning(rating),
            Reviewing => self.review_reviewing(rating),
        }
    }

    pub fn current_review(&self, rating: Rating, card: Card) -> Review {
        self.0.current_review(rating, card)
    }

    fn review_new(&self, rating: Rating) -> Card {
        let p = &self.0.parameters;

        let mut card = self.0.current;
        card.difficulty = p.init_difficulty(rating);
        card.stability = p.init_stability(rating);

        let (due, state) = match rating {
            Again => (Duration::minutes(1), Learning),
            Hard => (Duration::minutes(5), Learning),
            Good => (Duration::minutes(10), Learning),
            Easy => {
                let easy_interval =
                    p.next_interval(card.stability, card.elapsed_days(self.0.now)) as i64;
                (Duration::days(easy_interval), Reviewing)
            }
        };

        card.due = self.0.now + due;
        card.state = state;
        card
    }

    fn review_learning(&self, rating: Rating) -> Card {
        let p = &self.0.parameters;
        let last = &self.0.previous;
        let interval = self.0.current.elapsed_days(self.0.now);

        let mut card = self.0.current;
        card.difficulty = p.next_difficulty(last.difficulty, rating);
        card.stability = p.short_term_stability(last.stability, rating);

        let (due, state) = match rating {
            Again => (Duration::minutes(5), last.state),
            Hard => (Duration::minutes(10), last.state),
            Good => {
                let good_interval = p.next_interval(card.stability, interval) as i64;
                (Duration::days(good_interval), Reviewing)
            }
            Easy => {
                let good_stability = p.short_term_stability(last.stability, Good);
                let good_interval = p.next_interval(good_stability, interval);
                let easy_interval = p
                    .next_interval(card.stability, interval)
                    .max(good_interval + 1.0) as i64;
                (Duration::days(easy_interval), Reviewing)
            }
        };

        card.due = self.0.now + due;
        card.state = state;
        card
    }

    fn review_reviewing(&self, rating: Rating) -> Card {
        let p = &self.0.parameters;
        let interval = self.0.current.elapsed_days(self.0.now);
        let stability = self.0.previous.stability;
        let difficulty = self.0.previous.difficulty;
        let retrievability = self.0.previous.retrievability(p, self.0.now);

        let mut cards = Cards::splat(self.0.current);
        cards.update(|(rating, card)| {
            card.difficulty = p.next_difficulty(difficulty, rating);
            card.stability = p.next_stability(difficulty, stability, retrievability, rating);
        });

        let interval = self.review_intervals(cards.map(|(_, card)| card.stability), interval);

        let mut card = cards[rating];
        card.due = self.0.now
            + (match rating {
                Again => Duration::minutes(5),
                Hard | Good | Easy => Duration::days(interval[rating]),
            });
        card.state = next_state(rating);
        card
    }

    fn review_intervals(&self, stability: Cards<f64>, interval_previous: i64) -> Cards<i64> {
        let p = &self.0.parameters;
        let mut interval = Cards::splat(0.0f64);

        interval.hard = p.next_interval(stability.hard, interval_previous);
        interval.good = p.next_interval(stability.good, interval_previous);
        interval.hard = interval.hard.min(interval.good);
        interval.good = interval.good.max(interval.hard + 1.0);
        interval.easy = p
            .next_interval(stability.easy, interval_previous)
            .max(interval.good + 1.0);

        interval.map(|(_, i)| i as i64)
    }
}

fn next_state(rating: Rating) -> State {
    match rating {
        Again => Relearning,
        Hard | Good | Easy => Reviewing,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{Card, Rating},
        parameters::Parameters,
        scheduler::basic::Basic,
        testing::{string_to_utc, RoundFloat, TEST_RATINGS, WEIGHTS},
        State,
    };
    use chrono::Duration;

    #[test]
    fn interval() {
        let mut card = Card::new();
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let mut interval_history = vec![];

        for rating in TEST_RATINGS.into_iter() {
            let scheduler = Basic::new(Parameters::default(), card, now);
            card = scheduler.next_card(rating);
            let review = scheduler.current_review(rating, card);
            interval_history.push(review.scheduled_days);
            now = card.due;
        }
        let expected = [0, 4, 15, 48, 136, 351, 0, 0, 7, 13, 24, 43, 77];
        assert_eq!(interval_history, expected);
    }

    #[test]
    fn state() {
        let params = Parameters {
            w: WEIGHTS,
            ..Default::default()
        };

        let mut card = Card::new();
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let mut state_list = vec![];

        for rating in TEST_RATINGS.into_iter() {
            let scheduler = Basic::new(params, card, now);
            card = scheduler.next_card(rating);
            let rev_log = scheduler.current_review(rating, card);
            state_list.push(rev_log.state);
            now = card.due;
        }
        use State::*;
        let expected = [
            New, Learning, Reviewing, Reviewing, Reviewing, Reviewing, Reviewing, Relearning,
            Relearning, Reviewing, Reviewing, Reviewing, Reviewing,
        ];
        assert_eq!(state_list, expected);
    }

    #[test]
    fn memo_state() {
        let params = Parameters {
            w: WEIGHTS,
            ..Default::default()
        };

        let mut card = Card::new();
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let ratings = [
            Rating::Again,
            Rating::Good,
            Rating::Good,
            Rating::Good,
            Rating::Good,
            Rating::Good,
        ];
        let intervals = [0, 0, 1, 3, 8, 21];
        for (index, rating) in ratings.into_iter().enumerate() {
            card = Basic::new(params, card, now).next_card(rating);
            now += Duration::days(intervals[index] as i64);
        }

        card = Basic::new(params, card, now).next_card(Rating::Good);
        assert_eq!(card.stability.round_float(4), 71.4554);
        assert_eq!(card.difficulty.round_float(4), 5.0976);
    }

    #[test]
    fn retrievability() {
        let card = Card::new();
        let now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let expect_retrievability = [1.0, 1.0, 1.0, 0.9026208];

        for (i, rating) in [Rating::Again, Rating::Hard, Rating::Good, Rating::Easy]
            .into_iter()
            .enumerate()
        {
            let card = Basic::new(Parameters::default(), card, now).next_card(rating);
            let retrievability = card.retrievability(&Parameters::default(), card.due);

            assert_eq!(retrievability.round_float(7), expect_retrievability[i]);
        }
    }
}
