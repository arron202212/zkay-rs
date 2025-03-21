// This module defines the zkay options which are configurable by the user via command line arguments.
// The argument parser in :py:mod:`.__main__` uses the docstrings, type hints and _values for the help
//  strings and the _values fields for autocompletion
// WARNING: This is one of the only zkay modules that is imported before argcomplete.autocomplete is called. \
// For performance reasons it should thus not have any import side-effects or perform any expensive operations during import.

// from typing import Any, Union, Dict, List

// from appdirs import AppDirs
use app_dirs2::*; // or app_dirs::* if you've used package alias in Cargo.toml

// use zkay_transaction_crypto_params::params::CryptoParams;
// use zkay_ast::homomorphism::String;
use std::collections::BTreeMap;

#[macro_export]
macro_rules! lc_vec_s {
    () => { Vec::<String>::new() };
    ( $( $x:expr ),* ) => {
      {
       let mut temp= vec![ ];
        $(
        temp.push(String::from( $x ));
        )*
        temp
      }
  };
}

#[macro_export]
macro_rules! lc_string_vec {
  () => {
    Vec::<Option<String>>::new()
  };
  ( $x:expr ) => {
    {
       vec![Some(String::from($x)) ]
    }
  };
  ( null ,  $($rest: tt),* ) => {
    {
      let mut temp_vec = Vec::<Option<String>>::new();
      temp_vec.push(None);
      temp_vec.extend( ( $crate::lc_string_vec![ $($rest),*  ] ));
      temp_vec
    }
  };
  ( $x:expr ,  $($rest:tt),*  ) => {
    {
      let mut temp_vec = Vec::<Option<String>>::new();
      temp_vec.push( Some(String::from($x)) );
      temp_vec.extend( ( $crate::lc_string_vec![ $($rest),* ]  ));
      temp_vec
    }
  };
}

pub fn _check_is_one_of(val: &String, legal_vals: &Vec<String>) {
    assert!(
        legal_vals.contains(val),
        "Invalid config value {val}, must be one of {:?}",
        legal_vals
    );
    // raise ValueError(f"Invalid config value {val}, must be one of {legal_vals}")
}

// pub fn //_type_check(val: Any, t){
//     if not isinstance(val, t){
//         raise ValueError(f"Value {val} has wrong type (expected {t})")
#[derive(Clone)]
pub struct UserConfigBase {
    pub _appdirs: AppInfo,
    pub _proving_scheme: String,
    pub _proving_scheme_values: Vec<String>,

    pub _snark_backend: String,
    pub _snark_backend_values: Vec<String>,

    // These only exist for the auto-generated help strings in __main__ and should not be used directly
    pub _main_crypto_backend: String,
    pub _main_crypto_backend_values: Vec<Option<String>>,
    pub _addhom_crypto_backend: String,
    pub _addhom_crypto_backend_values: Vec<Option<String>>,

    // This map of crypto backends / values is actually used
    pub _crypto_backends: BTreeMap<String, String>,
    pub _crypto_backend_values: BTreeMap<String, Vec<Option<String>>>,

    pub _blockchain_backend: String,
    pub _blockchain_backend_values: Vec<String>,
    pub _blockchain_node_uri: Option<String>,
    pub _blockchain_pki_address: Vec<String>,
    pub _blockchain_crypto_lib_addresses: String,
    pub _blockchain_default_account: Option<String>,

    pub _indentation: String,
    pub _libsnark_check_verify_locally_during_proof_generation: bool,

    pub _opt_solc_optimizer_runs: i32,
    pub _opt_hash_threshold: i32,
    pub _opt_eval_constexpr_in_circuit: bool,
    pub _opt_cache_circuit_inputs: bool,
    pub _opt_cache_circuit_outputs: bool,

    pub _data_dir: String,
    pub _log_dir: String,
    pub _use_circuit_cache_during_testing_with_encryption: bool,
    pub _verbosity: i32,

