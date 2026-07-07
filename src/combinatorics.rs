//! Voice-leading combinatorics: minimal voice leading via smoothness.

/// Compute voice-leading distance (sum of absolute interval changes).
pub fn voice_leading_distance(from: &[u8], to: &[u8]) -> u32 {
    from.iter()
        .zip(to.iter())
        .map(|(&a, &b)| (a as i32 - b as i32).unsigned_abs())
        .sum()
}

/// Find a smooth voice leading from `from` to a permutation of `to`.
///
/// Returns the chosen permutation of `to` (as a `Vec<u8>`) together with its
/// total voice-leading distance (sum of absolute semitone motion per voice).
///
/// - For **n ≤ 6** voices the search is **exhaustive** over all `n!`
///   permutations, so the returned distance is the true minimum.
/// - For **n > 6** voices an exhaustive search is infeasible, so a greedy
///   nearest-neighbour heuristic is used instead: each `from` voice is
///   matched, in order, to the closest still-unused `to` note. This is *not*
///   guaranteed optimal, but it always returns a real, valid permutation and
///   a faithfully recomputed distance — never a fabricated "perfect match".
///
/// (`from` and `to` must have equal length; an empty or mismatched input
/// returns `to` unchanged with distance `0`.)
pub fn minimal_voice_leading(from: &[u8], to: &[u8]) -> (Vec<u8>, u32) {
    if from.len() != to.len() || from.is_empty() {
        return (to.to_vec(), 0);
    }
    if to.len() <= 6 {
        minimal_voice_leading_exact(from, to)
    } else {
        minimal_voice_leading_heuristic(from, to)
    }
}

/// Exhaustive optimal search over all permutations of `to`. Only called for
/// small inputs (n ≤ 6), where `n!` stays tractable.
fn minimal_voice_leading_exact(from: &[u8], to: &[u8]) -> (Vec<u8>, u32) {
    let n = to.len();
    let mut best_perm = to.to_vec();
    let mut best_dist = u32::MAX;
    let mut perm = to.to_vec();
    let mut indices: Vec<usize> = (0..n).collect();
    loop {
        let d = voice_leading_distance(from, &perm);
        if d < best_dist {
            best_dist = d;
            best_perm = perm.clone();
        }
        // Next permutation via indices.
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
    (best_perm, best_dist)
}

/// Greedy nearest-neighbour heuristic for large inputs (n > 6). Matches each
/// `from` voice, in order, to the nearest unused `to` note. Always produces a
/// valid permutation and an honest distance; not guaranteed optimal.
fn minimal_voice_leading_heuristic(from: &[u8], to: &[u8]) -> (Vec<u8>, u32) {
    let n = to.len();
    let mut used = vec![false; n];
    let mut perm = Vec::with_capacity(n);
    let mut total: u32 = 0;
    for &f in from {
        // Match this voice to the closest still-unused target note (ties go
        // to the lowest index for determinism).
        let (best_j, best_d) = to
            .iter()
            .enumerate()
            .filter(|(j, _)| !used[*j])
            .map(|(j, &t)| (j, (f as i32 - t as i32).unsigned_abs()))
            .min_by_key(|&(_, d)| d)
            .expect("non-empty `to` with unused entries always yields a match");
        used[best_j] = true;
        perm.push(to[best_j]);
        total += best_d;
    }
    (perm, total)
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
        // Result must be a genuine permutation of the target.
        let mut sorted_perm = perm.clone();
        sorted_perm.sort_unstable();
        let mut sorted_to = vec![5, 9, 0];
        sorted_to.sort_unstable();
        assert_eq!(sorted_perm, sorted_to);
        // Reported distance must match an independent recomputation.
        assert_eq!(d, voice_leading_distance(&[0, 4, 7], &perm));
        assert!(d <= 20);
    }
    #[test]
    fn test_minimal_heuristic_large() {
        // 8 voices: this previously hit a silent stub that returned the
        // *unpermuted* target together with a fabricated distance (u32::MAX)
        // and never actually searched. Now it must return a real permutation
        // of `to` whose distance matches an independent recomputation, and
        // the distance must be an ordinary small number rather than u32::MAX.
        // (`to` is `from` shifted up one semitone, so the voices genuinely
        // move — the minimal distance is 1 per voice, 8 total.)
        let from = vec![0, 2, 4, 5, 7, 9, 11, 12];
        let to = vec![1, 3, 5, 6, 8, 10, 12, 13];
        let (perm, d) = minimal_voice_leading(&from, &to);
        assert_eq!(perm.len(), to.len());
        // perm must be a true permutation of `to`.
        let mut a = perm.clone();
        a.sort_unstable();
        let mut b = to.clone();
        b.sort_unstable();
        assert_eq!(a, b);
        // Reported distance must equal a fresh recomputation against the
        // returned permutation — i.e. the distance is honest, not fabricated.
        assert_eq!(d, voice_leading_distance(&from, &perm));
        // Must not be the stub's fabricated sentinel.
        assert_ne!(d, u32::MAX);
        // Each voice moves one semitone -> total distance is 8.
        assert_eq!(d, 8);
    }
}
