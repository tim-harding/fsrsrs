#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rating {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

impl Rating {
    pub fn iter_variants() -> impl Iterator<Item = Self> {
        [Rating::Again, Rating::Hard, Rating::Good, Rating::Easy].into_iter()
    }
}