    pub _disable_verification: bool,
}
impl Default for UserConfigBase {
    fn default() -> Self {
        Self::new()
    }
}
impl UserConfigBase {
    pub fn new() -> Self {
        // self._appdirs = AppDirs("zkay", appauthor=False, version=None, roaming=True)
        let _appdirs: AppInfo = AppInfo {
            name: "zkay",
            author: "arron",
        };
        // User configuration
        // Each attribute must have a type hint and a docstring for correct help strings in the commandline interface.
        // If "Available Options: [...]" is specified, the options are used for autocomplete suggestions.
        // These only exist for the auto-generated help strings in __main__ and should not be used directly
        let _main_crypto_backend = String::from("ecdh-chaskey");
        let _main_crypto_backend_values = lc_string_vec![
            null,
            "dummy",
            "dummy-hom",
            "rsa-pkcs1.5",
            "rsa-oaep",
            "ecdh-aes",
            "ecdh-chaskey",
            "paillier",
            "elgamal"
        ];
        let _addhom_crypto_backend = String::from("elgamal");
        let _addhom_crypto_backend_values =
            lc_string_vec![null, "dummy-hom", "paillier", "elgamal"];

        // Global defaults
        Self {
            _proving_scheme: String::from("groth16"),
            _proving_scheme_values: lc_vec_s!["groth16", "gm17"],

            _snark_backend: String::from("jsnark"),
            _snark_backend_values: lc_vec_s!["jsnark"],

            // These only exist for the auto-generated help strings in __main__ and should not be used directly

            // This map of crypto backends / values is actually used
            _crypto_backends: BTreeMap::from([
                (
                    String::from("NON_HOMOMORPHIC"),
                    _main_crypto_backend.clone(),
                ),
                (String::from("ADDITIVE"), _addhom_crypto_backend.clone()),
            ]),
            _crypto_backend_values: BTreeMap::from([
                (
                    String::from("NON_HOMOMORPHIC"),
                    _main_crypto_backend_values.clone(),
                ),
                (
                    String::from("ADDITIVE"),
                    _addhom_crypto_backend_values.clone(),
                ),
            ]),
            _main_crypto_backend,
            _main_crypto_backend_values,
            _addhom_crypto_backend,
            _addhom_crypto_backend_values,

            _blockchain_backend: String::from("w3-eth-tester"),
            _blockchain_backend_values: lc_vec_s![
                "w3-eth-tester",
                "w3-ganache",
                "w3-ipc",
                "w3-websocket",
                "w3-http",
                "w3-custom"
            ],
            _blockchain_node_uri: Some(String::from("http://localhost:7545")),
            _blockchain_pki_address: Vec::new(),
            _blockchain_crypto_lib_addresses: String::new(),
            _blockchain_default_account: Some(String::from("0")),
            _indentation: " ".repeat(4),
            _libsnark_check_verify_locally_during_proof_generation: false,
            _opt_solc_optimizer_runs: 50,
            _opt_hash_threshold: 1,
            _opt_eval_constexpr_in_circuit: true,
            _opt_cache_circuit_inputs: true,
            _opt_cache_circuit_outputs: true,
            _data_dir: get_app_dir(AppDataType::UserData, &_appdirs, "data")
                .expect("")
                .to_str()
                .expect("")
                .to_string(),
            _log_dir: get_app_dir(AppDataType::UserData, &_appdirs, "log")
                .expect("")
                .to_str()
                .expect("")
                .to_string(),
            _use_circuit_cache_during_testing_with_encryption: true,
            _verbosity: 1,
            _disable_verification: false,
            _appdirs,
        }
    }
}

pub trait UserConfig {
    fn user_config_base_ref(&self) -> &UserConfigBase;
    fn user_config_base_mut(&mut self) -> &mut UserConfigBase;
    fn appdirs(&self) -> &AppInfo {
        &self.user_config_base_ref()._appdirs
    }
    fn proving_scheme(&self) -> String {
        // NIZK proving scheme to use.
        // Available Options: [gm17]

        self.user_config_base_ref()._proving_scheme.clone()
    }
    // @proving_scheme.setter
    fn set_proving_scheme(&mut self, val: String) {
        _check_is_one_of(&val, &self.user_config_base_ref()._proving_scheme_values);
        self.user_config_base_mut()._proving_scheme = val;
    }

    fn snark_backend(&self) -> String {
        // Snark backend to use.

        // Available Options: [jsnark]

        self.user_config_base_ref()._snark_backend.clone()
    }

