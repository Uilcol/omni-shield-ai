#![allow(dead_code)]

pub struct Claims {
    pub app_id: String,
    pub user: String,
}

pub fn generate_jwt(_app_id: &str, _private_key: &str) -> String {
    String::new()
}
