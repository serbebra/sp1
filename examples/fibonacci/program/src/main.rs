#![no_main]
sp1_zkvm::entrypoint!(main);

use std::hint::black_box;

#[no_mangle]
#[inline(never)]
fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut i = 0;
    while i != n {
        let tmp = a;
        a = b;
        b = tmp + b;
        i += 1;
    }
    b
}

pub fn main() {
    let result = fibonacci(25);
}