    // @snark_backend.setter
    fn set_snark_backend(&mut self, val: String) {
        _check_is_one_of(&val, &self.user_config_base_ref()._snark_backend_values);
        self.user_config_base_mut()._snark_backend = val;
    }

    fn main_crypto_backend(&self) -> String {
        // Main encryption backend to use.
        // Available Options: [dummy, dummy-hom, rsa-pkcs1.5, rsa-oaep, ecdh-aes, ecdh-chaskey, paillier]
        self.get_crypto_backend(&String::from("NON_HOMOMORPHIC"))
            .unwrap()
    }

    // @main_crypto_backend.setter
    fn set_main_crypto_backend(&mut self, val: String) {
        self.set_crypto_backend(&String::from("NON_HOMOMORPHIC"), val);
    }

    fn addhom_crypto_backend(&self) -> String {
        // Additively homomorphic encryption backend to use.
        // Available Options: [dummy-hom, paillier, elgamal]

        self.user_config_base_ref()._crypto_backends[&String::from("ADDITIVE")].clone()
    }

    // @addhom_crypto_backend.setter
    fn set_addhom_crypto_backend(&mut self, val: String) {
        self.set_crypto_backend(&String::from("ADDITIVE"), val);
    }

    fn get_crypto_backend(&self, hom: &String) -> Option<String> {
        self.user_config_base_ref()
            ._crypto_backends
            .get(hom)
            .cloned()
    }

    fn crypto_backend(&self) -> String {
        "ecdh-chaskey".to_owned()
    }

    fn set_crypto_backend(&mut self, hom: &String, val: String) {
        _check_is_one_of(
            &val,
            &self.user_config_base_ref()._crypto_backend_values[hom]
                .iter()
                .filter_map(|s| if s.is_some() { s.clone() } else { None })
                .collect(),
        );
        self.user_config_base_mut()
            ._crypto_backends
            .insert(hom.clone(), val);
    }

    fn get_crypto_params(&self, hom: &String) -> String {
        let backend_name = self.get_crypto_backend(hom);
        assert!(
            backend_name.is_some(),
            "No crypto backend set for homomorphism {:?}",
            hom
        );
        // raise ValueError(f"No crypto backend set for homomorphism {hom.name}");
        backend_name.unwrap()
    }

    fn all_crypto_params(&self) -> Vec<String> {
        // let crypto_backends: Vec<_> =
        [String::from("NON_HOMOMORPHIC"), String::from("ADDITIVE")]
            .iter()
            .filter_map(|hom| self.get_crypto_backend(hom))
            .collect()
        // crypto_backends
        // .iter()
        // .filter_map(|backend| {
        //     backend
        //         .as_ref()
        //         .map(|backend| CryptoParams::new(backend.clone()))
        // })
        // .collect()
    }

    fn blockchain_backend(&self) -> String {
        // Backend to use when interacting with the blockchain.
        // Running unit tests is only supported with w3-eth-tester and w3-ganache at the moment (because they need pre-funded dummy accounts).
        // See https://web3py.readthedocs.io/en/stable/providers.html for more information.
        // Available Options: [w3-eth-tester, w3-ganache, w3-ipc, w3-websocket, w3-http, w3-custom]

        self.user_config_base_ref()._blockchain_backend.clone()
    }

    // @blockchain_backend.setter
    fn set_blockchain_backend(&mut self, val: String) {
        _check_is_one_of(
            &val,
            &self.user_config_base_ref()._blockchain_backend_values,
        );
        self.user_config_base_mut()._blockchain_backend = val;
    }

    fn blockchain_node_uri(&self) -> Option<String> {
        //Union[Any, String, None]

        // Backend specific location of the ethereum node
        // w3-eth-tester : unused
        // w3-ganache    : url
        // w3-ipc        : path to ipc socket file
        // w3-websocket  : web socket uri
        // w3-http       : url
        // w3-custom     : web3 instance, must not be None

        self.user_config_base_ref()._blockchain_node_uri.clone()
    }

    // @blockchain_node_uri.setter
    fn set_blockchain_node_uri(&mut self, val: Option<String>) {
        //Union[Any, String, None]
        self.user_config_base_mut()._blockchain_node_uri = val;
    }

