use crate::{algo, Card, Duration, Grade, Time};

const SEC_PER_DAY: f64 = 60.0 * 60.0 * 24.0;

/// Compute the new FSRS card state after a review
///
/// # Parameters
///
/// - `card`: The card being reviewed, or None if it's the first review
/// - `now`: The time the card is reviewed
/// - `grade`: The difficulty of the review
pub fn review(card: Option<Card>, now: Time, grade: Grade) -> Card {
    let (difficulty, stability) = if let Some(card) = card {
        let elapsed = card.elapsed_days(now);
        let Card {
            stability,
            difficulty,
            ..
        } = card;
        (
            algo::difficulty(difficulty, grade),
            algo::stability(
                difficulty,
                stability,
                algo::retrievability(elapsed, stability),
                grade,
            ),
        )
    } else {
        (algo::d_0(grade), algo::s_0(grade))
    };

    Card {
        difficulty,
        stability,
        grade,
        when: now,
        due: now + Duration::seconds((stability * SEC_PER_DAY) as i64),
    }
}
