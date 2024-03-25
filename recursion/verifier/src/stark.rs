use sp1_core::stark::StarkGenericConfig;
use sp1_recursion_compiler::ir::Builder;

pub struct StarkVerifier<'a, C, SC: StarkGenericConfig> {
    builder: &'a mut Builder<C>,
}

impl<'a, C: Config, SC: StarkGenericConfig> StarkVerifier<'a, C, SC>
where
    SC: StarkGenericConfig<Val = C::F, Challenge = C::EF>,
{
    pub const fn new(builder: &'a mut Builder<C>) -> Self {
        Self { builder }
    }

    pub fn verify_shard<A>(
        &mut self,
        chips: &[&MachineChip<SC, A>],
        challenger: &mut SC::Challenger,
        proof: &ShardProof<SC>,
    ) where
        A: for<'a> Air<VerifierConstraintFolder<'a, SC>>,
    {
    }
}
