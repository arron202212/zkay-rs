#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// import json
// import os
// import tempfile
// from abc import abstractmethod
// from contextlib import contextmanager
// from pathlib import Path
// from typing import Any, Dict, Optional, Tuple, List, Union

// from eth_tester import PyEVMBackend, EthereumTester
// from web3 import Web3
// use my_logging;
use crate::interface::{ZkayBlockchainInterface, ZkayProverInterface};
use my_logging::{log_context::log_context, logger::data};
use privacy::library_contracts;
use serde_json::{json, Map, Result, Value as JsonValue};
use solidity::compiler::compile_solidity_json;
use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::path::PathBuf;
use zkay_config::{
    config::{library_compilation_environment, zk_print_banner, CFG},
    config_user::UserConfig,
    with_context_block, zk_print,
};
use zkay_transaction_crypto_params::params::CryptoParams;
// , IntegrityError, BlockChainError,
//     TransactionFailedException
use crate::types::{AddressValue, BlockStruct, MsgStruct, PublicKeyValue, TxStruct, Value};
use ast_builder::process_ast::get_verification_contract_names;
use rccell::RcCell;
use zkay_ast::global_defs::{
    array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars,
};
use zkay_utils::helpers::{get_contract_names, save_to_file};
use zkay_utils::timer::time_measure;
// max_gas_limit = 10000000
pub trait Web3Blockchain {
    fn _create_w3_instance(&self);
}
use std::marker::PhantomData;
#[derive(Clone)]
pub struct Web3BlockchainBase<P: ZkayProverInterface, W> {
    _lib_addresses: BTreeMap<String, JsonValue>,
    _pki_contract: BTreeMap<String, JsonValue>,
    prover: P,
    _web3: PhantomData<W>,
}
impl<P: ZkayProverInterface, W> Web3BlockchainBase<P, W> {
    pub fn new(prover: P) -> Self {
        Self {
            prover,
            _lib_addresses: BTreeMap::new(),
            _pki_contract: BTreeMap::new(),
            _web3: PhantomData,
        }
    }
}
impl<P: ZkayProverInterface, W> ZkayBlockchainInterface<P> for Web3BlockchainBase<P, W> {
    fn lib_addresses_mut(&mut self) -> &mut BTreeMap<String, JsonValue> {
        &mut self._lib_addresses
    }
    fn prover(&self) -> &P {
        &self.prover
    }
    fn default_address(&self) -> Option<AddressValue> {
        //  addr = self._default_address();
        // return None if addr is None else AddressValue(addr)
        None
    }
    fn create_test_accounts(&mut self, count: i32) -> Vec<String> {
        let accounts: Vec<String> = vec![]; //"self.w3.eth.accounts";
        let next_acc_idx = *self.next_acc_idx().unwrap() as usize;
        assert!(
            accounts[next_acc_idx..].len() >= count as usize,
            "Can have at most {} dummy accounts in total",
            accounts.len() - 1
        );
        let dummy_accounts = accounts[next_acc_idx..next_acc_idx + count as usize].to_vec();
        *self.next_acc_idx().unwrap() += count;
        dummy_accounts
    }
    fn deploy_solidity_contract<T: Clone + Default, V: Clone + Default>(
        &self,
        sol_filename: &str,
        contract_name: Option<String>,
        sender: &str,
    ) -> eyre::Result<JsonValue> {
        let contract_name = if let Some(contract_name) = contract_name {
            contract_name
        } else {
            get_contract_names(sol_filename)[0].clone()
        };
        let contract = self._deploy_contract(
            sender,
            self.compile_contract(&PathBuf::from(sol_filename), &contract_name, None, "."),
            &vec![],
            None,
        );
        contract
    }
    // class Web3Blockchain(ZkayBlockchainInterface):
    //     fn __init__(self) -> None:
    //         super().__init__()
    //         self.w3 = self._create_w3_instance()
    //         if not self.w3.is_connected():
    //             raise BlockChainError(f"Failed to connect to blockchain: {self.w3.provider}")

