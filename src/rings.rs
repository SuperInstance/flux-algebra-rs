//! Harmonic ring: Z/nZ ring structure for pitch classes.

use serde::{Deserialize, Serialize};

/// A ring over Z/nZ, representing pitch-class space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonicRing {
    pub n: u32,
}

impl HarmonicRing {
    pub fn new(n: u32) -> Self {
        Self {
            n: if n == 0 { 12 } else { n },
        }
    }
    pub fn chromatic() -> Self {
        Self::new(12)
    }

    /// Addition mod n (transposition).
    pub fn add(&self, a: i32, b: i32) -> u32 {
        ((a + b).rem_euclid(self.n as i32)) as u32
    }
    /// Multiplication mod n.
    pub fn mul(&self, a: i32, b: i32) -> u32 {
        ((a * b).rem_euclid(self.n as i32)) as u32
    }
    /// Additive inverse.
    pub fn neg(&self, a: i32) -> u32 {
        ((-a).rem_euclid(self.n as i32)) as u32
    }
    /// Subtraction mod n.
    pub fn sub(&self, a: i32, b: i32) -> u32 {
        self.add(a, self.neg(b) as i32)
    }
    /// Check if element is a unit (has multiplicative inverse).
    pub fn is_unit(&self, a: i32) -> bool {
        gcd(a.rem_euclid(self.n as i32) as u32, self.n) == 1
    }
    /// Multiplicative inverse if it exists.
    pub fn inv(&self, a: i32) -> Option<u32> {
        if !self.is_unit(a) {
            return None;
        }
        for x in 1..self.n {
            if self.mul(a, x as i32) == 1 {
                return Some(x);
            }
        }
        None
    }
    /// Transpose a pitch class sequence.
    pub fn transpose(&self, notes: &[u32], interval: i32) -> Vec<u32> {
        notes
            .iter()
            .map(|&n| self.add(n as i32, interval))
            .collect()
    }
    /// Invert a pitch class sequence around axis.
    pub fn invert(&self, notes: &[u32], axis: i32) -> Vec<u32> {
        notes.iter().map(|&n| self.sub(axis, n as i32)).collect()
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chromatic() {
        let r = HarmonicRing::chromatic();
        assert_eq!(r.add(7, 5), 0);
        assert_eq!(r.mul(7, 7), 1);
    }
    #[test]
    fn test_inv() {
        let r = HarmonicRing::chromatic();
        assert_eq!(r.inv(7), Some(7));
        assert_eq!(r.inv(2), None);
    }
    #[test]
    fn test_transpose() {
        let r = HarmonicRing::chromatic();
        assert_eq!(r.transpose(&[0, 4, 7], 5), vec![5, 9, 0]);
    }
}
