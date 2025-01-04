use crate::{algo, from_days, Card, Grade, Time};

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
        reviewed_at: now,
        due: now + from_days(stability),
    }
}