    //     @staticmethod
    fn get_special_variables(
        &self,
        sender: &String,
        wei_amount: i32,
    ) -> (MsgStruct, BlockStruct, TxStruct) {
        // let block = self.w3.eth.get_block("pending");
        zk_print!("Current block timestamp: "); //block["timestamp"]
        (
            MsgStruct::new(sender.clone(), wei_amount),
            BlockStruct::new(
                sender.clone(),
                0,
                0,
                0,
                0, // block["difficulty"],
                   // block["gasLimit"],
                   // block["number"],
                   // block["timestamp"],
            ),
            TxStruct::new(0, sender.clone()), //self.w3.eth.gas_price
        )
    }
    fn _default_address(&self) -> Option<String> {
        CFG.lock().unwrap().blockchain_default_account().clone()

        // elif isinstance(cfg.blockchain_default_account, int):
        //     return self.w3.eth.accounts[cfg.blockchain_default_account]
    }
    fn _get_balance(&self, _address: &str) -> i32 {
        // self.w3.eth.get_balance(address)
        0
    }

    fn _req_public_key(
        &mut self,
        address: &String,
        crypto_params: &CryptoParams,
    ) -> eyre::Result<Value<u8, PublicKeyValue>> {
        let pki_contract = self.pki_contract(&crypto_params.crypto_name);
        Ok(Value::<u8, PublicKeyValue>::new(
            self._req_state_var::<String>(&pki_contract, "getPk", address)
                .into_bytes(),
            Some(crypto_params.clone()),
            None,
        ))
    }

    fn _announce_public_key(
        &mut self,
        address: &str,
        pk: &Vec<String>,
        crypto_params: &CryptoParams,
    ) {
        //         with log_context(f"announcePk"):
        let pki_contract = self.pki_contract(&crypto_params.crypto_name);
        self._transact(&pki_contract, address, "announcePk", pk, None)
    }

    fn _req_state_var<R: Clone + Default>(
        &self,
        _contract_handle: &JsonValue,
        _name: &str,
        _indices: &String,
    ) -> R {
        //         try:
        // contract_handle.functions[name](indices).call()
        //         except Exception as e:
        //             raise BlockChainError(e.args)
        R::default()
    }
    fn _call(
        &self,
        _contract_handle: JsonValue,
        _sender: &String,
        _name: &str,
        _args: &String,
    ) -> String {
        //         try:
        // let fct = contract_handle.functions[name];
        // let gas_amount = self._gas_heuristic(sender, fct(args));
        // let tx = json!({"from": sender, "gas": gas_amount});
        // fct(args).call(tx)
        //         except Exception as e:
        //             raise BlockChainError(e.args)
        String::new()
    }
    fn _transact(
        &self,
        _contract_handle: &JsonValue,
        _sender: &str,
        _function: &str,
        _actual_args: &Vec<String>,
        _wei_amount: Option<i32>,
    ) {
        // let fct = if function == "constructor" {
        //     contract_handle.constructor
        // } else {
        //     contract_handle.functions[function]
        // };
        // let gas_amount = self._gas_heuristic(sender, fct(*actual_params));
        // let mut tx = json!({"from": sender, "gas": gas_amount});
        // if wei_amount.is_some() {
        //     tx["value"] = wei_amount;
        // }
        // let tx_hash = fct(*actual_params).transact(tx);
        // let tx_receipt = self.w3.eth.wait_for_transaction_receipt(tx_hash);
        // // except Exception as e:
        // //     raise BlockChainError(e.args)

        // if tx_receipt["status"] == 0 {
        //     eyre::bail!("Transaction failed")
        // }
        // let gas = tx_receipt["gasUsed"];
        // zk_print!("Consumed gas: {gas}");
        // data("gas", gas);
        // tx_receipt
    }

