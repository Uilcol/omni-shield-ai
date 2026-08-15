#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::fix_suggestion::FixSuggestion;

pub struct FixDatabase {

    fixes: Vec<FixSuggestion>,

}

impl FixDatabase {

    pub fn new() -> Self {

        Self {

            fixes: vec![

                FixSuggestion::new(
                    "PY-EVAL-001",
                    "Avoid using eval() with user input. Use safer parsing methods."
                ),

                FixSuggestion::new(
                    "PY-CMD-001",
                    "Avoid passing user input directly to system commands."
                ),

                FixSuggestion::new(
                    "PY-SQL-001",
                    "Use parameterized queries instead of string concatenation."
                ),

                FixSuggestion::new(
                    "PY-XSS-001",
                    "Escape user input before rendering in HTML."
                ),

            ],

        }

    }

    pub fn find(&self, rule_id: &str) -> Option<String> {

pub fn run() {
pub fn run() {
                for f in &self.fixes {

            if f.rule_id == rule_id {

                return Some(f.recommendation.clone());

            }

        }

        None

    }
}


