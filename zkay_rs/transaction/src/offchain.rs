#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(async_fn_in_trait)]
// from __future__ import annotations

// import inspect
// from contextlib import contextmanager, nullcontext
// from enum import IntEnum
// from typing import Dict, Union, CallableType, Any, Optional, List, Tuple, ContextManager
use crate::arc_cell_new;
use crate::blockchain::web3::Web3Tx;
use crate::keystore::simple::SimpleKeystore;
use crate::prover::jsnark::JsnarkProver;
use crate::runtime::BlockchainClass;
use crate::runtime::CryptoClass;
use crate::runtime::{_blockchain_classes, _crypto_classes, _prover_classes};
use alloy_json_abi::JsonAbi;
use alloy_primitives::Address;
use foundry_cli::opts::{EthereumOpts, RpcOpts};
use foundry_compilers::Project;
use my_logging::log_context::WithLogContext;
use proving_scheme::proving_scheme::ProvingScheme;
use rccell::{RcCell, WeakCell};
use serde_json::{Map, Result, Value as JsonValue, json};
use std::path::PathBuf;
use std::str::FromStr;

use super::int_casts::__convert as int_cast;
use my_logging::log_context::log_context;
pub use privacy::library_contracts::BN128_SCALAR_FIELD;
use privacy::manifest::Manifest;
use std::collections::BTreeMap;
use zkay_config::config_user::UserConfig;
use zkay_config::{
    config::{CFG, zk_print_banner},
    with_context_block,
};
use zkay_transaction_crypto_params::params::CryptoParams;
// use crate::blockchain::web3rs::Web3Blockchain;
use crate::interface::{
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayHomomorphicCryptoInterface,
    ZkayKeystoreInterface, ZkayProverInterface,
};
use crate::runtime::Runtime;
use crate::types::{
    ARcCell, AddressValue, BlockStruct, CipherValue, DataType, MsgStruct, PrivateKeyValue,
    PublicKeyValue, RandomnessValue, TxStruct, Value,
};
use ark_ff::{BigInteger, BigInteger256, Field, MontFp, PrimeField};
use zkay_ast::homomorphism::Homomorphism;
use zkay_utils::progress_printer::fail_print;
// bn128_scalar_field = bn128_scalar_field
use num_bigint::BigInt;
use num_traits::Zero;
const _BN128_COMP_SCALAR_FIELD: BigInteger256 = BigInteger256::one();
type CallableType = fn(String) -> DataType; //: Clone + Default + std::iter::FromIterator<T>

use std::ops::{Index, IndexMut};

// class RequireException(Exception):
//     pass

