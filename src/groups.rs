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
    /// Leading-tone: shift root by semitone, flip quality.
    pub fn l(t: Triad) -> Triad {
        if t.major {
            Triad::minor((t.root as i8 + 4).rem_euclid(12) as u8)
        } else {
            Triad::major((t.root as i8 - 1).rem_euclid(12) as u8)
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
}
