use crate::{fuzz_range::FuzzRange, prng::Prng, ParametersBuilder, Rating};

pub(crate) type Weights = [f64; 19];

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Parameters {
    pub(crate) w: Weights,
    pub(crate) request_retention: f64,
    pub(crate) maximum_interval: i32,
    pub(crate) decay: f64,
    pub(crate) factor: f64,
    pub(crate) enable_short_term: bool,
    pub(crate) enable_fuzz: bool,
}

impl Parameters {
    pub(crate) const DECAY: f64 = -0.5;
    pub(crate) const FACTOR: f64 = 19f64 / 81f64; // (9/10) ^ (1 / DECAY) - 1
    pub(crate) const WEIGHTS: Weights = [
        0.4072, 1.1829, 3.1262, 15.4722, 7.2102, 0.5316, 1.0651, 0.0234, 1.616, 0.1544, 1.0824,
        1.9813, 0.0953, 0.2975, 2.2042, 0.2407, 2.9466, 0.5034, 0.6567,
    ];

    pub(crate) fn forgetting_curve(&self, elapsed_days: f64, stability: f64) -> f64 {
        (1.0 + self.factor * elapsed_days / stability).powf(self.decay)
    }

    pub(crate) fn init_difficulty(&self, rating: Rating) -> f64 {
        let rating_int: i32 = rating as i32;

        (self.w[4] - f64::exp(self.w[5] * (rating_int as f64 - 1.0)) + 1.0).clamp(1.0, 10.0)
    }

    pub(crate) fn init_stability(&self, rating: Rating) -> f64 {
        let rating_int: i32 = rating as i32;
        self.w[(rating_int - 1) as usize].max(0.1)
    }

    pub(crate) fn next_interval(&self, stability: f64, elapsed_days: i64) -> f64 {
        let new_interval = (stability / Self::FACTOR
            * (self.request_retention.powf(1.0 / Self::DECAY) - 1.0))
            .round()
            .clamp(1.0, self.maximum_interval as f64);
        self.apply_fuzz(new_interval, elapsed_days)
    }

    pub(crate) fn next_difficulty(&self, difficulty: f64, rating: Rating) -> f64 {
        let rating_int = rating as i32;
        let next_difficulty = self.w[6].mul_add(-(rating_int as f64 - 3.0), difficulty);
        let mean_reversion =
            self.mean_reversion(self.init_difficulty(Rating::Easy), next_difficulty);
        mean_reversion.clamp(1.0, 10.0)
    }

    pub(crate) fn short_term_stability(&self, stability: f64, rating: Rating) -> f64 {
        let rating_int = rating as i32;
        stability * f64::exp(self.w[17] * (rating_int as f64 - 3.0 + self.w[18]))
    }

    pub(crate) fn next_stability(
        &self,
        difficulty: f64,
        stability: f64,
        retrievability: f64,
        rating: Rating,
    ) -> f64 {
        match rating {
            Rating::Again => self.next_forget_stability(difficulty, stability, retrievability),
            Rating::Hard | Rating::Good | Rating::Easy => {
                self.next_recall_stability(difficulty, stability, retrievability, rating)
            }
        }
    }

    pub fn next_recall_stability(
        &self,
        difficulty: f64,
        stability: f64,
        retrievability: f64,
        rating: Rating,
    ) -> f64 {
        let modifier = match rating {
            Rating::Hard => self.w[15],
            Rating::Easy => self.w[16],
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

    fn apply_fuzz(&self, interval: f64, elapsed_days: i64) -> f64 {
        if !self.enable_fuzz || interval < 2.5 {
            return interval;
        }

        let mut generator = Prng::new("RNG");
        let fuzz_factor = generator.double();
        let (min_interval, max_interval) =
            FuzzRange::get_fuzz_range(interval, elapsed_days, self.maximum_interval);

        fuzz_factor.mul_add(
            max_interval as f64 - min_interval as f64 + 1.0,
            min_interval as f64,
        )
    }
}

impl Default for Parameters {
    fn default() -> Self {
        ParametersBuilder::new().build()
    }
}
