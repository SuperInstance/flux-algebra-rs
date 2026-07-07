//! Tuning field: algebraic extensions for tuning systems.

/// A tuning field representing rational intervals.
#[derive(Debug, Clone)]
pub struct TuningField {
    pub base_frequency: f64,
    pub intervals: Vec<f64>,
}

impl TuningField {
    /// Build an `n`-tone equal-temperament field: each degree `i` holds the
    /// interval ratio `2^(i/n)` (the n-th root of 2, raised to the `i`-th
    /// power). Degree 0 is the unison (ratio 1.0); degree `n` would be a
    /// perfect octave (ratio 2.0). The base frequency is A4 = 440 Hz.
    ///
    /// (Previously this computed `2^i / n`, a *linear* fraction of 2, which
    /// is not equal temperament at all — the n-th root of 2 is multiplicative,
    /// not additive.)
    pub fn equal_temperament(n: usize) -> Self {
        Self {
            base_frequency: 440.0,
            intervals: (0..n).map(|i| 2.0_f64.powf(i as f64 / n as f64)).collect(),
        }
    }
    pub fn frequency(&self, degree: usize) -> f64 {
        self.base_frequency
            * self
                .intervals
                .get(degree % self.intervals.len())
                .unwrap_or(&1.0)
    }
    /// Standard 12-TET frequency for a MIDI note number, using this field's
    /// base frequency as A4 (MIDI 69): `base * 2^((midi - 69) / 12)`.
    /// (MIDI note numbers are defined relative to 12-TET, so this is
    /// independent of how many divisions `self` uses.)
    pub fn frequency_of_midi(&self, midi_note: i32) -> f64 {
        self.base_frequency * 2.0_f64.powf((midi_note as f64 - 69.0) / 12.0)
    }
    pub fn cents(&self, ratio: f64) -> f64 {
        1200.0 * ratio.log2()
    }
    /// Cents by which an interval `ratio` differs from the nearest 12-TET
    /// semitone. Returns 0.0 for exact equal-tempered pitches; e.g. a just
    /// major third (5/4 ≈ 386.3 cents) is about -13.7 cents flat of the
    /// 400-cent tempered third, revealing the tuning's colour.
    pub fn cents_deviation(&self, ratio: f64) -> f64 {
        let c = self.cents(ratio);
        c - (c / 100.0).round() * 100.0
    }
    /// Build a 5-limit just-intonation field for a major scale using the
    /// classic small-integer ratios above the base frequency (A4 = 440 Hz):
    /// unison 1/1, major second 9/8, major third 5/4, fourth 4/3, fifth 3/2,
    /// major sixth 5/3, major seventh 15/8.
    pub fn just_intonation() -> Self {
        Self {
            base_frequency: 440.0,
            intervals: vec![
                1.0,
                9.0 / 8.0,
                5.0 / 4.0,
                4.0 / 3.0,
                3.0 / 2.0,
                5.0 / 3.0,
                15.0 / 8.0,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_et() {
        let et = TuningField::equal_temperament(12);
        assert_eq!(et.intervals.len(), 12);
    }

    #[test]
    fn test_et_uses_geometric_ratios() {
        // Regression for a genuine math bug: equal temperament requires each
        // semitone to be the n-th root of 2, so interval[i] == 2^(i/n). The
        // old formula was `2^i / n` (a *linear* fraction of 2), which made
        // interval[0] == 1/n (e.g. 1/12) instead of the unison ratio 1.0, and
        // never produced a real octave. This assertion fails on the old code.
        let et = TuningField::equal_temperament(12);
        for i in 0..12 {
            let expected = 2.0_f64.powf(i as f64 / 12.0);
            assert!(
                (et.intervals[i] - expected).abs() < 1e-12,
                "interval[{i}] = {}, expected 2^(i/12) = {}",
                et.intervals[i],
                expected
            );
        }
        // Unison is exactly ratio 1.0.
        assert!((et.intervals[0] - 1.0).abs() < 1e-12);
        // A perfect fifth (7 semitones) is the familiar ~1.4983 ratio.
        assert!((et.intervals[7] - 1.498_307).abs() < 1e-4);
        // Twelve steps span exactly one octave: ratio 2.0. The old formula
        // only returned 2.0 at i == 1 and was never a real octave.
        assert!((2.0_f64.powf(12.0 / 12.0) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_frequency_of_midi() {
        let et = TuningField::equal_temperament(12);
        // A4 (MIDI 69) is the base frequency, 440 Hz.
        assert!((et.frequency_of_midi(69) - 440.0).abs() < 1e-9);
        // An octave up/down doubles/halves.
        assert!((et.frequency_of_midi(81) - 880.0).abs() < 1e-9);
        assert!((et.frequency_of_midi(57) - 220.0).abs() < 1e-9);
        // Middle C (MIDI 60) is ~261.6256 Hz.
        assert!((et.frequency_of_midi(60) - 261.6256).abs() < 1e-3);
    }

    #[test]
    fn test_cents_deviation() {
        let et = TuningField::equal_temperament(12);
        // 12-TET intervals sit exactly on 100-cent boundaries -> no deviation.
        for i in 0..12 {
            assert!(
                et.cents_deviation(et.intervals[i]).abs() < 1e-9,
                "et interval {i} should have zero deviation"
            );
        }
        // A just major third (5/4) is 386.31 cents, ~13.69 flat of 400 cents.
        let ji = TuningField::just_intonation();
        let dev = ji.cents_deviation(ji.intervals[2]);
        assert!(
            (dev - (-13.686_289)).abs() < 1e-3,
            "just M3 deviation = {dev}"
        );
    }

    #[test]
    fn test_just_intonation_ratios() {
        let ji = TuningField::just_intonation();
        assert_eq!(ji.intervals.len(), 7);
        assert!((ji.intervals[0] - 1.0).abs() < 1e-12); // unison 1/1
        assert!((ji.intervals[2] - 1.25).abs() < 1e-12); // just M3 5/4
        assert!((ji.intervals[4] - 1.5).abs() < 1e-12); // just P5 3/2
        assert!((ji.intervals[6] - 1.875).abs() < 1e-12); // just M7 15/8
    }
}
