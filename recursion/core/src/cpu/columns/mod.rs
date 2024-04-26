use crate::memory::MemoryReadWriteCols;
use sp1_derive::AlignedBorrow;

mod branch;
mod instruction;
mod opcode;
mod opcode_specific;

pub use instruction::*;
pub use opcode::*;

use self::opcode_specific::OpcodeSpecificCols;

/// The column layout for the chip.
#[derive(AlignedBorrow, Default, Clone, Debug)]
#[repr(C)]
pub struct CpuCols<T: Copy> {
    pub clk: T,
    pub pc: T,
    pub fp: T,

    pub instruction: InstructionCols<T>,
    pub selectors: OpcodeSelectorCols<T>,

    pub a: MemoryReadWriteCols<T>, // 11
    pub b: MemoryReadWriteCols<T>, // 11
    pub c: MemoryReadWriteCols<T>, // 11
    pub memory_addr: T,
    pub memory: MemoryReadWriteCols<T>, // 11

    pub opcode_specific: OpcodeSpecificCols<T>,

    pub is_real: T,
}

// 8 + 15
