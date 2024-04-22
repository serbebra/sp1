use p3_air::BaseAir;
use p3_field::{AbstractExtensionField, AbstractField};
use sp1_core::{
    air::MachineAir,
    stark::{AirOpenedValues, Chip, ChipOpenedValues},
};
use sp1_recursion_compiler::prelude::*;
use sp1_recursion_core::runtime::DIGEST_SIZE;

use crate::fri::types::TwoAdicPcsProofVariable;
use crate::fri::types::{DigestVariable, FriConfigVariable};
use crate::fri::TwoAdicMultiplicativeCosetVariable;

#[derive(DslVariable, Clone)]
pub struct SP1ReduceProofPublicValuesVariable<C: Config> {
    pub start_pc: Felt<C::F>,
    pub next_pc: Felt<C::F>,
    pub start_shard: Felt<C::F>,
    pub next_shard: Felt<C::F>,
    pub exit_code: Felt<C::F>,
}

impl<C: Config> SP1ReduceProofPublicValuesVariable<C> {
    pub fn to_array(&self, builder: &mut Builder<C>) -> Array<C, Felt<C::F>> {
        let mut array = builder.array(4);
        builder.set(&mut array, 0, self.start_pc);
        builder.set(&mut array, 1, self.start_shard);
        builder.set(&mut array, 2, self.next_pc);
        builder.set(&mut array, 3, self.next_shard);
        array
    }

    pub fn verify_digest(
        &self,
        builder: &mut Builder<C>,
        expected_digest: [Felt<C::F>; DIGEST_SIZE],
    ) {
        let elt_array = self.to_array(builder);
        let pv_digest = builder.poseidon2_hash(&elt_array);

        for (j, expected_digest_elt) in expected_digest.iter().enumerate().take(DIGEST_SIZE) {
            let digest_element = builder.get(&pv_digest, j);
            builder.assert_felt_eq(*expected_digest_elt, digest_element);
        }
    }
}

#[derive(DslVariable, Clone)]
pub struct SP1ReduceProofVariable<C: Config> {
    pub proof: ShardProofVariable<C>,
    pub public_values: SP1ReduceProofPublicValuesVariable<C>,
}

impl<C: Config> SP1ReduceProofVariable<C> {
    pub fn get_expected_pv_digest(&self, builder: &mut Builder<C>) -> [Felt<C::F>; DIGEST_SIZE] {
        let mut expected_digest = Vec::new();

        for i in 0..DIGEST_SIZE {
            expected_digest.push(builder.get(&self.shard_proof.public_values, i));
        }

        expected_digest.try_into().unwrap()
    }
}

/// Reference: [sp1_core::stark::ShardProof]
#[derive(DslVariable, Clone)]
pub struct ShardProofVariable<C: Config> {
    pub index: Var<C::N>,
    pub commitment: ShardCommitmentVariable<C>,
    pub opened_values: ShardOpenedValuesVariable<C>,
    pub opening_proof: TwoAdicPcsProofVariable<C>,
    pub public_values: Array<C, Felt<C::F>>,
}

/// Reference: [sp1_core::stark::VerifyingKey]
#[derive(DslVariable, Clone)]
pub struct VerifyingKeyVariable<C: Config> {
    pub commitment: DigestVariable<C>,
    pub pc_start: Felt<C::F>,
}

/// Reference: [sp1_core::stark::ShardCommitment]
#[derive(DslVariable, Clone)]
pub struct ShardCommitmentVariable<C: Config> {
    pub main_commit: DigestVariable<C>,
    pub permutation_commit: DigestVariable<C>,
    pub quotient_commit: DigestVariable<C>,
}

/// Reference: [sp1_core::stark::ShardOpenedValues]
#[derive(DslVariable, Debug, Clone)]
pub struct ShardOpenedValuesVariable<C: Config> {
    pub chips: Array<C, ChipOpenedValuesVariable<C>>,
}

/// Reference: [sp1_core::stark::ChipOpenedValues]
#[derive(Debug, Clone)]
pub struct ChipOpening<C: Config> {
    pub preprocessed: AirOpenedValues<Ext<C::F, C::EF>>,
    pub main: AirOpenedValues<Ext<C::F, C::EF>>,
    pub permutation: AirOpenedValues<Ext<C::F, C::EF>>,
    pub quotient: Vec<Vec<Ext<C::F, C::EF>>>,
    pub cumulative_sum: Ext<C::F, C::EF>,
    pub log_degree: Var<C::N>,
}

