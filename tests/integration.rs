use flux_algebra::*;

#[test]
fn test_ring_closure() {
    let r = rings::HarmonicRing::chromatic();
    assert_eq!(r.add(11, 2), 1);
    assert_eq!(r.add(0, 0), 0);
    assert_eq!(r.add(6, 6), 0);
}

#[test]
fn test_ring_multiplicative_inverse() {
    let r = rings::HarmonicRing::chromatic();
    assert_eq!(r.inv(7), Some(7));
    assert_eq!(r.inv(5), Some(5));
    assert_eq!(r.inv(2), None);
}

#[test]
fn test_transpose_roundtrip() {
    let r = rings::HarmonicRing::chromatic();
    let notes = vec![0, 4, 7];
    let transposed = r.transpose(&notes, 5);
    let back = r.transpose(&transposed, 7);
    for i in 0..notes.len() {
        assert_eq!(back[i], notes[i]);
    }
}

#[test]
fn test_plr_group_act() {
    let c_major = groups::Triad::major(0);
    let c_minor = groups::PlrGroup::p(c_major);
    assert_eq!(c_minor.root, 0);
    assert!(!c_minor.major);
}

#[test]
fn test_plr_involution() {
    let c = groups::Triad::major(0);
    let pp = groups::PlrGroup::p(groups::PlrGroup::p(c));
    assert_eq!(pp.root, c.root);
    assert_eq!(pp.major, c.major);
}

#[test]
fn test_tropical_semiring_axioms() {
    assert_eq!(tropical::TropicalSemiring::add(3.0, 5.0), 3.0f64.min(5.0));
    assert_eq!(tropical::TropicalSemiring::mul(3.0, 5.0), 3.0 + 5.0);
    assert_eq!(
        tropical::TropicalSemiring::add(5.0, tropical::TropicalSemiring::zero()),
        5.0
    );
    assert_eq!(
        tropical::TropicalSemiring::mul(5.0, tropical::TropicalSemiring::one()),
        5.0
    );
}

#[test]
fn test_voice_leading_identity() {
    let from = vec![0, 4, 7];
    let to = vec![0, 4, 7];
    let (_, d) = combinatorics::minimal_voice_leading(&from, &to);
    assert_eq!(d, 0);
}

#[test]
fn test_tuning_field_12tet() {
    let et = tuning::TuningField::equal_temperament(12);
    assert_eq!(et.intervals.len(), 12);
}
