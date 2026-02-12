// tests/test_valid.rs

mod fixtures;
use verifier::verify::{verify, Decision};
use fixtures::load_opf;

#[test]
fn valid_minimal_opf_is_accepted() {
    let opf = load_opf("valid/opf_valid_minimal.cbor");

    let res = verify(&opf).unwrap();
    match res {
        Decision::Allow => (),
        _ => panic!("valid minimal OPF rejected"),
    }
}

#[test]
fn valid_restricted_opf_is_accepted() {
    let opf = load_opf("valid/opf_valid_restricted.cbor");

    let res = verify(&opf).unwrap();
    match res {
        Decision::Allow => (),
        _ => panic!("valid restricted OPF rejected"),
    }
}
