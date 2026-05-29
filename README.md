# flux-algebra-rs

<<<<<<< HEAD
A pure Rust port of [flux-algebra](https://github.com/SuperInstance/flux-algebra) — algebraic structures for music theory.

## Features

- **HarmonicRing** — Z/nZ ring for pitch-class arithmetic (transposition, inversion, units)
- **PLR Group** — Parallel, Leading-tone, Relative operations on triads
- **Tropical Semiring** — (R ∪ {∞}, min, +) with polynomial evaluation
- **TuningField** — Equal temperament and custom tuning systems
- **Voice-leading combinatorics** — Minimal smooth voice leading via permutation search
=======
Rust port of [flux-algebra](https://github.com/SuperInstance/flux-algebra) — musical algebra for harmonic analysis.

## Features

- **Chord operations**: major/minor/diminished/augmented, transposition, quality inversion
- **PLR group**: Parallel, Leading-tone, Relative (Neo-Riemannian transformations)
- **Hexatonic cycles**: 6-chord cycles through P/L transformations
- **Tropical semiring**: min-plus algebra over intervals
- **Tuning fields**: equal temperament, just intonation, cents deviation
- **Voice leading**: distance computation and smoothest voice leading
>>>>>>> 83fe680 (Initial Rust port: musical algebra library)

## Usage

```rust
<<<<<<< HEAD
use flux_algebra::{HarmonicRing, PlrGroup, groups::Triad, TropicalSemiring};

let ring = HarmonicRing::chromatic();
let transposed = ring.transpose(&[0, 4, 7], 5); // C major → F major

let c_major = Triad::major(0);
let c_minor = PlrGroup::p(c_major); // Parallel: Cm

let trop_min = TropicalSemiring::add(3.0, 5.0); // = 3.0
```

## License
=======
use flux_algebra::{Chord, parallel, relative, leading_tone, TuningField, voice_leading_distance};

let c_major = Chord::major(0);
let a_minor = relative(&c_major);
let b_minor = leading_tone(&c_major);

let tuning = TuningField::just_intonation();
let freq = tuning.frequency(60); // C4 in just intonation
```

## License

>>>>>>> 83fe680 (Initial Rust port: musical algebra library)
MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem.
