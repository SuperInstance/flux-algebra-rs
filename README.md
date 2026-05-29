# flux-algebra-rs

Algebraic structures for music theory — harmonic rings (ℤ/nℤ), Neo-Riemannian PLR group, tropical semiring, tuning fields, and voice-leading combinatorics.

## What This Gives You

- **`HarmonicRing`** — ℤ/nℤ ring for pitch-class arithmetic: transposition, inversion, units
- **PLR group** — Parallel, Leading-tone, Relative operations on triads
- **Hexatonic cycles** — 6-chord cycles through P/L transformations
- **`TropicalSemiring`** — (ℝ ∪ {∞}, min, +) with polynomial evaluation
- **`TuningField`** — Equal temperament, just intonation, custom tuning systems
- **Voice-leading combinatorics** — Minimal smooth voice leading via permutation search
- **Zero dependencies** — Pure Rust, `std` only

## Quick Start

```rust
use flux_algebra::{HarmonicRing, PlrGroup, groups::Triad, TropicalSemiring, TuningField};

// Pitch-class ring: transposition and inversion
let ring = HarmonicRing::chromatic();
let transposed = ring.transpose(&[0, 4, 7], 5); // C major → F major

// Neo-Riemannian transformations
let c_major = Triad::major(0);
let c_minor = PlrGroup::p(c_major.clone());   // Parallel: Cm
let a_minor = PlrGroup::r(c_major.clone());    // Relative: Am
let b_dim = PlrGroup::l(c_major.clone());      // Leading-tone: Bdim

// Tropical semiring: min-plus arithmetic
let min_val = TropicalSemiring::add(3.0, 5.0);  // = 3.0 (min)
let sum_val = TropicalSemiring::mul(3.0, 5.0);   // = 8.0 (+)

// Tuning systems
let et = TuningField::equal_temperament(12);
let ji = TuningField::just_intonation();
let freq = ji.frequency(60);  // C4 frequency in just intonation
let cents = ji.cents_deviation(60);
```

## API Reference

### `HarmonicRing`

```rust
HarmonicRing::chromatic()              // 12-TET pitch-class ring
ring.transpose(&pitch_classes, n)      // Transpose by n semitones
ring.invert(&pitch_classes)            // Invert pitch classes
```

### `PlrGroup`

```rust
PlrGroup::p(triad)     // Parallel transformation
PlrGroup::l(triad)     // Leading-tone transformation
PlrGroup::r(triad)     // Relative transformation
```

### `TropicalSemiring`

```rust
TropicalSemiring::add(a, b)     // min(a, b)
TropicalSemiring::mul(a, b)     // a + b
TropicalSemiring::eval(poly, x) // Evaluate tropical polynomial
```

### `TuningField`

```rust
TuningField::equal_temperament(n)    // n-TET
TuningField::just_intonation()       // Just intonation ratios
field.frequency(midi)                // Frequency for MIDI note
field.cents_deviation(midi)          // Deviation from 12-TET
```

## How It Fits

- **[flux-algebra-c](https://github.com/SuperInstance/flux-algebra-c)** — C99 port of this library
- **[counterpoint-engine-rs](https://github.com/SuperInstance/counterpoint-engine-rs)** — Interval classification uses harmonic ring arithmetic
- **[creative-engine-rust](https://github.com/SuperInstance/creative-engine-rust)** — Map Lorenz coordinates to pitch-class space
- **[constraint-instrument](https://github.com/SuperInstance/constraint-instrument)** — PLR transformations as constraint operations in musical terrains

## Testing

14 tests covering ring arithmetic, PLR transformations, hexatonic cycles, tropical operations, tuning computations, and voice-leading search.

```bash
cargo test
```

## Installation

```toml
[dependencies]
flux-algebra = { git = "https://github.com/SuperInstance/flux-algebra-rs" }
```

```bash
git clone https://github.com/SuperInstance/flux-algebra-rs.git
cd flux-algebra-rs
cargo build
```

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance) ecosystem.
