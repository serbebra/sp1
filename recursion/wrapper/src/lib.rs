#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use p3_baby_bear::BabyBear;
use p3_baby_bear::DiffusionMatrixBabybear;
use p3_challenger::CanObserve;
use p3_challenger::DuplexChallenger;
use p3_challenger::FieldChallenger;
use p3_commit::ExtensionMmcs;
use p3_commit::TwoAdicMultiplicativeCoset;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::AbstractField;
use p3_field::Field;
use p3_field::TwoAdicField;
use p3_fri::FriConfig;
use p3_fri::FriProof;
use p3_fri::TwoAdicFriPcs;
use p3_fri::TwoAdicFriPcsProof;
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_poseidon2::Poseidon2;
use p3_symmetric::PaddingFreeSponge;
use p3_symmetric::TruncatedPermutation;
use sp1_core::air::MachineAir;
use sp1_core::stark::Proof;
use sp1_core::stark::VerifyingKey;
use sp1_core::utils::poseidon2_instance::RC_16_30;
use sp1_core::{
    air::{PublicValuesDigest, Word},
    runtime::Program,
    stark::{MachineStark, RiscvAir, ShardCommitment, ShardProof, StarkGenericConfig},
    utils::BabyBearPoseidon2,
};
use sp1_recursion_compiler::asm::VmBuilder;
use sp1_recursion_compiler::ir::Ext;
use sp1_recursion_compiler::ir::SymbolicExt;
use sp1_recursion_compiler::ir::SymbolicFelt;
use sp1_recursion_compiler::prelude::Array;
use sp1_recursion_compiler::prelude::Felt;
use sp1_recursion_compiler::{
    asm::AsmConfig,
    ir::{Builder, Config, Usize},
};
use sp1_recursion_core::runtime::Program as RecursionProgram;
use sp1_recursion_core::runtime::DIGEST_SIZE;
use sp1_recursion_program::challenger::CanObserveVariable;
use sp1_recursion_program::challenger::DuplexChallengerVariable;
use sp1_recursion_program::fri::BatchOpeningVariable;
use sp1_recursion_program::fri::TwoAdicFriPcsVariable;
use sp1_recursion_program::fri::TwoAdicMultiplicativeCosetVariable;
use sp1_recursion_program::fri::TwoAdicPcsProofVariable;
use sp1_recursion_program::stark::StarkVerifier;
use sp1_recursion_program::types::FriCommitPhaseProofStepVariable;
use sp1_recursion_program::types::FriConfigVariable;
use sp1_recursion_program::types::FriProofVariable;
use sp1_recursion_program::types::FriQueryProofVariable;
use sp1_recursion_program::types::{
    ChipOpenedValuesVariable, Commitment, ShardOpenedValuesVariable, ShardProofVariable,
};
use sp1_sdk::{SP1Prover, SP1Stdin};
use std::time::Instant;

type SC = BabyBearPoseidon2;
type F = <SC as StarkGenericConfig>::Val;
type EF = <SC as StarkGenericConfig>::Challenge;
type C = AsmConfig<F, EF>;
type A = RiscvAir<F>;

type Val = BabyBear;
type Challenge = BinomialExtensionField<Val, 4>;
type Perm = Poseidon2<Val, DiffusionMatrixBabybear, 16, 7>;
type Hash = PaddingFreeSponge<Perm, 16, 8, 8>;
type Compress = TruncatedPermutation<Perm, 2, 8, 16>;
type ValMmcs =
    FieldMerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, Hash, Compress, 8>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type Challenger = DuplexChallenger<Val, Perm, 16>;
type Dft = Radix2DitParallel;
type CustomPcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
type CustomFriProof = FriProof<Challenge, ChallengeMmcs, Val>;
type RecursionConfig = AsmConfig<Val, Challenge>;
type RecursionBuilder = Builder<RecursionConfig>;

