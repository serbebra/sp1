use sp1_core::stark::StarkGenericConfig;
use sp1_recursion_compiler::ir::Builder;

pub struct StarkVerifier<'a, C, SC: StarkGenericConfig> {
    builder: &'a mut Builder<SC::Val, SC::Challenge>,
}

impl<'a, SC: StarkGenericConfig, A> StarkVerifier<'a, SC, A> {
    pub const fn new(builder: &'a mut Builder<SC::Val, SC::Challenge>) -> Self {
        Self { builder }
    }

    pub fn verify_shard(
        &mut self,
        chips: &[&MachineChip<SC, A>],
        challenger: &mut SC::Challenger,
        proof: &ShardProof<SC>,
    ) where
        A: for<'a> Air<VerifierConstraintFolder<'a, SC>>,
    {
    }
}
