#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// from __future__ import annotations

// import inspect
// from contextlib import contextmanager, nullcontext
// from enum import IntEnum
// from typing import Dict, Union, Callable, Any, Optional, List, Tuple, ContextManager
use crate::runtime::CryptoClass;
use crate::keystore::simple::SimpleKeystore;
use crate::prover::jsnark::JsnarkProver;
use proving_scheme::proving_scheme::ProvingScheme;
use crate::runtime::BlockchainClass;
use std::str::FromStr;
use serde_json::{json, Map, Result, Value as JsonValue};

use my_logging::log_context::log_context;
use privacy::library_contracts::BN128_SCALAR_FIELDS;
use privacy::manifest::Manifest;
use std::collections::BTreeMap;
use zkay_config::config::{zk_print_banner, CFG};
use zkay_config::config_user::UserConfig;
use zkay_transaction_crypto_params::params::CryptoParams;
// use zkay::transaction::int_casts :: __convert as int_cast;
use crate::runtime::Runtime;
use crate::blockchain::web3rs::Web3Blockchain;
use ark_ff::{BigInteger, BigInteger256, Field, MontFp, PrimeField};
use zkay_ast::homomorphism::Homomorphism;
use crate::interface::{ZkayProverInterface,ZkayCryptoInterface,
    ZkayBlockchainInterface, ZkayHomomorphicCryptoInterface, ZkayKeystoreInterface,
};
use crate::types::{
    AddressValue, BlockStruct, CipherValue, MsgStruct, PrivateKeyValue, PublicKeyValue,
    RandomnessValue, TxStruct, Value,
};
use zkay_utils::progress_printer::fail_print;
// bn128_scalar_field = bn128_scalar_field
const _BN128_COMP_SCALAR_FIELD: BigInteger256 = BigInteger256::one();
type CallableType<T: Clone + Default+ std::iter::FromIterator<T>,> = fn(BigInteger256)->T;
// class RequireException(Exception):
//     pass

// class StateDict:
//     """Dictionary which wraps access to state variables"""
use std::marker::PhantomData;
pub struct StateDict<
    S:Clone+Default,
    Callable: Fn(BigInteger256)->T,
    P:ZkayProverInterface+Clone,
    B: ZkayBlockchainInterface<P>+Clone,
    K: ZkayKeystoreInterface<P,B>+Clone,
    C: ZkayCryptoInterface<P,B,K>+ZkayHomomorphicCryptoInterface<P, B, K>+Clone,
    T: Clone + Default+ std::iter::FromIterator<T>,
