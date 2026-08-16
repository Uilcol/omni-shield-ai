#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
// src/policy.rs

use crate::{parser::ParsedOpf, error::VerifierError};

// Validação de política
pub fn check_policy(parsed: &ParsedOpf) -> Result<(), VerifierError> {
    if parsed.policy_conflict {
        return Err(VerifierError::PolicyConflict);
    }
    Ok(())
}
