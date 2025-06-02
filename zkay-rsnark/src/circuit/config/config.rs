#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use std::sync::LazyLock;
pub static Configs: LazyLock<Config> = LazyLock::new(|| Config::new());
use num_bigint::BigInt;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;
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
// pub fn init() {
//     Configs.get_or_init(|| Config::new());
// }
impl Config {
    pub fn new() -> Self {
        let dir = crate::file_abs_workspace!();
        let mut config_dir = dir.parent().unwrap();
        let cp = config_dir.join("config.properties");
        //'/Users/lisheng/mygit/arron/zkay-rs/zkay-rsnark/src/circuit/config/config.properties'
        // /Users/lisheng/mygit/arron/zkay-rs/zkay-rsnark/circuit/config/config.properties
        // println!("==config_dir====={cp:?}====={config_dir:?}===={:?}===={:?}={:?}",cp.try_exists(),file!(),std::fs::canonicalize(".")
        //     );
        // println!("{:?}",std::path::PathBuf::from(file!()).parent().unwrap().to_str().unwrap().to_string());

        // println!("{:?}",std::fs::canonicalize(std::path::PathBuf::from(file!()).parent().unwrap().to_str().unwrap().to_string()));
        let mut c = std::fs::read_to_string(cp).unwrap();
        let mut m = std::collections::HashMap::new();
        for item in c.lines() {
            let v: Vec<_> = item.split("=").collect();
            m.insert(v[0].to_owned(), v[1].to_owned());
        }
        let field_prime =
            BigInt::parse_bytes(m.get("FIELD_PRIME").map_or("0", |v| v).as_bytes(), 10).unwrap();
        let log2_field_prime = field_prime.bits();
        let libsnark_exec = m.get("PATH_TO_LIBSNARK_EXEC").map_or(".", |v| v);
        let running_multi_generators =
            m.get("RUNNING_GENERATORS_IN_PARALLEL").map_or("0", |v| v) == "0";
        let hex_output_enabled = m.get("PRINT_HEX").map_or("0", |v| v) == "1";
        let output_verbose = m.get("OUTPUT_VERBOSE").map_or("0", |v| v) == "1";
        let debug_verbose = m.get("DEBUG_VERBOSE").map_or("0", |v| v) == "1";
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

#[macro_export]
macro_rules! file_abs_workspace {
    () => {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join($crate::circuit::config::config::pop_first_two_path_components(file!()))
    };
}

pub fn pop_first_two_path_components(path: &str) -> PathBuf {
    let mut components = std::path::Path::new(path).components();
    // println!("========={:?}===={:?},{}",std::path::Path::new(env!("CARGO_MANIFEST_DIR")),components,path);
    components.next();
    // components.next();
    // println!("=======2======{:?},{}",components,path);
    components.as_path().to_path_buf()
}
