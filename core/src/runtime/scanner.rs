use std::fs;
use std::path::Path;

pub fn discover_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    visit_dirs(Path::new(path), &mut files);
    files
}

fn visit_dirs(dir: &Path, files: &mut Vec<String>) {
    if !dir.exists() {
        return;
    }

    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    visit_dirs(&path, files);
                } else if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();

                    if ext == "py"
                        || ext == "js"
                        || ext == "ts"
                        || ext == "java"
                        || ext == "go"
                        || ext == "rs"
                    {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
}
