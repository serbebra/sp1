//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use reth_primitives::{Block, Header};
use serde::{Deserialize, Serialize};

pub fn main() {
    println!("cycle-tracker-start: r0 serde");
    let header = sp1_zkvm::io::read::<Block>();
    println!("cycle-tracker-end: r0 serde");

    println!("cycle-tracker-start: bincode serde");
    let bincode_serialized_header = sp1_zkvm::io::read_vec();
    let bincode_header: Block = bincode::deserialize(&bincode_serialized_header).unwrap();
    println!("cycle-tracker-end: bincode serde");

    println!("cycle-tracker-start: serde_cbor serde");
    let serde_serialized_header = sp1_zkvm::io::read_vec();
    let serde_header: Block = serde_cbor::from_slice(&serde_serialized_header).unwrap();
    println!("cycle-tracker-end: serde_cbor serde");

    // let mut a: u128 = 0;
    // let mut b: u128 = 1;
    // let mut sum: u128;
    // for _ in 1..n {
    //     sum = a + b;
    //     a = b;
    //     b = sum;
    // }

    // sp1_zkvm::io::commit(&a);
    // sp1_zkvm::io::commit(&b);
}
