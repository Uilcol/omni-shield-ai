#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#[derive(Debug)]
pub enum Decision {
    Allow,
    Reject(&'static str),
}
