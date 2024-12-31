use crate::{fuzz_range::FuzzRange, prng::Prng, Rating};
use std::time::Instant;

type Weights = [f64; 19];
const DEFAULT_WEIGHTS: Weights = [
    0.4072, 1.1829, 3.1262, 15.4722, 7.2102, 0.5316, 1.0651, 0.0234, 1.616, 0.1544, 1.0824, 1.9813,
    0.0953, 0.2975, 2.2042, 0.2407, 2.9466, 0.5034, 0.6567,
];

#[derive(Debug, Clone)]
pub struct Parameters {
    pub request_retention: f64,
    pub maximum_interval: i32,
    pub w: Weights,
    pub decay: f64,
    pub factor: f64,
    pub enable_short_term: bool,
    pub enable_fuzz: bool,
    pub seed: String,
}

impl Parameters {
    pub const DECAY: f64 = -0.5;
    /// (9/10) ^ (1 / DECAY) - 1
    pub const FACTOR: f64 = 19f64 / 81f64;

    pub fn forgetting_curve(elapsed_days: f64, stability: f64) -> f64 {
        (1.0 + Self::FACTOR * elapsed_days / stability).powf(Self::DECAY)
    }

    pub fn init_difficulty(&self, rating: Rating) -> f64 {
        let rating_int: i32 = rating as i32;

        (self.w[4] - f64::exp(self.w[5] * (rating_int as f64 - 1.0)) + 1.0).clamp(1.0, 10.0)
    }

    pub fn init_stability(&self, rating: Rating) -> f64 {
        let rating_int: i32 = rating as i32;
        self.w[(rating_int - 1) as usize].max(0.1)
    }

    #[allow(clippy::suboptimal_flops)]
    pub fn next_interval(&self, stability: f64, elapsed_days: i64) -> f64 {
        let new_interval = (stability / Self::FACTOR
            * (self.request_retention.powf(1.0 / Self::DECAY) - 1.0))
            .round()
            .clamp(1.0, self.maximum_interval as f64);
        self.apply_fuzz(new_interval, elapsed_days)
    }

    pub fn next_difficulty(&self, difficulty: f64, rating: Rating) -> f64 {
        let rating_int = rating as i32;
        let next_difficulty = self.w[6].mul_add(-(rating_int as f64 - 3.0), difficulty);
        let mean_reversion =
            self.mean_reversion(self.init_difficulty(Rating::Easy), next_difficulty);
        mean_reversion.clamp(1.0, 10.0)
    }

    pub fn short_term_stability(&self, stability: f64, rating: Rating) -> f64 {
        let rating_int = rating as i32;
        stability * f64::exp(self.w[17] * (rating_int as f64 - 3.0 + self.w[18]))
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

    pub fn next_forget_stability(
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

        let mut generator = Prng::new(self.seed.as_str());
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
        Self {
            request_retention: 0.9,
            maximum_interval: 36500,
            w: DEFAULT_WEIGHTS,
            decay: Self::DECAY,
            factor: Self::FACTOR,
            enable_short_term: true,
            enable_fuzz: false,
            seed: format!("{:?}", Instant::now()),
        }
    }
}
