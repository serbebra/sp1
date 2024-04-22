//! A simple script to generate and verify the proof of a given program.

use std::str::FromStr;

use reth_primitives::{address, b256, bytes, hex, Block, Header, B256, U256};
use serde::{Deserialize, Serialize};
use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn get_header() -> Header {
    Header {
        parent_hash: B256::from_str(
            "13a7ec98912f917b3e804654e37c9866092043c13eb8eab94eb64818e886cff5",
        )
        .unwrap(),
        ommers_hash: b256!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
        beneficiary: address!("f97e180c050e5ab072211ad2c213eb5aee4df134"),
        state_root: b256!("ec229dbe85b0d3643ad0f471e6ec1a36bbc87deffbbd970762d22a53b35d068a"),
        transactions_root: b256!(
            "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
        ),
        receipts_root: b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
        logs_bloom: Default::default(),
        difficulty: U256::from(0),
        number: 0x30598,
        gas_limit: 0x1c9c380,
        gas_used: 0,
        timestamp: 0x64c40d54,
        extra_data: bytes!("d883010c01846765746888676f312e32302e35856c696e7578"),
        mix_hash: b256!("70ccadc40b16e2094954b1064749cc6fbac783c1712f1b271a8aac3eda2f2325"),
        nonce: 0,
        base_fee_per_gas: Some(7),
        withdrawals_root: Some(b256!(
            "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
        )),
        parent_beacon_block_root: None,
        blob_gas_used: Some(0),
        excess_blob_gas: Some(0x1600000),
    }
}

fn get_block() -> Block {
    // let bytes = hex!("f90288f90218a0fe21bb173f43067a9f90cfc59bbb6830a7a2929b5de4a61f372a9db28e87f9aea01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a061effbbcca94f0d3e02e5bd22e986ad57142acabf0cb3d129a6ad8d0f8752e94a0d911c25e97e27898680d242b7780b6faef30995c355a2d5de92e6b9a7212ad3aa0056b23fbba480696b65fe5a59b8f2148a1299103c4f57df839233af2cf4ca2d2b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008003834c4b408252081e80a00000000000000000000000000000000000000000000000000000000000000000880000000000000000842806be9da056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421f869f86702842806be9e82520894658bdf435d810c91414ec09147daa6db624063798203e880820a95a040ce7918eeb045ebf8c8b1887ca139d076bda00fa828a07881d442a72626c42da0156576a68e456e295e4c9cf67cf9f53151f329438916e0f24fc69d6bbb7fbacfc0c0");
    // let bytes_buf = &mut bytes.as_ref();
    // let block = Block::decode(bytes_buf).unwrap();
    Block::default()
}

fn main() {
    // Generate proof.
    utils::setup_logger();

    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // let header = get_header();
    let block = get_block();

    stdin.write(&block);

    let bincode_serialized_block = bincode::serialize(&block).unwrap();
    let serde_cbor_serialized_block = serde_cbor::to_vec(&block).unwrap();

    stdin.write_vec(bincode_serialized_block);
    stdin.write_vec(serde_cbor_serialized_block);

    //
    // let n = 186u32;
    let client = ProverClient::new();
    ProverClient::execute(ELF, stdin).expect("proving failed");
    // let proof = client.prove(ELF, stdin).expect("proving failed");
    // let mut proof = client.prove(ELF, stdin).expect("proving failed");

    // // Read output.
    // let a = proof.public_values.read::<u128>();
    // let b = proof.public_values.read::<u128>();
    // println!("a: {}", a);
    // println!("b: {}", b);

    // // Verify proof.
    // client.verify(ELF, &proof).expect("verification failed");

    // // Save proof.
    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");

    // println!("successfully generated and verified proof for the program!")
}