#[allow(clippy::needless_range_loop)]
pub fn const_fri_proof<C>(
    builder: &mut Builder<C>,
    fri_proof: CustomFriProof,
) -> FriProofVariable<C>
where
    C: Config<F = Val, EF = Challenge>,
{
    // Initialize the FRI proof variable.
    let mut fri_proof_var = FriProofVariable {
        commit_phase_commits: builder.dyn_array(fri_proof.commit_phase_commits.len()),
        query_proofs: builder.dyn_array(fri_proof.query_proofs.len()),
        final_poly: builder.eval(SymbolicExt::Const(fri_proof.final_poly)),
        pow_witness: builder.eval(fri_proof.pow_witness),
    };

    // Set the commit phase commits.
    for i in 0..fri_proof.commit_phase_commits.len() {
        let mut commitment: Commitment<_> = builder.dyn_array(DIGEST_SIZE);
        let h: [Val; DIGEST_SIZE] = fri_proof.commit_phase_commits[i].into();
        for j in 0..DIGEST_SIZE {
            builder.set(&mut commitment, j, h[j]);
        }
        builder.set(&mut fri_proof_var.commit_phase_commits, i, commitment);
    }

    // Set the query proofs.
    for (i, query_proof) in fri_proof.query_proofs.iter().enumerate() {
        let mut commit_phase_openings_var: Array<_, FriCommitPhaseProofStepVariable<_>> =
            builder.dyn_array(query_proof.commit_phase_openings.len());

        for (j, commit_phase_opening) in query_proof.commit_phase_openings.iter().enumerate() {
            let mut commit_phase_opening_var = FriCommitPhaseProofStepVariable {
                sibling_value: builder.eval(SymbolicExt::Const(commit_phase_opening.sibling_value)),
                opening_proof: builder.dyn_array(commit_phase_opening.opening_proof.len()),
            };
            for (k, proof) in commit_phase_opening.opening_proof.iter().enumerate() {
                let mut proof_var = builder.dyn_array(DIGEST_SIZE);
                for l in 0..DIGEST_SIZE {
                    builder.set(&mut proof_var, l, proof[l]);
                }
                builder.set(&mut commit_phase_opening_var.opening_proof, k, proof_var);
            }
            builder.set(&mut commit_phase_openings_var, j, commit_phase_opening_var);
        }
        let query_proof = FriQueryProofVariable {
            commit_phase_openings: commit_phase_openings_var,
        };
        builder.set(&mut fri_proof_var.query_proofs, i, query_proof);
    }

    fri_proof_var
}

#[allow(clippy::needless_range_loop)]
pub fn const_two_adic_pcs_proof<C>(
    builder: &mut Builder<C>,
    proof: TwoAdicFriPcsProof<Val, Challenge, ValMmcs, ChallengeMmcs>,
) -> TwoAdicPcsProofVariable<C>
where
    C: Config<F = Val, EF = Challenge>,
{
    let fri_proof_var = const_fri_proof(builder, proof.fri_proof);
    let mut proof_var = TwoAdicPcsProofVariable {
        fri_proof: fri_proof_var,
        query_openings: builder.dyn_array(proof.query_openings.len()),
    };

    for (i, openings) in proof.query_openings.iter().enumerate() {
        let mut openings_var: Array<_, BatchOpeningVariable<_>> = builder.dyn_array(openings.len());
        for (j, opening) in openings.iter().enumerate() {
            let mut opened_values_var = builder.dyn_array(opening.opened_values.len());
            for (k, opened_value) in opening.opened_values.iter().enumerate() {
                let mut opened_value_var: Array<_, Ext<_, _>> =
                    builder.dyn_array(opened_value.len());
                for (l, ext) in opened_value.iter().enumerate() {
                    let el: Ext<_, _> =
                        builder.eval(SymbolicExt::Base(SymbolicFelt::Const(*ext).into()));
                    builder.set(&mut opened_value_var, l, el);
                }
                builder.set(&mut opened_values_var, k, opened_value_var);
            }

            let mut opening_proof_var = builder.dyn_array(opening.opening_proof.len());
            for (k, sibling) in opening.opening_proof.iter().enumerate() {
                let mut sibling_var = builder.dyn_array(DIGEST_SIZE);
                for l in 0..DIGEST_SIZE {
                    let el: Felt<_> = builder.eval(sibling[l]);
                    builder.set(&mut sibling_var, l, el);
                }
                builder.set(&mut opening_proof_var, k, sibling_var);
            }
            let batch_opening_var = BatchOpeningVariable {
                opened_values: opened_values_var,
                opening_proof: opening_proof_var,
            };
            builder.set(&mut openings_var, j, batch_opening_var);
        }

        builder.set(&mut proof_var.query_openings, i, openings_var);
    }

    proof_var
}

