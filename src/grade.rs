/// Difficulty of a review
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Grade {
    Fail,
    Hard,
    Good,
    Easy,
}

impl Grade {
    pub(crate) fn into_f64(self) -> f64 {
        use Grade::*;
        match self {
            Fail => 1.0,
            Hard => 2.0,
            Good => 3.0,
            Easy => 4.0,
        }
    }
}