> {
    api: ApiWrapper<P,B, K>,
    __state: BTreeMap<String, S>, 
    __constructors: BTreeMap<String, (bool, CryptoParams, Callable)>,
    _prover:PhantomData<P>,
    _bc:PhantomData<B>,
 _crypto:PhantomData<C>,
}
impl<S:Clone+Default, Callable: Fn(BigInteger256)->T, P:ZkayProverInterface+Clone,B: ZkayBlockchainInterface<P>+Clone, K: ZkayKeystoreInterface<P,B>+Clone,C: ZkayCryptoInterface<P,B,K>+ZkayHomomorphicCryptoInterface<P, B, K>+Clone,  T: Clone + Default+ std::iter::FromIterator<T>>
    StateDict<S, Callable,P, B, K, C, T>
{
        pub fn new(api:ApiWrapper<P,B, K>)->Self{
            Self{api,__state:BTreeMap::new(),__constructors:BTreeMap::new(),_prover:PhantomData,
                _bc:PhantomData,_crypto:PhantomData,
            }
        }
    //     fn __init__(&self, api) -> None:
    //         self.api = api
    //         self.__state: Dict[str, Any] = {}
    //         self.__constructors: Dict[str, (bool, CryptoParams, Callable)] = {}

    fn clear(&mut self) {
        self.__state.clear();
    }
    // """Define the wrapper constructor for a state variable."""= CFG.lock().unwrap().main_crypto_backend
    fn decl(
        &mut self,
        name: &str,
        constructor: Callable,
        args: Vec<String>,
        cipher: bool,
        crypto_backend: &str,
    ) {
        // assert name not in self.__constructors
        self.__constructors.insert(
            name.to_owned(),
            (cipher, CryptoParams::new(crypto_backend.to_owned()), constructor),
        );
    }

    //     @property
    fn names(&self) -> Vec<String> {
        self.__constructors.keys().cloned().collect()
    }

    fn get_plain(&self, name: &str, indices: Vec<String>)->Option<S>  {
        let (is_cipher, crypto_params, constr) = &self.__constructors[name];
        // let val = self.__get(vec![name.to_owned()].into_iter().chain(indices.into_iter()).collect(), false);
        // if is_cipher {
        //     let (ret, _) = self.api.dec(val, constr, &crypto_params.crypto_name);
        //     return ret
        // }
        //   val
        None
        
    }

    fn get_raw(&self, name: &str, indices: Vec<String>)->Option<S>  {
        self.__get(vec![name.to_owned()].into_iter().chain(indices.into_iter()).collect(), false)
    }

    // """
    // Return value of the state variable (or index of state variable) key

    // :param key: Either a string with the state variable name (primitive variables) or a Tuple with the name and all index key values
    // :raise KeyError: if location does not exist on the chain
    // :return: The requested value
    // """
    fn __getitem__(&self, key: &str)->Option<S>  {
        self.__get(vec![key.to_owned()], true)
    }

    // """
    // Assign value to state variable (or to index of state variable)

    // :param key: Either a string with the state variable name (primitive variables) or a Tuple with the name and all index key values
    // :param value: Correctly wrapped value which should be assigned to the specified state location
    // """
    fn __setitem__(&mut self, mut key: Vec<String>, value: S) {
        // if not isinstance(key, Tuple)
        //     key = (key, )
        let var = key[0].clone();
        let loc = var + &key[1..].iter().map(|k| format!("[{k}]")).collect::<Vec<_>>().concat();

        // # Write to state
        self.__state.insert(loc, value);
    }
    fn __get(&self, key: Vec<String>, cache: bool)->Option<S> {
        // if not isinstance(key, Tuple):
        //     key = (key, )
        let (var, indices) = (&key[0], &key[1..]);
        let loc = var.to_owned() + &key[1..].iter().map(|k| format!("[{k}]")).collect::<Vec<_>>().concat();

        // # Retrieve from state scope
        if cache && self.__state.contains_key(&loc) {
            return self.__state.get(&loc).cloned();
        }

        // let (is_cipher, crypto_params, constr) = self.__constructors[var.as_str()];
        // try:
        // let val = if is_cipher {
        //     let cipher_len = crypto_params.cipher_len();
        //     CipherValue(
        //         &self.api._req_state_var(&var, indices, cipher_len),
        //         crypto_params,
        //     )
        // } else {
        //     constr(&self.api._req_state_var(var, indices))
        // };
        // // except BlockChainError:
        // //     raise KeyError(key)
        // if cache {
        //     self.__state.insert(loc, val.clone());
        // }
        // val
        None
    }
}
// class LocalsDict:
//     """
//     Dictionary which supports multiple scopes with name shadowing.
#[derive(Clone)]
pub struct LocalsDict {
    pub _scopes: Vec<BTreeMap<String, String>>,
}
impl LocalsDict {
    //     This is needed since python does not natively support c-style nested local scopes.
    //     """
    //     fn __init__(&self) -> None:
    //         self._scopes: List[dict] = [{}]
    // """Introduce a new scope."""
    fn push_scope(&mut self) {
        self._scopes.push(BTreeMap::new());
    }

    // """End the current scope."""
    fn pop_scope(&mut self) {
        self._scopes.pop();
    }

    // """Introduce a new local variable with the given name and value into the current scope."""
    fn decl(&mut self, name: &String, val: String) {
        assert!(
            !self._scopes.last().unwrap().contains_key(name),
            "Variable declared twice in same scope"
        );
        self._scopes.last_mut().unwrap().insert(name.clone(), val);
    }

    // """
    // Return the value of the local variable which is referenced by the identifier key in the current scope.

    // If there are multiple variables with the name key in different scopes,
    // the variable with the lowest declaration scope is used.
    // """
    fn __getitem__(&self, key: &str)->String {
        for scope in self._scopes.iter().rev() {
            if let Some(v) = scope.get(key) {
                return v.clone();
            }
        }
        panic!("Variable not found");
    }