    fn blockchain_pki_address(&self) -> Vec<String> {
        // Address of the deployed pki contract.
        // Must be specified for backends other than w3-eth-tester.
        // This library can be deployed using ``zkay deploy-pki``.

        self.user_config_base_ref()._blockchain_pki_address.clone()
    }

    // @blockchain_pki_address.setter
    fn set_blockchain_pki_address(&mut self, val: Vec<String>) {
        //_type_check(val, String);
        self.user_config_base_mut()._blockchain_pki_address = val;
    }

    fn blockchain_crypto_lib_addresses(&self) -> String {
        // Comma separated list of the addresses of the deployed crypto library contracts required for the current proving_scheme.
        // e.g. "0xAb31...,0xec32C..."

        // Must be specified for backends other than w3-eth-tester.
        // The libraries can be deployed using ``zkay deploy-crypto-libs``.
        // The addresses in the list must appear in the same order as the corresponding
        // libraries were deployed by that command.

        self.user_config_base_ref()
            ._blockchain_crypto_lib_addresses
            .clone()
    }

    // @blockchain_crypto_lib_addresses.setter
    fn set_blockchain_crypto_lib_addresses(&mut self, val: String) {
        //_type_check(val, String);
        self.user_config_base_mut()._blockchain_crypto_lib_addresses = val;
    }

    fn blockchain_default_account(&self) -> Option<String> {
        //Union[i32, String, None]

        // Address of the wallet which should be made available under the name "me" in contract.py.

        // If None -> must always specify a sender, empty blockchain_pki_address is invalid
        // If i32 -> use eth.accounts[i32]
        // If String -> use address String

        self.user_config_base_ref()
            ._blockchain_default_account
            .clone()
    }

    // @blockchain_default_account.setter
    fn set_blockchain_default_account(&mut self, val: String) {
        // //_type_check(val, (i32, String, None))
        self.user_config_base_mut()._blockchain_default_account = Some(val);
    }

    fn indentation(&self) -> String {
        // Specifies the identation which should be used for the generated code output.
        self.user_config_base_ref()._indentation.clone()
    }

    // @indentation.setter
    fn set_indentation(&mut self, val: String) {
        // //_type_check(val, String)
        self.user_config_base_mut()._indentation = val;
    }

    // If true, the libsnark interface verifies locally whether the proof can be verified during proof generation.
    fn libsnark_check_verify_locally_during_proof_generation(&self) -> bool {
        self.user_config_base_ref()
            ._libsnark_check_verify_locally_during_proof_generation
    }

    // @libsnark_check_verify_locally_during_proof_generation.setter
    fn set_libsnark_check_verify_locally_during_proof_generation(&mut self, val: bool) {
        //_type_check(val, bool)
        self.user_config_base_mut()
            ._libsnark_check_verify_locally_during_proof_generation = val;
    }

    fn opt_solc_optimizer_runs(&self) -> i32 {
        // SOLC: optimize for how many times to run the code
        self.user_config_base_ref()._opt_solc_optimizer_runs
    }

    // @opt_solc_optimizer_runs.setter
    fn set_opt_solc_optimizer_runs(&mut self, val: i32) {
        //_type_check(val, i32)
        self.user_config_base_mut()._opt_solc_optimizer_runs = val
    }

    fn opt_hash_threshold(&self) -> i32 {
        // If there are more than this many public circuit inputs (in uints), the hashing optimization will be enabled.

        // This means that only the hash of all public inputs will be passed as public input,
        // public inputs are passed as private circuit inputs and the circuit verifies
        // that the hash matches to ensure correctness.

        // When hashing is enabled -> cheaper on-chain costs for verification (O(1) in #public args instead of O(n)),
        // but much higher off-chain costs (key and proof generation time, memory consumption).

        self.user_config_base_ref()._opt_hash_threshold
    }

    // @opt_hash_threshold.setter
    fn set_opt_hash_threshold(&mut self, val: i32) {
        //_type_check(val, i32)
        self.user_config_base_mut()._opt_hash_threshold = val;
    }

