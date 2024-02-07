mod columns;
mod internal_linear_permute;

#[cfg(test)]
pub mod internal_tests {

    use crate::{
        precompiles::poseidon2::internal::columns::NUM_POSEIDON2_INTERNAL_COLS,
        runtime::Program,
        utils::{prove, setup_logger},
    };

    pub fn poseidon2_internal_program() -> Program {
        todo!("");
    }

    #[test]
    fn test() {
        println!(
            "NUM_POSEIDON2_INTERNAL_COLS: {:?}",
            NUM_POSEIDON2_INTERNAL_COLS
        );
    }
}