    fn _deploy(
        &mut self,
        project_dir: &str,
        sender: &str,
        contract: &str,
        actual_args: Vec<String>,
        wei_amount: Option<i32>,
    ) -> eyre::Result<JsonValue> {
        let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        // with open(os.path.join(project_dir, "contract.zkay")) as f:
        let verifier_names = get_verification_contract_names(
            (
                std::fs::read_to_string(project_dir.to_owned() + "contract.zkay").ok(),
                None,
            ),
            global_vars,
        );

        // Deploy verification contracts if not already done
        let external_contract_addresses =
            self._deploy_dependencies(sender, project_dir, verifier_names);
        let cout;
        with_context_block!(var filename= self.__hardcoded_external_contracts_ctx(project_dir, &external_contract_addresses) =>{
            cout = self.compile_contract(&PathBuf::from(filename), contract, None,project_dir);
        });
        let handle;
        with_context_block!(var _a= log_context("constructor")=>{
            with_context_block!(var _b= log_context(&format!("{contract}"))=>{
                handle = self._deploy_contract(sender, cout, &actual_args, wei_amount);});
        });
        zk_print!(r#"Deployed contract "{contract}" at address "{handle:?}""#);
        handle
    }

    fn _deploy_dependencies(
        &mut self,
        sender: &str,
        project_dir: &str,
        verifier_names: Vec<String>,
    ) -> BTreeMap<String, JsonValue> {
        // # Deploy verification contracts if not already done
        let mut vf = BTreeMap::new();
        for verifier_name in verifier_names {
            with_context_block!(var _a= log_context("constructor")=>{
                with_context_block!(var _b=log_context(&format!("{verifier_name}"))=>{
                    let filename = PathBuf::from(project_dir).join( &format!("{verifier_name}.sol"));
                    let lib_addresses=self.lib_addresses();
                    let cout = self.compile_contract(&filename, &verifier_name, Some(lib_addresses),".");
                    with_context_block!(var _tm= time_measure("transaction_full",false,false)=>{
                        vf.insert(verifier_name.clone(),self._deploy_contract(sender, cout,&vec![],None).unwrap());
                    });
            });
            });
        }
        for crypto_params in CFG.lock().unwrap().all_crypto_params() {
            let pki_contract_name = CFG.lock().unwrap().get_pki_contract_name(&crypto_params);
            let pki_contract_address = self.pki_contract(&crypto_params);
            vf.insert(pki_contract_name, pki_contract_address.into());
        }
        vf
    }

    fn _connect_libraries(&mut self) {
        assert!(
            !CFG.lock().unwrap().blockchain_pki_address().is_empty(),
            "Must specify pki address in config."
        );

        let mut lib_addresses = vec![];
        if !CFG.lock().unwrap().external_crypto_lib_names().is_empty() {
            if !CFG
                .lock()
                .unwrap()
                .blockchain_crypto_lib_addresses()
                .is_empty()
            {
                lib_addresses = CFG
                    .lock()
                    .unwrap()
                    .blockchain_crypto_lib_addresses()
                    .split(",")
                    .map(|addr| addr.trim().to_owned())
                    .collect();
            }

            assert!(
                lib_addresses.len() == CFG.lock().unwrap().external_crypto_lib_names().len(),
                "Must specify all crypto library addresses in config\nExpected {} was {}",
                CFG.lock().unwrap().external_crypto_lib_names().len(),
                lib_addresses.len()
            );
        }
        // with_context_block!(var _lce=library_compilation_environment()=>{
        let tmpdir = std::env::temp_dir();
        // let mut _pki_contract=BTreeMap::new();
        for crypto_params in CFG.lock().unwrap().all_crypto_params() {
            let contract_name = CFG.lock().unwrap().get_pki_contract_name(&crypto_params);
            let pki_sol = save_to_file(
                Some(tmpdir.clone()),
                &format!("{contract_name}.sol"),
                &library_contracts::get_pki_contract(&CryptoParams::new(crypto_params.clone())),
            );
            self._pki_contract.insert(
                crypto_params,
                self._verify_contract_integrity(
                    &CFG.lock().unwrap().blockchain_pki_address().into(),
                    &PathBuf::from(pki_sol),
                    None,
                    Some(contract_name),
                    false,
                    None,
                ),
            );
        }
        //  *self._pki_contract.borrow_mut()=_pki_contract;
        let verify_sol = save_to_file(
            Some(tmpdir),
            "verify_libs.sol",
            &library_contracts::get_verify_libs_code(),
        );
        // let mut _lib_addresses = BTreeMap::new();
        for (lib, addr) in CFG
            .lock()
            .unwrap()
            .external_crypto_lib_names()
            .iter()
            .zip(&lib_addresses)
        {
            let out = self._verify_contract_integrity(
                &addr.clone().into(),
                &PathBuf::from(verify_sol.clone()),
                None,
                Some(lib.clone()),
                true,
                None,
            );
            self._lib_addresses.insert(lib.clone(), out);
        }
        // self._lib_addresses = _lib_addresses;
        // });
    }

    fn _connect(
        &self,
        project_dir: &str,
        contract: &str,
        _address: JsonValue,
    ) -> eyre::Result<JsonValue> {
        let filename = PathBuf::from(project_dir).join("contract.sol");
        let _cout = self.compile_contract(&filename, contract, None, ".");
        //  self.w3.eth.contract(
        //     address=address, abi=cout["abi"]
        // )
        Ok(json!({}))
    }

    fn _verify_contract_integrity(
        &self,
        mut address: &JsonValue,
        sol_filename: &PathBuf,
        libraries: Option<BTreeMap<String, JsonValue>>,
        mut contract_name: Option<String>,
        is_library: bool,
        cwd: Option<String>,
    ) -> JsonValue {
        // if is_instance(address, bytes)
        //     {address = self.w3.toChecksumAddress(address);}

        let contract_name = if let Some(contract_name) = contract_name {
            contract_name
        } else {
            get_contract_names(&sol_filename.to_string_lossy().to_string())[0].clone()
        };
        let actual_byte_code = self.__normalized_hex("get_code(address)".to_owned()); //self.w3.eth.get_code(address) MYTODO
        assert!(
            !actual_byte_code.is_empty(),
            "Expected contract {} is not deployed at address {address}",
            contract_name
        );

        let cout = self.compile_contract(
            sol_filename,
            &contract_name,
            libraries,
            cwd.as_ref().unwrap_or(&String::new()),
        );
        let mut expected_byte_code =
            self.__normalized_hex(cout["deployed_bin"].as_str().unwrap().to_owned());

        if is_library {
            // # https://github.com/ethereum/solidity/issues/7101
            expected_byte_code = expected_byte_code[..2].to_owned()
                + &self.__normalized_hex("address.clone()".to_owned())
                + &expected_byte_code[42..]
        };

        assert!(
            actual_byte_code == expected_byte_code,
            "Deployed contract at address {address} does not match local contract {sol_filename:?}"
        );
        zk_print!(
            "Contract@{address} matches {}:{contract_name}",
            sol_filename.file_name().unwrap().to_str().unwrap()
        );

        // self.w3.eth.contract(
        //     address=address, abi=cout["abi"]
        // )
        cout
    }
    fn _verify_library_integrity(
        &self,
        libraries: BTreeMap<String, PathBuf>,
        contract_with_libs_addr: &String,
        sol_with_libs_filename: &PathBuf,
    ) -> BTreeMap<String, JsonValue> {
        let cname =
            get_contract_names(&sol_with_libs_filename.to_string_lossy().to_string())[0].clone();
        let actual_code =
            self.__normalized_hex("self.w3.eth.getCode(contract_with_libs_addr)".to_owned());
        assert!(
            !actual_code.is_empty(),
            "Expected contract {cname} is not deployed at address {contract_with_libs_addr}"
        );
        let code_with_placeholders = self.__normalized_hex(
            self.compile_contract(sol_with_libs_filename, &cname, None, ".")["deployed_bin"]
                .to_string(),
        );

        assert!(
            actual_code.len() == code_with_placeholders.len(),
            "Local code of contract {cname} has different length than remote contract"
        );

        let mut addresses = BTreeMap::new();
        for (lib_name, lib_sol) in libraries {
            // # Compute placeholder according to
            // # https://solidity.readthedocs.io/en/v0.5.13/using-the-compiler.html#using-the-commandline-compiler
            let hash = String::new(); //self.w3.solidityKeccak(
                                      //     vec!["string"],
                                      //     vec![format!(
                                      //         "{}:{lib_name}",
                                      //         lib_sol.file_name()
                                      //     )],
                                      // );

            let placeholder = format!("__${}$__", &self.__normalized_hex(hash)[..34]);

            // # Retrieve concrete address in deployed code at placeholder offset in local code and verify library contract integrity
            let lib_address_offset = code_with_placeholders.find(&placeholder);
            if let Some(_lib_address_offset) = lib_address_offset {
                let lib_address: JsonValue = "".into(); // self
                                                        // .w3
                                                        // .toChecksumAddress(actual_code[lib_address_offset..lib_address_offset + 40]);
                                                        // with cfg.library_compilation_environment():
                self._verify_contract_integrity(
                    &lib_address,
                    &lib_sol,
                    None,
                    Some(lib_name.clone()),
                    true,
                    None,
                );
                addresses.insert(lib_name.clone(), lib_address.clone());
            }
        }
        addresses
    }
    fn _verify_zkay_contract_integrity(
        &self,
        address: &JsonValue,
        project_dir: &str,
        pki_verifier_addresses: &BTreeMap<String, JsonValue>,
    ) {
        let sol_file =
            self.__hardcoded_external_contracts_ctx(project_dir, &pki_verifier_addresses);
        self._verify_contract_integrity(
            address,
            &PathBuf::from(sol_file),
            None,
            None,
            false,
            Some(project_dir.to_owned()),
        );
    }

    //     @contextmanager
    fn lib_addresses(&mut self) -> BTreeMap<String, JsonValue> {
        if self._lib_addresses.is_empty() {
            self._connect_libraries();
        }
        self._lib_addresses.clone()
    }
}

impl<P: ZkayProverInterface, W> Web3BlockchainBase<P, W> {
    pub fn compile_contract(
        &self,
        sol_filename: &PathBuf,
        contract_name: &str,
        libs: Option<BTreeMap<String, JsonValue>>,
        cwd: &str,
    ) -> JsonValue {
        let solp = sol_filename; //std::path::PathBuf::from(sol_filename);
        let jout = compile_solidity_json(
            &sol_filename.to_string_lossy().to_string(),
            libs.map(|s| s.into_iter().map(|(k, v)| (k, v.to_string())).collect()),
            CFG.lock().unwrap().opt_solc_optimizer_runs(),
            vec![],
            cwd,
        )
        .unwrap();
        let jout = &jout["contracts"][solp.file_name().unwrap().to_str().unwrap()][&contract_name];
        json!({
            "abi": jout["metadata"]["output"]["abi"],
            "bin": jout["evm"]["bytecode"]["object"],
            "deployed_bin": jout["evm"]["deployedBytecode"]["object"],
        })
    }
    fn _deploy_contract(
        &self,
        sender: &str,
        _contract_interface: JsonValue,
        args: &Vec<String>,
        wei_amount: Option<i32>,
    ) -> eyre::Result<JsonValue> {
        // let contract = self.w3.eth.contract(
        //     abi=contract_interface["abi"],
        //     bytecode=contract_interface["bin"]
        // );

        let _tx_receipt =
            self._transact(&"contract".into(), sender, "constructor", args, wei_amount);
        // let contract = self.w3.eth.contract(
        //     tx_receipt.contractAddress, contract_interface["abi"]
        // );
        //  contract
        Ok(json!({}))
    }

