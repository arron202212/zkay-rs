#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// import json
// import os
// use  contextlib::contextmanager;
// use  typing::Dict, Any, ContextManager, List;
// use  semantic_version::NpmSpec;

use crate::meta::PROVINGSCHEMEPARAMS;
// use  crate::config_user::UserConfig;
use crate::config_version::{Versions, VersionsBase};
// use zkay_transaction_crypto_params::params::CryptoParams;
// use zkay_ast::homomorphism::String;
// use circuit_generation::circuit_helper::CircuitHelper;
use crate::config_user::{UserConfig, UserConfigBase};
use crate::lc_vec_s;
use app_dirs2::*;
use lazy_static::lazy_static;
use serde_json::{json, Map, Result, Value};
use std::collections::HashMap;
use std::sync::Mutex;

#[macro_export]
macro_rules! with_context_block {
    (var $name:ident = $initializer:expr => $body:expr) => {{
        let $name = $initializer;
        $body;
    }};
}

lazy_static! {
    pub static ref CFG: Mutex<Config> = Mutex::new(Config::new());
    // pub static ref VERSIONS: Mutex<Versions> = Mutex::new({
    //     let mut versions_internal = Versions::new();
    //     versions_internal.set_solc_version(String::from("0.6.12"));
    //     versions_internal
    // });
}
#[macro_export]
macro_rules! zk_print {
    (verbosity_level: $verbosity_level:expr, $fmt:expr $(, $($arg:tt)*)?) => {
        if ($verbosity_level) <= CFG.lock().unwrap().verbosity() && !CFG.lock().unwrap().is_unit_test(){
            println!(concat!("zk: ", $fmt),$($($arg)*)?);
        }

    };
    ($fmt:expr $(, $($arg:tt)*)?) => {
    if 1 <= CFG.lock().unwrap().verbosity() && !CFG.lock().unwrap().is_unit_test(){
        println!($fmt, $($($arg)*)?);
    }
    };
}

// fn zk_print(*args, verbosity_level=1, **kwargs){
//     if (verbosity_level <= CFG.verbosity) and not CFG.is_unit_test:
//         print(*args, **kwargs)

pub fn zk_print_banner(title: String) {
    let l = "#".repeat(title.len() + 4);
    zk_print!("{}\n// {title} #\n{}\n", l, l);
}

