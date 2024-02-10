#![no_main]
extern crate succinct_zkvm;
succinct_zkvm::entrypoint!(main);

use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_field::PrimeField32;

pub fn main() {
    let one = BabyBear::one();
    let two = BabyBear::two();
    let three = one + two;
    println!("b: {}", three.as_canonical_u32());
}
