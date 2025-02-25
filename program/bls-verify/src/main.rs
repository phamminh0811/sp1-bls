//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use bls12_381_bls::{PublicKey, Signature};
use dusk_bytes::{DeserializableSlice, Serializable};

pub fn main() {
    // Read an input to the program.
    let pk_bytes = sp1_zkvm::io::read_vec();
    let sig_bytes = sp1_zkvm::io::read_vec();
    let msg: String = sp1_zkvm::io::read();

    let pk = PublicKey::from_slice(&pk_bytes).unwrap();
    if sig_bytes.len() < 48 {
        panic!("invalid length");
    }
    let mut bytes = [0u8; 48];
    bytes[..48].copy_from_slice(&sig_bytes[..48]);
    let sig = Signature::from_bytes(&bytes).unwrap();

    let verify = pk.verify(&sig, msg.as_bytes());
    let is_valid = verify.is_ok();
    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit(&is_valid);
}
