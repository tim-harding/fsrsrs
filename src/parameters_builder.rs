use crate::{parameters::Weights, Parameters};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct ParametersBuilder {
    request_retention: Option<f64>,
    maximum_interval: Option<i32>,
    w: Option<Weights>,
}

impl ParametersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Parameters {
        Parameters {
            request_retention: self.request_retention.unwrap_or(0.9),
            maximum_interval: self.maximum_interval.unwrap_or(36500),
            w: self.w.unwrap_or(Parameters::DEFAULT_WEIGHTS),
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
}
