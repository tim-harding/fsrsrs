use crate::Seed;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Alea {
    c: f64,
    s0: f64,
    s1: f64,
    s2: f64,
}

impl Alea {
    fn new(seed: Seed) -> Self {
        let mut mash = Mash::new();
        let blank_seed = Seed::new(" ");
        let mut alea = Self {
            c: 1.0,
            s0: mash.mash(&blank_seed),
            s1: mash.mash(&blank_seed),
            s2: mash.mash(&blank_seed),
        };

        alea.s0 -= mash.mash(&seed);
        if alea.s0 < 0.0 {
            alea.s0 += 1.0;
        }
        alea.s1 -= mash.mash(&seed);
        if alea.s1 < 0.0 {
            alea.s1 += 1.0;
        }
        alea.s2 -= mash.mash(&seed);
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

const TWO_TO_THE_POWER_OF_32: u64 = 1 << 32;
const TWO_TO_THE_POWER_OF_21: u64 = 1 << 21;
const TWO_TO_THE_POWER_OF_MINUS_32: f64 = 1.0 / (TWO_TO_THE_POWER_OF_32 as f64);
const TWO_TO_THE_POWER_OF_MINUS_53: f64 = 1.0 / ((1u64 << 53) as f64);

struct Mash {
    n: f64,
}

impl Mash {
    const N: u64 = 0xefc8249d;

    const fn new() -> Self {
        Self { n: Self::N as f64 }
    }

    fn mash(&mut self, seed: &Seed) -> f64 {
        let mut n: f64 = self.n;
        for c in seed.inner_str().chars() {
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
        self.n * TWO_TO_THE_POWER_OF_MINUS_32 // 2^-32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Prng {
    pub xg: Alea,
}

impl Prng {
    fn new(seed: Seed) -> Self {
        Self {
            xg: Alea::new(seed),
        }
    }

    pub fn gen_next(&mut self) -> f64 {
        self.xg.next().unwrap()
    }

    pub fn int32(&mut self) -> i32 {
        wrap_to_i32(self.gen_next() * TWO_TO_THE_POWER_OF_32 as f64)
    }

    pub fn double(&mut self) -> f64 {
        ((self.gen_next() * TWO_TO_THE_POWER_OF_21 as f64) as u64 as f64)
            .mul_add(TWO_TO_THE_POWER_OF_MINUS_53, self.gen_next())
    }

    pub fn set_state(&mut self, xg: Alea) {
        self.xg = xg;
    }

    pub fn state(self) -> Alea {
        self.xg
    }
}

fn wrap_to_i32(input: f64) -> i32 {
    // The rem_euclid() wraps within a positive range,
    // then casting u32 to i32 makes half of that range negative.
    input.rem_euclid((u32::MAX as f64) + 1.0) as u32 as i32
}

pub fn alea(seed: Seed) -> Prng {
    match seed {
        Seed::String(_) => Prng::new(seed),
        Seed::Empty => Prng::new(Seed::default()),
        Seed::Default => Prng::new(Seed::default()),
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_prng_get_state() {
        let prng_1 = alea(Seed::new(1));
        let prng_2 = alea(Seed::new(2));
        let prng_3 = alea(Seed::new(1));

        let alea_state_1 = prng_1.state();
        let alea_state_2 = prng_2.state();
        let alea_state_3 = prng_3.state();

        assert_eq!(alea_state_1, alea_state_3);
        assert_ne!(alea_state_1, alea_state_2);
    }

    #[test]
    fn test_alea_get_next() {
        let seed = Seed::new(12345);
        let mut generator = alea(seed);
        assert_eq!(generator.gen_next(), 0.27138191112317145);
        assert_eq!(generator.gen_next(), 0.19615925149992108);
        assert_eq!(generator.gen_next(), 0.6810678059700876);
    }

    #[test]
    fn test_alea_int32() {
        let seed = Seed::new(12345);
        let mut generator = alea(seed);
        assert_eq!(generator.int32(), 1165576433);
        assert_eq!(generator.int32(), 842497570);
        assert_eq!(generator.int32(), -1369803343);
    }

    #[test]
    fn test_alea_import_state() {
        let mut rng = rand::thread_rng();
        let mut prng_1 = alea(Seed::new(rng.gen::<i32>()));
        prng_1.gen_next();
        prng_1.gen_next();
        prng_1.gen_next();
        let prng_1_state = prng_1.state();
        let mut prng_2 = alea(Seed::Empty);
        prng_2.set_state(prng_1_state);

        assert_eq!(prng_1.state(), prng_2.state());

        for _ in 1..10000 {
            let a = prng_1.gen_next();
            let b = prng_2.gen_next();

            assert_eq!(a, b);
            assert!((0.0..1.0).contains(&a));
            assert!((0.0..1.0).contains(&b));
        }
    }

    #[test]
    fn test_seed_example_1() {
        let seed = Seed::new("1727015666066");
        let mut generator = alea(seed);
        let results = generator.gen_next();
        let state = generator.state();

        let expect_alea_state = Alea {
            c: 1828249.0,
            s0: 0.5888567129150033,
            s1: 0.5074866858776659,
            s2: 0.6320083506871015,
        };
        assert_eq!(results, 0.6320083506871015);
        assert_eq!(state, expect_alea_state);
    }

    #[test]
    fn test_seed_example_2() {
        let seed = Seed::new("Seedp5fxh9kf4r0");
        let mut generator = alea(seed);
        let results = generator.gen_next();
        let state = generator.state();

        let expect_alea_state = Alea {
            c: 1776946.0,
            s0: 0.6778371171094477,
            s1: 0.0770602801349014,
            s2: 0.14867847645655274,
        };
        assert_eq!(results, 0.14867847645655274);
        assert_eq!(state, expect_alea_state);
    }

    #[test]
    fn test_seed_example_3() {
        let seed = Seed::new("NegativeS2Seed");
        let mut generator = alea(seed);
        let results = generator.gen_next();
        let state = generator.state();

        let expect_alea_state = Alea {
            c: 952982.0,
            s0: 0.25224833423271775,
            s1: 0.9213257452938706,
            s2: 0.830770346801728,
        };
        assert_eq!(results, 0.830770346801728);
        assert_eq!(state, expect_alea_state);
    }
}