    fn __hardcoded_external_contracts_ctx(
        &self,
        contract_dir: &str,
        pki_verifier_addresses: &BTreeMap<String, JsonValue>,
    ) -> String {
        // # Hardcode contract addresses
        //     use std::fs::OpenOptions;
        // let f=OpenOptions::new().read(true).open(output_filename);
        // with open(os.path.join(contract_dir, "contract.sol")) as f:
        let mut c = std::fs::read_to_string(contract_dir.to_owned() + "contract.sol").unwrap();
        for (key, _val) in pki_verifier_addresses {
            c = c.replace(
                &format!("{key}(0)"),
                &format!("{key}({})", "self.w3.toChecksumAddress(val)"),
            );
        }

        // with tempfile.TemporaryDirectory() as tempd:
        // # Save in temporary file to compile
        let output_filename = std::env::temp_dir().join("contract.inst.sol");
        // let f=OpenOptions::new().write(true).create(true).open(output_filename);
        let _ = std::fs::write(output_filename.clone(), c);
        format!("{}", output_filename.display())
        // pass
    }

    fn __normalized_hex(&self, mut val: String) -> String {
        // if not isinstance(val, str):
        // val = val.hex()
        if val.starts_with("0x") {
            val = val[2..].to_owned();
        }
        val.make_ascii_lowercase();
        val
    }

