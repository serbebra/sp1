use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sp1_core::{
    io::{proof_serde, SP1PublicValues, SP1Stdin},
    stark::{Proof, StarkGenericConfig},
};
use std::{fs, marker::PhantomData};

use crate::{InnerSC, ReduceProof, SP1CoreProof};

/// A proof of a RISCV ELF execution with given inputs and outputs.
#[derive(Serialize, Deserialize)]
pub struct SP1ProofWithIO<P: Serialize + DeserializeOwned> {
    #[serde(with = "proof_serde")]
    pub proof: P,
    pub stdin: SP1Stdin,
    pub public_values: SP1PublicValues,
}

#[derive(Serialize, Deserialize)]
pub struct Groth16Proof {
    pub a: Vec<Vec<u8>>,
    pub b: Vec<Vec<Vec<u8>>>,
    pub c: Vec<Vec<u8>>,
}

pub enum SP1Proof {
    Mock(SP1ProofWithIO<PhantomData<()>>),
    Default(SP1ProofWithIO<SP1CoreProof>),
    Compressed(SP1ProofWithIO<ReduceProof<InnerSC>>),
    Groth16(SP1ProofWithIO<Groth16Proof>),
}

impl<SC: StarkGenericConfig + Serialize + DeserializeOwned> SP1ProofWithIO<SC> {
    /// Saves the proof as a JSON to the given path.
    pub fn save(&self, path: &str) -> Result<()> {
        let data = serde_json::to_string(self).unwrap();
        fs::write(path, data).unwrap();
        Ok(())
    }
}
