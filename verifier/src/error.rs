#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
// src/error.rs

#[derive(Debug)]
pub enum VerifierError {
    BadSignature,
    InvalidCbor,
    InvalidFormat,
    InvalidStructure,
    PolicyConflict,
}