pub(crate) fn const_proof<C>(
    builder: &mut Builder<C>,
    machine: &MachineStark<SC, A>,
    proof: ShardProof<SC>,
) -> ShardProofVariable<C>
where
    C: Config<F = F, EF = EF>,
{
    let index = builder.materialize(Usize::Const(proof.index));

    // Set up the public values digest.
    let public_values_digest = PublicValuesDigest::from(core::array::from_fn(|i| {
        let word_val = proof.public_values_digest[i];
        Word(core::array::from_fn(|j| builder.eval(word_val[j])))
    }));

    // Set up the commitments.
    let mut main_commit: Commitment<_> = builder.dyn_array(DIGEST_SIZE);
    let mut permutation_commit: Commitment<_> = builder.dyn_array(DIGEST_SIZE);
    let mut quotient_commit: Commitment<_> = builder.dyn_array(DIGEST_SIZE);

    let main_commit_val: [_; DIGEST_SIZE] = proof.commitment.main_commit.into();
    let perm_commit_val: [_; DIGEST_SIZE] = proof.commitment.permutation_commit.into();
    let quotient_commit_val: [_; DIGEST_SIZE] = proof.commitment.quotient_commit.into();
    for (i, ((main_val, perm_val), quotient_val)) in main_commit_val
        .into_iter()
        .zip(perm_commit_val)
        .zip(quotient_commit_val)
        .enumerate()
    {
        builder.set(&mut main_commit, i, main_val);
        builder.set(&mut permutation_commit, i, perm_val);
        builder.set(&mut quotient_commit, i, quotient_val);
    }

    let commitment = ShardCommitment {
        main_commit,
        permutation_commit,
        quotient_commit,
    };

    // Set up the opened values.
    let num_shard_chips = proof.opened_values.chips.len();
    let mut opened_values = builder.dyn_array(num_shard_chips);
    for (i, values) in proof.opened_values.chips.iter().enumerate() {
        let values: ChipOpenedValuesVariable<_> = builder.eval_const(values.clone());
        builder.set(&mut opened_values, i, values);
    }
    let opened_values = ShardOpenedValuesVariable {
        chips: opened_values,
    };

    let opening_proof = const_two_adic_pcs_proof(builder, proof.opening_proof);

    let sorted_indices = machine
        .chips()
        .iter()
        .map(|chip| {
            let index = proof
                .chip_ordering
                .get(&chip.name())
                .map(|i| C::N::from_canonical_usize(*i))
                .unwrap_or(C::N::neg_one());
            builder.eval(index)
        })
        .collect();

    ShardProofVariable {
        index: Usize::Var(index),
        commitment,
        opened_values,
        opening_proof,
        sorted_indices,
        public_values_digest,
    }
}

pub fn const_fri_config(
    builder: &mut RecursionBuilder,
    config: FriConfig<ChallengeMmcs>,
) -> FriConfigVariable<RecursionConfig> {
    let two_addicity = Val::TWO_ADICITY;
    let mut generators = builder.dyn_array(two_addicity);
    let mut subgroups = builder.dyn_array(two_addicity);
    for i in 0..two_addicity {
        let constant_generator = Val::two_adic_generator(i);
        builder.set(&mut generators, i, constant_generator);

        let constant_domain = TwoAdicMultiplicativeCoset {
            log_n: i,
            shift: Val::one(),
        };
        let domain_value: TwoAdicMultiplicativeCosetVariable<_> =
            builder.eval_const(constant_domain);
        builder.set(&mut subgroups, i, domain_value);
    }
    FriConfigVariable {
        log_blowup: Val::from_canonical_usize(config.log_blowup),
        num_queries: config.num_queries,
        proof_of_work_bits: config.proof_of_work_bits,
        subgroups,
        generators,
    }
}

pub fn default_fri_config() -> FriConfig<ChallengeMmcs> {
    let perm = Perm::new(8, 22, RC_16_30.to_vec(), DiffusionMatrixBabybear);
    let hash = Hash::new(perm.clone());
    let compress = Compress::new(perm.clone());
    let challenge_mmcs = ChallengeMmcs::new(ValMmcs::new(hash, compress));
    FriConfig {
        log_blowup: 1,
        num_queries: 100,
        proof_of_work_bits: 8,
        mmcs: challenge_mmcs,
    }
}

fn generate_sp1_proof() -> (Proof<SC>, VerifyingKey<SC>, Challenger) {
    let elf = include_bytes!("../../../examples/fibonacci/program/elf/riscv32im-succinct-zkvm-elf");

    let machine = A::machine(SC::default());

    let (_, vk) = machine.setup(&Program::from(elf));
    let challenger_val = machine.config().challenger();
    let proof = SP1Prover::prove_with_config(elf, SP1Stdin::new(), machine.config().clone())
        .unwrap()
        .proof;
    let mut challenger_ver = machine.config().challenger();
    machine.verify(&vk, &proof, &mut challenger_ver).unwrap();
    println!("Proof generated successfully");
    (proof, vk, challenger_val)
}

