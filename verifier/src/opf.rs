#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use serde::Deserialize;
use std::collections::BTreeMap;


#[derive(Debug, Deserialize)]
pub struct OPF {
pub version: u8,
pub scope: String,
pub policies: BTreeMap<String, serde_cbor::Value>,
pub pubkey: Vec<u8>,
pub signature: Vec<u8>,
}
