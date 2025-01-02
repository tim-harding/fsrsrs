use crate::{parameters::Weights, Parameters};

/// Builder for [`Parameters`]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct ParametersBuilder {
    retention: Option<f64>,
    maximum_interval: Option<i32>,
    w: Option<Weights>,
}

impl ParametersBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the [Parameters], using default values for unspecified parameters.
    pub fn build(self) -> Parameters {
        Parameters {
            retention: self.retention.unwrap_or(0.9),
            maximum_interval: self.maximum_interval.unwrap_or(36500),
            w: self.w.unwrap_or(Parameters::DEFAULT_WEIGHTS),
        }
    }

    /// Set the desired retention rate
    ///
    /// Default is 0.9
    pub fn retention(mut self, request_retention: f64) -> Self {
        self.retention = Some(request_retention);
        self
    }

    /// Set the maximum interval between reviews in days
    ///
    /// Default is 36,500
    pub fn maximum_interval(mut self, maximum_interval: i32) -> Self {
        self.maximum_interval = Some(maximum_interval);
        self
    }

    /// Set the FSRS algorithm weights
    pub fn weights(mut self, weights: Weights) -> Self {
        self.w = Some(weights);
        self
    }
}
