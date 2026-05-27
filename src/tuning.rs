//! Tuning field: algebraic extensions for tuning systems.

/// A tuning field representing rational intervals.
#[derive(Debug, Clone)]
pub struct TuningField {
    pub base_frequency: f64,
    pub intervals: Vec<f64>,
}

impl TuningField {
    pub fn equal_temperament(n: usize) -> Self {
        Self { base_frequency: 440.0, intervals: (0..n).map(|i| 2.0_f64.powi(i as i32) / n as f64).collect() }
    }
    pub fn frequency(&self, degree: usize) -> f64 {
        self.base_frequency * self.intervals.get(degree % self.intervals.len()).unwrap_or(&1.0)
    }
    pub fn cents(&self, ratio: f64) -> f64 { 1200.0 * ratio.log2() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_et() { let et = TuningField::equal_temperament(12); assert_eq!(et.intervals.len(), 12); }
}
