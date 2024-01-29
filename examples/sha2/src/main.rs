#![no_main]

extern crate succinct_zkvm;
use hex_literal::hex;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::BufReader;

succinct_zkvm::entrypoint!(main, crate::Sha2Initializer);

struct Sha2Initializer;

#[derive(Debug, Deserialize)]
struct InputJson {
    pub input: String,
}

impl succinct_zkvm::outside::Initializer for Sha2Initializer {
    fn init(&self, input: Option<String>) -> Vec<u8> {
        let file =
            File::open(input.expect("Input JSON file required")).expect("Failed to open file");
        let reader = BufReader::new(file);
        let input: InputJson = serde_json::from_reader(reader).unwrap();

        let len = input.input.len();
        let bytes = input.input.into_bytes();

        let mut output = Vec::new();
        output.extend(bytes.clone());
        output.extend(bytes);
        output
    }
}

pub fn main() {
    let input_string = succinct_zkvm::io::read::<String>();
    // let hash = Sha256::digest(b"hello world");
    println!("input: {:?}", input_string);
    let hash = Sha256::digest(input_string.as_bytes());
    let mut ret = [0u8; 32];
    ret.copy_from_slice(&hash);
    println!("{}", hex::encode(ret));
    assert_eq!(
        ret,
        hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
    );
}
