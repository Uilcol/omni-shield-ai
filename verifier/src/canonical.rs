use crate::error::VerifierError;

pub fn validate_cbor(bytes: &[u8]) -> Result<(), VerifierError> {
    // Validação simulada para Fase 1
    if bytes.is_empty() {
        return Err(VerifierError::InvalidCbor);
    }
    Ok(())
}
