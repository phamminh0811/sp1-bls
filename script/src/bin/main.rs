//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use std::time::Instant;

use bls12_381_bls::{PublicKey, SecretKey, Signature};
use dusk_bytes::Serializable;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const BLS_SIGN_ELF: &[u8] = include_elf!("bls-sign");
pub const BLS_VERIFY_ELF: &[u8] = include_elf!("bls-verify");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    let sk_bytes = [
        78, 252, 122, 126, 32, 0, 75, 89, 252, 31, 42, 130, 254, 88, 6, 90, 138, 202, 135, 194,
        233, 117, 181, 75, 96, 238, 79, 100, 237, 59, 140, 111,
    ];

    // Load some keys from a serialized secret key.
    let secret_key = SecretKey::from_bytes(&sk_bytes).unwrap();
    let public_key = PublicKey::from(&secret_key);
    let message = String::from("bls_test");

    // // Setup the inputs.
    // let mut stdin = SP1Stdin::new();
    // stdin.write_vec(Vec::from(sk_bytes));
    // stdin.write(&message);

    // let (pk, _) = client.setup(BLS_SIGN_ELF);

    // let start = Instant::now();
    // // Generate the proof
    // let proof = client
    //     .prove(&pk, &stdin)
    //     .groth16()
    //     .run()
    //     .expect("failed to generate proof");

    // println!("sign time: {}", start.elapsed().as_millis());

    // let sig = proof.public_values.as_slice();

    let sig = secret_key.sign(message.as_bytes());
    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&public_key.to_bytes());
    stdin.write_slice(&sig.to_bytes());
    stdin.write(&message);

    let (pk, _) = client.setup(BLS_VERIFY_ELF);
    let start = Instant::now();
    // Generate the proof
    let mut proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("failed to generate proof");

    println!("verify time: {}", start.elapsed().as_millis());

    let verify: bool = proof.public_values.read();
    println!("pub val: {}", proof.public_values.raw());
    println!("Verify: {}", verify);
}