    fn _gas_heuristic(&self, _sender: &str, _tx: &str) -> i32 {
        let limit = i32::MAX; //self.w3.eth.get_block("latest")["gasLimit"];
        let estimate = 1; //tx.estimateGas({"from": sender, "gas": limit});
        limit.min((estimate as f64 * 1.2) as i32)
    }
}
#[derive(Clone)]
pub struct Web3TesterBlockchain;
// class Web3TesterBlockchain(Web3Blockchain):
//     fn __init__(self) -> None:
//         self.eth_tester = None
//         super().__init__()
//         self.next_acc_idx = 1
impl<P: ZkayProverInterface, Web3TesterBlockchain> Web3BlockchainBase<P, Web3TesterBlockchain> {
    //     @classmethod
    fn is_debug_backend(&self) -> bool {
        true
    }
    fn next_acc_idx(&mut self) -> Option<&mut i32> {
        None
    }
    fn _connect_libraries(&self) {
        zk_print_banner("Deploying Libraries".to_owned());

        let sender = "self.w3.eth.accounts[0]";
        // # Since eth-tester is not persistent -> always automatically deploy libraries
        // with cfg.library_compilation_environment():
        // with tempfile.TemporaryDirectory() as tmpdir:
        let tmpdir = std::env::temp_dir();
        // with log_context("deploy_pki"):
        let mut _pki_contract = BTreeMap::new();
        for crypto_params in CFG.lock().unwrap().all_crypto_params() {
            // with log_context(crypto_params.crypto_name):
            let pki_contract_code =
                library_contracts::get_pki_contract(&CryptoParams::new(crypto_params.clone()));
            let pki_contract_name = CFG.lock().unwrap().get_pki_contract_name(&crypto_params);
            let pki_sol = save_to_file(
                Some(tmpdir.clone()),
                &format!("{pki_contract_name}.sol"),
                &pki_contract_code,
            );
            let contract = self._deploy_contract(
                sender,
                self.compile_contract(&PathBuf::from(pki_sol), &pki_contract_name, None, "."),
                &vec![],
                None,
            );
            let backend_name = crypto_params.clone();
            _pki_contract.insert(backend_name.clone(), contract.as_ref().unwrap().clone());
            zk_print!(
                r#"Deployed pki contract for crypto back-end {backend_name} at address "{:?}""#,
                contract
            );
        }
        // with log_context("deploy_verify_libs"):
        let verify_sol = save_to_file(
            Some(tmpdir.clone()),
            "verify_libs.sol",
            &library_contracts::get_verify_libs_code(),
        );
        let mut _lib_addresses = BTreeMap::new();
        for lib in CFG.lock().unwrap().external_crypto_lib_names() {
            let out = self._deploy_contract(
                sender,
                self.compile_contract(&PathBuf::from(verify_sol.clone()), &lib, None, "."),
                &vec![],
                None,
            );
            _lib_addresses.insert(lib.clone(), out.as_ref().unwrap().clone());
            zk_print!(r#"Deployed crypto lib {lib} at address "{out:?}""#);
        }
    }

    fn _gas_heuristics(&self, _sender: &str, _tx: &str) -> i32 {
        MAX_GAS_LIMIT
    }
}
const MAX_GAS_LIMIT: i32 = 10000000;

impl Web3Blockchain for Web3TesterBlockchain {
    fn _create_w3_instance(&self) {
        // let genesis_overrides = json!({"gas_limit": max_gas_limit * 1.2});
        // let custom_genesis_params = PyEVMBackend._generate_genesis_params(genesis_overrides);
        // self.eth_tester = EthereumTester(PyEVMBackend(genesis_parameters = custom_genesis_params));
        // let w3 = Web3(Web3.EthereumTesterProvider(self.eth_tester));
        //  w3
    }
}

pub struct Web3IpcBlockchain;
// class Web3IpcBlockchain(Web3Blockchain):
//     fn _create_w3_instance(self) -> Web3:
//         assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         return Web3(Web3.IPCProvider(cfg.blockchain_node_uri))
// pub struct Web3WebsocketBlockchain;
// class Web3WebsocketBlockchain(Web3Blockchain):
//     fn _create_w3_instance(self) -> Web3:
//         assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         return Web3(Web3.WebsocketProvider(cfg.blockchain_node_uri))

// class Web3HttpBlockchain(Web3Blockchain):
pub struct Web3HttpBlockchain;
// impl Web3Blockchain for Web3HttpBlockchain
// {
//     fn _create_w3_instance(&self)
//       {  // assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//          Web3(Web3.HTTPProvider(cfg.blockchain_node_uri))}
// }
#[derive(Clone)]
pub struct Web3HttpGanacheBlockchain {
    next_acc_idx: i32,
}
// class Web3HttpGanacheBlockchain(Web3HttpBlockchain):
//     fn __init__(self) -> None:
//         super().__init__()
//         self.next_acc_idx = 1
// impl Web3Blockchain for Web3HttpGanacheBlockchain {
//     fn _create_w3_instance(&self) {
//         // assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         // Web3(Web3.HTTPProvider(cfg.blockchain_node_uri))
//     }
// }
// impl Web3HttpGanacheBlockchain {
//     //     @classmethod
//     fn is_debug_backend(&self) -> bool {
//         true
//     }

//     fn create_test_accounts(&mut self, count: i32) -> Vec<String> {
//         let accounts = vec![]; //self.w3.eth.accounts;
//         assert!(
//             accounts.len() >= self.next_acc_idx + count,
//             "Can have at most {} dummy accounts in total",
//             accounts.len() - 1
//         );
//         let idx = self.next_acc_idx as usize;
//         let dummy_accounts = accounts[idx..idx + count as usize].to_vec();
//         self.next_acc_idx += count;
//         dummy_accounts
//     }

//     fn _gas_heuristic(&self, sender: &str, tx: &str) -> i32 {
//         // self.w3.eth.get_block("latest")["gasLimit"]
//         0
//     }
// }

pub struct Web3CustomBlockchain;
// impl Web3Blockchain for Web3CustomBlockchain {}
// class Web3CustomBlockchain(Web3Blockchain):
//     fn _create_w3_instance(self) -> Web3:
//         assert isinstance(cfg.blockchain_node_uri, Web3)
//         return cfg.blockchain_node_uri
