use sp1_build::build_program_with_args;

fn main() {
    build_program_with_args("../program/bls-sign", Default::default());
    build_program_with_args("../program/bls-verify", Default::default())
}
