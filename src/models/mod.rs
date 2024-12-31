use std::collections::HashMap;

mod card;
pub use card::Card;

mod rating;
pub use rating::Rating;

mod review_log;
pub use review_log::ReviewLog;

mod scheduling_info;
pub use scheduling_info::SchedulingInfo;

mod state;
pub use state::State;

pub type RecordLog = HashMap<Rating, SchedulingInfo>;
