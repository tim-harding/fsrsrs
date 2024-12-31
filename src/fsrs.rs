use crate::{models::Card, parameters::Parameters, scheduler::Scheduler};
use chrono::{DateTime, Utc};

#[derive(Debug, Default, Clone)]
pub struct Fsrs {
    parameters: Parameters,
}

impl Fsrs {
    pub const fn new(parameters: Parameters) -> Self {
        Self { parameters }
    }

    pub fn scheduler(&self, card: Card, now: DateTime<Utc>) -> Scheduler {
        // TODO: Avoid cloning this each time
        Scheduler::new(self.parameters.clone(), card, now)
    }
}

#[cfg(test)]
mod tests {
    use super::Fsrs;
    use crate::{
        models::{Card, Rating},
        parameters::Parameters,
        State,
    };
    use chrono::{DateTime, Duration, TimeZone, Utc};

    static TEST_RATINGS: [Rating; 13] = [
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

    static WEIGHTS: [f64; 19] = [
        0.4197, 1.1869, 3.0412, 15.2441, 7.1434, 0.6477, 1.0007, 0.0674, 1.6597, 0.1712, 1.1178,
        2.0225, 0.0904, 0.3025, 2.1214, 0.2498, 2.9466, 0.4891, 0.6468,
    ];

    fn string_to_utc(date_string: &str) -> DateTime<Utc> {
        let datetime = DateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S %z %Z").unwrap();
        Utc.from_local_datetime(&datetime.naive_utc()).unwrap()
    }

    trait RoundFloat {
        fn round_float(self, precision: i32) -> f64;
    }

    impl RoundFloat for f64 {
        fn round_float(self, precision: i32) -> f64 {
            let multiplier = 10.0_f64.powi(precision);
            (self * multiplier).round() / multiplier
        }
    }

    #[test]
    fn test_basic_scheduler_interval() {
        let fsrs = Fsrs::default();
        let mut card = Card::new();
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let mut interval_history = vec![];

        for rating in TEST_RATINGS.into_iter() {
            card = fsrs.scheduler(card, now).next_card(rating);
            interval_history.push(card.scheduled_days);
            now = card.due;
        }
        let expected = [0, 4, 15, 48, 136, 351, 0, 0, 7, 13, 24, 43, 77];
        assert_eq!(interval_history, expected);
    }

    #[test]
    fn test_basic_scheduler_state() {
        let params = Parameters {
            w: WEIGHTS,
            ..Default::default()
        };

        let fsrs = Fsrs::new(params);
        let mut card = Card::new();
        let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let mut state_list = vec![];

        for rating in TEST_RATINGS.into_iter() {
            let record = fsrs.scheduler(card, now).schedule(rating);
            card = record.card;
            let rev_log = record.review;
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
    fn test_basic_scheduler_memo_state() {
        let params = Parameters {
            w: WEIGHTS,
            ..Default::default()
        };

        let fsrs = Fsrs::new(params);
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
            card = fsrs.scheduler(card, now).next_card(rating);
            now += Duration::days(intervals[index] as i64);
        }

        card = fsrs.scheduler(card, now).next_card(Rating::Good);
        assert_eq!(card.stability.round_float(4), 71.4554);
        assert_eq!(card.difficulty.round_float(4), 5.0976);
    }

    #[test]
    fn test_get_retrievability() {
        let fsrs = Fsrs::default();
        let card = Card::new();
        let now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
        let expect_retrievability = [1.0, 1.0, 1.0, 0.9026208];

        for (i, rating) in [Rating::Again, Rating::Hard, Rating::Good, Rating::Easy]
            .into_iter()
            .enumerate()
        {
            let card = fsrs.scheduler(card, now).next_card(rating);
            let retrievability = card.retrievability(&Parameters::new(), card.due);

            assert_eq!(retrievability.round_float(7), expect_retrievability[i]);
        }
    }
}
