#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub mod finding;
pub mod html;
pub mod json;
pub mod sarif;
pub mod table;

// Exporta Finding pelo caminho canônico
pub use crate::finding::Finding;
