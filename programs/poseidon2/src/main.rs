#![no_main]
use p3_baby_bear::BabyBear;
use p3_field::PrimeField64;
use p3_mds::{babybear::MdsMatrixBabyBear, MdsPermutation};
use p3_poseidon2::{self, DiffusionMatrixBabybear, DiffusionPermutation, Poseidon2};
use p3_symmetric::Permutation;

extern crate succinct_zkvm;
succinct_zkvm::entrypoint!(main);

fn poseidon2<F, Mds, Diffusion, const WIDTH: usize, const D: u64>()
where
    F: PrimeField64,
    Mds: MdsPermutation<F, WIDTH> + Default,
    Diffusion: DiffusionPermutation<F, WIDTH> + Default,
{
    let external_mds = Mds::default();
    let internal_mds = Diffusion::default();

    // TODO: Should be calculated for the particular field, width and ALPHA.
    let rounds_f = 8;
    let rounds_p = 22;
    let rounds = rounds_f + rounds_p;

    println!("cycle-tracker-start: poseidon2::new()");
    let poseidon2 = Poseidon2::<F, Mds, Diffusion, WIDTH, D>::new(
        rounds_f,
        rounds_p,
        vec![[F::zero(); WIDTH]; rounds], // constants
        external_mds,
        internal_mds,
    );
    println!("cycle-tracker-end: poseidon2::new()");

    poseidon2.permute_mut(&mut [F::zero(); WIDTH]);
}

pub fn main() {
    poseidon2::<BabyBear, MdsMatrixBabyBear, DiffusionMatrixBabybear, 16, 7>();
}
