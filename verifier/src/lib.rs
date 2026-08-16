#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
// ─── MÓDULOS INTERNOS ─────────────────────────────
pub mod parser;
pub mod canonical;
pub mod signature;
pub mod policy;
pub mod decision;
pub mod error;
pub mod verify;

// (opcional, se existir)
// pub mod ir_hash;
// pub mod hardware;

// ─── API PÚBLICA ──────────────────────────────────
pub use verify::verify;
pub use decision::Decision;
pub use error::VerifierError;
