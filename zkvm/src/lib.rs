#[cfg(target_os = "zkvm")]
use core::arch::asm;

#[cfg(target_os = "zkvm")]
use syscall::syscall_halt;

use core::alloc::{GlobalAlloc, Layout};

extern crate alloc;

pub mod io;
mod memory;
#[allow(clippy::missing_safety_doc)]
pub mod syscall;

pub const WORD_SIZE: usize = 4;

#[cfg(not(target_os = "zkvm"))]
pub mod outside {
    use clap::Parser;
    use serde::de::DeserializeOwned;
    use std::fs::File;
    use std::io::BufReader;

    pub trait Initializer {
        fn init(&self, input: Option<String>) -> Vec<u8>;
    }

    // pub trait JsonInitializer {
    //     type Json: DeserializeOwned;
    //     fn init_json(&self, input: Self::Json) -> Vec<u8>;
    // }

    // impl<T: JsonInitializer> Initializer for T {
    //     fn init(&self, input: String) -> Vec<u8> {
    //         let file = File::open(input).unwrap();
    //         let reader = BufReader::new(file);
    //         let input: T::Json = serde_json::from_reader(reader).unwrap();
    //         self.init_json(input)
    //     }
    // }

    /// Simple program to greet a person
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        // /// Name of the person to greet
        // #[arg(short, long)]
        // name: String,

        // /// Number of times to greet
        // #[arg(short, long, default_value_t = 1)]
        // count: u8,

        // Whether to just run the code instead of proving
        #[arg(long)]
        pub novm: bool,

        #[arg(short, long)]
        pub input: Option<String>,
    }
}

// #[macro_export]
// macro_rules! entrypoint {
//     ($path:path) => {
//         const ZKVM_ENTRY: fn() = $path;

//         mod zkvm_generated_main {
//             use succinct_zkvm::get_initializer;

//             #[no_mangle]
//             fn main() {
//                 #[cfg(target_os = "zkvm")]
//                 super::ZKVM_ENTRY();

//                 #[cfg(not(target_os = "zkvm"))]
//                 {
//                     get_initializer().init("".to_string());
//                 }
//             }
//         }
//     };
// }

#[macro_export]
macro_rules! entrypoint {
    // Variant with initializer override
    ($path:path, $initializer:expr) => {
        const ZKVM_ENTRY: fn() = $path;

        #[cfg(target_os = "zkvm")]
        mod zkvm_generated_main {

            #[no_mangle]
            fn main() {
                super::ZKVM_ENTRY();
            }
        }

        #[cfg(not(target_os = "zkvm"))]
        mod novm_main {
            use clap::Parser;
            use std::io::Write;
            use succinct_zkvm::outside::{Cli, Initializer};

            fn ensure_initializer_trait<T: Initializer>(_t: T) {}

            #[no_mangle]
            fn main() {
                ensure_initializer_trait($initializer);
                let args = Cli::parse();

                if args.novm {
                    super::ZKVM_ENTRY();
                } else {
                    let bytes = $initializer.init(args.input);
                    // Write bytes directly to stdout
                    println!("{}", hex::encode(bytes));
                }

                // let args = super::ZKVM_ENTRY();
            }
        }
    }; // // Variant without initializer override
       // ($path:path) => {
       //     const ZKVM_ENTRY: fn() = $path;

       //     mod zkvm_generated_main {

       //         #[no_mangle]
       //         fn main() {
       //             #[cfg(target_os = "zkvm")]
       //             super::ZKVM_ENTRY();

       //             #[cfg(not(target_os = "zkvm"))]
       //             {
       //                 println!("default");
       //             }
       //         }
       //     }
       // };
}

#[cfg(target_os = "zkvm")]
#[no_mangle]
unsafe extern "C" fn __start() {
    {
        extern "C" {
            fn main();
        }
        main()
    }

    syscall_halt();
}

#[cfg(target_os = "zkvm")]
static STACK_TOP: u32 = 0x0020_0400; // TODO: put in whatever.

#[cfg(target_os = "zkvm")]
core::arch::global_asm!(
    r#"
.section .text._start;
.globl _start;
_start:
    .option push;
    .option norelax;
    la gp, __global_pointer$;
    .option pop;
    la sp, {0}
    lw sp, 0(sp)
    jal ra, __start;
"#,
    sym STACK_TOP
);

/// RUNTIME

struct SimpleAlloc;

unsafe impl GlobalAlloc for SimpleAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        memory::sys_alloc_aligned(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

// TODO: should we use this even outside of vm?
#[cfg(target_os = "zkvm")]
#[global_allocator]
static HEAP: SimpleAlloc = SimpleAlloc;

// enum ProgramInput {
//     Bytes(Vec<u8>),
//     Json
// }

// impl<T: JsonInitializer> Initializer for T {
//     fn init(&self, input: String) -> Vec<u8> {
//         let file = File::open(input).unwrap();
//         let reader = BufReader::new(file);
//         let input: T::Json = serde_json::from_reader(reader).unwrap();
//         self.init_json(input)
//     }
// }

// pub trait HexInitializer {
//     const a: u8 = 2;
//     fn init_hex(&self, input: Vec<u8>) -> Vec<u8>;
// }

// impl<T: HexInitializer> Initializer for T {
//     fn init(&self, input: String) -> Vec<u8> {
//         let hex = input.trim_start_matches("0x");
//         let bytes = hex::decode(hex).unwrap();
//         self.init_hex(bytes)
//     }
// }

// pub struct DefaultInitializer;

// impl Initializer for DefaultInitializer {
//     fn init(&self, _: String) -> Vec<u8> {
//         println!("default initializer");
//         vec![]
//     }
// }
