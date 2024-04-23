#![no_main]
sp1_zkvm::entrypoint!(main);

use std::hint::black_box;

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    b
}

pub fn main() {
    let result = fibonacci(100);
}
