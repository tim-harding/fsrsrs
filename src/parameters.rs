use crate::{Grade, ParametersBuilder};

/// FSRS algorithm weights
pub type Weights = [f64; 19];

/// FSRS algorithm parameters
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Parameters {
    pub(crate) w: Weights,
    pub(crate) retention: f64,
    pub(crate) maximum_interval: i32,
}

impl Parameters {
    pub const DECAY: f64 = -0.5;
    pub const FACTOR: f64 = 19f64 / 81f64;
    pub(crate) const DEFAULT_WEIGHTS: Weights = [
        0.4072, 1.1829, 3.1262, 15.4722, 7.2102, 0.5316, 1.0651, 0.0234, 1.616, 0.1544, 1.0824,
        1.9813, 0.0953, 0.2975, 2.2042, 0.2407, 2.9466, 0.5034, 0.6567,
    ];

    pub(crate) fn forgetting_curve(&self, elapsed_days: f64, stability: f64) -> f64 {
        (1.0 + Self::FACTOR * elapsed_days / stability).powf(Self::DECAY)
    }

    pub(crate) fn init_difficulty(&self, grade: Grade) -> f64 {
        let grade_int: i32 = grade as i32;

        (self.w[4] - f64::exp(self.w[5] * (grade_int as f64 - 1.0)) + 1.0).clamp(1.0, 10.0)
    }

    pub(crate) fn init_stability(&self, grade: Grade) -> f64 {
        let grade_int: i32 = grade as i32;
        self.w[(grade_int - 1) as usize].max(0.1)
    }

    pub(crate) fn next_interval(&self, stability: f64) -> f64 {
        (stability / Self::FACTOR * (self.retention.powf(1.0 / Self::DECAY) - 1.0))
            .round()
            .clamp(1.0, self.maximum_interval as f64)
    }

    pub(crate) fn next_difficulty(&self, difficulty: f64, grade: Grade) -> f64 {
        let grade_int = grade as i32;
        let next_difficulty = self.w[6].mul_add(-(grade_int as f64 - 3.0), difficulty);
        let mean_reversion =
            self.mean_reversion(self.init_difficulty(Grade::Easy), next_difficulty);
        mean_reversion.clamp(1.0, 10.0)
    }

    pub(crate) fn next_stability(
        &self,
        difficulty: f64,
        stability: f64,
        retrievability: f64,
        grade: Grade,
    ) -> f64 {
        match grade {
            Grade::Again => self.next_forget_stability(difficulty, stability, retrievability),
            Grade::Hard | Grade::Good | Grade::Easy => {
                self.next_recall_stability(difficulty, stability, retrievability, grade)
            }
        }
    }

    pub fn next_recall_stability(
        &self,
        difficulty: f64,
        stability: f64,
        retrievability: f64,
        grade: Grade,
    ) -> f64 {
        let modifier = match grade {
            Grade::Hard => self.w[15],
            Grade::Easy => self.w[16],
            _ => 1.0,
        };

        stability
            * (((self.w[8]).exp()
                * (11.0 - difficulty)
                * stability.powf(-self.w[9])
                * (((1.0 - retrievability) * self.w[10]).exp_m1()))
            .mul_add(modifier, 1.0))
    }

    pub(crate) fn next_forget_stability(
        &self,
        difficulty: f64,
        stability: f64,
        retrievability: f64,
    ) -> f64 {
        self.w[11]
            * difficulty.powf(-self.w[12])
            * ((stability + 1.0).powf(self.w[13]) - 1.0)
            * f64::exp((1.0 - retrievability) * self.w[14])
    }

    fn mean_reversion(&self, initial: f64, current: f64) -> f64 {
        self.w[7].mul_add(initial, (1.0 - self.w[7]) * current)
    }
}

impl Default for Parameters {
    fn default() -> Self {
        ParametersBuilder::new().build()
    }
}
