cfg_if::cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        use core::arch::asm;
        use p3_field::PrimeField32;
        use sha2::Digest;
        use crate::syscalls::PV_DIGEST_NUM_WORDS;
        use crate::syscalls::POSEIDON_NUM_WORDS;
        use crate::zkvm;
    }
}

/// Halts the program.
#[allow(unused_variables)]
pub extern "C" fn syscall_halt(exit_code: u8) -> ! {
    #[cfg(target_os = "zkvm")]
    unsafe {
        asm!(
            "ecall",
            in("t0") crate::syscalls::HALT,
            in("a0") exit_code
        );
        unreachable!()
    }

    #[cfg(not(target_os = "zkvm"))]
    unreachable!()
}
