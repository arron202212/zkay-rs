#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// """
// This module defines the Runtime API, an abstraction layer which is used by the generated PythonOffchainSimulator classes.

// It provides high level functions for

// * blockchain interaction (deployment, state variable retrieval, transaction issuing, ...),
// * cryptographic operations (encryption, decryption, key generation) and key management (local keystore)
// * NIZK-proof generation
// """

// import os
// from abc import ABCMeta, abstractmethod
// from builtins import type
// from typing import Tuple, List, Optional, Union, Any, Dict, Collection
// use ast_builder::process_ast::get_verification_contract_names;
use enum_dispatch::enum_dispatch;
use crate::crypto::ecdh_chaskey::EcdhChaskeyCrypto;
use crate::crypto::elgamal::ElgamalCrypto;
use crate::runtime::CryptoClass;
use ark_ff::BigInteger256;
use path_absolutize::Absolutize;
use privacy::library_contracts::BN128_SCALAR_FIELD;
use privacy::library_contracts::BN128_SCALAR_FIELDS;
use proving_scheme::proving_scheme::ProvingScheme;
use rccell::RcCell;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;
use zkay_transaction_crypto_params::params::CryptoParams;
use zkp_u256::{Zero, U256};
// use zkay_frontend::compile_zkay_file;
// use crate::runtime::Runtime;
use crate::types::{
    AddressValue, BlockStruct, CipherValue, KeyPair, MsgStruct, PrivateKeyValue, PublicKeyValue,
    RandomnessValue, TxStruct, Value,
};
use serde_json::{Map, Result,json, Value as JsonValue};
use zkay_config::{
    config::{zk_print_banner, CFG},
    config_user::UserConfig,
    zk_print,
};
use zkay_utils::progress_printer::success_print;
use zkay_utils::timer::time_measure;
use crate::blockchain::web3rs::Web3BlockchainBase;
 use crate::blockchain::web3rs::Web3TesterBlockchain;
use crate::blockchain::web3rs::Web3HttpGanacheBlockchain;
 use crate::runtime::BlockchainClass;
// class IntegrityError(Exception){
//     """Exception which is raised when any part of a deployed zkay contract does not match the local contract file."""
//     pass

// class BlockChainError(Exception){
//     """
//     Exception which is raised when a blockchain interaction fails for any reason.
//     """
//     pass

// class TransactionFailedException(BlockChainError){
//     """Exception which is raised when a transaction fails."""
//     pass

// class ProofGenerationError(Exception){
//     """Exception which is raised when proof generation fails."""
//     pass

// class ZkayBlockchainInterface(metaclass=ABCMeta){
#[enum_dispatch]
pub trait ZkayBlockchainInterface<P:ZkayProverInterface> {
    //     """
    //     API to interact with the blockchain.

    //     It automatically ensures that all needed library contracts are accessible.
    //     For most backends (except eth-tester), the necessary library contracts must be deployed in advance and \
    //     cfg.blockchain_pki_address or cfg.blockchain_crypto_lib_address must be specified.

    //     For safety reasons, zkay always verifies the integrity of remote contracts by comparing the evm bytecode \
    //     from the blockchain with the output obtained via local compilation of the corresponding source files.

    //     (Zkay ensures reproducibility via hard-coded solc versions/settings for global library contracts and by \
    //     using the version/settings stored in the manifest file for the main and verification contracts)

    //     See documentation of :py:meth:`connect` for more information.
    //     """

    //     fn __init__(self){
    //         self._pki_contract = None
    //         self._lib_addresses = None
    fn _pki_contract(&self) -> Option<RcCell<BTreeMap<String, JsonValue>>> {
        None
    }
    fn pki_contract(&mut self, crypto_backend: &str) -> JsonValue {
        if self._pki_contract().is_none() {
            self._connect_libraries();
        }
        self._pki_contract()
            .unwrap()
            .borrow()
            .get(crypto_backend)
            .unwrap()
            .clone()
    }

    //     @property
    fn lib_addresses(&mut self) -> BTreeMap<String, JsonValue>;
    fn lib_addresses_mut(&mut self) -> &mut BTreeMap<String, JsonValue>;
    // if self._lib_addresses is None:
    //     self._connect_libraries()
    // return self._lib_addresses

    //     @abstractmethod
    fn _connect_libraries(&mut self);
    //         pass

    //     # PUBLIC API

    //     @property
    //         """Return wallet address to use as from address when no address is explicitly specified."""
    fn default_address(&self) -> Option<AddressValue>;
    //  {
    //     //  addr = self._default_address();
    //     // return None if addr is None else AddressValue(addr)
    //     None
    // }
    //         """
    //         Return addresses of pre-funded accounts (only implemented for w3-eth-tester and w3-ganache, for debugging).

