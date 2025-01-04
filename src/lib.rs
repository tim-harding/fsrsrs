mod algo;

mod fsrs;
pub use fsrs::Fsrs;

mod card;
pub use card::{Card, Grade};

mod parameters;
pub use parameters::Parameters;

mod parameters_builder;
pub use parameters_builder::ParametersBuilder;

#[doc = include_str!("../README.md")]
mod readme {}

pub type Time = chrono::DateTime<chrono::Utc>;
pub use chrono::Duration;
