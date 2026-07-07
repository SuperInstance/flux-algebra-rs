//! Algebraic structures for music theory — pure Rust port of flux-algebra.
//!
//! Provides:
//! - [`HarmonicRing`] — ℤ/nℤ ring for pitch-class arithmetic (transposition,
//!   inversion, units, inverses).
//! - [`Triad`] and [`PlrGroup`] — Neo-Riemannian Parallel / Leading-tone /
//!   Relative operations on major/minor triads, plus [`PlrGroup::hexatonic_cycle`].
//! - [`TropicalSemiring`] — the (ℝ ∪ {∞}, min, +) min-plus semiring with
//!   tropical polynomial evaluation.
//! - [`TuningField`] — equal temperament, 5-limit just intonation, and
//!   frequency / cents helpers.
//! - voice-leading combinatorics ([`combinatorics`]) — minimal-distance voice
//!   leading via exhaustive search (≤ 6 voices) or a greedy heuristic (> 6).
//!
//! # Quick example
//!
//! ```
//! use flux_algebra::{HarmonicRing, PlrGroup, Triad, TropicalSemiring, TuningField};
//!
//! // ℤ/12ℤ pitch-class ring: a perfect fifth plus a perfect fourth is an octave.
//! let ring = HarmonicRing::chromatic();
//! assert_eq!(ring.transpose(&[0u32, 4, 7], 5), vec![5, 9, 0]);
//!
//! // Neo-Riemannian Parallel: C major -> C minor.
//! let c_minor = PlrGroup::p(Triad::major(0));
//! assert!(!c_minor.major);
//!
//! // Tropical (min-plus) multiplication is ordinary addition.
//! assert_eq!(TropicalSemiring::mul(3.0, 5.0), 8.0);
//!
//! // 12-TET: MIDI A4 (69) is the 440 Hz reference.
//! let et = TuningField::equal_temperament(12);
//! assert!((et.frequency_of_midi(69) - 440.0).abs() < 1e-9);
//! ```

pub mod combinatorics;
pub mod groups;
pub mod rings;
pub mod tropical;
pub mod tuning;

pub use groups::{PlrGroup, Triad};
pub use rings::HarmonicRing;
pub use tropical::TropicalSemiring;
pub use tuning::TuningField;