    //         :param count: how many accounts
    //         :raise NotImplementedError: if the backend does not support dummy accounts
    //         :raise ValueError: if not enough unused pre-funded accounts are available
    //         :return: the account addresses (either a single value if count = 1 or a tuple otherwise)
    //         """
    fn create_test_accounts(&mut self, _count: i32) -> Vec<String>;//
    //  {
    //     //         # may not be supported by all backends
    //     //         raise NotImplementedError("Current blockchain backend does not support creating pre-funded test accounts.")
    //     unimplemented!(
    //         "Current blockchain backend does not support creating pre-funded test accounts."
    //     )
    // }
    //     @abstractmethod
    //         """
    //         Return message, block and transaction objects, populated according to the current chain state.

    //         :param sender: transaction sender address
    //         :param wei_amount: transaction value (if payable)
    //         :return: populated builtin objects
    //         """
    fn get_special_variables(
        &self,
        sender: &String,
        wei_amount: i32,
    ) -> (MsgStruct, BlockStruct, TxStruct);

    //         pass
    // """Return the balance of the wallet with the designated address (in wei)."""
    fn get_balance(&self, _address: AddressValue) -> i32 {
        //  self._get_balance(address.val)
        0
    }

    //         """
    //         Request the public key for the designated address from the PKI contract.