    fn opt_eval_constexpr_in_circuit(&self) -> bool {
        // If true, literal expressions are folded and the result is baked into the circuit as a constant
        // (as opposed to being evaluated outside the circuit and the result being moved in as an additional circuit input)

        self.user_config_base_ref()._opt_eval_constexpr_in_circuit
    }

    // @opt_eval_constexpr_in_circuit.setter
    fn set_opt_eval_constexpr_in_circuit(&mut self, val: bool) {
        //_type_check(val, bool)
        self.user_config_base_mut()._opt_eval_constexpr_in_circuit = val;
    }

    fn opt_cache_circuit_inputs(&self) -> bool {
        // If true, identifier circuit inputs will be cached
        // (i.e. if an identifier is referenced multiple times within a private expression,
        // or multiple times in different private expressions without being publicly written to in between,
        // then the identifier will only be added to the circuit inputs once and all private
        // uses will share the same input variable.

        self.user_config_base_ref()._opt_cache_circuit_inputs
    }

    // @opt_cache_circuit_inputs.setter
    fn set_opt_cache_circuit_inputs(&mut self, val: bool) {
        //_type_check(val, bool)
        self.user_config_base_mut()._opt_cache_circuit_inputs = val;
    }

    fn opt_cache_circuit_outputs(&self) -> bool {
        // Normally, the value cached in the circuit for a particular identifier must be invalidated whenever the
        // identifier is assigned to in public code.

        // If this optimization is enabled, assignments where the lhs is an Identifier and the rhs is a private expression
        // will update the cached value stored in the circuit instead of invalidating it.
        // (since updated value == private expression result, the corresponding plaintext value is already
        // available in the circuit)

        self.user_config_base_ref()._opt_cache_circuit_outputs
    }

    // @opt_cache_circuit_outputs.setter
    fn set_opt_cache_circuit_outputs(&mut self, val: bool) {
        //_type_check(val, bool)
        self.user_config_base_mut()._opt_cache_circuit_outputs = val;
    }

    fn data_dir(&self) -> String {
        // Path to directory where to store user data (e.g. generated encryption keys).
        self.user_config_base_ref()._data_dir.clone()
    }

    // @data_dir.setter
    fn set_data_dir(&mut self, val: String) {
        //_type_check(val, String)
        use std::path::Path;
        if !Path::new(&val).exists() {
            let _ = std::fs::create_dir_all(val.clone());
        }
        self.user_config_base_mut()._data_dir = val;
    }

    fn log_dir(&self) -> String {
        // Path to default log directory.
        self.user_config_base_ref()._log_dir.clone()
    }

    // @log_dir.setter
    fn set_log_dir(&mut self, val: String) {
        //_type_check(val, String)
        use std::path::Path;
        if !Path::new(&val).exists() {
            let _ = std::fs::create_dir_all(val.clone());
        }
        self.user_config_base_mut()._log_dir = val;
    }

    fn use_circuit_cache_during_testing_with_encryption(&self) -> bool {
        // If true, snark keys for the test cases are cached
        // (i.e. they are not regenerated on every run unless the circuit was modified)

        self.user_config_base_ref()
            ._use_circuit_cache_during_testing_with_encryption
    }

    // @use_circuit_cache_during_testing_with_encryption.setter
    fn set_use_circuit_cache_during_testing_with_encryption(&mut self, val: bool) {
        //_type_check(val, bool)
        self.user_config_base_mut()
            ._use_circuit_cache_during_testing_with_encryption = val;
    }

    fn verbosity(&self) -> i32 {
        // If 0, no output
        // If 1, normal output
        // If 2, verbose output

        // This includes for example snark key- and proof generation output and
        // information about intermediate transaction simulation steps.

        self.user_config_base_ref()._verbosity
    }

    // @verbosity.setter
    fn set_verbosity(&mut self, val: i32) {
        //_type_check(val, i32)
        self.user_config_base_mut()._verbosity = val;
    }

    fn disable_verification(&self) -> bool {
        // If true, proof verification in output contract is disabled (only for benchmarks)
        self.user_config_base_ref()._disable_verification
    }

    // @disable_verification.setter
    fn set_disable_verification(&mut self, val: bool) {
        //_type_check(val, bool)
        self.user_config_base_mut()._disable_verification = val;
    }
}
