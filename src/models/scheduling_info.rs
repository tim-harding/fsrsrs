use super::{Card, ReviewLog};

#[derive(Debug, Clone)]
pub struct SchedulingInfo {
    pub card: Card,
    pub review_log: ReviewLog,
}
