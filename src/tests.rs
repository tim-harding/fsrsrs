#![cfg(test)]

use crate::{
    algo::Fsrs,
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

    for rating in TEST_RATINGS.iter() {
        let next = fsrs.next(card, now, *rating);
        card = next.card;
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
    let mut record_log = fsrs.repeat(card, now);

    for rating in TEST_RATINGS.iter() {
        card = record_log[rating].card.clone();
        let rev_log = record_log[rating].review_log.clone();
        state_list.push(rev_log.state);
        now = card.due;
        record_log = fsrs.repeat(card, now);
    }
    use State::*;
    let expected = [
        New, Learning, Review, Review, Review, Review, Review, Relearning, Relearning, Review,
        Review, Review, Review,
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
    let mut record_log = fsrs.repeat(card.clone(), now);
    let ratings = [
        Rating::Again,
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Good,
        Rating::Good,
    ];
    let intervals = [0, 0, 1, 3, 8, 21];
    for (index, rating) in ratings.iter().enumerate() {
        card = record_log[rating].card.clone();
        now += Duration::days(intervals[index] as i64);
        record_log = fsrs.repeat(card.clone(), now);
    }

    card = record_log[&Rating::Good].to_owned().card;
    assert_eq!(card.stability.round_float(4), 71.4554);
    assert_eq!(card.difficulty.round_float(4), 5.0976);
}

#[test]
fn test_long_term_scheduler() {
    let params = Parameters {
        w: WEIGHTS,
        enable_short_term: false,
        ..Default::default()
    };

    let fsrs = Fsrs::new(params);
    let mut card = Card::new();
    let mut now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
    let mut interval_history = vec![];
    let mut stability_history = vec![];
    let mut difficulty_history = vec![];

    for rating in TEST_RATINGS.iter() {
        let record = fsrs.repeat(card.clone(), now)[rating].to_owned();
        let next = fsrs.next(card, now, *rating);

        assert_eq!(record.card, next.card);

        card = record.card;
        interval_history.push(card.scheduled_days);
        stability_history.push(card.stability.round_float(4));
        difficulty_history.push(card.difficulty.round_float(4));
        now = card.due;
    }

    let expected_interval = [3, 13, 48, 155, 445, 1158, 17, 3, 9, 27, 74, 190, 457];
    let expected_stability = [
        3.0412, 13.0913, 48.1585, 154.9373, 445.0556, 1158.0778, 16.6306, 2.9888, 9.4633, 26.9474,
        73.9723, 189.7037, 457.4379,
    ];
    let expected_difficulty = [
        4.4909, 4.2666, 4.0575, 3.8624, 3.6804, 3.5108, 5.219, 6.8122, 6.4314, 6.0763, 5.7452,
        5.4363, 5.1483,
    ];

    assert_eq!(interval_history, expected_interval);
    assert_eq!(stability_history, expected_stability);
    assert_eq!(difficulty_history, expected_difficulty);
}

#[test]
fn test_get_retrievability() {
    let fsrs = Fsrs::default();
    let card = Card::new();
    let now = string_to_utc("2022-11-29 12:30:00 +0000 UTC");
    let expect_retrievability = [1.0, 1.0, 1.0, 0.9026208];
    let scheduler = fsrs.repeat(card, now);

    for (i, rating) in Rating::iter_variants().enumerate() {
        let card = scheduler.get(&rating).unwrap().card.clone();
        let retrievability = card.retrievability(card.due);

        assert_eq!(retrievability.round_float(7), expect_retrievability[i]);
    }
}
