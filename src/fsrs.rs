use crate::{Card, Parameters, Rating};
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Default, Clone)]
pub struct Fsrs {
    parameters: Parameters,
}

impl Fsrs {
    pub const fn new(parameters: Parameters) -> Self {
        Self { parameters }
    }

    pub fn next_card(&self, card: Option<Card>, now: DateTime<Utc>, rating: Rating) -> Card {
        let p = &self.parameters;

        let (difficulty, stability) = if let Some(card) = card {
            let Card {
                stability,
                difficulty,
                ..
            } = card;
            (
                p.next_difficulty(difficulty, rating),
                p.next_stability(difficulty, stability, card.retrievability(p, now), rating),
            )
        } else {
            (p.init_difficulty(rating), p.init_stability(rating))
        };

        Card {
            difficulty,
            stability,
            rating,
            reviewed_at: now,
            interval: Duration::days(p.next_interval(stability) as i64),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Rating;
    use crate::{parameters::Parameters, Fsrs};
    use chrono::{DateTime, TimeZone, Utc};

    pub const TEST_RATINGS: [Rating; 13] = [
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Again,
        Rating::Again,
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Good,
    ];

    pub const WEIGHTS: [f64; 19] = [
        0.4197, 1.1869, 3.0412, 15.2441, 7.1434, 0.6477, 1.0007, 0.0674, 1.6597, 0.1712, 1.1178,
        2.0225, 0.0904, 0.3025, 2.1214, 0.2498, 2.9466, 0.4891, 0.6468,
    ];

    pub fn string_to_utc(date_string: &str) -> DateTime<Utc> {
        let datetime = DateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S %z %Z").unwrap();
        Utc.from_local_datetime(&datetime.naive_utc()).unwrap()
    }

    pub trait RoundFloat {
        fn round_float(self, precision: i32) -> f64;
    }

    impl RoundFloat for f64 {
        fn round_float(self, precision: i32) -> f64 {
            let multiplier = 10.0_f64.powi(precision);
            (self * multiplier).round() / multiplier
        }
    }

    #[test]
    fn longterm() {
        let params = Parameters {
            w: WEIGHTS,
            ..Default::default()
        };

        let mut card = None;
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let mut interval_history = vec![];
        let mut stability_history = vec![];
        let mut difficulty_history = vec![];

        for rating in TEST_RATINGS.into_iter() {
            let scheduler = Fsrs::new(params);
            card = Some(scheduler.next_card(card, now, rating));
            let card = card.unwrap();
            interval_history.push(card.interval.num_days());
            stability_history.push(card.stability.round_float(4));
            difficulty_history.push(card.difficulty.round_float(4));
            now += card.interval;
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