// class StateDict:
//     """Dictionary which wraps access to state variables"""
use std::marker::PhantomData;
pub struct StateDict<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
> {
    api: ARcCell<ApiWrapper<P, B, K>>,
    __state: ARcCell<BTreeMap<String, DataType>>,
    __constructors: ARcCell<BTreeMap<String, (bool, CryptoParams, CallableType)>>,
    _prover: PhantomData<P>,
    _bc: PhantomData<B>,
    _crypto: PhantomData<C>,
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
> StateDict<P, B, K, C>
{
    pub fn new(api: ARcCell<ApiWrapper<P, B, K>>) -> Self {
        Self {
            api,
            __state: arc_cell_new!(BTreeMap::new()),
            __constructors: arc_cell_new!(BTreeMap::new()),
            _prover: PhantomData,
            _bc: PhantomData,
            _crypto: PhantomData,
        }
    }
    //     fn __init__(&self, api) -> None:
    //         self.api = api
    //         self.__state: Dict[str, Any] = {}
    //         self.__constructors: Dict[str, (bool, CryptoParams, CallableType)] = {}

    pub fn clear(&self) {
        self.__state.lock().clear();
    }
    // """Define the wrapper constructor for a state variable."""= CFG.lock().unwrap().main_crypto_backend
    pub fn decl(&self, name: &str, constructor: CallableType, cipher: bool, crypto_backend: &str) {
        let crypto_backend = if crypto_backend.is_empty() {
            CFG.lock().unwrap().main_crypto_backend()
        } else {
            crypto_backend.to_owned()
        };
        // assert name not in self.__constructors
        self.__constructors.lock().insert(
            name.to_owned(),
            (cipher, CryptoParams::new(crypto_backend), constructor),
        );
    }

    //     @property
    pub fn names(&self) -> Vec<String> {
        self.__constructors.lock().keys().cloned().collect()
    }

    pub async fn get_plain(&self, name: &str, indices: Vec<String>) -> Option<DataType> {
        let (is_cipher, crypto_params, constr) = self.__constructors.lock()[name].clone();
        let val = self
            .__get(
                vec![name.to_owned()]
                    .into_iter()
                    .chain(indices.into_iter())
                    .collect(),
                false,
            )
            .await;
        if is_cipher {
            let (ret, _) = self
                .api
                .lock()
                .dec(val.unwrap(), constr, &crypto_params.crypto_name);
            return Some(ret);
        }
        val
    }

    pub async fn get_raw(&self, name: &str, indices: Vec<String>) -> Option<DataType> {
        self.__get(
            vec![name.to_owned()]
                .into_iter()
                .chain(indices.into_iter())
                .collect(),
            false,
        )
        .await
    }

    // """
    // Return value of the state variable (or index of state variable) key

    // :param key: Either a string with the state variable name (primitive variables) or a Tuple with the name and all index key values
    // :raise KeyError: if location does not exist on the chain
    // :return: The requested value
    // """
    pub async fn get_item(&self, key: &[&str]) -> DataType {
        self.__get(key.into_iter().map(|v| v.to_string()).collect(), true)
            .await
            .unwrap()
    }

    // """
    // Assign value to state variable (or to index of state variable)

    // :param key: Either a string with the state variable name (primitive variables) or a Tuple with the name and all index key values
    // :param value: Correctly wrapped value which should be assigned to the specified state location
    // """
    pub fn set_item(&self, key: &[&str], value: DataType) {
        // if not isinstance(key, Tuple)
        //     key = (key, )
        let var = key[0].to_owned();
        let loc = var
            + &key[1..]
                .iter()
                .map(|k| format!("[{k}]"))
                .collect::<Vec<_>>()
                .concat();

        // # Write to state
        self.__state.lock().insert(loc, value);
    }
    pub async fn __get(&self, key: Vec<String>, cache: bool) -> Option<DataType> {
        // if not isinstance(key, Tuple):
        //     key = (key, )
        println!("======__get==========================={key:?}");
        let (var, indices) = (&key[0], key[1..].to_vec());
        let loc = var.to_owned()
            + &key[1..]
                .iter()
                .map(|k| format!("[{k}]"))
                .collect::<Vec<_>>()
                .concat();

        // # Retrieve from state scope
        match self.__state.lock().get(&loc) {
            v @ Some(_) if cache => {
                return v.cloned();
            }
            _ => {}
        }
        println!("=======var===================={var}");
        let (is_cipher, crypto_params, constr) = self.__constructors.lock()[var.as_str()].clone();
        let val = if is_cipher {
            let cipher_len = crypto_params.cipher_len();
            let v = self
                .api
                .lock()
                ._req_state_var(&var, indices, cipher_len)
                .await;
            println!("==__get=====_req_state_var=========={v}============");
            let v: Vec<_> = v.split(",").map(|x| x.to_owned()).collect();
            println!("==__get=====_req_state_var====vv======{v:?}============");
            DataType::CipherValue(Value::<String, CipherValue>::new(
                v,
                Some(crypto_params.clone()),
                None,
            ))
        } else {
            constr(self.api.lock()._req_state_var(var, indices, 0).await)
        };

        if cache {
            self.__state.lock().insert(loc, val.clone());
        }
        Some(val)
    }
    // pub fn get(&self, index: &[&str]) -> DataType {
    //     let var = index[0].to_owned();
    //     let loc = var
    //         + &index[1..]
    //             .iter()
    //             .map(|k| format!("[{k}]"))
    //             .collect::<Vec<_>>()
    //             .concat();
    //     futures::executor::block_on(self.__getitem__(&loc))
    // }
    // pub fn set(&self, index: &[&str], data: DataType) {
    //     let var = index[0].to_owned();
    //     let loc = var
    //         + &index[1..]
    //             .iter()
    //             .map(|k| format!("[{k}]"))
    //             .collect::<Vec<_>>()
    //             .concat();
    //     // # Write to state
    //     self.__state.lock().insert(loc, data);
    //     // panic!("Variable not found");
    // }
}

// impl<
//     P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
//     B: ZkayBlockchainInterface<P> + Clone,
//     K: ZkayKeystoreInterface<P, B> + Clone,
//     C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
// > Index<&[&str]> for StateDict<P, B, K, C>
// {
//     type Output = DataType;

//     fn index(& self, index: &[&str]) -> &Self::Output {
//         let var = index[0].to_owned();
//         let loc = var
//             + &index[1..]
//                 .iter()
//                 .map(|k| format!("[{k}]"))
//                 .collect::<Vec<_>>()
//                 .concat();
//         &futures::executor::block_on(self.__getitem__(&loc))
//     }
// }

// impl<
//     P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
//     B: ZkayBlockchainInterface<P> + Clone,
//     K: ZkayKeystoreInterface<P, B> + Clone,
//     C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
// > IndexMut<&[&str]> for StateDict<P, B, K, C>
// {
//     fn index_mut(&mut self, index: &[&str]) -> &mut Self::Output {
//         let var = index[0].to_owned();
//         let loc = var
//             + &index[1..]
//                 .iter()
//                 .map(|k| format!("[{k}]"))
//                 .collect::<Vec<_>>()
//                 .concat();
//         self.__state.lock().entry(loc.clone()).or_default();
//         // # Write to state
//         self.__state.lock().get_mut(&loc).unwrap()
//         // panic!("Variable not found");
//     }
// }

// class LocalsDict:
//     """
//     Dictionary which supports multiple scopes with name shadowing.
#[derive(Clone)]
pub struct LocalsDict {
    pub _scopes: Vec<BTreeMap<String, DataType>>,
}
impl LocalsDict {
    //     This is needed since python does not natively support c-style nested local scopes.
    //     """
    //     fn __init__(&self) -> None:
    //         self._scopes: List[dict] = [{}]
    // """Introduce a new scope."""
    pub fn push_scope(&mut self) {
        self._scopes.push(BTreeMap::new());
    }

    // """End the current scope."""
    pub fn pop_scope(&mut self) {
        self._scopes.pop();
    }

    // """Introduce a new local variable with the given name and value into the current scope."""
    pub fn decl(&mut self, name: &str, val: DataType) {
        assert!(
            !self._scopes.last().unwrap().contains_key(name),
            "Variable declared twice in same scope"
        );
        self._scopes
            .last_mut()
            .unwrap()
            .insert(name.to_owned(), val);
    }

    // """
    // Return the value of the local variable which is referenced by the identifier key in the current scope.

    // If there are multiple variables with the name key in different scopes,
    // the variable with the lowest declaration scope is used.
    // """
    pub fn __getitem__(&self, key: &str) -> &DataType {
        for scope in self._scopes.iter().rev() {
            if let Some(v) = scope.get(key) {
                return v;
            }
        }
        panic!("Variable not found");
    }

    // """
    // Assign value to the local variable which is referenced by the identifier key in the current scope.

    // If there are multiple variables with the name key in different scopes, the variable with the lowest declaration scope is used.
    // """
    pub fn __setitem__(&mut self, key: &str, value: DataType) {
        for scope in self._scopes.iter_mut().rev() {
            if scope.contains_key(key) {
                scope.insert(key.to_owned(), value);
                return;
            }
        }
        panic!("Variable not found");
    }
}

impl Index<&str> for LocalsDict {
    type Output = DataType;

    fn index(&self, index: &str) -> &Self::Output {
        self.__getitem__(index)
    }
}

impl IndexMut<&str> for LocalsDict {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        for scope in self._scopes.iter_mut().rev() {
            if let Some(v) = scope.get_mut(index) {
                return v;
            }
        }
        panic!("Variable not found");
    }
}
pub trait ContractSimulatorRef<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>
{
    fn contract_simulator_ref(&self) -> ARcCell<ContractSimulator<C, P, B, K>>;
}

