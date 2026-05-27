# flux-algebra-rs

A pure Rust port of [flux-algebra](https://github.com/SuperInstance/flux-algebra) — algebraic structures for music theory.

## Features

- **HarmonicRing** — Z/nZ ring for pitch-class arithmetic (transposition, inversion, units)
- **PLR Group** — Parallel, Leading-tone, Relative operations on triads
- **Tropical Semiring** — (R ∪ {∞}, min, +) with polynomial evaluation
- **TuningField** — Equal temperament and custom tuning systems
- **Voice-leading combinatorics** — Minimal smooth voice leading via permutation search

## Usage

```rust
use flux_algebra::{HarmonicRing, PlrGroup, groups::Triad, TropicalSemiring};

let ring = HarmonicRing::chromatic();
let transposed = ring.transpose(&[0, 4, 7], 5); // C major → F major

let c_major = Triad::major(0);
let c_minor = PlrGroup::p(c_major); // Parallel: Cm

let trop_min = TropicalSemiring::add(3.0, 5.0); // = 3.0
```

## License
MIT
