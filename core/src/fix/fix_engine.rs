#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::fix_database::FixDatabase;

pub struct FixEngine {

    db: FixDatabase,

}

impl FixEngine {

    pub fn new() -> Self {

        Self {

            db: FixDatabase::new(),

        }

    }

    pub fn suggest(&self, rule_id: &str) -> Option<String> {

        self.db.find(rule_id)

    }
}
