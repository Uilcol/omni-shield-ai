use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DatasetSample {
    pub code: String,
    pub vulnerable: bool,
}

pub struct DatasetLoader;

impl DatasetLoader {
    /// Compatibility loader: returns only source-code strings.
    pub fn load_from_file(path: &str) -> Result<Vec<String>, io::Error> {
        let samples = Self::load_labeled_from_file(path)?;

        Ok(samples.into_iter().map(|sample| sample.code).collect())
    }

    /// Loads the benchmark corpus preserving the expected vulnerability label.
    pub fn load_labeled_from_file(path: &str) -> Result<Vec<DatasetSample>, io::Error> {
        let content = fs::read_to_string(path)?;

        serde_json::from_str::<Vec<DatasetSample>>(&content).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid benchmark dataset '{}': {}", path, error),
            )
        })
    }
}
