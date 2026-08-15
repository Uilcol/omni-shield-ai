#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashMap;
pub struct FileCache {
    pub cache: HashMap<String, String>,
}
impl FileCache {
    pub fn new() -> Self {
        FileCache {
            cache: HashMap::new(),
        }
    }
    pub fn get(&mut self, _path: &str) -> String {
        todo!()
    }
}
