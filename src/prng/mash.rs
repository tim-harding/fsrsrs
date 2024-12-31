use super::{TWO_TO_THE_POWER_OF_32, TWO_TO_THE_POWER_OF_MINUS_32};

pub struct Mash {
    n: f64,
}

impl Mash {
    const N: u64 = 0xefc8249d;

    pub const fn new() -> Self {
        Self { n: Self::N as f64 }
    }

    pub fn mash(&mut self, seed: &str) -> f64 {
        let mut n: f64 = self.n;
        for c in seed.chars() {
            n += c as u32 as f64;
            let mut h = 0.02519603282416938 * n;
            n = (h as u32) as f64;
            h -= n;
            h *= n;
            n = (h as u32) as f64;
            h -= n;
            n += h * TWO_TO_THE_POWER_OF_32 as f64;
        }
        self.n = n;
        self.n * TWO_TO_THE_POWER_OF_MINUS_32
    }
}