impl<
    CS: ContractSimulatorRef<C, P, B, K>,
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ContractSimulatorConfig<C, P, B, K> for CS
{
    fn api(&self) -> ARcCell<ApiWrapper<P, B, K>> {
        self.contract_simulator_ref().lock().api.clone()
    }
    fn locals(&self) -> ARcCell<LocalsDict> {
        self.contract_simulator_ref().lock().locals.clone()
    }
    fn state(&self) -> ARcCell<StateDict<P, B, K, C>> {
        self.contract_simulator_ref().lock().state.clone()
    }
    // @contextmanager
    // """Return context manager which manages the lifetime of a local scope."""
    fn _scope(&self) -> WithScope {
        // self.locals.lock().push_scope();
        // // yield
        // self.locals.lock().pop_scope();
        WithScope::new(self.contract_simulator_ref().lock().locals.clone())
    }

    // @contextmanager   -1  = 0 = '?'
    async fn _function_ctx(
        &self,
        trans_sec_size: i32,
        wei_amount: i32,
        name: &str,
    ) -> (
        bool,
        WithFunctionCtx<C, P, B, K, WithApiFunctionCtx, WithLogContext>,
    ) {
        println!("===========_function_ctx=============beg==========");
        let fc;
        let is_external;
        let cs = self.contract_simulator_ref().lock().clone();
        println!("===========_function_ctx=============beg====2======");
        let api = cs.api.lock().clone();
        println!("===========_function_ctx=============beg=====3=====");
        with_context_block!(var afc= api.api_function_ctx(trans_sec_size, Some(wei_amount)).await=>{
        println!("===========_function_ctx==============sss======{}===",line!());
                     is_external=afc.is_external.lock().map_or(false,|x|x);
                        let mut t_idx=0;

                        if is_external {
                            zk_print_banner(format!("Calling {name}"));
                            // assert self.locals is None
                            self.contract_simulator_ref().lock().state.lock().clear();
                            t_idx = *self.contract_simulator_ref().lock().tidx.lock().get(name).unwrap_or(&0);
                            *self.contract_simulator_ref().lock().tidx.lock().entry(name.to_owned()).or_insert(0) += 1;
                        }

                        // with nullcontext() if not is_external else log_context(f'{name}_{t_idx}'):
                        with_context_block!(var lc=is_external.then(||log_context(&format!("{name}_{t_idx}")))=>{
                println!("========1====");
                        // let prev_locals = self.locals.clone();
                        // self.locals = LocalsDict { _scopes: vec![] };
                        let locals=self.contract_simulator_ref().lock().locals.clone();
                        fc=WithFunctionCtx::new(locals,self.contract_simulator_ref().lock().state.clone(),Some(afc),lc);
                // println!("====2====1====");
                        // try:
                        // yield is_external
                        // except (ValueError, BlockChainError, RequireException) as e:
                        // if is_external and not CFG.lock().unwrap().is_unit_test:
                        //     // # uncomment to raise errors instead of just printing message (for debugging)
                        //     // # raise e
                        //     with fail_print():
                        //         print(f'ERROR: {e}')
                        // else:
                        //     raise e
                        // finally:
                        //     self.locals = prev_locals
                        //     if is_external:
                        //         self.state.clear()
                        });
                        });
        println!("============");
        (is_external, fc)
    }
}

pub trait ContractSimulatorConfig<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>
{
    fn api(&self) -> ARcCell<ApiWrapper<P, B, K>>;
    fn locals(&self) -> ARcCell<LocalsDict>;
    fn state(&self) -> ARcCell<StateDict<P, B, K, C>>;
    // @contextmanager
    // """Return context manager which manages the lifetime of a local scope."""
    fn _scope(&self) -> WithScope;

    // @contextmanager   -1  = 0 = '?'
    async fn _function_ctx(
        &self,
        trans_sec_size: i32,
        wei_amount: i32,
        name: &str,
    ) -> (
        bool,
        WithFunctionCtx<C, P, B, K, WithApiFunctionCtx, WithLogContext>,
    );
}

#[derive(Clone)]
pub struct ContractSimulator<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    pub tidx: ARcCell<BTreeMap<String, i32>>,
    pub api: ARcCell<ApiWrapper<P, B, K>>,
    pub locals: ARcCell<LocalsDict>,
    pub state: ARcCell<StateDict<P, B, K, C>>,
    pub runtime: ARcCell<Runtime<P, B, K>>,
}
impl<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ContractSimulator<C, P, B, K>
{
    // """
    //     Create new contract simulator instance.
    //     :param project_dir: Directory where the zkay contract, the manifest and the prover/verification key files are located
    //     :param user_addr: From address for all transactions which are issued by this ContractSimulator
    //     """
    pub fn new(runtime: ARcCell<Runtime<P, B, K>>, api: ARcCell<ApiWrapper<P, B, K>>) -> Self {
        // # Transaction instance values (reset between transactions)
        // let api = arc_cell_new!(ApiWrapper::<P, B, K>::new(
        //     project_dir,
        //     contract_name,
        //     user_addr,
        //     runtime.clone(),
        // ));

        // """Hierarchical dictionary (scopes are managed internally) which holds the currently accessible local variables"""
        let locals = arc_cell_new!(LocalsDict { _scopes: vec![] });

        // """
        // Dict which stores stores state variable values. Empty at the beginning of a transaction.
        // State variable read: 1. if not in dict -> request from chain and insert into dict, 2. return dict value
        // State variable write: store in dict
        // """
        let state = arc_cell_new!(StateDict::<P, B, K, C>::new(api.clone()));
        Self {
            tidx: arc_cell_new!(BTreeMap::new()),
            api,
            locals,
            state,
            runtime,
        }
    }

    // @property
    pub fn address(&self) -> String {
        // self.api.address().clone()
        String::new()
    }

    // @staticmethod
    pub fn help(_module: &str, _contract: &str, _contract_name: &str) {
        // fn pred(obj)
        //     return inspect.isfunction(obj) and (not hasattr(obj, '_can_be_external') or obj._can_be_external)
        // let global_fcts = inspect.getmembers(module, inspect.isfunction);
        // let members = inspect.getmembers(contract, pred);

        // """Display help for contract functions."""
        // global_fcts = [(name, sig) for name, sig in global_fcts if not name.startswith('i32') and not name.startswith('uint')]

        // signatures = [(fname, str(inspect.signature(sig))) for fname, sig in global_fcts]
        // print!("Global functions:");
        // print!('\n'.join([f'{fname}({sig[1:]}' for fname, sig in signatures
        //                  if not fname.startswith('_') and fname != 'help' and fname != 'zk__init']));
        // print!("");
        // print!(f'Members for {contract_name} contract instances (either deploy or connect to create one):');
        // signatures = [(fname, str(inspect.signature(sig))) for fname, sig in members]
        // print!('\n'.join([f'{fname}({sig[5:] if not sig[5:].startswith(",") else sig[7:]}'
        //                  for fname, sig in signatures
        //                  if sig.startswith('(&self') and not fname.endswith('_check_proof') and not fname.startswith('_')]));
    }
    // @staticmethod
    pub fn reduced_help(_contract: &str) {
        //  fn pred(obj):
        //     return inspect.isfunction(obj) and (not hasattr(obj, '_can_be_external') or obj._can_be_external) and obj.__name__ != 'constructor'
        // members = inspect.getmembers(contract, pred)

        // print(f'Externally callable functions:')
        // signatures = [(fname, str(inspect.signature(sig))) for fname, sig in members]
        // print('\n'.join([f'{fname}({sig[5:] if not sig[5:].startswith(",") else sig[7:]}'
        //                  for fname, sig in signatures
        //                  if sig.startswith('(&self') and not fname.endswith('_check_proof') and not fname.startswith('_')]))
    }
    // @staticmethod
    // """Return default wallet address (if supported by backend, otherwise empty address is returned)."""
    pub fn default_address(&self) -> Option<AddressValue> {
        // (*self.runtime.blockchain()).default_address().clone()
        None
    }

