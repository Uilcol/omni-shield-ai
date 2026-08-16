#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use ed25519_dalek::{Signature, VerifyingKey};
use crate::error::VerifierError;

pub fn verify_ed25519(
    pubkey: &[u8],
    msg: &[u8],
    sig: &[u8],
) -> Result<(), VerifierError> {
    let pubkey: [u8; 32] = pubkey.try_into().map_err(|_| VerifierError::BadSignature)?;
    let sig: [u8; 64] = sig.try_into().map_err(|_| VerifierError::BadSignature)?;

    let key = VerifyingKey::from_bytes(&pubkey)
        .map_err(|_| VerifierError::BadSignature)?;

    let signature = Signature::from_bytes(&sig);

    key.verify_strict(msg, &signature)
        .map_err(|_| VerifierError::BadSignature)?;

    Ok(())
}
