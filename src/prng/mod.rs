use alea::Alea;

mod alea;
mod mash;

const TWO_TO_THE_POWER_OF_32: u64 = 1 << 32;
const TWO_TO_THE_POWER_OF_21: u64 = 1 << 21;
const TWO_TO_THE_POWER_OF_MINUS_32: f64 = 1.0 / (TWO_TO_THE_POWER_OF_32 as f64);
const TWO_TO_THE_POWER_OF_MINUS_53: f64 = 1.0 / ((1u64 << 53) as f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Prng {
    xg: Alea,
}

impl Prng {
    pub fn new(seed: &str) -> Self {
        Self {
            xg: Alea::new(seed),
        }
    }

    pub fn gen_next(&mut self) -> f64 {
        self.xg.next().unwrap()
    }

    #[allow(unused)]
    pub fn int32(&mut self) -> i32 {
        wrap_to_i32(self.gen_next() * TWO_TO_THE_POWER_OF_32 as f64)
    }

    pub fn double(&mut self) -> f64 {
        ((self.gen_next() * TWO_TO_THE_POWER_OF_21 as f64) as u64 as f64)
            .mul_add(TWO_TO_THE_POWER_OF_MINUS_53, self.gen_next())
    }
}

fn wrap_to_i32(input: f64) -> i32 {
    // The rem_euclid() wraps within a positive range,
    // then casting u32 to i32 makes half of that range negative.
    input.rem_euclid((u32::MAX as f64) + 1.0) as u32 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prng_get_state() {
        let prng_1 = Prng::new("1");
        let prng_2 = Prng::new("2");
        let prng_3 = Prng::new("1");

        let alea_state_1 = prng_1.xg;
        let alea_state_2 = prng_2.xg;
        let alea_state_3 = prng_3.xg;

        assert_eq!(alea_state_1, alea_state_3);
        assert_ne!(alea_state_1, alea_state_2);
    }

    #[test]
    fn test_alea_get_next() {
        let mut generator = Prng::new("12345");
        assert_eq!(generator.gen_next(), 0.27138191112317145);
        assert_eq!(generator.gen_next(), 0.19615925149992108);
        assert_eq!(generator.gen_next(), 0.6810678059700876);
    }

    #[test]
    fn test_alea_int32() {
        let mut generator = Prng::new("12345");
        assert_eq!(generator.int32(), 1165576433);
        assert_eq!(generator.int32(), 842497570);
        assert_eq!(generator.int32(), -1369803343);
    }

    #[test]
    fn test_alea_import_state() {
        let mut prng_1 = Prng::new("Whatever");
        prng_1.gen_next();
        prng_1.gen_next();
        prng_1.gen_next();
        let prng_1_state = prng_1.xg;
        let mut prng_2 = Prng::new("ASDF");
        prng_2.xg = prng_1_state;

        assert_eq!(prng_1.xg, prng_2.xg);

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
        let mut generator = Prng::new("1727015666066");
        let results = generator.gen_next();
        let state = generator.xg;

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
        let mut generator = Prng::new("Seedp5fxh9kf4r0");
        let results = generator.gen_next();
        let state = generator.xg;

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
        let mut generator = Prng::new("NegativeS2Seed");
        let results = generator.gen_next();
        let state = generator.xg;

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
