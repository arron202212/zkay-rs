// import os

use crate::config::CFG;
use crate::utils::run_command::run_command;

const libsnark_runner: &str = "run_snark";
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref proving_scheme_map: HashMap<&'static str, i32> =
        HashMap::from([("pghr13", 0), ("groth16", 1), ("gm17", 2)]);
}

pub fn generate_keys(input_dir: &str, output_dir: &str, proving_scheme: &str)
// """
// Generate prover and verification keys for the circuit in output_dir with the specified proving_scheme.

// :param input_dir: path to directory where the circuit.arith and .in files are located
// :param output_dir: path to the directory where the keys should be saved
// :param proving_scheme: name of the proving scheme to use
// :raise SubprocessError: if key generation fails
// :raise KeyError: if proving scheme name is invalid
// """
{
    run_command(
        vec![
            libsnark_runner,
            "keygen",
            input_dir,
            output_dir,
            proving_scheme_map[proving_scheme],
        ],
        true,
    );
}

pub fn generate_proof(key_dir: &str, input_dir: &str, output_path: &str, proving_scheme: &str)
// """
// Generate a NIZK-proof for the circuit and input files in output_dir.

// :param key_dir: directory where proving.key and verifying.key.bin are located
// :param input_dir: directory where circuit.arith and circuit.in for this circuit are located.
// :param output_path: output path for the generated proof file
// :param proving_scheme: name of the proving scheme to use
// :raise SubprocessError: if proof generation fails
// :raise KeyError: if proving scheme name is invalid
// """
{
    run_command(
        [
            libsnark_runner,
            "proofgen",
            input_dir,
            output_path,
            key_dir,
            proving_scheme_map[proving_scheme],
            CFG.lock()
                .unwrap()
                .libsnark_check_verify_locally_during_proof_generation,
        ],
        true,
    );
}