    // @staticmethod
    // """Generate/Load keys for the given address."""
    pub async fn initialize_keys_for(&self, address: &str) {
        let all_crypto_params = CFG.lock().unwrap().all_crypto_params();
        println!(
            "====initialize_keys_for=={address}==all_crypto_params===={all_crypto_params:?}====="
        );
        for crypto_params in all_crypto_params {
            println!("====initialize_keys_for====crypto_params===={crypto_params}=====");
            if !self
                .runtime
                .lock()
                .keystore(&CryptoParams::new(crypto_params.clone()))
                .lock()
                .has_initialized_keys_for(address)
            {
                println!(
                    "====initialize_keys_for====has_initialized_keys_for===={crypto_params}====="
                );
                let _ = self
                    .runtime
                    .lock()
                    .crypto(&CryptoParams::new(crypto_params.clone()))
                    .lock()
                    .generate_or_load_key_pair(address)
                    .await;
            }
        }
    }

    // @staticmethod
    pub fn use_config_from_manifest(&self, project_dir: &str) {
        // """Override zkay configuration with values from the manifest file in project_dir."""
        let manifest = Manifest::load(project_dir);
        Manifest::import_manifest_config(manifest);
        // self.runtime.lock().reset();
    }

    // @staticmethod
    // """
    // Create count pre-funded dummy accounts (if supported by backend)

    // :param count: # of accounts to create
    // :return: if count == 1 -> returns a address, otherwise returns a tuple of count addresses
    // """
    pub async fn create_dummy_accounts(&self, _count: i32) -> Vec<String> {
        let accounts: Vec<String> = vec![]; //self.runtime.blockchain().create_test_accounts(count);
        for account in &accounts {
            self.initialize_keys_for(account).await;
        }
        if accounts.len() == 1 {
            accounts[..1].to_vec()
        } else {
            accounts
        }
    }
}

