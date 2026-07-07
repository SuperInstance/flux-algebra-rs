//! PLR group (Parallel-Leading-tone-Relative) operations on triads.

use serde::{Deserialize, Serialize};

/// A major or minor triad represented as (root, quality).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Triad {
    pub root: u8, // 0-11
    pub major: bool,
}

impl Triad {
    pub fn major(root: u8) -> Self {
        Self { root, major: true }
    }
    pub fn minor(root: u8) -> Self {
        Self { root, major: false }
    }
    pub fn third(&self) -> u8 {
        if self.major {
            (self.root + 4) % 12
        } else {
            (self.root + 3) % 12
        }
    }
    pub fn fifth(&self) -> u8 {
        (self.root + 7) % 12
    }
    pub fn pcs(&self) -> [u8; 3] {
        [self.root, self.third(), self.fifth()]
    }
}

/// PLR group operations.
pub struct PlrGroup;

impl PlrGroup {
    /// Parallel: same root, flip quality.
    pub fn p(t: Triad) -> Triad {
        Triad {
            root: t.root,
            major: !t.major,
        }
    }
    /// Leading-tone exchange (Leittonwechsel): connects a triad to the
    /// consonant triad that shares its third and fifth (for a major triad)
    /// — equivalently, its root and third (for a minor triad). `L` is an
    /// involution: `L(L(t)) == t`.
    ///
    /// Examples: L(C major) = E minor (shares E, G); L(A minor) = F major
    /// (shares A, C).
    pub fn l(t: Triad) -> Triad {
        if t.major {
            Triad::minor((t.root as i8 + 4).rem_euclid(12) as u8)
        } else {
            Triad::major((t.root as i8 - 4).rem_euclid(12) as u8)
        }
    }
    /// Relative: move to relative major/minor.
    pub fn r(t: Triad) -> Triad {
        if t.major {
            Triad::minor((t.root as i8 + 9).rem_euclid(12) as u8)
        } else {
            Triad::major((t.root as i8 + 3).rem_euclid(12) as u8)
        }
    }
    /// Compose operations left-to-right.
    pub fn compose(ops: &[fn(Triad) -> Triad], t: Triad) -> Triad {
        ops.iter().fold(t, |acc, f| f(acc))
    }

    /// Generate the PL hexatonic cycle starting from `start`: the orbit of
    /// `start` under repeated alternation of the Parallel (`P`) and
    /// Leading-tone (`L`) operations.
    ///
    /// This is one of Cohn's hexatonic systems — a closed cycle of six
    /// consonant triads linked by P and L whose six pitch classes form an
    /// augmented hexatonic collection. Starting from C major the cycle is
    /// `C → Cm → A♭ → A♭m → E → Em → (back to C)`.
    ///
    /// Requires `P` and `L` to be correct involutions (see `l`); otherwise
    /// the orbit need not close at six.
    pub fn hexatonic_cycle(start: Triad) -> Vec<Triad> {
        let mut cycle = Vec::with_capacity(6);
        let mut current = start;
        for step in 0..6 {
            cycle.push(current);
            current = if step % 2 == 0 {
                PlrGroup::p(current)
            } else {
                PlrGroup::l(current)
            };
        }
        cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p() {
        let c = Triad::major(0);
        let cm = PlrGroup::p(c);
        assert!(!cm.major);
        assert_eq!(cm.root, 0);
    }
    #[test]
    fn test_l() {
        let c = Triad::major(0);
        let em = PlrGroup::l(c);
        assert!(!em.major);
        assert_eq!(em.root, 4);
    }
    #[test]
    fn test_l_involution() {
        // Regression: the minor branch of `l` previously computed
        // `major(root - 1)`, which broke the defining involution property
        // L(L(t)) == t. It also mapped A-minor to A-flat-major (sharing only
        // the single pitch class C) instead of to F-major, which shares both
        // A and C with A-minor. This test fails on the old branch.
        for root in 0..12u8 {
            for &major in &[true, false] {
                let t = Triad { root, major };
                let ll = PlrGroup::l(PlrGroup::l(t));
                assert_eq!(ll.root, t.root, "L(L({t:?})) root mismatch");
                assert_eq!(ll.major, t.major, "L(L({t:?})) quality mismatch");
            }
        }
        // L(A minor) must be F major (the two shared tones A, C are preserved).
        let f = PlrGroup::l(Triad::minor(9));
        assert!(f.major);
        assert_eq!(f.root, 5);
    }
    #[test]
    fn test_r() {
        let cm = Triad::minor(0);
        let eb = PlrGroup::r(cm);
        assert!(eb.major);
        assert_eq!(eb.root, 3);
    }
    #[test]
    fn test_plr_roundtrip() {
        let c = Triad::major(0);
        let r = PlrGroup::compose(&[PlrGroup::p, PlrGroup::l, PlrGroup::r], c);
        assert!(r.root < 12);
    }
    #[test]
    fn test_hexatonic_cycle() {
        // Alternating P, L must close after exactly six triads, and those six
        // triads' pitch classes must be a single hexatonic collection (six
        // distinct pitch classes). With the buggy old `L` the orbit never
        // closed at six, so this also guards the `L` fix.
        let cycle = PlrGroup::hexatonic_cycle(Triad::major(0));
        assert_eq!(cycle.len(), 6);

        // Expected cycle from C major: C, Cm, Ab, Abm, E, Em.
        assert_eq!(cycle[0], Triad::major(0));
        assert_eq!(cycle[1], Triad::minor(0));
        assert_eq!(cycle[2], Triad::major(8));
        assert_eq!(cycle[3], Triad::minor(8));
        assert_eq!(cycle[4], Triad::major(4));
        assert_eq!(cycle[5], Triad::minor(4));

        // Union of all pitch classes is the hexatonic collection {0,3,4,7,8,11}.
        let mut pcs: Vec<u8> = cycle.iter().flat_map(|t| t.pcs()).collect();
        pcs.sort_unstable();
        pcs.dedup();
        assert_eq!(pcs, vec![0, 3, 4, 7, 8, 11]);

        // Starting from any member yields the same set of triads (rotation).
        let from_em = PlrGroup::hexatonic_cycle(Triad::minor(4));
        let mut a: Vec<[u8; 3]> = cycle.iter().map(|t| t.pcs()).collect();
        let mut b: Vec<[u8; 3]> = from_em.iter().map(|t| t.pcs()).collect();
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, b);
    }
}
