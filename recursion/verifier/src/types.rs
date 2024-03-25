use sp1_core::stark::ShardCommitment;
use sp1_recursion_compiler::{
    ir::Config,
    verifier::{
        fri::{types::Commitment, TwoAdicPcsProof},
        ChipOpening,
    },
};

pub struct ShardProofVariable<C: Config> {
    pub index: usize,
    pub commitment: ShardCommitment<Commitment<C>>,
    pub opened_values: ShardOpenedValues<C>,
    pub opening_proof: TwoAdicPcsProof<C>,
    pub chip_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ShardOpenedValues<C: Config> {
    pub chips: Vec<ChipOpening<C>>,
}
