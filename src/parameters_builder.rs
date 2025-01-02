use crate::{parameters::Weights, Parameters};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct ParametersBuilder {
    request_retention: Option<f64>,
    maximum_interval: Option<i32>,
    w: Option<Weights>,
    decay: Option<f64>,
    factor: Option<f64>,
    enable_short_term: Option<bool>,
}

impl ParametersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Parameters {
        Parameters {
            request_retention: self.request_retention.unwrap_or(0.9),
            maximum_interval: self.maximum_interval.unwrap_or(36500),
            w: self.w.unwrap_or(Parameters::WEIGHTS),
            decay: self.decay.unwrap_or(Parameters::DECAY),
            factor: self.factor.unwrap_or(Parameters::FACTOR),
            enable_short_term: self.enable_short_term.unwrap_or(true),
        }
    }

    pub fn request_retention(mut self, request_retention: Option<f64>) -> Self {
        self.request_retention = request_retention;
        self
    }

    pub fn maximum_interval(mut self, maximum_interval: Option<i32>) -> Self {
        self.maximum_interval = maximum_interval;
        self
    }

    pub fn weights(mut self, weights: Option<Weights>) -> Self {
        self.w = weights;
        self
    }

    pub fn decay(mut self, decay: Option<f64>) -> Self {
        self.decay = decay;
        self
    }

    pub fn factor(mut self, factor: Option<f64>) -> Self {
        self.factor = factor;
        self
    }

    pub fn enable_short_term(mut self, enable_short_term: Option<bool>) -> Self {
        self.enable_short_term = enable_short_term;
        self
    }
}
