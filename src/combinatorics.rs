//! Voice-leading combinatorics: minimal voice leading via smoothness.

/// Compute voice-leading distance (sum of absolute interval changes).
pub fn voice_leading_distance(from: &[u8], to: &[u8]) -> u32 {
    from.iter()
        .zip(to.iter())
        .map(|(&a, &b)| (a as i32 - b as i32).unsigned_abs())
        .sum()
}

/// Find the smoothest voice leading by trying all permutations.
/// Returns (permutation_of_to, distance).
pub fn minimal_voice_leading(from: &[u8], to: &[u8]) -> (Vec<u8>, u32) {
    if from.len() != to.len() || from.is_empty() {
        return (to.to_vec(), 0);
    }
    let mut best_perm = to.to_vec();
    let mut best_dist = u32::MAX;
    let n = to.len();
    // For small n, try all permutations
    if n <= 6 {
        let mut perm = to.to_vec();
        let mut indices: Vec<usize> = (0..n).collect();
        loop {
            let d = voice_leading_distance(from, &perm);
            if d < best_dist {
                best_dist = d;
                best_perm = perm.clone();
            }
            // Next permutation via indices
            let k = (0..n - 1).rev().find(|&i| indices[i] < indices[i + 1]);
            match k {
                None => break,
                Some(k) => {
                    let l = (k + 1..n).rev().find(|&i| indices[i] > indices[k]).unwrap();
                    indices.swap(k, l);
                    perm.swap(k, l);
                    indices[k + 1..].reverse();
                    perm[k + 1..].reverse();
                }
            }
        }
    }
    (best_perm, best_dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vl_distance() {
        assert_eq!(voice_leading_distance(&[0, 4, 7], &[0, 3, 7]), 1);
    }
    #[test]
    fn test_minimal() {
        let (perm, d) = minimal_voice_leading(&[0, 4, 7], &[5, 9, 0]);
        assert!(d <= 20);
    }
}
