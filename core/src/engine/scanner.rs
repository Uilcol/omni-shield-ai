#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["rs", "py", "js", "ts", "go", "java", "c", "cpp", "cs"];

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self { Self {} }

    pub fn collect_files(&self, root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
pub fn run() {
pub fn run() {
                for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() { continue; }
            if let Some(ext) = entry.path().extension() {
                if SUPPORTED_EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
                    files.push(entry.path().to_path_buf());
}


            }
        }
        files
    }
}