#[derive(Clone)]
pub struct ApiWrapper<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    __conn: ARcCell<B>,
    __keystore: ARcCell<BTreeMap<String, ARcCell<K>>>,
    __crypto: ARcCell<BTreeMap<String, ARcCell<CryptoClass<P, B, K>>>>,
    __prover: ARcCell<P>,
    __project_dir: ARcCell<String>,
    __contract_name: ARcCell<String>,
    __contract_handle: ARcCell<Option<Address>>,
    __user_addr: ARcCell<Address>,
    __current_msg: ARcCell<Option<MsgStruct>>,
    __current_block: ARcCell<Option<BlockStruct>>,
    __current_tx: ARcCell<Option<TxStruct>>,
    current_priv_values: ARcCell<BTreeMap<String, i32>>,
    all_priv_values: ARcCell<Option<Vec<String>>>,
    current_all_index: ARcCell<Option<i32>>,
    is_external: ARcCell<Option<bool>>,
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ApiWrapper<P, B, K>
{
    pub fn new(
        project_dir: &str,
        contract_name: &str,
        user_addr: Address,
        __conn: ARcCell<B>,
        __keystore: ARcCell<BTreeMap<String, ARcCell<K>>>,
        __crypto: ARcCell<BTreeMap<String, ARcCell<CryptoClass<P, B, K>>>>,
        __prover: ARcCell<P>,
    ) -> Self {
        // super().__init__()
        // let __conn = runtime.lock().blockchain().clone();
        // let mut __keystore = BTreeMap::new();
        // let mut __crypto = BTreeMap::new();
        // let __prover = runtime.lock().prover().clone();
        // for crypto_params in CFG.lock().unwrap().all_crypto_params() {
        //     let crypto_param=CryptoParams::new(crypto_params.clone());
        //     __keystore.insert(
        //         crypto_params.clone(),
        //         runtime
        //             .lock()
        //             .keystore(&crypto_param)
        //             .clone(),
        //     );
        //     __crypto.insert(
        //         crypto_params.clone(),
        //         runtime
        //             .lock()
        //             .crypto(&crypto_param)
        //             .clone(),
        //     );
        // }
        println!("=====project_dir==========={project_dir}");
        let __project_dir = arc_cell_new!(project_dir.to_owned());

        let __contract_name = arc_cell_new!(contract_name.to_owned());

        // """Handle which refers to the deployed contract, this is passed to the blockchain interface when e.g. issuing transactions."""
        let __contract_handle = arc_cell_new!(Some(user_addr.clone()));

        // """From address for all transactions which are issued by this ContractSimulator"""
        let __user_addr = arc_cell_new!(user_addr);

        // """
        // Builtin variable (msg, block, tx) values for the current transaction
        // """
        let __current_msg: ARcCell<Option<MsgStruct>> = arc_cell_new!(None);
        let __current_block: ARcCell<Option<BlockStruct>> = arc_cell_new!(None);
        let __current_tx: ARcCell<Option<TxStruct>> = arc_cell_new!(None);

        // """Dictionary which stores the private circuit values (secret inputs) for the current function (no transitivity)"""
        let current_priv_values: ARcCell<BTreeMap<String, i32>> = arc_cell_new!(BTreeMap::new());

        // """List which stores all secret circuit inputs for the current transaction in correct order (order of use)"""
        let all_priv_values: ARcCell<Option<Vec<String>>> = arc_cell_new!(None);

        // """
        // Index which designates where in all_priv_values the secret circuit inputs of the current function should be inserted.
        // This is basically private analogue of the start_index parameters which are passed to functions which require verification
        // to designate where in the public IO arrays the functions should store/retrieve public circuit inputs/outputs.
        // """
        let current_all_index: ARcCell<Option<i32>> = arc_cell_new!(None);

        // """
        // True whenever simulation is inside a function which was directly (without transitivity) called by the user.
        // This is mostly used for some checks (e.g. to prevent the user from calling internal functions), or to change
        // function behavior depending on whether a call is external or not (e.g. encrypting parameters or not)
        // """
        let is_external: ARcCell<Option<bool>> = arc_cell_new!(None);
        Self {
            __conn,
            __keystore,
            __crypto,
            __prover,
            __project_dir,
            __contract_name,
            __contract_handle,
            __user_addr,
            __current_msg,
            __current_block,
            __current_tx,
            current_priv_values,
            all_priv_values,
            current_all_index,
            is_external,
        }
    }
    // @property
    pub fn address(&self) -> String {
        // self.__contract_handle.lock().as_ref().unwrap()["address"].to_string()
        self.__user_addr.lock().to_string()
    }

    // @property
    pub fn user_address(&self) -> Address {
        self.__user_addr.lock().clone()
    }

    // @property
    pub fn keystore(&self) -> ARcCell<K> {
        // # Method only exists for compatibility, new code generators only generate calls to get_keystore
        self.get_keystore(&CFG.lock().unwrap().main_crypto_backend())
    }

    pub fn get_keystore(&self, crypto_backend: &str) -> ARcCell<K> {
        self.__keystore.lock()[crypto_backend].clone()
    }

    pub fn get_my_sk(&self, crypto_backend: &str) -> Value<String, PrivateKeyValue> {
        let crypto_backend = if crypto_backend.is_empty() {
            CFG.lock().unwrap().main_crypto_backend()
        } else {
            crypto_backend.to_owned()
        };
        self.__keystore.lock()[&crypto_backend]
            .lock()
            .sk(&self.user_address().to_string())
    }
    // = CFG.lock().unwrap().main_crypto_backend
    pub fn get_my_pk(&self, crypto_backend: &str) -> Value<String, PublicKeyValue> {
        let crypto_backend = if crypto_backend.is_empty() {
            CFG.lock().unwrap().main_crypto_backend()
        } else {
            crypto_backend.to_owned()
        };
        self.__keystore.lock()[&crypto_backend]
            .lock()
            .pk(&self.user_address().to_string())
    }

    pub async fn call_fct<F: AsyncFn()>(&self, sec_offset: i32, fct: F) {
        with_context_block!(var _cc=self.__call_ctx(sec_offset)=>{
             fct().await;
        });
    }

    // @staticmethod
    // """
    // Check whether a comparison with value 'val' can be evaluated correctly in the circuit.

    // :param val: the value to check
    // :raises ValueError:
    // """
    pub fn range_checked(val: BigInteger256) -> BigInteger256 {
        assert!(
            val < _BN128_COMP_SCALAR_FIELD,
            "Value {val} is too large for comparison, circuit would produce wrong results."
        );
        val
    }

    pub async fn deploy(
        &self,
        actual_args: Vec<String>,
        should_encrypt: Vec<bool>,
        wei_amount: Option<i32>,
    ) -> Option<Address> {
        println!(
            "==__contract_handle==begin====={:?}",
            self.__contract_handle.lock().clone()
        );
        *self.__contract_handle.lock() = self
            .__conn
            .lock()
            .deploy(
                &PathBuf::from(self.__project_dir.lock().clone()),
                &self.__user_addr.lock(),
                &self.__contract_name.lock(),
                actual_args,
                should_encrypt,
                wei_amount,
            )
            .await
            .ok();
        println!(
            "==__contract_handle======={:?}",
            self.__contract_handle.lock().clone()
        );
        self.__contract_handle.lock().clone()
    }
    pub async fn connect<PS: ProvingScheme>(
        &self,
        address: &Address,
        compile_zkay_file: fn(
            input_file_path: &str,
            output_dir: &str,
            import_keys: bool,
        ) -> anyhow::Result<()>,
        get_verification_contract_names: fn(code_or_ast: String) -> Vec<String>,
    ) {
        *self.__contract_handle.lock() = self
            .__conn
            .lock()
            .connect::<PS>(
                &PathBuf::from(self.__project_dir.lock().clone()),
                &self.__contract_name.lock(),
                address,
                self.user_address(),
                compile_zkay_file,
                get_verification_contract_names,
            )
            .await
            .ok();
    }

    pub async fn transact(
        &self,
        fname: &str,
        args: Vec<DataType>,
        should_encrypt: Vec<bool>,
        wei_amount: Option<i32>,
    ) -> eyre::Result<String> {
        self.__conn
            .lock()
            .transact(
                self.__contract_handle.lock().as_ref().unwrap(),
                &self.__user_addr.lock(),
                fname,
                args,
                should_encrypt,
                wei_amount,
            )
            .await
    }

    pub async fn call(
        &self,
        fname: &str,
        args: Vec<DataType>,
        ret_val_constructors: Vec<(bool, String, CallableType)>,
    ) -> DataType {
        let retvals = self
            .__conn
            .lock()
            .call(
                self.__contract_handle.lock().as_ref().unwrap(),
                &self.__user_addr.lock(),
                fname,
                args,
            )
            .await;
        if ret_val_constructors.len() == 1 {
            let (is_cipher, crypto_params_name, callable) = ret_val_constructors[0].clone();
            self.__get_decrypted_retval(retvals.unwrap(), is_cipher, crypto_params_name, callable)
        } else {
            DataType::List(
                [retvals]
                    .iter()
                    .zip(ret_val_constructors)
                    .map(|(retval, (is_cipher, homomorphism, constr))| {
                        self.__get_decrypted_retval(
                            retval.as_ref().unwrap().clone(),
                            is_cipher,
                            homomorphism,
                            constr,
                        )
                    })
                    .collect(),
            )
        }
    }
    pub fn __get_decrypted_retval(
        &self,
        raw_value: String,
        is_cipher: bool,
        crypto_params_name: String,
        constructor: CallableType,
    ) -> DataType {
        if is_cipher {
            self.dec(
                DataType::CipherValue(Value::<String, CipherValue>::new(
                    vec![raw_value],
                    Some(CryptoParams::new(crypto_params_name.clone())),
                    None,
                )),
                constructor,
                &crypto_params_name,
            )
            .0
        } else {
            constructor(raw_value)
        }
    }

    pub fn get_special_variables(
        &self,
    ) -> (
        ARcCell<Option<MsgStruct>>,
        ARcCell<Option<BlockStruct>>,
        ARcCell<Option<TxStruct>>,
    ) {
        // assert self.__current_msg is not None and self.__current_block is not None and self.__current_tx is not None
        (
            self.__current_msg.clone(),
            self.__current_block.clone(),
            self.__current_tx.clone(),
        )
    }

    pub async fn update_special_variables(&self, wei_amount: i32) {
        let (__current_msg, __current_block, __current_tx) = self
            .__conn
            .lock()
            .get_special_variables(&self.__user_addr.lock().to_string(), wei_amount)
            .await;
        (
            *self.__current_msg.lock(),
            *self.__current_block.lock(),
            *self.__current_tx.lock(),
        ) = (
            Some(__current_msg),
            Some(__current_block),
            Some(__current_tx),
        );
    }

    pub fn clear_special_variables(&mut self) {
        (
            *self.__current_msg.lock(),
            *self.__current_block.lock(),
            *self.__current_tx.lock(),
        ) = (None, None, None);
    }
    //= CFG.lock().unwrap().main_crypto_backend
    pub async fn enc(
        &self,
        plain: String,
        target_addr: Option<String>,
        crypto_backend: &str,
    ) -> (
        Value<String, CipherValue>,
        Option<Value<String, RandomnessValue>>,
    ) {
        println!("===target_addr===enc======={target_addr:?}=========");
        let target_addr = target_addr.map_or(self.__user_addr.lock().to_string(), |ta| ta);
        println!("===target_addr=========={target_addr}=========");
        self.__crypto
            .lock()
            .get(crypto_backend)
            .unwrap()
            .lock()
            .enc(plain, &self.__user_addr.lock().to_string(), &target_addr)
            .await
    }
    //= CFG.lock().unwrap().main_crypto_backend
    pub fn dec(
        &self,
        cipher: DataType,
        constr: CallableType,
        crypto_backend: &str,
    ) -> (DataType, Option<Value<String, RandomnessValue>>) {
        println!("==offchain====dec=cipher===********************************************=========={cipher:?}======");
        let res = self.__crypto.lock()[crypto_backend].lock().dec(
            cipher.try_as_cipher_value_ref().unwrap(),
            &self.__user_addr.lock().to_string(),
        );
        (constr(res.0.to_string()), res.1)
    }

    pub async fn do_homomorphic_op(
        &self,
        op: &str,
        crypto_backend: &str,
        target_addr: String,
        args: Vec<DataType>,
    ) -> Value<String, CipherValue> {
        let params = CryptoParams::new(crypto_backend.to_owned());
        let pk = self
            .__keystore
            .lock()
            .get(&params.crypto_name)
            .unwrap()
            .lock()
            .getPk(&target_addr)
            .await;
        // assert!(
        //     args.iter().all(|arg| !(isinstance(arg, CipherValue)
        //         && params.crypto_name != arg.params.crypto_name)),
        //     "CipherValues from different crypto backends used in homomorphic operation"
        // );

        let mut crypto_inst = self.__crypto.lock()[&params.crypto_name].clone();
        // assert isinstance(crypto_inst, ZkayHomomorphicCryptoInterface);
        let result = crypto_inst.lock().do_op(
            op,
            pk[..].to_vec(),
            args.iter()
                .map(|arg| arg.to_string().split(",").map(String::from).collect())
                .collect(),
        );
        Value::<String, CipherValue>::new(result, Some(params), None)
    }

    // """
    // Re-randomizes arg using fresh randomness, which is stored in data[rnd_key] (side-effect!)
    // """
    pub async fn do_rerand(
        &self,
        arg: Value<String, CipherValue>,
        crypto_backend: &str,
        target_addr: String,
        data: &mut BTreeMap<String, String>,
        rnd_key: &str,
    ) {
        let params = CryptoParams::new(crypto_backend.to_owned());
        let pk = self
            .__keystore
            .lock()
            .get(&params.crypto_name)
            .unwrap()
            .lock()
            .getPk(&target_addr)
            .await;
        let mut crypto_inst = self.__crypto.lock()[&params.crypto_name].clone();
        // assert isinstance(crypto_inst, ZkayHomomorphicCryptoInterface);
        let (_result, _rand) = crypto_inst.lock().do_rerand(arg, pk[..].to_vec());
        data.insert(rnd_key.to_owned(), params.crypto_name.clone()); //# store randomness
        // CipherValue(result, params)
    }

    pub async fn _req_state_var(&self, name: &str, indices: Vec<String>, count: i32) -> String {
        // if self.__contract_handle is None:
        // # TODO check this statically in the type checker
        assert!(
            self.__contract_handle.lock().is_some(),
            "Cannot read state variable {name} within constructor before it is assigned a value."
        );
        println!("==off===_req_state_var==================={name}===={indices:?}=={count}=");
        if count == 0 {
            self.__conn
                .lock()
                .req_state_var(
                    self.__contract_handle.lock().as_ref().unwrap(),
                    name,
                    indices,
                )
                .await
        } else {
            (0..count)
                .map(|i| {
                    futures::executor::block_on(self.__conn.lock().req_state_var(
                        self.__contract_handle.lock().as_ref().unwrap(),
                        &name,
                        indices.iter().cloned().chain([i.to_string()]).collect(),
                    ))
                })
                .collect::<Vec<String>>()
                .join(",")
        }
    }
    // @staticmethod
    pub fn __serialize_val(val: DataType, bitwidth: i32) -> String {
        println!("===__serialize_val==, val===============,{val}");
        // if isinstance(val, AddressValue):
        //     val = i32.from_be_bytes(val.val)
        // else if isinstance(val, IntEnum):
        //     val = val.value
        // elif isinstance(val, bool):
        //     val = i32(val)
        // elif isinstance(val, i32):
        //     if val < 0:
        //         val = int_cast(val, bitwidth, signed=False)
        //     elif bitwidth == 256:
        //         val %= bn128_scalar_field
        // val
        match &val {
            DataType::AddressValue(v) => v.contents[0].clone(),
            DataType::Bool(v) => (*v as u8).to_string(),
            DataType::Int(v) => {
                if *v < BigInt::zero() {
                    int_cast(val, (bitwidth != 0).then(|| bitwidth), false)
                } else if bitwidth == 256 {
                    (v.clone() % BigInt::parse_bytes(BN128_SCALAR_FIELD.as_bytes(), 10).unwrap())
                        .to_string()
                } else {
                    v.to_string()
                }
            }
            DataType::String(v) => {
                let Some(v) = BigInt::parse_bytes(v.as_bytes(), 10) else {
                    return v.clone();
                };
                if v < BigInt::zero() {
                    int_cast(val, (bitwidth != 0).then(|| bitwidth), false)
                } else if bitwidth == 256 {
                    (v % BigInt::parse_bytes(BN128_SCALAR_FIELD.as_bytes(), 10).unwrap())
                        .to_string()
                } else {
                    v.to_string()
                }
            }
            _ => {
                panic!("unexpected val {val:?}");
            }
        }
    }
    // @staticmethod
    pub fn __serialize_circuit_array(
        data: BTreeMap<String, DataType>,
        target_array: &mut Vec<String>,
        target_out_start_idx: i32,
        elem_bitwidths: Vec<i32>,
    ) {
        let mut idx = target_out_start_idx as usize;
        for ((_name, val), &bitwidth) in data.iter().zip(&elem_bitwidths) {
            println!("=====_name, val==============={_name},{val}");
            // if isinstance(val, (list, Value)) && !isinstance(val, AddressValue) {
            //     target_array[idx..idx + vallen()] =
            //     // if isinstance(val, CipherValue) {
            //     //     val[..val.len()]
            //     // } else {
            //     //     val[..]
            //     // }
            //     val
            //     ;
            //     idx += len(val);
            // } else {
            // target_array[idx as usize] = ApiWrapper::<P, B, K>::__serialize_val(val, bitwidth);
            // idx += 1;
            // }
            match val {
                DataType::CipherValue(v) => {
                    target_array[idx..idx + v.contents.len()].clone_from_slice(&v[..]);
                    idx += v.contents.len();
                }
                DataType::PrivateKeyValue(v) => {
                    target_array[idx..idx + v.contents.len()].clone_from_slice(&v[..]);
                    idx += v.contents.len();
                }
                DataType::PublicKeyValue(v) => {
                    target_array[idx..idx + v.contents.len()].clone_from_slice(&v[..]);
                    idx += v.contents.len();
                }
                DataType::RandomnessValue(v) => {
                    target_array[idx..idx + v.contents.len()].clone_from_slice(&v[..]);
                    idx += v.contents.len();
                }
                DataType::List(v) => {
                    //TODO recursive list
                    target_array[idx..idx + v.len()]
                        .clone_from_slice(&v.iter().map(|x| x.to_string()).collect::<Vec<_>>());
                    idx += v.len();
                }
                _ => {
                    target_array[idx] =
                        ApiWrapper::<P, B, K>::__serialize_val(val.clone(), bitwidth);
                    idx += 1;
                }
            }
        }
    }
    pub fn serialize_circuit_outputs(
        &self,
        zk_data: BTreeMap<String, DataType>,
        out_elem_bitwidths: Vec<i32>,
    ) -> Vec<String> {
        // # TODO don't depend on out var names for correctness
        let out_vals: BTreeMap<_, _> = zk_data
            .clone()
            .into_iter()
            .filter(|(name, _val)| name.starts_with(&CFG.lock().unwrap().zk_out_name()))
            .collect();

        let count = out_vals.values().map(|val| val.len()).sum::<usize>();
        let mut zk_out = vec![String::new(); count];
        Self::__serialize_circuit_array(out_vals, &mut zk_out, 0, out_elem_bitwidths);
        zk_out
    }
    pub fn serialize_private_inputs(
        &self,
        zk_priv: BTreeMap<String, DataType>,
        priv_elem_bitwidths: Vec<i32>,
    ) {
        let mut all_priv_values = self.all_priv_values.lock().as_ref().unwrap().clone();
        Self::__serialize_circuit_array(
            zk_priv,
            &mut all_priv_values,
            self.current_all_index.lock().as_ref().unwrap().clone(),
            priv_elem_bitwidths,
        );
        *self.all_priv_values.lock() = Some(all_priv_values);
    }

    pub fn gen_proof(
        &self,
        fname: &str,
        in_vals: Vec<String>,
        out_vals: Vec<String>,
    ) -> Vec<String> {
        self.__prover.lock().generate_proof(
            &self.__project_dir.lock(),
            self.__contract_name.lock().clone(),
            fname.to_owned(),
            self.all_priv_values.lock().clone().unwrap(),
            in_vals,
            out_vals,
        )
    }

    // @contextmanager
    // """Return context manager which sets the correct current_all_index for the given sec_offset during its lifetime."""
    pub fn __call_ctx(&self, sec_offset: i32) -> WithCalCtx {
        WithCalCtx::new(
            sec_offset,
            self.current_priv_values.clone(),
            self.current_all_index.clone(),
        )
    }
    // @contextmanager
    pub async fn api_function_ctx(
        &self,
        trans_sec_size: i32,
        wei_amount: Option<i32>,
    ) -> WithApiFunctionCtx {
        println!(
            "===========api_function_ctx==============aaa======{}===",
            line!()
        );
        let was_external = *self.is_external.lock();
        if was_external.is_none() {
            assert!(self.all_priv_values.lock().is_none());
            *self.is_external.lock() = Some(true);
            println!("=trans_sec_size===={trans_sec_size}");
            *self.all_priv_values.lock() =
                Some(vec![0.to_string(); trans_sec_size.max(0) as usize]);
            *self.current_all_index.lock() = Some(0);
            self.current_priv_values.lock().clear();
            self.update_special_variables(wei_amount.unwrap()).await;
        } else {
            *self.is_external.lock() = Some(false);
        }
        WithApiFunctionCtx::new(
            self.__current_msg.clone(),
            self.__current_block.clone(),
            self.__current_tx.clone(),
            self.current_priv_values.clone(),
            self.all_priv_values.clone(),
            self.current_all_index.clone(),
            self.is_external.clone(),
            was_external,
        )
        // try:
        //     yield self.is_external
        // finally:
        //     if self.is_external:
        //         assert was_external is None
        //         self.all_priv_values = None
        //         self.current_all_index = 0
        //         self.current_priv_values.clear()
        //         self.clear_special_variables()

        //     self.is_external = was_external
    }
}

