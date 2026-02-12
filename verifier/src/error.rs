// src/error.rs

#[derive(Debug)]
pub enum VerifierError {
    BadSignature,
    InvalidCbor,
    InvalidFormat,
    InvalidStructure,
    PolicyConflict,
}
