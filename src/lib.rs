//! Algebraic structures for music theory — pure Rust port of flux-algebra.
//!
//! Provides HarmonicRing (Z/nZ), PLR group operations, Tropical semiring,
//! TuningField, and voice-leading combinatorics.

pub mod rings;
pub mod groups;
pub mod tropical;
pub mod tuning;
pub mod combinatorics;

pub use rings::HarmonicRing;
pub use groups::PlrGroup;
pub use tropical::TropicalSemiring;