#[allow(drop_bounds)]
pub struct WithFunctionCtx<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
    A: Drop,
    L: Drop,
> {
    locals: ARcCell<LocalsDict>,
    prev_locals: LocalsDict,
    state: ARcCell<StateDict<P, B, K, C>>,
    api_ctx: Option<A>,
    log_ctx: Option<L>,
}

#[allow(drop_bounds)]
impl<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
    A: Drop,
    L: Drop,
> WithFunctionCtx<C, P, B, K, A, L>
{
    pub fn new(
        locals: ARcCell<LocalsDict>,
        state: ARcCell<StateDict<P, B, K, C>>,
        api_ctx: Option<A>,
        log_ctx: Option<L>,
    ) -> Self {
        let prev_locals = locals.lock().clone();
        *locals.lock() = LocalsDict { _scopes: vec![] };
        Self {
            locals,
            prev_locals,
            state,
            api_ctx,
            log_ctx,
        }
    }
}

#[allow(drop_bounds)]
impl<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
    A: Drop,
    L: Drop,
> Drop for WithFunctionCtx<C, P, B, K, A, L>
{
    fn drop(&mut self) {
        *self.locals.lock() = self.prev_locals.clone();
        self.state.lock().clear();
        if let Some(log_ctx) = self.log_ctx.take() {
            drop(log_ctx);
        }
        if let Some(api_ctx) = self.api_ctx.take() {
            drop(api_ctx);
        }
    }
}

