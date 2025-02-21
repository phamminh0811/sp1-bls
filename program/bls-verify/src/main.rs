//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use blst::{min_pk::{PublicKey, Signature}, BLST_ERROR};

pub fn main() {
    // Read an input to the program.
    let pk_bytes  = sp1_zkvm::io::read_vec();
    let sig_bytes = sp1_zkvm::io::read_vec();
    let msg: String = sp1_zkvm::io::read();

    let pk = PublicKey::from_bytes(&pk_bytes).unwrap();
    let sig = Signature::from_bytes(&sig_bytes).unwrap();

    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    let verify = matches!(sig.verify(true, msg.as_bytes(), dst, &[], &pk, true), BLST_ERROR::BLST_SUCCESS);

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit(&verify);
}
