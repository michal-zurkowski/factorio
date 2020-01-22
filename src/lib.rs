#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate flate2;
extern crate serde_json;

use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::io::prelude::*;

pub mod registry;
pub mod oil;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub output: i32,
    pub time_sec: f64,
    pub input: HashMap<String, i32>,
}

pub fn blueprint_string_to_json(blueprint: &str) -> std::io::Result<String> {
    let base64_encoded: &[u8] = &blueprint.as_bytes()[1..];
    let compressed = match base64::decode(base64_encoded) {
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "blueprint is not base64 encoded",
            ))
        }
        Ok(f) => f,
    };

    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = String::new();

    decoder.read_to_string(&mut decompressed)?;
    Ok(decompressed)
}
