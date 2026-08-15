pub struct Score {
    pub risk: f32,
    pub confidence: f32,
}

pub struct Scorer;

impl Scorer {
    pub fn calculate(path_len: usize, constraints: usize) -> Score {
        let risk = (constraints as f32) * 1.5;
        let confidence = 1.0 / (1.0 + path_len as f32);

        Score { risk, confidence }
    }
}
