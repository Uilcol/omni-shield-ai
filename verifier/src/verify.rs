#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
// src/verify.rs

use crate::{
    parser::{parse_opf, ParsedOpf},
    signature::verify_signature,
    policy::check_policy,
    error::VerifierError,
};

pub enum Decision {
    Allow,
    Reject(&'static str),
}

// Função principal de verificação
pub fn verify(opf_bytes: &[u8]) -> Result<Decision, VerifierError> {
    let parsed: ParsedOpf = parse_opf(opf_bytes)?;

    if parsed.has_extra_field {
        return Ok(Decision::Reject("extra_field"));
    }

    verify_signature(&parsed)?;
    check_policy(&parsed)?;

    Ok(Decision::Allow)
}
