// tests/test_invalid.rs

mod fixtures;
use verifier::verify::{verify, Decision};
use fixtures::load_opf;

#[test]
fn opf_with_extra_field_is_rejected() {
    let opf = load_opf("invalid/opf_invalid_extra_field.cbor");

    let res = verify(&opf).unwrap();
    match res {
        Decision::Reject(reason) => assert_eq!(reason, "extra_field"),
        _ => panic!("OPF with extra field accepted"),
    }
}

#[test]
fn opf_with_policy_conflict_is_rejected() {
    let opf = load_opf("invalid/opf_invalid_policy_conflict.cbor");

    let res = verify(&opf);
    assert!(matches!(res, Err(verifier::error::VerifierError::PolicyConflict)), "OPF with policy conflict accepted");
}

#[test]
fn opf_with_invalid_signature_is_rejected() {
    let opf = load_opf("invalid/opf_invalid_signature.cbor");

    let res = verify(&opf);
    assert!(matches!(res, Err(verifier::error::VerifierError::BadSignature)), "OPF with invalid signature accepted");
}