    //         :param address: Address for which to request public key
    //         :raise BlockChainError: if request fails
    //         :return: the public key
    //         """
    fn req_public_key(
        &mut self,
        address: &str,
        crypto_params: &CryptoParams,
    ) -> eyre::Result<Value<u8, PublicKeyValue>> {
        //         assert isinstance(address, AddressValue)
        zk_print!(r#"Requesting public key for address "{address}""#);
        self._req_public_key(&address.to_owned(), crypto_params)
    }

    //         """
    //         Announce a public key to the PKI

    //         **WARNING: THIS ISSUES A CRYPTO CURRENCY TRANSACTION (GAS COST)**

    //         :param sender: public key owner, its eth private key must be hosted in the eth node to which the backend connects.
    //         :param pk: the public key to announce
    //         :raise BlockChainError: if there is an error in the backend
    //         :raise TransactionFailedException: if the announcement transaction failed
    //         :return: backend-specific transaction receipt
    //         """
    fn announce_public_key(
        &mut self,
        sender: &str,
        pk: &String,
        crypto_params: &CryptoParams,
    ) {
        //         assert isinstance(sender, AddressValue)
        //         assert isinstance(pk, PublicKeyValue)
        zk_print!(r#"Announcing public key "{pk}" for address "{sender}""#);
        self._announce_public_key(sender, pk, crypto_params)
    }

    //         """
    //         Request the contract state variable value name[indices[0]][indices[1]][...] from the chain.

    //         :param contract_handle: contract from which to read state
    //         :param name: name of the state variable
    //         :param indices: if the request is for an (nested) array/map index value, the values of all index keys.
    //         :raise BlockChainError: if request fails
    //         :return: The value
    //         """
    fn req_state_var(
        &self,
        contract_handle: &JsonValue,
        name: &str,
        indices: &String,
    ) -> String {
        //bool, int, str, bytes
        //         assert contract_handle is not None
        zk_print!(r#"Requesting state variable "{name}""#);
        let val = self._req_state_var(
            contract_handle,
            name,
            indices,
        );
        zk_print!(r#"Got value {val} for state variable "{name}""#);
        val
    }
    //         """
    //         Call the specified pure/view function in the given contract with the provided arguments.

    //         :param contract_handle: the contract in which the function resides
    //         :param sender: sender address, its eth private key must be hosted in the eth node to which the backend connects.
    //         :param name: name of the function to call
    //         :param args: argument values
    //         :raise BlockChainError: if request fails
    //         :return: function return value (single value if one return value, list if multiple return values)
    //         """
    fn call(
        &self,
        contract_handle: JsonValue,
        sender: &String,
        name: &str,
        args: String,
    ) -> String {
        //-> Union[bool, int, str, bytes, List]:
        //         assert contract_handle is not None
        zk_print!(
            "Calling contract function {name}{}",
           args
        );
        let val = self._call(contract_handle, sender, name, &args);
        zk_print!("Got return value {val}");
        val
    }

    //         """
    //         Issue a transaction for the specified function in the given contract with the provided arguments

    //         **WARNING: THIS ISSUES A CRYPTO CURRENCY TRANSACTION (GAS COST)**

    //         :param contract_handle: the contract in which the function resides
    //         :param sender: sender address, its eth private key must be hosted in the eth node to which the backend connects.
    //         :param function: name of the function
    //         :param actual_args: the function argument values
    //         :param should_encrypt: a list which contains a boolean value for each argument, which should be true if the corresponding
    //                                parameter expects an encrypted/private value (this is only used for a last sanity-check)
    //         :param wei_amount: how much money to send along with the transaction (only for payable functions)
    //         :raise BlockChainError: if there is an error in the backend
    //         :raise TransactionFailedException: if the transaction failed
    //         :return: backend-specific transaction receipt
    //         """
    fn transact(
        &self,
        contract_handle: &JsonValue,
        sender: &str,
        function: &str,
        actual_args: String,
        should_encrypt: Vec<bool>,
        wei_amount: Option<i32>,
    ) {
        //         assert contract_handle is not None
        self.__check_args(actual_args.clone(), should_encrypt);
        zk_print!(r#"Issuing transaction for function "{function}" from account "{sender}""#);
        zk_print!("{}", actual_args);
        let ret = self._transact(
            &contract_handle,
            sender,
            function,
            &actual_args,
            wei_amount,
        );
        zk_print!("");
        ret
    }
    // Issue a deployment transaction which constructs the specified contract with the provided constructor arguments on the chain.

    // **WARNING: THIS ISSUES A CRYPTO CURRENCY TRANSACTION (GAS COST)**

    // :param project_dir: directory where the zkay file, manifest and snark keys reside
    // :param sender: creator address, its eth private key must be hosted in the eth node to which the backend connects.
    // :param contract: name of the contract to instantiate
    // :param actual_args: the constructor argument values
    // :param should_encrypt: a list which contains a boolean value for each argument, which should be true if the corresponding
    //                        parameter expects an encrypted/private value (this is only used for a last sanity-check)
    // :param wei_amount: how much money to send along with the constructor transaction (only for payable constructors)
    // :raise BlockChainError: if there is an error in the backend
    // :raise TransactionFailedException: if the deployment transaction failed
    // :return: handle for the newly created contract
    fn deploy(
        &mut self,
        project_dir: &str,
        sender: &str,
        contract: &str,
        actual_args: String,
        should_encrypt: Vec<bool>,
        wei_amount: Option<i32>,
    ) -> eyre::Result<JsonValue> {
        if !self.is_debug_backend() && CFG.lock().unwrap().crypto_backend() == "dummy" {
            eyre::bail!("SECURITY ERROR: Dummy encryption can only be used with debug blockchain backends (w3-eth-tester or w3-ganache).")
        }
        zk_print_banner(format!("Deploy {contract}"));

        self.__check_args(actual_args.clone(), should_encrypt);
        zk_print!(
            "Deploying contract {contract}{:?}",
            actual_args
        ); //
        let _ret = self._deploy(project_dir, sender, contract, actual_args, wei_amount);
        zk_print!("");
        Ok(json!({}))
    }

    //         """
    //         Create a handle which can be used to interact with an existing contract on the chain after verifying its integrity.

    //         Project dir must contain a .zkay file, a manifest.json file as well as a \
    //         subdirectory *verification_contract_name*\\ _out containing "proving.key" and "verification.key" for each verification contract.
    //         These files are referred to as "local" files in the following explanation.

    //         If this function succeeds, it is guaranteed, that:

    //         * the remote main contract at contract_address, matches the solidity contract obtained by running zkay on the local zkay file
    //           using the configuration stored in the local manifest
    //         * the pki contract referenced in the remote main contract matches the correct zkay pki contract
    //         * the verification contracts referenced in the remote solidity contract were generated by running zkay on a zkay file
    //           equivalent to local zkay file, with zk-snark keys which match the local keys.
    //         * the library contract referenced in the verification contracts matches the correct zkay library contract

    //         This reduces the required trust to the zk-snark setup phase (i.e. you must trust that prover/verification keys
    //         were generated for the correct circuit), since you can inspect the source code of the local zkay file and check it
    //         for malicious behavior yourself (and the zkay implementation, which performs the transformation, is open source as well).

    //         Example Scenarios:

    //         a) the remote zkay contract is benign (generated by and deployed using zkay){
    //            -> you will only be able to connect if the local files are equivalent -> correctness is guaranteed
    //         b) the remote zkay contract was tampered with (any of the .sol files was modified was modified before deployment)
    //            -> connection will fail, because local zkay compilation will not produce matching evm bytecode
    //         c) the prover/verification keys were tampered with (they were generated for a different circuit than the one produced by zkay)

    //            * local keys are genuine -> connection will be refused because the keys don"t match what is baked into the remote verification contract
    //            * same tampered keys locally -> NO GUARANTEES, since the trust assumption is violated

    //         :param project_dir: directory where the zkay file, manifest and snark keys reside
    //         :param contract: name of the contract to connect to
    //         :param contract_address: address of the deployed contract
    //         :param user_address: account which connects to the contract
    //         :raise IntegrityError: if the integrity check fails (mismatch between local code and remote contract)
    //         :return: contract handle for the specified contract
    //         """
    fn connect<PS: ProvingScheme>(
        &mut self,
        project_dir: &str,
        contract: &str,
        contract_address: JsonValue,
        user_address: String,
        compile_zkay_file: fn(
            input_file_path: &str,
            output_dir: &str,
            import_keys: bool,
        ) -> anyhow::Result<String>,
        get_verification_contract_names: fn(code_or_ast: String) -> Vec<String>,
    ) -> eyre::Result<JsonValue> {
        assert!( self.is_debug_backend() || CFG.lock().unwrap().crypto_backend() != "dummy","SECURITY ERROR: Dummy encryption can only be used with debug blockchain backends (w3-eth-tester or w3-ganache).");

        zk_print_banner(format!("Connect to {contract}@{contract_address}"));

        // If not already done, compile zkay file to generate main and verification contracts (but don"t generate new prover/verification keys and manifest)
        let zk_file = PathBuf::from(project_dir).join("contract.zkay");
        assert!(
            zk_file.try_exists().is_ok(),
            "No zkay contract found in specified directory"
        );
        let mut verifier_names = vec![];
        if PathBuf::from(project_dir)
            .join("contract.sol")
            .try_exists()
            .is_ok()
        {
            verifier_names =
                get_verification_contract_names(std::fs::read_to_string(zk_file).unwrap());
        } else {
            let _ = compile_zkay_file(&zk_file.to_string_lossy().to_string(), project_dir, true);
        }

        zk_print!("Connecting to contract {contract}@{contract_address}");
        let contract_on_chain = self._connect(project_dir, contract, contract_address.clone());

        let mut pki_verifier_addresses = BTreeMap::new();

        // # Check integrity of all pki contracts
        let mut _pki_contract = BTreeMap::new();
        for crypto_params in CFG.lock().unwrap().all_crypto_params() {
            let contract_name = CFG.lock().unwrap().get_pki_contract_name(&crypto_params);
            let mut pki_address = self._req_state_var::<JsonValue>(
                contract_on_chain.as_ref().unwrap(),
                &format!("{contract_name}_inst"),
                &String::default(),
            );
            pki_verifier_addresses.insert(contract_name.clone(), pki_address.clone());
            // with cfg.library_compilation_environment(){
            let contract = self._verify_contract_integrity(
                &pki_address,
                &PathBuf::from(project_dir).join(format!("{contract_name}.sol")),
                None,
                None,
                false,
                None,
            );
            _pki_contract.insert(
                CryptoParams::new(crypto_params).crypto_name,
                contract,
            );
            // }
        }
        *self._pki_contract().as_ref().unwrap().borrow_mut() = _pki_contract;
        // # Check verifier contract and library integrity
        if !verifier_names.is_empty() {
            let some_vname = verifier_names[0].clone();

            let libraries: BTreeMap<_, _> = CFG
                .lock()
                .unwrap()
                .external_crypto_lib_names()
                .iter()
                .map(|lib_name| {
                    (
                        lib_name.clone(),
                        PathBuf::from(project_dir).join(PS::verify_libs_contract_filename()),
                    )
                })
                .collect();
            let some_vcontract = self._req_state_var(
                contract_on_chain.as_ref().unwrap(),
                &format!("{some_vname}_inst"),
                &String::default(),
            );
            let mut libs = self._verify_library_integrity(
                libraries,
                &some_vcontract,
                &PathBuf::from(project_dir).join(format!("{some_vname}.sol")),
            );
            self.lib_addresses_mut().append(&mut libs);

            for verifier in verifier_names {
                let v_address = self._req_state_var::<JsonValue>(
                    contract_on_chain.as_ref().unwrap(),
                    &format!("{verifier}_inst"),
                    &String::default(),
                );
                pki_verifier_addresses.insert(verifier.clone(), v_address.clone());
                let vcontract = self._verify_contract_integrity(
                    &v_address,
                    &PathBuf::from(project_dir).join(format!("{verifier}.sol")),
                    Some(libs.clone()),
                    None,
                    false,
                    None,
                );

                // # Verify prover key
                let expected_hash = self._req_state_var::<String>(
                    &vcontract,
                    &CFG.lock().unwrap().prover_key_hash_name(),
                    &String::default(),
                );
                // from zkay.transaction.runtime import Runtime
                let actual_hash = self.prover().get_prover_key_hash(
                    &PathBuf::from(project_dir)
                        .join(
                            CFG.lock()
                                .unwrap()
                                .get_circuit_output_dir_name(verifier.clone()),
                        )
                        .to_string_lossy()
                        .to_string(),
                );
                assert!(
                    expected_hash.into_bytes() == actual_hash,
                    r#"Prover key hash in deployed verification contract does not match local prover key file for "{verifier}""#
                );
            }
        }
        // # Check zkay contract integrity
        self._verify_zkay_contract_integrity(
            &contract_address, //contract_on_chain["address"]
            project_dir,
            &pki_verifier_addresses,
        );

        // with success_print(){
        zk_print!("OK: Bytecode on blockchain matches local zkay contract");
        zk_print!("Connection from account 0x{user_address} established\n");

        contract_on_chain
    }
    fn prover(&self) -> &P;
    //     @abstractmethod
    //         """
    //         Compile and deploy the specified solidity contract.

    //         :param sol_filename: solidity file
    //         :param contract_name: specifies which contract from the .sol file to compile (None -> take first contract in file)
    //         :param sender: account address from which to issue the deployment transaction (keys must be hosted in node)
    //         :raise BlockChainError: if there is an error in the backend
    //         :raise TransactionFailedException: if the deployment transaction failed
    //         :return: Address of the deployed contract
    //         """
    fn deploy_solidity_contract<T: Clone + Default, V: Clone + Default>(
        &self,
        sol_filename: &str,
        contract_name: Option<String>,
        sender: &str,
    ) -> eyre::Result<JsonValue>;
    //         pass

    //     @classmethod
    fn is_debug_backend(&self) -> bool {
        false
    }

    //     # INTERNAL FUNCTIONALITY
    //         """
    //         Check if the bytecode of the contract at address matches the bytecode obtained by locally compiling sol_filename.

    //         :param address: address of the remote contract
    //         :param sol_filename: path to the local contract code file
    //         :param libraries: library dict which should be passed during compilation (for linking)
    //         :param contract_name: contract name, if not specified, the first contract in the file is used
    //         :param is_library: set to true if this a library instead of a contract
    //         :raise IntegrityError: if there is a mismatch
    //         :return: a contract handle for the remote contract
    //         """
    //     @abstractmethod
    fn _verify_contract_integrity(
        &self,
        address: &JsonValue,
        sol_filename: &PathBuf,
        libraries: Option<BTreeMap<String, JsonValue>>,
        contract_name: Option<String>,
        is_library: bool,
        cwd: Option<String>,
    ) -> JsonValue ;

    //     @abstractmethod
    //         """
    //         Check if the libraries linked in contract_with_libs match library_sol and return the addresses of the library contracts.

    //         :param libraries: = List of (library name, library.sol) tuples
    //         :raise IntegrityError: if there is a mismatch
    //         :return: Dict of library name -> address for all libs from libraries which occurred in contract@contract_with_libs_addr
    //         """
    fn _verify_library_integrity(
        &self,
        libraries: BTreeMap<String, PathBuf>,
        contract_with_libs_addr: &String,
        sol_with_libs_filename: &PathBuf,
    ) -> BTreeMap<String, JsonValue>;
    //         pass

    //     @abstractmethod
    //         """
    //         Check if the zkay main contract at address matches the local file

    //         :param address: address of the remote main contract
    //         :param project_dir: path to the zkay contract directory
    //         :param pki_verifier_addresses: dictionary which maps pki and verification contract names to the corresponding remote addresses
    //         :raise IntegrityError: if there is a mismatch
    //         """
    fn _verify_zkay_contract_integrity(
        &self,
        address: &JsonValue,
        project_dir: &str,
        pki_verifier_addresses: &BTreeMap<String, JsonValue>,
    );
    //         pass

    //     @abstractmethod
    fn _default_address(&self) -> Option<String>;
    //         pass

    //     @abstractmethod
    fn _get_balance(&self, address: &str) -> i32;
    //         pass

    //     @abstractmethod
    fn _deploy_dependencies(
        &mut self,
        sender: &str,
        project_dir: &str,
        verifier_names: Vec<String>,
    ) -> BTreeMap<String, JsonValue>;
    // pass

    //     @abstractmethod

    fn _req_public_key(
        &mut self,
        address: &String,
        crypto_params: &CryptoParams,
    ) -> eyre::Result<Value<u8, PublicKeyValue>>;
    //         pass

    //     @abstractmethod
    fn _announce_public_key(
        &mut self,
        address: &str,
        pk: &String,
        crypto_params: &CryptoParams,
    );
    //         pass

    //     @abstractmethod
    fn _call(
        &self,
        contract_handle: JsonValue,
        sender: &String,
        name: &str,
        args: &String,
    ) -> String;
    //         pass

    //     @abstractmethod
    fn _req_state_var<R:Clone +Default>(
        &self,
        contract_handle: &JsonValue,
        name: &str,
        indices: &String,
    ) -> R;
    //         pass

    //     @abstractmethod
    fn _transact(
        &self,
        contract_handle: &JsonValue,
        sender: &str,
        function: &str,
        actual_args: &String,
        wei_amount: Option<i32>,
    );
    // pass

    //     @abstractmethod
    fn _deploy(
        &mut self,
        project_dir: &str,
        sender: &str,
        contract: &str,
        actual_args: String,
        wei_amount: Option<i32>,
    )->eyre::Result<JsonValue>;
    //         pass

    //     @abstractmethod
    fn _connect(
        &self,
        project_dir: &str,
        contract: &str,
        address: JsonValue,
    ) -> eyre::Result<JsonValue>;
    //         pass

    //     @staticmethod
    fn __check_args(
        &self,
        actual_args: String,
        should_encrypt: Vec<bool>,
    ) {
        assert!(actual_args.len() == should_encrypt.len());
        for (_idx, _arg) in actual_args.chars().enumerate() {
            // assert! (! is_instance(arg, PrivateKeyValue) && ! is_instance(arg, RandomnessValue));
            // assert! (should_encrypt[idx] == is_instance(arg, CipherValue))
        }
    }
}

// class ZkayKeystoreInterface(metaclass=ABCMeta){
//     """API to add and retrieve local key pairs, and to request public keys."""
pub trait ZkayKeystoreInterface<P:ZkayProverInterface,B:ZkayBlockchainInterface<P>> {
    fn conn(&mut self) -> &mut B;
    fn local_key_pairs(&self) -> &BTreeMap<String, KeyPair>;
    fn local_key_pairs_mut(&mut self) -> &mut BTreeMap<String, KeyPair>;
    fn local_pk_store(&self) -> &BTreeMap<String, Value<u8, PublicKeyValue>>;
    fn local_pk_store_mut(
        &mut self,
    ) -> &mut BTreeMap<String, Value<u8, PublicKeyValue>>;
    fn crypto_params(&self) -> &CryptoParams;
    //     fn __init__(&self, conn: ZkayBlockchainInterface, crypto_params: CryptoParams){
    //         self.conn = conn
    //         self.crypto_params = crypto_params
    //         self.local_pk_store: Dict[AddressValue, PublicKeyValue] = {}
    //         self.local_key_pairs: Dict[AddressValue, KeyPair] = {}

    //         """
    //         Import cryptographic keys for address into this keystore and announce the public key to the pki if necessary.

    //         :param address: Address to which the keys belong
    //         :param key_pair: cryptographic keys
    //         :raise TransactionFailedException: if announcement transaction fails
    //         """
    fn add_keypair(&mut self, address: &str, key_pair: KeyPair) {
        self.local_key_pairs_mut()
            .insert(address.to_owned(), key_pair.clone());
        //         # Announce if not yet in pki
        //         try:
        let crypto_params=self.crypto_params().clone();
        if self
            .conn()
            .req_public_key(address,&crypto_params )
            .is_err()
        {
            //         except BlockChainError:
            self.conn()
                .announce_public_key(address, &key_pair.pk,&crypto_params );
        }
    }
    //         """Return true if keys for address are already in the store."""
    fn has_initialized_keys_for(&self, address: &String) -> bool {
        self.local_key_pairs().contains_key(address)
    }

    //         """
    //         Return public key for address.

    //         If the key is cached locally, returned the cached copy, otherwise request from pki contract.

    //         NOTE: At the moment, the name of this function must match the name in the pki contract.

    //         :param address: address to which the public key belongs
    //         :raise BlockChainError: if key request fails
    //         :return: the public key
    //         """
    fn getPk(&mut self, address: &String) -> Vec<u8> {
        // assert isinstance(address, AddressValue)
        zk_print!("Requesting public key for address {address}"); //, verbosity_level=2
        if let Some(pk) = self.local_pk_store().get(address) {
            pk.contents.clone()
        } else {
            let crypto_params=self.crypto_params().clone();
            let pk = self
                .conn()
                .req_public_key(address, &crypto_params)
                .unwrap();
            self.local_pk_store_mut()
                .insert(address.clone(), pk.clone());
            pk.contents.clone()
        }
    }
    //         """
    //         Return secret key for address from the local key store.

    //         Only works for keys which were previously added through add_keypair

    //         :param address: address to which the private key belongs
    //         :raise KeyError: if key not in local store
    //         :return: private key
    //         """
    fn sk(&self, address: &String) -> Vec<u8> {
        self.local_key_pairs()
            .get(address)
            .unwrap()
            .sk
            .clone().into_bytes()
    }
    //         """
    //         Return public key for address from the local key store.

    //         Only works for keys which were previously added through add_keypair

    //         :param address: address to which the public key belongs
    //         :raise KeyError: if key not in local store
    //         :return: public key
    //         """
    fn pk(&self, address: &String) -> Vec<u8> {
        self.local_key_pairs()
            .get(address)
            .unwrap()
            .pk
            .clone().into_bytes()
    }
}
// class ZkayCryptoInterface(metaclass=ABCMeta){
//     """API to generate cryptographic keys and perform encryption/decryption operations."""
#[enum_dispatch]
pub trait ZkayCryptoInterface<P:ZkayProverInterface,B:ZkayBlockchainInterface<P>,K: ZkayKeystoreInterface<P,B>> {
    fn keystore(&self) -> &K;
    fn keystore_mut(&mut self) -> &mut K;
    //     fn __init__(&self, keystore: ZkayKeystoreInterface){
    //         self.keystore = keystore

    //     @property
    //     @abstractmethod
    fn params(&self) -> CryptoParams;

    //         pass

    //         """
    //         Store cryptographic keys for the account with the specified address in the keystore.

    //         If the pre-existing keys are found for this address, they are loaded from the filesystem, \
    //         otherwise new keys are generated.

    //         :param address: the address for which to generate keys
    //         """
    fn generate_or_load_key_pair(&mut self, address: &str) {
        let v = self._generate_or_load_key_pair(&address.to_owned());
        self.keystore_mut().add_keypair(address, v);
    }

    //         """
    //         Encrypt plain for receiver with target_addr.

    //         :param plain: plain text to encrypt
    //         :param my_addr: address of the sender who encrypts
    //         :param target_addr: address of the receiver for whom to encrypt
    //         :return: if symmetric -> (iv_cipher, None), if asymmetric (cipher, randomness which was used to encrypt plain)
    //         """
    fn enc(
        &mut self,
        mut plain: Vec<u8>,
        my_addr: &String,
        target_addr: &String,
    ) -> (
        Value<u8, CipherValue>,
        Option<Value<u8, RandomnessValue>>,
    ) {
        // if isinstance(plain, AddressValue){
        //     plain = int.from_bytes(plain.val, byteorder="big")
        // assert not isinstance(plain, JsonValue), f"Tried to encrypt value of type {type(plain).__name__}"
        // assert isinstance(my_addr, AddressValue) and isinstance(target_addr, AddressValue)
        // assert int(plain) < bn128_scalar_field, f"Integer overflow, plaintext is >= field prime"
        zk_print!(r#"Encrypting value {plain:?} for destination "{target_addr}""#); //, verbosity_level=2

        let sk = self.keystore().sk(my_addr);
        let raw_pk = self.keystore_mut().getPk(target_addr);
        let pk = if self.params().is_symmetric_cipher() {
            assert!(raw_pk.len() == 1);
            raw_pk.clone()
        } else {
            self.deserialize_pk(raw_pk)
        };
        for i in 0..=100 {
            // # Retry until cipher text is not 0
            let (cipher, rnd) = self._enc(plain.clone(), sk.clone(), pk.clone());
            let ciphers = Value::<u8, CipherValue> {
                value: CipherValue,
                contents: cipher,
                params: Some(self.params()),
                crypto_backend: None,
            };
            let rnds = Value::<u8, RandomnessValue> {
                value: RandomnessValue,
                contents: rnd,
                params: Some(self.params()),
                crypto_backend: None,
            };
            let v = Value::<u8, CipherValue> {
                value: CipherValue,
                contents: vec![],
                params: Some(self.params()),
                crypto_backend: None,
            };
            if ciphers != v {
                return (ciphers, Some(rnds));
            }
            assert!(i < 100, "loop end");
        }
        (Value::<u8, CipherValue>::default(), None)
    }
    //         """
    //         Decrypt cipher encrypted for my_addr.

    //         :param cipher: encrypted value
    //         :param my_addr: cipher is encrypted for this address
    //         :return: if symmetric -> (plain, None), if asymmetric (plain, randomness which was used to encrypt plain)
    //         """
    fn dec(
        &self,
        cipher: &Value<u8, CipherValue>,
        my_addr: &String,
    ) -> (u64, Option<Value<u8, RandomnessValue>>) {
        // assert isinstance(cipher, CipherValue), f"Tried to decrypt value of type {type(cipher).__name__}"
        // assert isinstance(my_addr, AddressValue)
        zk_print!("Decrypting value {:?} for {my_addr}",cipher.contents); //, verbosity_level=2
        let v = Value::<u8, CipherValue> {
            value: CipherValue,
            contents: vec![],
            params: Some(self.params()),
            crypto_backend: None,
        };
        if cipher == &v {
            // # Ciphertext is all zeros, i.e. uninitialized -> zero
            (
                0,
                if self.params().is_symmetric_cipher() {
                    None
                } else {
                    Some(Value::<u8, RandomnessValue> {
                        value: RandomnessValue,
                        contents: vec![],
                        params: Some(self.params()),
                        crypto_backend: None,
                    })
                },
            )
        } else {
            let sk = self.keystore().sk(my_addr);
            let (plain, rnd) = self._dec(cipher.contents.clone(), &sk);
            (
                plain,
                Some(Value::<u8, RandomnessValue> {
                    value: RandomnessValue,
                    contents: rnd,
                    params: Some(self.params()),
                    crypto_backend: None,
                }),
            )
        }
    }
    //         """Serialize a large integer into an array of {params.cipher_chunk_size}-byte ints."""
    fn serialize_pk(&self, key: Vec<u8>, _total_bytes: i32) -> Vec<u8> {
        let data = key; //total_bytes
        self.pack_byte_array(data.into(), self.params().cipher_chunk_size() as usize)
    }
    // """Deserialize an array of {params.cipher_chunk_size}-byte ints into a single large int"""
    fn deserialize_pk(&self, arr: Vec<u8>) -> Vec<u8> {
        let data = self.unpack_to_byte_array(arr, self.params().cipher_chunk_size(), 0);
        data
    }

    //     @staticmethod
    //         """Pack byte array into an array of {chunk_size}-byte ints"""
    fn pack_byte_array(&self,bin: Vec<u8>, chunk_size: usize) -> Vec<u8> {
        let total_bytes = bin.len();
        let first_chunk_size = total_bytes % chunk_size;
        let mut arr = vec![];
        if first_chunk_size > 0 {
            arr = vec![BigInteger256::from_str(
                &String::from_utf8(bin[..first_chunk_size].to_vec()).unwrap(),
            )
            .unwrap()]
        };
        for i in (first_chunk_size..total_bytes - first_chunk_size).step_by(chunk_size) {
            arr.push(
                BigInteger256::from_str(
                    &String::from_utf8(bin[i..i + chunk_size].to_vec()).unwrap(),
                )
                .unwrap(),
            );
        }
        arr.into_iter().flat_map(|v|v.to_string().into_bytes()).rev().collect()
    }

    //     @staticmethod
    //         """Unpack an array of {chunk_size}-byte ints into a byte array"""
    fn unpack_to_byte_array(&self,arr: Vec<u8>, _chunk_size: i32, desired_length: i32) -> Vec<u8> {
        let mut a: Vec<_> = arr
            .into_iter()
            .rev()
            .flat_map(|chunk| chunk.to_be_bytes())
            .collect();
        let _n = a.len();
        a.split_off(a.len() - desired_length as usize)
    }

    //     # Interface implementation
    //     @abstractmethod
    fn _generate_or_load_key_pair(&self, address: &String) -> KeyPair;
    //         pass

    //     @abstractmethod
    fn _enc(
        &self,
        plain: Vec<u8>,
        my_sk: Vec<u8>,
        target_pk: Vec<u8>,
    ) -> (Vec<u8>, Vec<u8>);
    //         pass

    //     @abstractmethod
    fn _dec(&self, cipher: Vec<u8>, sk: &Vec<u8>) -> (u64, Vec<u8>);
    //         pass
}
// class ZkayHomomorphicCryptoInterface(ZkayCryptoInterface){
#[enum_dispatch]
pub trait ZkayHomomorphicCryptoInterface<P:ZkayProverInterface,B:ZkayBlockchainInterface<P>,K: ZkayKeystoreInterface<P,B>>: ZkayCryptoInterface<P,B,K> {
    //     @abstractmethod
    fn do_op(
        &self,
        op: &str,
        public_key: Vec<u8>,
        args: Vec<u8>,
    ) -> Vec<u8>;
    //         pass

    //     @abstractmethod
    //         """
    //         Re-randomizes the given argument.
    //         Returns (new_cipher, randomness).
    //         """
    fn do_rerand(
        &self,
        arg: Vec<u8>,
        public_key: Vec<u8>,
    ) -> (Vec<u8>, Vec<u8>);
    //         pass
}
// class ZkayProverInterface(metaclass=ABCMeta){
//     """API to generate zero knowledge proofs for a particular circuit and arguments."""
pub trait ZkayProverInterface {
    //     fn __init__(&self, proving_scheme: str = None){
    //         self.proving_scheme = cfg.proving_scheme if proving_scheme is None else proving_scheme

    //         """
    //         Generate a NIZK-proof using the provided circuit for the given arguments.

    //         Note: circuit arguments must be in the same order as they are declared inside the circuit. (i.e. in execution order)

    //         :param project_dir: directory where the manifest and the prover keys are located
    //         :param contract: contract of which the function which requires verification is part of
    //         :param function: the contract member function for which a proof needs to be generated
    //         :param priv_values: private/auxiliary circuit inputs in correct order
    //         :param in_vals: public circuit inputs in correct order
    //         :param out_vals: public circuit outputs in correct order
    //         :raise ProofGenerationError: if proof generation fails
    //         :return: the proof, serialized into an uint256 array
    //         """
    fn generate_proof(
        &self,
        project_dir: &str,
        contract: String,
        function: String,
        mut priv_values: Vec<i32>,
        in_vals: Vec<i32>,
        out_vals: Vec<i32>,
    ) -> Vec<i32> {
        for i in 0..priv_values.len() {
            let arg = priv_values[i];
            // assert not isinstance(arg, JsonValue) or isinstance(arg, (RandomnessValue, AddressValue))
            // if isinstance(arg, AddressValue) {
            priv_values[i] = arg; //i32::from_be_bytes(arg);
                                  // }
        }
        zk_print!("Generating proof for {contract}.{function}");
        zk_print!(
            "[priv: {:?}] [in: {:?}] [out: {:?}]",
            priv_values,
            in_vals,
            out_vals
        );

        let (priv_values, in_vals, out_vals) = (priv_values, in_vals, out_vals);

        // # Check for overflows
        assert!(
            priv_values
                .iter()
                .chain(&in_vals)
                .chain(&out_vals)
                .all(|&arg| U256::from(arg) < *BN128_SCALAR_FIELDS),
            "argument overflow"
        );

        // with time_measure(f"generate_proof", True){
        let verify_dir = CFG.lock().unwrap().get_circuit_output_dir_name(
            CFG.lock()
                .unwrap()
                .get_verification_contract_name(contract, function),
        );
        let proof = self._generate_proof(
            &PathBuf::from(project_dir)
                .join(verify_dir)
                .absolutize()
                .unwrap()
                .to_path_buf(),
            priv_values,
            in_vals,
            out_vals,
        );
        proof
    }
    //     @abstractmethod
    fn _generate_proof(
        &self,
        verifier_dir: &PathBuf,
        priv_values: Vec<i32>,
        in_vals: Vec<i32>,
        out_vals: Vec<i32>,
    ) -> Vec<i32>;
    //         pass

    //     @abstractmethod
    //         """Return the hash of the prover key stored in the given verification contract output directory."""
    fn get_prover_key_hash(&self, verifier_directory: &str) -> Vec<u8>;
    //         pass
}
