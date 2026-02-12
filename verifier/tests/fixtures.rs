// tests/fixtures.rs

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Carrega um arquivo CBOR do diretório `tests/invalid` ou `tests/valid`
/// 
/// # Exemplo
/// ```
/// let opf = load_opf("invalid/opf_invalid_extra_field.cbor");
/// ```
pub fn load_opf<P: AsRef<Path>>(filename: P) -> Vec<u8> {
    let path = Path::new("tests").join(filename);
    let mut file = File::open(&path)
        .unwrap_or_else(|_| panic!("Não foi possível abrir o arquivo {:?}", path));
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .unwrap_or_else(|_| panic!("Erro lendo o arquivo {:?}", path));
    
    buffer
}
