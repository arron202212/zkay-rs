#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// import os

use zkay_config::config::CFG;
use zkay_utils::run_command::run_commands;

const libsnark_runner: &str = "run_snark";
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    pub static ref RUNNER_DIR: String = crate::file_abs_workspace!()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    pub static ref LIBSNARK_RUNNER_DIRS: String = RUNNER_DIR.clone() + "/" + libsnark_runner;
    static ref proving_scheme_map: HashMap<&'static str, i32> =
        HashMap::from([("pghr13", 0), ("groth16", 1), ("gm17", 2)]);
}
// """
// Generate prover and verification keys for the circuit in output_dir with the specified proving_scheme.

// :param input_dir: path to directory where the circuit.arith and .in files are located
// :param output_dir: path to the directory where the keys should be saved
// :param proving_scheme: name of the proving scheme to use
// :raise SubprocessError: if key generation fails
// :raise KeyError: if proving scheme name is invalid
// """
pub fn generate_keys(input_dir: &str, output_dir: &str, proving_scheme: &str) {
    run_commands(
        // vec![
        //     &LIBSNARK_RUNNER_DIRS,
        //     "keygen",
        //     input_dir,
        //     output_dir,
        //     &proving_scheme_map.get(&proving_scheme).unwrap().to_string(),
        // ],
        generate_keys_docker_command(input_dir, output_dir, proving_scheme),
        None,
        true,
    );
}
fn generate_keys_docker_command(
    input_dir: &str,
    output_dir: &str,
    proving_scheme: &str,
) -> Vec<String> {
    let Ok(zkay_path) = std::env::var("ZKAY_PATH") else {
        panic!("=====ZKAY_PATH=====Not found====");
    };
    let mut volumes: Vec<_> = "docker run -it --rm"
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect();
    volumes.append(&mut vec![String::from("-v"), format!("{}:/tmp", zkay_path)]);
    volumes.append(&mut vec![
        String::from("-v"),
        format!("{input_dir}:{input_dir}"),
    ]);
    if input_dir != output_dir {
        volumes.append(&mut vec![
            String::from("-v"),
            format!("{output_dir}:{output_dir}"),
        ]);
    }
    volumes.append(
        &mut "--name zkay-container-cmd ethsrilab/zkay:version-1.5"
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect(),
    );
    volumes.append(&mut vec![String::from("/bin/bash"), format!("-c")]);
    volumes.push(format!(
        "/tmp/zkay/jsnark_interface/run_snark keygen {input_dir} {output_dir} {}",
        proving_scheme_map.get(&proving_scheme).unwrap()
    ));
    volumes
}
// """
// Generate a NIZK-proof for the circuit and input files in output_dir.

// :param key_dir: directory where proving.key and verifying.key.bin are located
// :param input_dir: directory where circuit.arith and circuit.in for this circuit are located.
// :param output_path: output path for the generated proof file
// :param proving_scheme: name of the proving scheme to use
// :raise SubprocessError: if proof generation fails
// :raise KeyError: if proving scheme name is invalid
// """
pub fn generate_proof(key_dir: &str, input_dir: &str, output_path: &str, proving_scheme: &str) {
    run_commands(
        // vec![
        //     &LIBSNARK_RUNNER_DIRS,
        //     "proofgen",
        //     input_dir,
        //     output_path,
        //     key_dir,
        //     &proving_scheme_map.get(&proving_scheme).unwrap().to_string(),
        //     &CFG.lock()
        //         .unwrap()
        //         .user_config
        //         .libsnark_check_verify_locally_during_proof_generation()
        //         .to_string()
        //         .to_ascii_lowercase(),
        // ],
        generate_proof_docker_command(key_dir, input_dir, output_path, proving_scheme),
        None,
        true,
    );
}
fn generate_proof_docker_command(
    key_dir: &str,
    input_dir: &str,
    output_path: &str,
    proving_scheme: &str,
) -> Vec<String> {
    let Ok(zkay_path) = std::env::var("ZKAY_PATH") else {
        panic!("=====ZKAY_PATH=====Not found====");
    };
    let mut volumes: Vec<_> = "docker run -it --rm"
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect();
    volumes.append(&mut vec![String::from("-v"), format!("{}:/tmp", zkay_path)]);
    volumes.append(&mut vec![
        String::from("-v"),
        format!("{input_dir}:{input_dir}"),
    ]);
    if input_dir != output_path {
        volumes.append(&mut vec![
            String::from("-v"),
            format!("{output_path}:{output_path}"),
        ]);
    }
    if key_dir != input_dir && key_dir != output_path {
        volumes.append(&mut vec![
            String::from("-v"),
            format!("{key_dir}:{key_dir}"),
        ]);
    }
    volumes.append(
        &mut "--name zkay-container-cmd ethsrilab/zkay:version-1.5"
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect(),
    );
    volumes.append(&mut vec![String::from("/bin/bash"), format!("-c")]);
    volumes.push(format!(
        "/tmp/zkay/jsnark_interface/run_snark proofgen {input_dir} {output_path} {key_dir} {} {}",
        proving_scheme_map.get(&proving_scheme).unwrap(),
        CFG.lock()
            .unwrap()
            .user_config
            .libsnark_check_verify_locally_during_proof_generation()
    ));
    volumes
}
