use crate::Grade::{self, *};

pub const W: [f64; 19] = [
    0.40255, 1.18385, 3.173, 15.69105, 7.1949, 0.5345, 1.4604, 0.0046, 1.54575, 0.1192, 1.01925,
    1.9395, 0.11, 0.29605, 2.2698, 0.2315, 2.9898, 0.51655, 0.6621,
];
const F: f64 = 19.0 / 81.0;
const C: f64 = -0.5;

pub type R = f64;
pub type S = f64;
pub type D = f64;
pub type T = f64;

pub fn retrievability(t: T, s: S) -> R {
    (1.0 + F * (t / s)).powf(C)
}

// r_d ~ requested retention, and I(r,S)=S when r=0.9
#[allow(unused)]
pub fn interval(r_d: R, s: S) -> T {
    (s / F) * (r_d.powf(1.0 / C) - 1.0)
}

pub fn s_0(g: Grade) -> S {
    match g {
        Fail => W[0],
        Hard => W[1],
        Good => W[2],
        Easy => W[3],
    }
}

fn s_success(d: D, s: S, r: R, g: Grade) -> S {
    let t_d = 11.0 - d;
    let t_s = s.powf(-W[9]);
    let t_r = f64::exp(W[10] * (1.0 - r)) - 1.0;
    let h = if g == Hard { W[15] } else { 1.0 };
    let b = if g == Easy { W[16] } else { 1.0 };
    let c = f64::exp(W[8]);
    let alpha = 1.0 + t_d * t_s * t_r * h * b * c;
    s * alpha
}

fn s_fail(d: D, s: S, r: R) -> S {
    let d_f = d.powf(-W[12]);
    let s_f = (s + 1.0).powf(W[13]) - 1.0;
    let r_f = f64::exp(W[14] * (1.0 - r));
    let c_f = W[11];
    let s_f = d_f * s_f * r_f * c_f;
    f64::min(s_f, s)
}

pub fn stability(d: D, s: S, r: R, g: Grade) -> S {
    if g == Fail {
        s_fail(d, s, r)
    } else {
        s_success(d, s, r, g)
    }
}

fn clamp_d(d: D) -> D {
    d.clamp(1.0, 10.0)
}

pub fn d_0(g: Grade) -> D {
    let g: f64 = g.into_f64();
    clamp_d(W[4] - f64::exp(W[5] * (g - 1.0)) + 1.0)
}

pub fn difficulty(d: D, g: Grade) -> D {
    clamp_d(W[7] * d_0(Easy) + (1.0 - W[7]) * dp(d, g))
}

fn dp(d: D, g: Grade) -> f64 {
    d + delta_d(g) * ((10.0 - d) / 9.0)
}

fn delta_d(g: Grade) -> f64 {
    let g: f64 = g.into_f64();
    -W[6] * (g - 3.0)
}
