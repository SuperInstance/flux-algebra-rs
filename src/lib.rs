//! Algebraic structures for music theory — pure Rust port of flux-algebra.
//!
//! Provides HarmonicRing (Z/nZ), PLR group operations, Tropical semiring,
//! TuningField, and voice-leading combinatorics.

pub mod combinatorics;
pub mod groups;
pub mod rings;
pub mod tropical;
pub mod tuning;

pub use groups::PlrGroup;
pub use rings::HarmonicRing;
pub use tropical::TropicalSemiring;
