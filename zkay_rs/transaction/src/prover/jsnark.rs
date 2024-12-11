#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
//::os
// from subprocess::SubprocessError
// use tempfile::TemporaryDirectory
// use typing::List
use jsnark_interface::jsnark_interface as jsnark;
use jsnark_interface::libsnark_interface as libsnark;
use std::path::PathBuf;
use crate::interface::ZkayProverInterface;
use zkay_utils::helpers::hash_file;
use zkay_utils::timer::time_measure;
#[derive(Clone)]
pub struct JsnarkProver;

impl ZkayProverInterface for JsnarkProver {
    // class JsnarkProver(ZkayProverInterface):
    fn _generate_proof(
        &self,
        verifier_dir: &PathBuf,
        priv_values: Vec<i32>,
        in_vals: Vec<i32>,
        out_vals: Vec<i32>,
    ) -> Vec<i32> {
        let args: Vec<_> = in_vals.iter().chain(&out_vals).chain(&priv_values).cloned().collect();

        // # Generate proof in temporary directory
        // with TemporaryDirectory() as tempd:
        let proof_path = std::env::temp_dir().join("proof.out");
        // try:
        // with time_measure("jsnark_prepare_proof"):
        jsnark::prepare_proof(verifier_dir.to_str().unwrap(), std::env::temp_dir().to_str().unwrap(), args);

        // with time_measure("libsnark_gen_proof"):
        libsnark::generate_proof(
            verifier_dir.to_str().unwrap(),
            std::env::temp_dir().to_str().unwrap(),
            proof_path.to_str().unwrap(),
            "self.proving_scheme()",
        );
        // except SubprocessError as e:
        //     raise ProofGenerationError(e.args)

        // with open(proof_path) as f:
            let s= std::fs::read_to_string(proof_path).unwrap();
               let proof_lines =s.split("\n");
        let proof:Vec<_> = proof_lines.filter_map(|x| x.parse::<i32>().ok()).collect(); //list(map(lambda x: int(x, 0), ));
        proof
    }
    fn get_prover_key_hash(&self, verifier_directory: &str) -> Vec<u8> {
        hash_file(PathBuf::from(verifier_directory).join("proving.key").to_str().unwrap(),0)
    }
}
