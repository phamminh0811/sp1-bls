//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use blst::min_pk::SecretKey;

pub fn main() {
    // Read an input to the program.
    let sk_bytes = sp1_zkvm::io::read_vec();
    let msg: String = sp1_zkvm::io::read();
    
    let sk = SecretKey::from_bytes(&sk_bytes).unwrap();

    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    let signature = sk.sign(msg.as_bytes(), dst, &[]);
    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&signature.to_bytes());
}
