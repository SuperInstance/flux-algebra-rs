# NOTES — flux-algebra-rs graduation pass

Permanent record of the reasoning behind each change in this pass. Every claim
below was verified against the actual code (not just the scouting summary),
and each regression test was confirmed to fail on the pre-fix code before the
fix was accepted.

## 1. `equal_temperament` math bug (scouting issue #1) — FIXED

`src/tuning.rs` computed each interval as `2^i / n`. That is a *linear* fraction
of 2, not equal temperament. The definition of n-TET is that each step is the
n-th root of 2 raised to the i-th power: `2^(i/n)`.

- Fix: `2.0_f64.powf(i as f64 / n as f64)`.
- Regression test `test_et_uses_geometric_ratios` asserts `interval[i] ==
  2^(i/12)` for all 12 degrees, the unison `interval[0] == 1.0`, and the perfect
  fifth `interval[7] ≈ 1.4983`. **Verified to fail on the old formula** (it
  produced `interval[0] = 0.0833…`).

## 2. `PlrGroup::l` involution bug (discovered during verification) — FIXED

Not in the scouting report, but found while checking whether the README's
"hexatonic cycles" claim could be made real. The minor branch of `l` computed
`major(root - 1)`, which violates the defining property that P, L, R are
**involutions** (`L(L(t)) == t`):

- Old: `L(C minor) = G♯/A♭ major`. A-minor {A,C,E} and A♭-major {A♭,C,E♭} share
  only **one** pitch class (C), so this is not even a Neo-Riemannian
  transformation (which must share two common tones).
- Correct: `L(C minor) = F major` (shares A and C) → `major(root - 4)`
  (equivalently `root + 8`). With this, `L(L(t)) == t` holds for every triad.

This is also a **prerequisite** for hexatonic cycles: only with the corrected
`L` does alternating P, L close after exactly six triads (see §3).

- Fix: `Triad::major((t.root as i8 - 4).rem_euclid(12) as u8)`.
- Regression test `test_l_involution` iterates all 24 triads and asserts
  `L(L(t)) == t`, plus a concrete `L(A minor) == F major`. **Verified to fail on
  the old branch.**
- No existing test depended on the old (wrong) behaviour: the only prior `l`
  test covered the major branch (`L(C) == Em`), which was already correct.

## 3. README-promised features that didn't exist (scouting issue #5) — IMPLEMENTED

Rather than delete the claims, all four were implemented for real (they are
small, well-defined, and fit the existing types naturally):

- **`PlrGroup::hexatonic_cycle(start)`** — the PL hexatonic cycle (one of Cohn's
  hexatonic systems): the orbit of `start` under alternating P, L. From C major
  this is `C → Cm → A♭ → A♭m → E → Em`. Test `test_hexatonic_cycle` checks the
  length is exactly 6, the cycle closes, the six triads' pitch classes form a
  single hexatonic collection `{0,3,4,7,8,11}`, and that starting from any
  member yields the same set (rotation). Requires the §2 fix.
- **`TuningField::just_intonation()`** — 5-limit just major-scale ratios
  (1/1, 9/8, 5/4, 4/3, 3/2, 5/3, 15/8). Test `test_just_intonation_ratios`.
- **`TuningField::frequency_of_midi(midi)`** — standard 12-TET MIDI → Hz using
  the field's base frequency as A4 (MIDI 69). Named distinctly from
  `frequency(degree)` to avoid overloading one method with two meanings (scale
  degree vs. MIDI note). Test `test_frequency_of_midi` checks A4/A5/A3/middle-C.
- **`TuningField::cents_deviation(ratio)`** — cents by which a ratio differs
  from the nearest 12-TET semitone (e.g. the just major third 5/4 is ≈ -13.7¢
  flat of 400¢). Test `test_cents_deviation`.

