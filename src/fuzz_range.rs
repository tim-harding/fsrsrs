pub struct FuzzRange {
    start: f64,
    end: f64,
    factor: f64,
}

impl FuzzRange {
    pub const fn new(start: f64, end: f64, factor: f64) -> Self {
        Self { start, end, factor }
    }

    pub fn get_fuzz_range(interval: f64, elapsed_days: i64, maximum_interval: i32) -> (i64, i64) {
        let mut delta: f64 = 1.0;
        for fuzz_range in FUZZ_RANGE {
            delta += fuzz_range.factor
                * f64::max(f64::min(interval, fuzz_range.end) - fuzz_range.start, 0.0);
        }

        let i = f64::min(interval, maximum_interval as f64);
        let mut min_interval = f64::max(2.0, f64::round(i - delta));
        let max_interval: f64 = f64::min(f64::round(i + delta), maximum_interval as f64);

        if i > elapsed_days as f64 {
            min_interval = f64::max(min_interval, elapsed_days as f64 + 1.0);
        }

        min_interval = f64::min(min_interval, max_interval);

        (min_interval as i64, max_interval as i64)
    }
}

const FUZZ_RANGE: [FuzzRange; 3] = [
    FuzzRange::new(2.5, 7.0, 0.15),
    FuzzRange::new(7.0, 20.0, 0.1),
    FuzzRange::new(20.0, f64::MAX, 0.05),
];