pub struct WithApiFunctionCtx {
    __current_msg: ARcCell<Option<MsgStruct>>,
    __current_block: ARcCell<Option<BlockStruct>>,
    __current_tx: ARcCell<Option<TxStruct>>,
    current_priv_values: ARcCell<BTreeMap<String, i32>>,
    all_priv_values: ARcCell<Option<Vec<String>>>,
    current_all_index: ARcCell<Option<i32>>,
    is_external: ARcCell<Option<bool>>,
    was_external: Option<bool>,
}
impl WithApiFunctionCtx {
    pub fn new(
        __current_msg: ARcCell<Option<MsgStruct>>,
        __current_block: ARcCell<Option<BlockStruct>>,
        __current_tx: ARcCell<Option<TxStruct>>,
        current_priv_values: ARcCell<BTreeMap<String, i32>>,
        all_priv_values: ARcCell<Option<Vec<String>>>,
        current_all_index: ARcCell<Option<i32>>,
        is_external: ARcCell<Option<bool>>,
        was_external: Option<bool>,
    ) -> Self {
        Self {
            __current_msg,
            __current_block,
            __current_tx,
            current_priv_values,
            all_priv_values,
            current_all_index,
            is_external,
            was_external,
        }
    }
}

impl Drop for WithApiFunctionCtx {
    fn drop(&mut self) {
        if self.is_external.lock().map_or(false, |x| x) {
            assert!(self.was_external.is_none());
            *self.all_priv_values.lock() = None;
            *self.current_all_index.lock() = Some(0);
            self.current_priv_values.lock().clear();
            (
                *self.__current_msg.lock(),
                *self.__current_block.lock(),
                *self.__current_tx.lock(),
            ) = (None, None, None);
        }
        *self.is_external.lock() = self.was_external.clone();
    }
}

