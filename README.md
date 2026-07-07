# flux-algebra-rs

Algebraic structures for music theory — pure Rust port of the
[flux-algebra](https://github.com/SuperInstance/flux-algebra) family.

It provides harmonic rings (ℤ/nℤ), Neo-Riemannian PLR operations on triads
(including hexatonic cycles), the tropical (min-plus) semiring, tuning fields
(equal temperament and 5-limit just intonation), and minimal voice-leading
search.

> **Naming.** The repository is `flux-algebra-rs`, but the Cargo package (and
> therefore the crate you import) is named **`flux-algebra`** — i.e.
> `use flux_algebra::...`. The `-rs` suffix is only the repo name. This matches
> what `Cargo.toml` already declares, so the install/`use` instructions below
> work as written.

## What This Gives You

- **`HarmonicRing`** — ℤ/nℤ ring for pitch-class arithmetic: addition,
  multiplication, units, inverses, transposition, and inversion.
- **`Triad`** / **`PlrGroup`** — Neo-Riemannian Parallel (`P`), Leading-tone
  (`L`), and Relative (`R`) transformations on major/minor triads, operation
  composition, and PL hexatonic cycles.
- **`TropicalSemiring`** — the (ℝ ∪ {∞}, min, +) semiring with tropical
  polynomial evaluation.
- **`TuningField`** — equal temperament, 5-limit just intonation, and
  frequency / cents helpers (including MIDI → Hz and cents deviation).
- **Voice-leading combinatorics** — minimal-distance voice leading via
  exhaustive search (≤ 6 voices) or a greedy nearest-neighbour heuristic (> 6).
- **Minimal, audited dependencies** — only `serde` and `thiserror`.

## Quick Start

```rust
use flux_algebra::{HarmonicRing, PlrGroup, Triad, TropicalSemiring, TuningField};

// Pitch-class ring ℤ/12ℤ: a perfect fifth + a perfect fourth = an octave.
let ring = HarmonicRing::chromatic();
assert_eq!(ring.add(7, 5), 0);          // 7 + 5 ≡ 0 (mod 12)
assert_eq!(ring.transpose(&[0u32, 4, 7], 5), vec![5, 9, 0]); // C major → F major

// Neo-Riemannian transformations.
let c_major = Triad::major(0);
let c_minor = PlrGroup::p(c_major);     // Parallel: Cm
let e_minor = PlrGroup::l(Triad::major(0)); // Leading-tone exchange: Em
let a_minor = PlrGroup::r(Triad::major(0)); // Relative: Am

// PL hexatonic cycle: C → Cm → A♭ → A♭m → E → Em.
let cycle = PlrGroup::hexatonic_cycle(Triad::major(0));
assert_eq!(cycle.len(), 6);

// Tropical (min-plus) semiring: "add" is min, "mul" is +.
assert_eq!(TropicalSemiring::add(3.0, 5.0), 3.0); // min
assert_eq!(TropicalSemiring::mul(3.0, 5.0), 8.0); // sum

// Tuning systems.
let et = TuningField::equal_temperament(12);
assert!((et.frequency_of_midi(69) - 440.0).abs() < 1e-9); // MIDI A4 = 440 Hz

let ji = TuningField::just_intonation();
let just_m3 = ji.intervals[2];          // 5/4, the just major third
assert!((ji.cents_deviation(just_m3) - (-13.69)).abs() < 0.01); // ≈ 13.7¢ flat of ET
```

## API Reference

| Type | What it does |
|---|---|
| `HarmonicRing` | ℤ/nℤ ring: `add`, `sub`, `mul`, `neg`, `is_unit`, `inv`, `transpose`, `invert` |
| `Triad` | Major/minor triad as `(root, quality)` with `third`, `fifth`, `pcs` |
| `PlrGroup` | Neo-Riemannian `p`, `l`, `r` (all involutions), `compose`, `hexatonic_cycle` |
| `TropicalSemiring` | min-plus: `add = min`, `mul = +`, `pow`, `zero`, `one`, `eval_poly` |
| `TuningField` | `equal_temperament`, `just_intonation`, `frequency`, `frequency_of_midi`, `cents`, `cents_deviation` |
| `combinatorics` | `voice_leading_distance`, `minimal_voice_leading` |

```text
HarmonicRing::chromatic()           // 12-tone pitch-class ring
ring.transpose(&pitch_classes, n)   // transpose by n semitones
ring.invert(&pitch_classes, axis)   // invert around an axis

PlrGroup::p(triad)                  // Parallel: same root, flip quality
PlrGroup::l(triad)                  // Leading-tone exchange (involution)
PlrGroup::r(triad)                  // Relative (involution)
PlrGroup::compose(&[ops], triad)    // compose operations left-to-right
PlrGroup::hexatonic_cycle(triad)    // 6-triad PL hexatonic cycle

TuningField::equal_temperament(n)   // n equal divisions of the octave (2^(i/n))
TuningField::just_intonation()      // 5-limit just major-scale ratios
field.frequency(degree)             // Hz for a scale degree (wraps mod n)
field.frequency_of_midi(midi)       // standard 12-TET Hz for a MIDI note (A4 = base)
field.cents(ratio)                  // cents value of an interval ratio
field.cents_deviation(ratio)        // cents off the nearest 12-TET semitone

voice_leading_distance(from, to)    // sum of absolute per-voice motion
minimal_voice_leading(from, to)     // (permuted_to, distance); exact for n≤6, heuristic for n>6
```

## Installation

This crate is not (yet) on crates.io. Use it from git:

```toml
[dependencies]
flux-algebra = { git = "https://github.com/SuperInstance/flux-algebra-rs" }
```

Build from source:

```bash
git clone https://github.com/SuperInstance/flux-algebra-rs.git
cd flux-algebra-rs
cargo build
```

## Testing

```bash
cargo test           # 21 unit tests + 8 integration tests + 1 compiled doctest
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

Coverage spans ring arithmetic, PLR transformations and the PL hexatonic cycle,
tropical semiring axioms, equal-temperament ratios and just-intonation cents,
MIDI/cents helpers, and voice-leading search (exact and heuristic).

## Related

The Rust port of the flux-algebra family:

- [flux-algebra](https://github.com/SuperInstance/flux-algebra) — original Python implementation
- [flux-algebra-c](https://github.com/SuperInstance/flux-algebra-c) — C99 port

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance) ecosystem.

## License

MIT
