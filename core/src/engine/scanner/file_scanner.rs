#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::fs;


    let mut files = Vec::new();

    visit_dirs(Path::new(path), &mut files);

    files
}

fn visit_dirs(dir: &Path, files: &mut Vec<String>) {

    if dir.is_dir() {

pub fn run() {
pub fn run() {
                for entry in fs::read_dir(dir).unwrap() {

            let entry = entry.unwrap();

            let path = entry.path();

            if path.is_dir() {

                visit_dirs(&path, files);

            } else {

                if let Some(p) = path.to_str() {

                    files.push(p.to_string());
                }
            }
        }
    }
}