    // """
    // Assign value to the local variable which is referenced by the identifier key in the current scope.

    // If there are multiple variables with the name key in different scopes, the variable with the lowest declaration scope is used.
    // """
    fn __setitem__(&mut self, key: &String, value: String) {
        for scope in self._scopes.iter_mut().rev() {
            if scope.contains_key(key) {
                scope.insert(key.clone(), value);
                return;
            }
        }
        panic!("Variable not found");
    }
}

pub struct ContractSimulator<S:Clone+Default,C: ZkayCryptoInterface<P,B,K>+ZkayHomomorphicCryptoInterface<P, B, K>+Clone,
    P:ZkayProverInterface+Clone,
    B: ZkayBlockchainInterface<P>+Web3Blockchain+Clone,
    K: ZkayKeystoreInterface<P,B>+Clone,
    T: Clone + Default+ std::iter::FromIterator<T>,
    Callable: Fn(BigInteger256)->T,
> {
    tidx: BTreeMap<String, i32>,
    api: ApiWrapper<P,B, K>,
    locals:LocalsDict,
    state:StateDict<S,Callable,P,B,K,C,T>,
    runtime:Runtime<P,B,K>
}
impl<S:Clone+Default,C: ZkayCryptoInterface<P,B,K>+ZkayHomomorphicCryptoInterface<P, B, K>+Clone, P:ZkayProverInterface+Clone,B: ZkayBlockchainInterface<P> +Web3Blockchain+Clone, K: ZkayKeystoreInterface<P,B>+Clone, T: Clone + Default+ std::iter::FromIterator<T>,Callable: Fn(BigInteger256)->T,>
    ContractSimulator<S,C,P,B, K,  T,Callable>
{
    // """
    //     Create new contract simulator instance.
    //     :param project_dir: Directory where the zkay contract, the manifest and the prover/verification key files are located
    //     :param user_addr: From address for all transactions which are issued by this ContractSimulator
    //     """
    fn new(project_dir: &str, user_addr: &str, contract_name: &str,mut runtime:Runtime<P,B,K>,k:K) -> Self {
        // # Transaction instance values (reset between transactions)
        // let  mut runtime=Runtime::<P,B,SimpleKeystore<P, BlockchainClass<P>>>::new();
        let api = ApiWrapper::<P,B,K>::new(project_dir, contract_name, user_addr,&mut runtime,k);

        // """Hierarchical dictionary (scopes are managed internally) which holds the currently accessible local variables"""
        let locals = LocalsDict{_scopes:vec![]};

        // """
        // Dict which stores stores state variable values. Empty at the beginning of a transaction.
        // State variable read: 1. if not in dict -> request from chain and insert into dict, 2. return dict value
        // State variable write: store in dict
        // """
        let state = StateDict::<S,Callable,P,B,K,C,T>::new(api.clone());
        Self {
            tidx: BTreeMap::new(),
            api,
            locals,
            state,
            runtime
        }
    }

    // @property
    fn address(&self)->String {
        // self.api.address().clone()
        String::new()
    }

    // @contextmanager
    // """Return context manager which manages the lifetime of a local scope."""
    fn _scope(&mut self) {
        self.locals.push_scope();
        // yield
        self.locals.pop_scope();
    }

    // @staticmethod
    fn help(_module: &str, _contract: &str, _contract_name: &str) {
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
    fn reduced_help(_contract: &str) {
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
    fn default_address(&self) -> Option<AddressValue> {
        // (*self.runtime.blockchain()).default_address().clone()
        None
    }

    // @staticmethod
    // """Generate/Load keys for the given address."""
    fn initialize_keys_for(&mut self,address: &str) {
        let account = address.to_owned();
        for crypto_params in CFG.lock().unwrap().all_crypto_params() {
            // if !self.runtime.keystore(&CryptoParams::new(crypto_params.clone())).has_initialized_keys_for(&address.to_owned()) {
            //     self.runtime.crypto(&CryptoParams::new(crypto_params.clone())).generate_or_load_key_pair(&account)
            // };
        }
    }

    // @staticmethod
    fn use_config_from_manifest(&mut self,project_dir: &str) {
        // """Override zkay configuration with values from the manifest file in project_dir."""
        let manifest = Manifest::load(project_dir);
        Manifest::import_manifest_config(manifest);
        self.runtime.reset();
    }

    // @staticmethod
    // """
    // Create count pre-funded dummy accounts (if supported by backend)

    // :param count: # of accounts to create
    // :return: if count == 1 -> returns a address, otherwise returns a tuple of count addresses
    // """
    fn create_dummy_accounts(&mut self, count: i32) -> Vec<String> {
        let accounts:Vec<String> = vec![];//self.runtime.blockchain().create_test_accounts(count);
        for account in &accounts {
            self.initialize_keys_for(account);
        }
        if accounts.len() == 1 {
            accounts[..1].to_vec()
        } else {
            accounts
        }
    }

    // @contextmanager   -1  = 0 = '?'
    fn _function_ctx(&mut self, trans_sec_size: i32, args: Vec<String>, wei_amount: i32, name: &str) {
        // with self.api.api_function_ctx(trans_sec_size, wei_amount) as is_external:
        if false {
            zk_print_banner(format!("Calling {name}"));
            // assert self.locals is None
            self.state.clear();
            // let t_idx = self.tidx.get(name).unwrap_or(0);
            *self.tidx.entry(name.to_owned()).or_insert(0) += 1;
        }
        // with nullcontext() if not is_external else log_context(f'{name}_{t_idx}'):
        let prev_locals = self.locals.clone();
        self.locals = LocalsDict { _scopes: vec![] };

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
    }
}
#[derive(Clone)]
pub struct ApiWrapper<P:ZkayProverInterface+Clone,B: ZkayBlockchainInterface<P>+Clone, K: ZkayKeystoreInterface<P,B>+Clone> {
    __conn: BlockchainClass<P>,
    __keystore: BTreeMap<String, K>,
    __crypto: BTreeMap<String, CryptoClass<P,B,K>>,
    __prover: JsnarkProver,
    __project_dir: String,
    __contract_name: String,
    __contract_handle: Option<JsonValue>,
    __user_addr: String,
    __current_msg: Option<MsgStruct>,
    __current_block: Option<BlockStruct>,
    __current_tx: Option<TxStruct>,
    current_priv_values: BTreeMap<String, i32>,
    all_priv_values: Option<Vec<i32>>,
    current_all_index: Option<i32>,
    is_external: Option<bool>,
}
impl<P:ZkayProverInterface+Clone,B: ZkayBlockchainInterface<P>+Web3Blockchain+Clone, K: ZkayKeystoreInterface<P,B>+Clone>
    ApiWrapper< P,B, K>
{
    fn new(project_dir: &str, contract_name: &str, user_addr: &str,runtime:&mut Runtime<P,B,  K>,k:K) -> Self {
        // super().__init__()
        let __conn:BlockchainClass<P> = runtime.blockchain().clone();
        let mut __keystore = BTreeMap::new();
        let mut __crypto = BTreeMap::new();
        let __prover = runtime.prover().clone();

        for crypto_params in CFG.lock().unwrap().all_crypto_params() {
            __keystore.insert(crypto_params.clone(), runtime.keystore(&CryptoParams::new(crypto_params.clone()),k.clone()).clone());
            __crypto.insert(crypto_params.clone(), runtime.crypto(&CryptoParams::new(crypto_params.clone()),k.clone()).clone());
        }

        let __project_dir = project_dir.to_owned();
        let __contract_name = contract_name.to_owned();

        // """Handle which refers to the deployed contract, this is passed to the blockchain interface when e.g. issuing transactions."""
        let __contract_handle = None;

        // """From address for all transactions which are issued by this ContractSimulator"""
        let __user_addr = user_addr.to_owned();

        // """
        // Builtin variable (msg, block, tx) values for the current transaction
        // """
        let __current_msg: Option<MsgStruct> = None;
        let __current_block: Option<BlockStruct> = None;
        let __current_tx: Option<TxStruct> = None;

        // """Dictionary which stores the private circuit values (secret inputs) for the current function (no transitivity)"""
        let current_priv_values: BTreeMap<String, i32> = BTreeMap::new();

        // """List which stores all secret circuit inputs for the current transaction in correct order (order of use)"""
        let all_priv_values: Option<Vec<i32>> = None;

        // """
        // Index which designates where in all_priv_values the secret circuit inputs of the current function should be inserted.
        // This is basically private analogue of the start_index parameters which are passed to functions which require verification
        // to designate where in the public IO arrays the functions should store/retrieve public circuit inputs/outputs.
        // """
        let current_all_index: Option<i32> = None;

        // """
        // True whenever simulation is inside a function which was directly (without transitivity) called by the user.
        // This is mostly used for some checks (e.g. to prevent the user from calling internal functions), or to change
        // function behavior depending on whether a call is external or not (e.g. encrypting parameters or not)
        // """
        let is_external: Option<bool> = None;
        Self{
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
    all_priv_values   ,
    current_all_index,
    is_external}
    }
    // @property
    fn address(&self)->String {
        self.__contract_handle.as_ref().unwrap()["address"].to_string()
    }

    // @property
    fn user_address(&self)->String {
        self.__user_addr.clone()
    }

    // @property
    fn keystore(&self) -> &K {
        // # Method only exists for compatibility, new code generators only generate calls to get_keystore
        self.get_keystore(&CFG.lock().unwrap().main_crypto_backend())
    }

    fn get_keystore(&self, crypto_backend: &str)->&K{
        &self.__keystore[crypto_backend]
    }

    fn get_my_sk(&self, crypto_backend: &str) -> Vec<u8> {
        // = CFG.lock().unwrap().main_crypto_backend
        // self.__keystore[crypto_backend].sk(&self.user_address())
        vec![]
    }
    // = CFG.lock().unwrap().main_crypto_backend
    fn get_my_pk(&self, crypto_backend: &str) -> Vec<u8> {
        self.__keystore[crypto_backend].pk(&self.user_address())
    }

    fn call_fct<F: Fn(Vec<String>)>(&self, _sec_offset: i32, fct: F, args: Vec<String>) {
        // with self.__call_ctx(sec_offset):
        fct(args)
    }

    // @staticmethod
    // """
    // Check whether a comparison with value 'val' can be evaluated correctly in the circuit.

    // :param val: the value to check
    // :raises ValueError:
    // """
    fn range_checked(val: BigInteger256)->BigInteger256 {
        assert!(
            val < _BN128_COMP_SCALAR_FIELD,
            "Value {val} is too large for comparison, circuit would produce wrong results."
        );
        val
    }

    fn deploy(
        &mut self,
        actual_args: String,
        should_encrypt: Vec<bool>,
        wei_amount: Option<i32>,
    )->Option<JsonValue> {
        self.__contract_handle = self.__conn.deploy(
            &self.__project_dir,
            &self.__user_addr,
            &self.__contract_name,
            actual_args,
            should_encrypt,
            wei_amount,
        ).ok();
        self.__contract_handle.clone()
    }
    fn connect<PS: ProvingScheme>(&mut self, address: JsonValue,
       compile_zkay_file: fn(
            input_file_path: &str,
            output_dir: &str,
            import_keys: bool,
        ) -> anyhow::Result<String>,
        get_verification_contract_names: fn(code_or_ast: String) -> Vec<String>,
    ) {
        self.__contract_handle = self.__conn.connect::<PS>(
            &self.__project_dir,
            &self.__contract_name,
            address,
            self.user_address(),
            compile_zkay_file,
            get_verification_contract_names,
        ).ok();
    }

    fn transact(
        &self,
        fname: &str,
        args: Vec<String>,
        should_encrypt: Vec<bool>,
        wei_amount: Option<i32>,
    ) {
        self.__conn.transact(
            self.__contract_handle.as_ref().unwrap(),
            &self.__user_addr,
            fname,
            args.concat(),
            should_encrypt,
            wei_amount,
        )
    }

    fn call<T>(
        &self,
        fname: &str,
        args: Vec<String>,
        ret_val_constructors: Vec<(bool, String, CallableType<T>)>,
    ) ->Vec<T>{
        let retvals = self
            .__conn
            .call(self.__contract_handle.as_ref().unwrap().clone(), &self.__user_addr, fname, args.concat());
        if ret_val_constructors.len() == 1 {
            let (is_cipher,crypto_params_name,callable)=ret_val_constructors[0].clone();
            vec![self.__get_decrypted_retval(BigInteger256::from_str(&retvals).unwrap(), is_cipher,crypto_params_name,callable)]
        } else {
            [retvals]
                .iter()
                .zip(ret_val_constructors)
                .map(|(retval, (is_cipher, homomorphism, constr))| {
                    self.__get_decrypted_retval(BigInteger256::from_str(retval).unwrap(), is_cipher, homomorphism, constr)
                })
                .collect()
        }
    }
    fn __get_decrypted_retval<T>(
        &self,
        raw_value: BigInteger256,
        is_cipher: bool,
        crypto_params_name: String,
        constructor: CallableType<T>,
    ) ->T{
        if is_cipher {
            self.dec(
                Value::<u8, CipherValue>::new(
                    raw_value.to_string().into_bytes(),
                    Some(CryptoParams::new(crypto_params_name.clone())),None
                ),
                constructor,
                &crypto_params_name,
            ).0
        } else {
            constructor(raw_value)
        }
    }

    fn get_special_variables(
        &self,
    ) -> (MsgStruct, BlockStruct, TxStruct) {
        // assert self.__current_msg is not None and self.__current_block is not None and self.__current_tx is not None
        (
            self.__current_msg.clone().unwrap(),
            self.__current_block.clone().unwrap(),
            self.__current_tx.clone().unwrap(),
        )
    }

    fn update_special_variables(&mut self, wei_amount: i32) {
         let (__current_msg, __current_block, __current_tx) = self
            .__conn
            .get_special_variables(&self.__user_addr, wei_amount);
        (self.__current_msg, self.__current_block, self.__current_tx) = (Some(__current_msg),Some(__current_block),Some(__current_tx));
    }

    fn clear_special_variables(&mut self) {
        (self.__current_msg, self.__current_block, self.__current_tx) = (None, None, None);
    }
    //= CFG.lock().unwrap().main_crypto_backend
    fn enc(
        &mut self,
        plain: i32,
        target_addr: Option<String>,
        crypto_backend: &str,
    ) -> (Value<u8, CipherValue>, Option<Value<u8, RandomnessValue>>) {
        let target_addr =target_addr.map_or(self.__user_addr.clone(),|ta|ta);
        self.__crypto.get_mut(crypto_backend).unwrap().enc(plain.to_be_bytes().try_into().unwrap(), &self.__user_addr, &target_addr)
    }
    //= CFG.lock().unwrap().main_crypto_backend
    fn dec<T>(
        &self,
        cipher: Value<u8, CipherValue>,
        constr: CallableType<T>,
        crypto_backend: &str,
    ) -> (T, Option<Value<u8, RandomnessValue>>) {
        let res = self.__crypto[crypto_backend].dec(&cipher, &self.__user_addr);
        (constr(res.0.into()), res.1)
    }

    fn do_homomorphic_op(
        &mut self,
        op: &str,
        crypto_backend: &str,
        target_addr: String,
        args: Vec<u8>,
    ) {
        let params = CryptoParams::new(crypto_backend.to_owned());
        let pk = self.__keystore.get_mut(&params.crypto_name).unwrap().getPk(&target_addr);
        // assert!(
        //     args.iter().all(|arg| !(isinstance(arg, CipherValue)
        //         && params.crypto_name != arg.params.crypto_name)),
        //     "CipherValues from different crypto backends used in homomorphic operation"
        // );

        let mut crypto_inst = self.__crypto[&params.crypto_name].clone();
        // assert isinstance(crypto_inst, ZkayHomomorphicCryptoInterface);
        let result = crypto_inst.do_op(op, pk, args);
        // CipherValue(result, params)
    }

    // """
    // Re-randomizes arg using fresh randomness, which is stored in data[rnd_key] (side-effect!)
    // """
    fn do_rerand(
        &mut self,
        arg: Vec<u8>,
        crypto_backend: &str,
        target_addr: String,
        mut data: BTreeMap<String, String>,
        rnd_key: &str,
    ) {
        let params = CryptoParams::new(crypto_backend.to_owned());
        let pk = self.__keystore.get_mut(&params.crypto_name).unwrap().getPk(&target_addr);
        let mut crypto_inst = self.__crypto[&params.crypto_name].clone();
        // assert isinstance(crypto_inst, ZkayHomomorphicCryptoInterface);
        let (result, rand) = crypto_inst.do_rerand(arg, pk);
        data.insert(rnd_key.to_owned(),  params.crypto_name.clone()); //# store randomness
        // CipherValue(result, params)
    }

    fn _req_state_var(&self, name: &str, indices: String, count: i32)->String {
        // if self.__contract_handle is None:
        // # TODO check this statically in the type checker
        assert!(
            self.__contract_handle.is_some(),
            "Cannot read state variable {name} within constructor before it is assigned a value."
        );

        if count == 0 {
            self.__conn
                .req_state_var(self.__contract_handle.as_ref().unwrap(), name, &indices)
        } else {
            (0..count)
                .map(|i| {
                    self.__conn
                        .req_state_var(self.__contract_handle.as_ref().unwrap(), &name, &indices)
                })
                .collect()
        }
    }
    // @staticmethod
    fn __serialize_val<T>(val: T, bitwidth: i32)->i32 {
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
        0
    }
    // @staticmethod
    fn __serialize_circuit_array(
        data: BTreeMap<String, String>,
        mut target_array: Vec<i32>,
        target_out_start_idx: i32,
        elem_bitwidths: Vec<i32>,
    ) {
        let mut idx = target_out_start_idx;
        for ((name, val), &bitwidth) in data.iter().zip(&elem_bitwidths) {
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
            target_array[idx as usize] = ApiWrapper::<P, B, K>::__serialize_val(val, bitwidth);
            idx += 1;
            // }
        }
    }
    fn serialize_circuit_outputs(
        &self,
        zk_data: BTreeMap<String, String>,
        out_elem_bitwidths: Vec<i32>,
    ) -> Vec<i32> {
        // # TODO don't depend on out var names for correctness
        let out_vals: BTreeMap<_, _> = zk_data.clone()
            .into_iter()
            .filter(|(name, _val)| name.starts_with(&CFG.lock().unwrap().zk_out_name()))
            .collect();

        let count = out_vals
            .values()
            .map(|val| {
                // if isinstance(val, (Tuple, list)) {
                // val.len()
                // } else {
                1
                // }
            })
            .sum::<i32>();
        let zk_out =  vec![count];
        Self::__serialize_circuit_array(out_vals, zk_out.clone(), 0, out_elem_bitwidths);
        zk_out
    }
    fn serialize_private_inputs(
        &self,
        zk_priv: BTreeMap<String, String>,
        priv_elem_bitwidths: Vec<i32>,
    ) {
    Self::__serialize_circuit_array(
            zk_priv,
            self.all_priv_values.as_ref().unwrap().clone(),
            self.current_all_index.as_ref().unwrap().clone(),
            priv_elem_bitwidths,
        );
    }

    fn gen_proof(&self, fname: &str, in_vals: Vec<i32>, out_vals: Vec<i32>) -> Vec<i32> {
        self.__prover.generate_proof(
            &self.__project_dir,
            self.__contract_name.clone(),
            fname.to_owned(),
            self.all_priv_values.clone().unwrap(),
            in_vals,
            out_vals,
        )
    }

    // @contextmanager
    // """Return context manager which sets the correct current_all_index for the given sec_offset during its lifetime."""
    fn __call_ctx(&mut self, sec_offset: i32) {
        let (old_priv_values, old_all_idx) = (self.current_priv_values.clone(), self.current_all_index);
        self.current_priv_values = BTreeMap::new();
        *self.current_all_index.as_mut().unwrap() += sec_offset;
        // yield
        (self.current_priv_values, self.current_all_index) = (old_priv_values, old_all_idx);
    }
    // @contextmanager
    fn api_function_ctx(&mut self, trans_sec_size: usize, wei_amount: Option<i32>) {
        let was_external = self.is_external;
        if was_external.is_none() {
            assert!(self.all_priv_values.is_none());
            self.is_external = Some(true);
            self.all_priv_values = Some(vec![0; trans_sec_size]);
            self.current_all_index = Some(0);
            self.current_priv_values.clear();
            self.update_special_variables(wei_amount.unwrap());
        } else {
            self.is_external = Some(false);
        }

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