The README's API table and Quick Start were rewritten to document these with
their **real** signatures (so there are no documented APIs that don't compile).

## 4. `minimal_voice_leading` >6-note stub (scouting issue #4) — FIXED (option a)

**Verification correction.** The scouting summary said the stub "returns the
unpermuted target vector with distance `0`". The actual pre-fix code returned
`(to.to_vec(), u32::MAX)` for `n > 6` — i.e. the unpermuted target with distance
`u32::MAX`, and crucially **no search was performed**. Either way it was
dishonest: it presented a fabricated distance for a permutation it never
searched.

Chosen fix: **option (a) — a real algorithm.** For `n > 6` an exhaustive `n!`
search is infeasible, so `minimal_voice_leading_heuristic` runs a greedy
nearest-neighbour match (each `from` voice → closest still-unused `to` note,
ties broken by lowest index). It is O(n²), deterministic, always returns a valid
permutation of `to`, and the distance is recomputed honestly. It is **not**
guaranteed optimal, which is stated plainly in the doc comment; `n ≤ 6` still
uses the exact exhaustive search. The function signature is unchanged (no API
break) and the contract ("best effort for large n") is documented rather than
hidden.

Option (b) (return `Option`/`Result`) was rejected because a usable heuristic is
strictly more useful than an error, and the brief explicitly permitted it.

Regression test `test_minimal_heuristic_large` (8 voices) asserts the result is
a true permutation of `to`, the reported distance equals an independent
recomputation, and it is neither the stub's `u32::MAX` sentinel nor wrong.
**Verified to fail against a reproduction of the old stub.**

## 5. README merge conflicts (scouting issue #2) — RESOLVED

Five `<<<<<<<` / `=======` / `>>>>>>>` blocks were resolved into one coherent
document. Merge decisions (both sides compared for accuracy against the code):

- **"What This Gives You"** — kept the HEAD side's structure but corrected it:
  dropped the false "zero dependencies" claim (the crate uses `serde` +
  `thiserror`) and dropped the unverified "14 tests" phrasing.
- **Quick Start** — used HEAD's correct associated-function style
  (`PlrGroup::p`, `TropicalSemiring::add`) and discarded the other side's
  incorrect `PlrGroup::new().parallel(&c_maj)` and the `HarmonmicRing` typo,
  which never matched the code.
- **API reference** — merged both into a table + signature listing that reflects
  the **actual** signatures (e.g. `TropicalSemiring::eval_poly`, not `eval`).
- **"How it fits"** — kept only the corroborated family links (the Python
  original `flux-algebra` and the C port `flux-algebra-c`, both referenced in
  commit history); dropped unverified consumer repos to avoid fabrication.
- **Testing** — states the real current count (21 unit + 8 integration + 1
  doctest) and lists the exact gates.

Verified: zero conflict markers remain, and every identifier in the README
exists in `src/`.

## 6. Package naming (scouting issue #3) — STANDARDIZED on `flux-algebra`

`Cargo.toml` declares `name = "flux-algebra"`, so the importable crate is
`flux_algebra` (this is also what `tests/integration.rs` already uses:
`use flux_algebra::*`). The repo/GitHub URL keeps the `-rs` suffix
(`flux-algebra-rs`), which is a common convention for a Rust port and does not
need to match the crate name.

Decision: **crate name `flux-algebra` / import `flux_algebra` wins** (not
`flux_algebra_rs`), because (a) it is what the manifest and the existing
integration tests already use — renaming the package would be the more invasive,
breaking change; (b) the `-rs` repo suffix is purely a GitHub naming convention.
Accordingly the README's `use` statements, the dependency line
(`flux-algebra = { git = … }`), and a short "Naming" callout all agree. No code
or manifest rename was needed — only the README was made consistent.

## 7. Baseline quality gates + crate-root exports

While fixing the above, also resolved pre-existing gate failures so the repo
actually passes its own CI:

- `tests/integration.rs`: two `clippy::clone_on_copy` errors (calling `.clone()`
  on `Triad`, which is `Copy`) — removed the redundant clones.
- `cargo fmt` normalized the tree (the integration tests were not fmt-clean).
- `.github/workflows/ci.yml`: gates aligned to `cargo fmt --all -- --check`,
  `cargo clippy --all-targets -- -D warnings`, and `cargo test` (kept plain so
  the doctest runs).
- `src/lib.rs`: re-exported `Triad` and `TuningField` at the crate root (they
  were only reachable as `groups::Triad` / `tuning::TuningField`, which made the
  README's `use flux_algebra::{…, Triad, TuningField}` not compile). Added a
  compiled, verified doctest that exercises the full public surface so the
  README's claims are now machine-checked.

## Final gate state

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --all-targets -- -D warnings` — clean.
- `cargo test` — 21 unit + 8 integration + 1 doctest, all passing.