pub fn indent(s: String) -> String {
    s.split("\n")
        .map(|v| {
            if v.trim().is_empty() {
                String::new()
            } else {
                format!("{}{}", CFG.lock().unwrap().indentation(), v)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub trait ConstructorOrFunctionDefinitionAttr {
    fn get_requires_verification_when_external(&self) -> bool;
    fn get_name(&self) -> String;
}
// Versions::set_solc_version("latest")
pub struct Config {
    pub user_config_base: UserConfigBase,
    pub versions_base: VersionsBase,
    _options_with_effect_on_circuit_output: Vec<String>,
    _is_unit_test: bool,
    _concrete_solc_version: Option<String>,
    vals: HashMap<String, String>,
    attrs: HashMap<String, String>,
}
impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
impl Config {
    //(UserConfig){
    //     fn __init__(&self){
    //         super().__init__()
    pub fn new() -> Self {
        // Internal values
        Self {
            user_config_base: UserConfigBase::new(),
            versions_base: VersionsBase::new(),
            _options_with_effect_on_circuit_output: lc_vec_s![
                "proving_scheme",
                "snark_backend",
                "main_crypto_backend",
                "addhom_crypto_backend",
                "opt_solc_optimizer_runs",
                "opt_hash_threshold",
                "opt_eval_constexpr_in_circuit",
                "opt_cache_circuit_inputs",
                "opt_cache_circuit_outputs"
            ],
            _is_unit_test: false,
            _concrete_solc_version: None,
            vals: HashMap::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn _load_cfg_file_if_exists(&mut self, filename: String) {
        if std::path::Path::new(&filename).exists() {
            // with open(filename) as conf:
            //     try:
            let _v: Value =
                serde_json::from_str(&std::fs::read_to_string(&filename).unwrap()).unwrap();
            // self.override_defaults(v);
            // except ValueError as e:
            //     raise ValueError(f"{e} (in file "{filename}")")
        }
    }

    pub fn load_configuration_from_disk(&mut self, local_cfg_file: String) {
        // Load global configuration file
        let global_config_dir = get_app_dir(AppDataType::SharedConfig, &self.appdirs(), "");
        let global_cfg_file =
            std::path::PathBuf::from(global_config_dir.expect("").to_str().expect(""))
                .join("config.json");
        self._load_cfg_file_if_exists(global_cfg_file.to_str().expect("").to_string());

        // Load user configuration file
        let user_config_dir = get_app_dir(AppDataType::UserConfig, self.appdirs(), "");
        let user_cfg_file =
            std::path::PathBuf::from(&user_config_dir.expect("").to_str().expect(""))
                .join("config.json");
        self._load_cfg_file_if_exists(user_cfg_file.to_str().expect("").to_string());

        // Load local configuration file
        self._load_cfg_file_if_exists(local_cfg_file);
    }

    pub fn has_attr(&self, arg: &String) -> bool {
        self.attrs.get(arg).is_some()
    }
    pub fn get_attr(&self, arg: &String) -> String {
        self.attrs.get(arg).unwrap_or(&String::new()).clone()
    }
    pub fn set_attr(&mut self, arg: &str, val: &Value) {
        self.attrs
            .insert(arg.to_owned(), val.as_str().unwrap().to_owned());
    }
    pub fn override_defaults(&mut self, overrides: &Map<String, Value>) {
        for (arg, val) in overrides {
            if !self.has_attr(arg) {
                // raise ValueError(f"Tried to override non-existing config value {arg}")
                return;
            }
            // try:
            self.set_attr(arg, val);
            // except ValueError as e:
            //     raise ValueError(f"{e} (for entry "{arg}")")
            // }
        }
    }

    pub fn export_compiler_settings(&self) -> Value {
        // self._options_with_effect_on_circuit_output
        //     .iter()
        //     .map(|k| (k.clone(), self.get_attr(k)))
        //     .collect()
        let data = json!(
        {
                "proving_scheme":self.proving_scheme(),
                "snark_backend": self.snark_backend(),
                "main_crypto_backend": self.main_crypto_backend(),
                "addhom_crypto_backend": self.addhom_crypto_backend(),
                "opt_solc_optimizer_runs": self.opt_solc_optimizer_runs(),
                "opt_hash_threshold": self.opt_hash_threshold(),
                "opt_eval_constexpr_in_circuit": self.opt_eval_constexpr_in_circuit(),
                "opt_cache_circuit_inputs": self.opt_cache_circuit_inputs(),
                "opt_cache_circuit_outputs":self.opt_cache_circuit_outputs(),
        });
        data
    }

    pub fn import_compiler_settings(&mut self, vals: Value) {
        let vals = vals.as_object().unwrap();
        for (k, v) in vals {
            if !self._options_with_effect_on_circuit_output.contains(k) {
                // raise KeyError(f"vals contains unknown option "{k}"")
                return;
            }
            self.set_attr(k, v);
        }
    }

    // pub fn is_symmetric_cipher(&self, hom: String) -> bool {
    //     self.get_crypto_params(&hom).is_symmetric_cipher()
    // }

    pub fn proof_len(&self) -> i32 {
        // println!("=====proving_scheme============={:?}",(&String::from("proving_scheme")));
        PROVINGSCHEMEPARAMS[&self.proving_scheme()].proof_len
    }

    // Names of all solidity libraries in verify_libs.sol, which need to be linked against.
    pub fn external_crypto_lib_names(&self) -> Vec<String> {
        PROVINGSCHEMEPARAMS[&self.proving_scheme()]
            .external_sol_libs
            .clone()
    }

    // This function determines whether input hashing is used for a particular circuit.
    // :return: if true, all public circuit inputs are passed as private inputs into the circuit and only their combined hash-
    //          value is passed as a public input. This makes verification constant-cost,
    //          but increases offchain resource usage during key and proof generation.
    pub fn should_use_hash(&self, pub_arg_size: i32) -> bool {
        // let pub_arg_size = circuit.trans_in_size + circuit.trans_out_size;
        pub_arg_size > self.opt_hash_threshold()
    }

    // Identifiers in user code must not start with this prefix.
    // This is to ensure that user code does not interfere with the additional code generated by the zkay compiler.
    pub fn reserved_name_prefix(&self) -> String {
        String::from("zk__")
    }

    // Identifiers in user code must not end with this suffix.
    // This is used for resolving conflicts with python globals in the generated offchain simulation code.
    pub fn reserved_conflict_resolution_suffix(&self) -> String {
        String::from("_zalt")
    }

    pub fn get_internal_name(&self, fct: &impl ConstructorOrFunctionDefinitionAttr) -> String {
        if fct.get_requires_verification_when_external() {
            format!("_{}{}", self.reserved_name_prefix(), fct.get_name())
        } else {
            fct.get_name()
        }
    }

    pub fn get_verification_contract_name(&self, contract: String, fct: String) -> String {
        format! {"{}Verify_{contract}_{fct}",self.reserved_name_prefix()}
    }
    // Return the output directory for an individual circuit
    pub fn get_circuit_output_dir_name(&self, verifier_name: String) -> String {
        format!("{verifier_name}_out")
    }

    // Return an identifier referring to the address variable of verification contract of type "type_name"
    // :param type_name: name of the unqualified verification contract type
    // :return: new identifier
    pub fn get_contract_var_name(&self, type_name: String) -> String {
        format!("{type_name}_inst")
    }

    pub fn pki_contract_name(&self) -> String {
        format!("{}PublicKeyInfrastructure", self.reserved_name_prefix())
    }

    pub fn get_pki_contract_name(&self, params_identifier_name: &String) -> String {
        format!("{}_{}", self.pki_contract_name(), params_identifier_name)
    }

    pub fn zk_out_name(&self) -> String {
        format!("{}out", self.reserved_name_prefix())
    }

    pub fn zk_in_name(&self) -> String {
        format!("{}in", self.reserved_name_prefix())
    }

    pub fn proof_param_name(&self) -> String {
        format!("{}proof", self.reserved_name_prefix())
    }

    pub fn return_var_name(&self) -> String {
        format!("{}ret", self.reserved_name_prefix())
    }

    pub fn field_prime_var_name(&self) -> String {
        format!("{}field_prime", self.reserved_name_prefix())
    }

    pub fn prover_key_hash_name(&self) -> String {
        format!("{}prover_key_hash", self.reserved_name_prefix())
    }

    pub fn zk_struct_prefix(&self) -> String {
        format!("{}data", self.reserved_name_prefix())
    }

    pub fn zk_data_var_name(&self) -> String {
        self.zk_struct_prefix()
    }

    pub fn jsnark_circuit_classname(&self) -> String {
        String::from("ZkayCircuit")
    }

    pub fn verification_function_name(&self) -> String {
        String::from("check_verify")
    }

    pub fn is_unit_test(&self) -> bool {
        self._is_unit_test
    }
}

impl UserConfig for Config {
    fn user_config_base_ref(&self) -> &UserConfigBase {
        &self.user_config_base
    }
    fn user_config_base_mut(&mut self) -> &mut UserConfigBase {
        &mut self.user_config_base
    }
}
impl Versions for Config {
    fn versions_base_ref(&self) -> &VersionsBase {
        &self.versions_base
    }
    fn versions_base_mut(&mut self) -> &mut VersionsBase {
        &mut self.versions_base
    }
}
//     @contextmanager
//    Use this fixed configuration compiling libraries to get reproducible output.
pub fn library_compilation_environment() -> WithLibraryCompilationEnvironment {
    // old_solc, old_opt_runs = self.solc_version, self.opt_solc_optimizer_runs
    // self.override_solc(self.library_solc_version());
    // self.opt_solc_optimizer_runs = 1000
    // yield
    // self.opt_solc_optimizer_runs = old_opt_runs
    // self.override_solc(old_solc)
    WithLibraryCompilationEnvironment::new()
}
pub struct WithLibraryCompilationEnvironment {
    old_solc: Option<String>,
    old_opt_runs: i32,
}
impl WithLibraryCompilationEnvironment {
    pub fn new() -> Self {
        let old_solc = Some(CFG.lock().unwrap().solc_version());
        let old_opt_runs = CFG.lock().unwrap().opt_solc_optimizer_runs();
        let library_solc_version = CFG.lock().unwrap().library_solc_version();
        CFG.lock().unwrap().override_solc(library_solc_version);
        CFG.lock().unwrap().set_opt_solc_optimizer_runs(1000);
        Self {
            old_solc,
            old_opt_runs,
        }
    }
}
impl Drop for WithLibraryCompilationEnvironment {
    fn drop(&mut self) {
        CFG.lock()
            .unwrap()
            .override_solc(self.old_solc.clone().unwrap());
        CFG.lock()
            .unwrap()
            .set_opt_solc_optimizer_runs(self.old_opt_runs);
    }
}
