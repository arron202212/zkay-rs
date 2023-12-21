// """
// This module stores metadata about the different proving schemes, which is used by config.py
// """
use std::collections::HashMap;
pub struct ProvingSchemeParams {
    pub proof_len: i32,
    pub external_sol_libs: Vec<String>,
}
use lazy_static::lazy_static;
lazy_static! {
 pub static ref PROVINGSCHEMEPARAMS:HashMap<String,ProvingSchemeParams> = HashMap::from([
    (String::from("groth16"), ProvingSchemeParams{
        proof_len: 8,
        external_sol_libs: vec![]
    }),
    (String::from("gm17"), ProvingSchemeParams{
        proof_len: 8,
        external_sol_libs: vec![String::from("BN256G2")]
    }),
 ]);
}
