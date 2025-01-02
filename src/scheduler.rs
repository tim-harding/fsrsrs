use crate::{Card, Parameters, Rating, State::*};
use chrono::{DateTime, Duration, Utc};

pub struct Scheduler {
    pub now: DateTime<Utc>,
    pub parameters: Parameters,
    pub card: Card,
}

impl Scheduler {
    pub fn new(parameters: Parameters, card: Card, now: DateTime<Utc>) -> Self {
        Self {
            parameters,
            card,
            now,
        }
    }

    pub fn next_card(&self, rating: Rating) -> Card {
        let mut out = match self.card.state {
            New => self.review_new(rating),
            Learning | Relearning | Reviewing => self.review_reviewing(rating),
        };
        out.state = Reviewing;
        out.rating = rating;
        out
    }

    fn review_new(&self, rating: Rating) -> Card {
        let p = &self.parameters;

        let mut next = Card {
            difficulty: p.init_difficulty(rating),
            stability: p.init_stability(rating),
            reviewed_at: self.now,
            ..self.card
        };

        let interval = self.parameters.next_interval(next.stability);
        next.due = self.now + Duration::days(interval as i64);
        next
    }

    fn review_reviewing(&self, rating: Rating) -> Card {
        let p = &self.parameters;
        let stability = self.card.stability;
        let difficulty = self.card.difficulty;
        let retrievability = self.card.retrievability(&self.parameters, self.now);

        let mut next = Card {
            difficulty: p.next_difficulty(difficulty, rating),
            stability: p.next_stability(difficulty, stability, retrievability, rating),
            reviewed_at: self.now,
            ..self.card
        };

        let interval = self.parameters.next_interval(next.stability);
        next.due = self.now + Duration::days(interval as i64);
        next
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::Card,
        parameters::Parameters,
        scheduler::Scheduler,
        testing::{string_to_utc, RoundFloat, TEST_RATINGS, WEIGHTS},
    };

    #[test]
    fn longterm() {
        let params = Parameters {
            w: WEIGHTS,
            ..Default::default()
        };

        let mut card = Card::new();
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let mut interval_history = vec![];
        let mut stability_history = vec![];
        let mut difficulty_history = vec![];

        for rating in TEST_RATINGS.into_iter() {
            let scheduler = Scheduler::new(params, card, now);
            card = scheduler.next_card(rating);

            interval_history.push(card.scheduled_days());
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