pub struct WithScope {
    locals: ARcCell<LocalsDict>,
}
impl WithScope {
    pub fn new(locals: ARcCell<LocalsDict>) -> Self {
        locals.lock().push_scope();
        Self { locals }
    }
}

impl Drop for WithScope {
    fn drop(&mut self) {
        self.locals.lock().pop_scope();
    }
}

pub struct WithCalCtx {
    current_priv_values: ARcCell<BTreeMap<String, i32>>,
    current_all_index: ARcCell<Option<i32>>,
    old_priv_values: BTreeMap<String, i32>,
    old_all_index: Option<i32>,
}
impl WithCalCtx {
    pub fn new(
        sec_offset: i32,
        current_priv_values: ARcCell<BTreeMap<String, i32>>,
        current_all_index: ARcCell<Option<i32>>,
    ) -> Self {
        let (old_priv_values, old_all_index) = (
            current_priv_values.lock().clone(),
            current_all_index.lock().clone(),
        );

        *current_priv_values.lock() = BTreeMap::new();
        *current_all_index.lock().as_mut().unwrap() += sec_offset;
        Self {
            current_priv_values,
            current_all_index,
            old_priv_values,
            old_all_index,
        }
    }
}

impl Drop for WithCalCtx {
    fn drop(&mut self) {
        (
            *self.current_priv_values.lock(),
            *self.current_all_index.lock(),
        ) = (self.old_priv_values.clone(), self.old_all_index.clone());
    }
}

pub type BlockchainClassType = BlockchainClass<JsnarkProver>;
pub type KeystoreType = SimpleKeystore<JsnarkProver, BlockchainClassType>;
pub type CryptoClassType = CryptoClass<JsnarkProver, BlockchainClassType, KeystoreType>;

pub fn new_contract_simulator(
    project_dir: &str,
    contract_name: &str,
    user_addr: Address,
    web3tx: Web3Tx,
) -> ContractSimulator<CryptoClassType, JsnarkProver, BlockchainClassType, KeystoreType> {
    // -> ContractSimulator<
    //     CryptoClass<
    //         JsnarkProver,
    //         BlockchainClass<JsnarkProver>,
    //         SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
    //     >,
    //     JsnarkProver,
    //     BlockchainClass<JsnarkProver>,
    //     SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
    // >
    // contract_simulator.use_config_from_manifest(file!());
    // os.path.dirname(os.path.realpath(__file__);
    // let me = contract_simulator.default_address();
    // if me.is_some(){
    //     me = me.val;
    // }
    // import code
    // code.interact(local=globals())
    let __prover = arc_cell_new!(_prover_classes(&CFG.lock().unwrap().snark_backend()));
    let __blockchain = arc_cell_new!(_blockchain_classes(
        &CFG.lock().unwrap().blockchain_backend(),
        __prover.clone(),
        web3tx,
    ));
    // let __keystore=BTreeMap::from([SimpleKeystore::<P,BlockchainClass<P>>::new(blockchain.clone(), crypto_params.clone())]);
    let mut __keystore = BTreeMap::new();
    let mut __crypto = BTreeMap::new();
    for crypto_params in CFG.lock().unwrap().all_crypto_params() {
        let crypto_param = CryptoParams::new(crypto_params.clone());
        let crypto_backend = crypto_param.crypto_name.clone();
        let keystore = arc_cell_new!(
            SimpleKeystore::<JsnarkProver, BlockchainClass<JsnarkProver>>::new(
                __blockchain.clone(),
                crypto_param.clone(),
            )
        );
        __keystore.insert(crypto_params.clone(), keystore.clone());
        let crypto = arc_cell_new!(_crypto_classes::<
            JsnarkProver,
            BlockchainClass<JsnarkProver>,
            SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
        >(&crypto_backend, keystore));
        __crypto.insert(crypto_params.clone(), crypto);
    }
    let __crypto = arc_cell_new!(__crypto);
    let __keystore = arc_cell_new!(__keystore);
    let runtime = arc_cell_new!(Runtime::new(
        __blockchain.clone(),
        __crypto.clone(),
        __keystore.clone(),
        __prover.clone(),
    ));
    // let contract_simulator=ContractSimulator::new(".","","",runtime.clone());
    // arc_cell_new!(_crypto_classes::<P, B, K>(&crypto_backend, keystore)),

    // let runtime=arc_cell_new!(Runtime::<JsnarkProver,BlockchainClass::<JsnarkProver>,SimpleKeystore::<JsnarkProver,BlockchainClass::<JsnarkProver>>>::new());
    // fn f(crypto_params:&CryptoParams)->ARcCell<SimpleKeystore>{
    //     arc_cell_new!(SimpleKeystore::new(runtime.lock().blockchain(),crypto_params.clone()))
    // }
    let api = arc_cell_new!(ApiWrapper::<
        JsnarkProver,
        BlockchainClass<JsnarkProver>,
        SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
    >::new(
        project_dir,
        contract_name,
        user_addr,
        __blockchain.clone(),
        __keystore.clone(),
        __crypto.clone(),
        __prover,
    ));
    ContractSimulator::<CryptoClassType, JsnarkProver, BlockchainClassType, KeystoreType>::new(
        runtime, api,
    )
}
