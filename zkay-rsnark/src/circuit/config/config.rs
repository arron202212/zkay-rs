#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use std::sync::OnceLock;
pub static Configs: OnceLock<Config> = OnceLock::new();
use num_bigint::BigInt;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Config {
    pub field_prime: BigInt,
    pub log2_field_prime: u64,
    pub libsnark_exec: String,
    pub running_multi_generators: bool,
    pub hex_output_enabled: bool,
    pub output_verbose: bool,
    pub debug_verbose: bool,
    pub print_stack_trace_at_warnings: bool,
}
pub fn init() {
    Configs.get_or_init(|| Config::new());
}
impl Config {
    pub fn new() -> Self {
        let mut config_dir = Path::new(".");
        let mut c = std::fs::read_to_string(config_dir.join("config.properties")).unwrap();
        let mut m = std::collections::HashMap::new();
        for item in c.lines() {
            let v: Vec<_> = item.split("=").collect();
            m.insert(v[0].to_owned(), v[1].to_owned());
        }
        let field_prime =
            BigInt::parse_bytes(m.get("FIELD_PRIME").unwrap().as_bytes(), 10).unwrap();
        let log2_field_prime = field_prime.bits();
        let libsnark_exec = m.get("PATH_TO_LIBSNARK_EXEC").unwrap();
        let running_multi_generators = m.get("RUNNING_GENERATORS_IN_PARALLEL").unwrap() == "0";
        let hex_output_enabled = m.get("PRINT_HEX").unwrap() == "1";
        let output_verbose = m.get("OUTPUT_VERBOSE").unwrap() == "1";
        let debug_verbose = m.get("DEBUG_VERBOSE").unwrap() == "1";
        let print_stack_trace_at_warnings = false;
        Self {
            field_prime,
            log2_field_prime,
            libsnark_exec: libsnark_exec.to_owned(),
            running_multi_generators,
            hex_output_enabled,
            output_verbose,
            debug_verbose,
            print_stack_trace_at_warnings,
        }
    }
}