/// Reference: [sp1_core::stark::ChipOpenedValues]
#[derive(DslVariable, Debug, Clone)]
pub struct ChipOpenedValuesVariable<C: Config> {
    pub preprocessed: AirOpenedValuesVariable<C>,
    pub main: AirOpenedValuesVariable<C>,
    pub permutation: AirOpenedValuesVariable<C>,
    pub quotient: Array<C, Array<C, Ext<C::F, C::EF>>>,
    pub cumulative_sum: Ext<C::F, C::EF>,
    pub log_degree: Var<C::N>,
}

/// Reference: [sp1_core::stark::AirOpenedValues]
#[derive(DslVariable, Debug, Clone)]
pub struct AirOpenedValuesVariable<C: Config> {
    pub local: Array<C, Ext<C::F, C::EF>>,
    pub next: Array<C, Ext<C::F, C::EF>>,
}

impl<C: Config> ChipOpening<C> {
    pub fn from_variable<A>(
        builder: &mut Builder<C>,
        chip: &Chip<C::F, A>,
        opening: &ChipOpenedValuesVariable<C>,
    ) -> Self
    where
        A: MachineAir<C::F>,
    {
        let mut preprocessed = AirOpenedValues {
            local: vec![],
            next: vec![],
        };

        let preprocessed_width = chip.preprocessed_width();
        for i in 0..preprocessed_width {
            preprocessed
                .local
                .push(builder.get(&opening.preprocessed.local, i));
            preprocessed
                .next
                .push(builder.get(&opening.preprocessed.next, i));
        }

        let mut main = AirOpenedValues {
            local: vec![],
            next: vec![],
        };
        let main_width = chip.width();
        for i in 0..main_width {
            main.local.push(builder.get(&opening.main.local, i));
            main.next.push(builder.get(&opening.main.next, i));
        }

        let mut permutation = AirOpenedValues {
            local: vec![],
            next: vec![],
        };
        let permutation_width =
            C::EF::D * ((chip.num_interactions() + 1) / chip.logup_batch_size() + 1);
        for i in 0..permutation_width {
            permutation
                .local
                .push(builder.get(&opening.permutation.local, i));
            permutation
                .next
                .push(builder.get(&opening.permutation.next, i));
        }

        let num_quotient_chunks = 1 << chip.log_quotient_degree();

        let mut quotient = vec![];
        for i in 0..num_quotient_chunks {
            let chunk = builder.get(&opening.quotient, i);
            let mut quotient_vals = vec![];
            for j in 0..C::EF::D {
                let value = builder.get(&chunk, j);
                quotient_vals.push(value);
            }
            quotient.push(quotient_vals);
        }

        ChipOpening {
            preprocessed,
            main,
            permutation,
            quotient,
            cumulative_sum: opening.cumulative_sum,
            log_degree: opening.log_degree,
        }
    }
}

impl<C: Config> FromConstant<C> for AirOpenedValuesVariable<C> {
    type Constant = AirOpenedValues<C::EF>;

    fn constant(value: Self::Constant, builder: &mut Builder<C>) -> Self {
        AirOpenedValuesVariable {
            local: builder.constant(value.local),
            next: builder.constant(value.next),
        }
    }
}

impl<C: Config> FromConstant<C> for ChipOpenedValuesVariable<C> {
    type Constant = ChipOpenedValues<C::EF>;

    fn constant(value: Self::Constant, builder: &mut Builder<C>) -> Self {
        ChipOpenedValuesVariable {
            preprocessed: builder.constant(value.preprocessed),
            main: builder.constant(value.main),
            permutation: builder.constant(value.permutation),
            quotient: builder.constant(value.quotient),
            cumulative_sum: builder.eval(value.cumulative_sum.cons()),
            log_degree: builder.eval(C::N::from_canonical_usize(value.log_degree)),
        }
    }
}

impl<C: Config> FriConfigVariable<C> {
    pub fn get_subgroup(
        &self,
        builder: &mut Builder<C>,
        log_degree: impl Into<Usize<C::N>>,
    ) -> TwoAdicMultiplicativeCosetVariable<C> {
        builder.get(&self.subgroups, log_degree)
    }

    pub fn get_two_adic_generator(
        &self,
        builder: &mut Builder<C>,
        bits: impl Into<Usize<C::N>>,
    ) -> Felt<C::F> {
        builder.get(&self.generators, bits)
    }
}
