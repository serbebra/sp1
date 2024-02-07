use core::borrow::Borrow;
use core::borrow::BorrowMut;
use std::mem::size_of;

use valida_derive::AlignedBorrow;

use crate::memory::MemoryReadCols;
use crate::memory::MemoryWriteCols;
use crate::precompiles::poseidon2::P2_INTERNAL_ROUND_COUNT;
use crate::precompiles::poseidon2::P2_SBOX_EXPONENT_LOG2;
use crate::precompiles::poseidon2::P2_WIDTH;

use super::internal_linear_permute::InternalLinearPermuteOperation;

pub const NUM_POSEIDON2_INTERNAL_COLS: usize = size_of::<Poseidon2InternalCols<u8>>();

/// Cols to perform the either the internal round of Poseidon2.
#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Poseidon2InternalCols<T> {
    pub segment: T,
    pub clk: T,

    pub state_ptr: T,

    pub mem_reads: [MemoryReadCols<T>; P2_WIDTH],
    pub mem_writes: [MemoryWriteCols<T>; P2_WIDTH],

    pub mem_addr: [T; P2_WIDTH],

    pub internal_linear_permute: InternalLinearPermuteOperation<T>,

    /// The index of the current round.                                                                             
    pub round_number: T,

    /// The single round constant necessary for this internal round.
    pub round_constant: T,

    /// Powers of `state[0] + rc` necessary to calculate the SBox.
    pub state_0_powers: [T; P2_SBOX_EXPONENT_LOG2],

    /// Intermediate results of the exponentiation by squaring algorithm.
    pub state_0_exponents: [T; P2_SBOX_EXPONENT_LOG2],

    /// A boolean array whose `n`th element indicates whether this is the `n`th round.
    ///
    /// Note that the round count in the internal rounds start from `P2_EXTERNAL_ROUND_COUNT`.
    pub is_round_n: [T; P2_INTERNAL_ROUND_COUNT],

    pub is_real: T,
}
