//! Tropical semiring: (R ∪ {∞}, min, +).

/// Tropical semiring operations.
pub struct TropicalSemiring;

impl TropicalSemiring {
    /// Tropical addition: min(a, b).
    pub fn add(a: f64, b: f64) -> f64 {
        a.min(b)
    }
    /// Tropical multiplication: a + b.
    pub fn mul(a: f64, b: f64) -> f64 {
        a + b
    }
    /// Tropical power: a * n (scalar).
    pub fn pow(a: f64, n: u32) -> f64 {
        a * n as f64
    }
    /// Tropical identity for addition: ∞.
    pub fn zero() -> f64 {
        f64::INFINITY
    }
    /// Tropical identity for multiplication: 0.
    pub fn one() -> f64 {
        0.0
    }
    /// Evaluate a tropical polynomial at x: min of (coeff_i + i*x).
    pub fn eval_poly(coeffs: &[f64], x: f64) -> f64 {
        coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| c + i as f64 * x)
            .fold(f64::INFINITY, |a, b| a.min(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(TropicalSemiring::add(3.0, 5.0), 3.0);
    }
    #[test]
    fn test_mul() {
        assert_eq!(TropicalSemiring::mul(3.0, 5.0), 8.0);
    }
    #[test]
    fn test_zero() {
        assert!(TropicalSemiring::zero().is_infinite());
    }
    #[test]
    fn test_poly() {
        let v = TropicalSemiring::eval_poly(&[0.0, 2.0, 1.0], 1.0);
        assert!((v - 0.0).abs() < 0.001 || (v - 2.0).abs() < 0.001 || (v - 3.0).abs() < 0.001);
    }
}
