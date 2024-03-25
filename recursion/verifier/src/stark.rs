use p3_air::Air;
use sp1_core::stark::{MachineChip, StarkGenericConfig, VerifierConstraintFolder};
use sp1_recursion_compiler::{
    ir::{Builder, Config},
    verifier::challenger::DuplexChallengerVariable,
};

use crate::types::ShardProofVariable;

pub struct StarkVerifier<'a, C: Config, SC: StarkGenericConfig> {
    builder: &'a mut Builder<C>,
    _phantom: std::marker::PhantomData<SC>,
}

impl<'a, C: Config, SC: StarkGenericConfig> StarkVerifier<'a, C, SC>
where
    SC: StarkGenericConfig<Val = C::F, Challenge = C::EF>,
{
    pub fn new(builder: &'a mut Builder<C>) -> Self {
        Self {
            builder,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn verify_shard<A>(
        &mut self,
        chips: &[&MachineChip<SC, A>],
        challenger: &mut DuplexChallengerVariable<C>,
        proof: &ShardProofVariable<C>,
    ) where
        A: for<'b> Air<VerifierConstraintFolder<'b, SC>>,
    {
        let ShardProofVariable {
            commitment,
            opened_values,
            opening_proof,
            ..
        } = proof;
    }
}
