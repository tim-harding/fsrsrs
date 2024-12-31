use super::{mash::Mash, TWO_TO_THE_POWER_OF_MINUS_32};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Alea {
    pub c: f64,
    pub s0: f64,
    pub s1: f64,
    pub s2: f64,
}

impl Alea {
    pub fn new(seed: &str) -> Self {
        let mut mash = Mash::new();
        let mut alea = Self {
            c: 1.0,
            s0: mash.mash(" "),
            s1: mash.mash(" "),
            s2: mash.mash(" "),
        };

        alea.s0 -= mash.mash(seed);
        if alea.s0 < 0.0 {
            alea.s0 += 1.0;
        }
        alea.s1 -= mash.mash(seed);
        if alea.s1 < 0.0 {
            alea.s1 += 1.0;
        }
        alea.s2 -= mash.mash(seed);
        if alea.s2 < 0.0 {
            alea.s2 += 1.0;
        }

        alea
    }
}

impl Iterator for Alea {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let t = 2091639.0f64.mul_add(self.s0, self.c * TWO_TO_THE_POWER_OF_MINUS_32);
        self.s0 = self.s1;
        self.s1 = self.s2;
        self.c = t.floor();
        self.s2 = t - self.c;

        Some(self.s2)
    }
}
