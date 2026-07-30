use std::fs;
use std::io;

pub struct DatasetLoader;

impl DatasetLoader {
    pub fn load_from_file(path: &str) -> Result<Vec<String>, io::Error> {
        let content = fs::read_to_string(path)?;
        Ok(Self::parse_samples(&content))
    }

    fn parse_samples(content: &str) -> Vec<String> {
        let trimmed = content.trim();

        if trimmed.is_empty() {
            return Vec::new();
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let body = &trimmed[1..trimmed.len() - 1];

            return body
                .split(',')
                .map(str::trim)
                .map(|item| item.trim_matches('"'))
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToOwned::to_owned)
                .collect();
        }

        trimmed
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    }
}