fn build() -> RecursionProgram<BabyBear> {
    // Generate a dummy proof.
    sp1_core::utils::setup_logger();

    let (proof, vk, mut challenger_val) = generate_sp1_proof();
    let machine = A::machine(SC::default());
    challenger_val.observe(vk.commit);
    proof.shard_proofs.iter().for_each(|proof| {
        challenger_val.observe(proof.commitment.main_commit);
    });

    // Observe the public input digest
    let pv_digest_field_elms: Vec<F> =
        PublicValuesDigest::<Word<F>>::new(proof.public_values_digest).into();
    challenger_val.observe_slice(&pv_digest_field_elms);

    let permutation_challenges = (0..2)
        .map(|_| challenger_val.sample_ext_element::<EF>())
        .collect::<Vec<_>>();

    let time = Instant::now();
    let mut builder = VmBuilder::<F, EF>::default();
    let config = const_fri_config(&mut builder, default_fri_config());
    let pcs = TwoAdicFriPcsVariable { config };

    let mut challenger = DuplexChallengerVariable::new(&mut builder);

    let preprocessed_commit_val: [F; DIGEST_SIZE] = vk.commit.into();
    let preprocessed_commit: Array<C, _> = builder.eval_const(preprocessed_commit_val.to_vec());
    challenger.observe(&mut builder, preprocessed_commit);

    let mut shard_proofs = vec![];
    for proof_val in proof.shard_proofs {
        let proof = const_proof(&mut builder, &machine, proof_val);
        let ShardCommitment { main_commit, .. } = &proof.commitment;
        challenger.observe(&mut builder, main_commit.clone());
        shard_proofs.push(proof);
    }
    // Observe the public input digest
    let pv_digest_felt: Vec<Felt<F>> = pv_digest_field_elms
        .iter()
        .map(|x| builder.eval(*x))
        .collect();
    challenger.observe_slice(&mut builder, &pv_digest_felt);

    for proof in shard_proofs {
        StarkVerifier::<C, SC>::verify_shard(
            &mut builder,
            &vk,
            &pcs,
            &machine,
            &mut challenger.clone(),
            &proof,
            &permutation_challenges,
        );
    }

    let program = builder.compile();
    let elapsed = time.elapsed();
    println!("Building took: {:?}", elapsed);
    program

    // let mut runtime = Runtime::<F, EF, _>::new(&program, machine.config().perm.clone());

    // let time = Instant::now();
    // runtime.run();
    // let elapsed = time.elapsed();
    // runtime.print_stats();
    // println!("Execution took: {:?}", elapsed);

    // let config = BabyBearPoseidon2::new();
    // let machine = RecursionAir::machine(config);
    // let (pk, vk) = machine.setup(&program);
    // let mut challenger = machine.config().challenger();

    // // debug_interactions_with_all_chips::<BabyBearPoseidon2, RecursionAir<BabyBear>>(
    // //     machine.chips(),
    // //     &runtime.record,
    // //     vec![InteractionKind::Memory],
    // // );

    // let start = Instant::now();
    // let proof = machine.prove::<LocalProver<_, _>>(&pk, runtime.record, &mut challenger);
    // let duration = start.elapsed().as_secs();

    // let mut challenger = machine.config().challenger();
    // machine.verify(&vk, &proof, &mut challenger).unwrap();
    // println!("proving duration = {}", duration);
}

#[cfg(test)]
mod tests {
    use sp1_core::stark::LocalProver;
    use sp1_recursion_core::{runtime::Runtime, stark::RecursionAir};
    use sp1_sdk::utils::setup_logger;

    use super::*;

    #[test]
    fn test_build() {
        setup_logger();
        let program = build();

        let machine = RecursionAir::machine(BabyBearPoseidon2::new());
        let mut runtime = Runtime::<F, EF, _>::new(&program, machine.config().perm.clone());

        let time = Instant::now();
        runtime.run();
        let elapsed = time.elapsed();
        runtime.print_stats();
        println!("Execution took: {:?}", elapsed);

        let (pk, vk) = machine.setup(&program);
        let mut challenger = machine.config().challenger();

        let start = Instant::now();
        let proof = machine.prove::<LocalProver<_, _>>(&pk, runtime.record, &mut challenger);
        let duration = start.elapsed().as_secs();

        let mut challenger = machine.config().challenger();
        machine.verify(&vk, &proof, &mut challenger).unwrap();
        println!("proving duration = {}", duration);
    }
}
