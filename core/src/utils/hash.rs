#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use sha2::{Sha256, Digest};
use std::fs;

    let content = fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(content);
    Some(format!("{:x}", hex::encode(hasher.finalize())))
}
