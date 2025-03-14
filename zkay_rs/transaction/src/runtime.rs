#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::blockchain::web3::Web3Tx;
use alloy_json_abi::JsonAbi;
use alloy_primitives::Address;
use foundry_cli::opts::{EthereumOpts, RpcOpts};
use foundry_compilers::ArtifactId;
use foundry_compilers::Project;
use foundry_compilers::{
    Artifact, ProjectCompileOutput,
    artifacts::{CompactBytecode, CompactDeployedBytecode, Settings},
    cache::{CacheEntry, CompilerCache},
    utils::read_json_file,
};

// use alloy_sol_types::sol_data::Address;
use crate::blockchain::web3rs::{
    Web3BlockchainBase, Web3HttpGanacheBlockchain, Web3TesterBlockchain,
};
use crate::types::{
    ARcCell, AddressValue, BlockStruct, CipherValue, DataType, KeyPair, MsgStruct, PublicKeyValue,
    RandomnessValue, TxStruct, Value,
};
use proving_scheme::proving_scheme::ProvingScheme;
use rccell::{RcCell, WeakCell};

use serde_json::{Map, Result, Value as JsonValue, json};
use std::path::PathBuf;
// use crate::crypto::dummy::DummyCrypto;
// use crate::crypto::dummy_hom::DummyHomCrypto;
// use crate::crypto::ecdh_aes::EcdhAesCrypto;
use crate::crypto::ecdh_chaskey::EcdhChaskeyCrypto;
use crate::crypto::elgamal::ElgamalCrypto;
use zkay_config::config_user::UserConfig;
// use crate::crypto::paillier::PaillierCrypto;
// use crate::crypto::params::CryptoParams;
// use crate::crypto::rsa_oaep::RSAOAEPCrypto;
// use crate::crypto::rsa_pkcs15::RSAPKCS15Crypto;
use crate::interface::{
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayHomomorphicCryptoInterface,
    ZkayKeystoreInterface, ZkayProverInterface,
};
use crate::keystore::simple::SimpleKeystore;
use crate::prover::jsnark::*;
use enum_dispatch::enum_dispatch;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use zkay_config::config::CFG;
use zkay_transaction_crypto_params::params::CryptoParams;
#[enum_dispatch(ZkayCryptoInterface<P,B,K>,ZkayHomomorphicCryptoInterface<P,B,K>)]
#[derive(Clone)]
pub enum CryptoClass<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    EcdhChaskeyCrypto(EcdhChaskeyCrypto<P, B, K>),
    ElgamalCrypto(ElgamalCrypto<P, B, K>),
}
// impl <P:ZkayProverInterface,B:ZkayBlockchainInterface<P>,K: ZkayKeystoreInterface<P,B>, C: ZkayCryptoInterface<P,B,K>> CryptoClass<P,B,K,C>{
// pub fn new(crypto_class:C)->Self{
//     Self{crypto_class,
// _prover:PhantomData,
// _blockchain:PhantomData,
// _key_store:PhantomData,
// }
// }
// }
pub fn _crypto_classes<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>(
    crypto_backend: &str,
    key_store: ARcCell<K>,
) -> CryptoClass<P, B, K> {
    match crypto_backend {
        // "dummy" => DummyCrypto,
        // "dummy-hom" => DummyHomCrypto,
        // "rsa-pkcs1.5" => RSAPKCS15Crypto,
        // "rsa-oaep" => RSAOAEPCrypto,
        // "ecdh-aes" => EcdhAesCrypto,
        "ecdh-chaskey" => {
            CryptoClass::EcdhChaskeyCrypto(EcdhChaskeyCrypto::<P, B, K>::new(key_store))
        }
        // "paillier" => PaillierCrypto,
        "elgamal" => CryptoClass::ElgamalCrypto(ElgamalCrypto::<P, B, K>::new(key_store)),
        _ => panic!("unknown {crypto_backend}"),
    }
}

pub fn _prover_classes(snark_backend: &str) -> JsnarkProver {
    match snark_backend {
        "jsnark" => JsnarkProver,
        _ => panic!("unknown {snark_backend}"),
    }
}

#[enum_dispatch(ZkayBlockchainInterface<P>)]
#[derive(Clone)]
pub enum BlockchainClass<P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync> {
    Web3TesterBlockchain(Web3BlockchainBase<P, Web3TesterBlockchain>),
    Web3HttpGanacheBlockchain(Web3BlockchainBase<P, Web3HttpGanacheBlockchain>),
}

#[macro_export]
macro_rules! blockchain_class_dispatch {
    ($exp: expr,$self: expr) => {{
        match $self {
            Web3TesterBlockchain(tester) => tester.$exp,
            Web3HttpGanacheBlockchain(ganache) => ganache.$exp,
        }
    }};
}

#[macro_export]
macro_rules! class_fn {
    ($vis:vis fn $name:ident(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis fn $name(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {
             match self{
                Self::Web3TesterBlockchain(tester)=>tester.$name($($arg_name),*),
                Self::Web3HttpGanacheBlockchain(ganache)=>ganache.$name($($arg_name),*),
            }
        }
    };
    ($vis:vis fn $name:ident< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+  >(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis fn $name<$( $lt $(: $clt$(+$dlt)*)? ),+>(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {
             match self{
                Self::Web3TesterBlockchain(tester)=>tester.$name::<$( $lt),+ >($($arg_name),*),
                Self::Web3HttpGanacheBlockchain(ganache)=>ganache.$name::<$( $lt),+ >($($arg_name),*),
            }
        }
    };
    ($vis:vis async fn $name:ident(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis async fn $name(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {
             match self{
                Self::Web3TesterBlockchain(tester)=>tester.$name($($arg_name),*).await,
                Self::Web3HttpGanacheBlockchain(ganache)=>ganache.$name($($arg_name),*).await,
            }
        }
    };
    ($vis:vis async fn $name:ident<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis async fn $name<$( $lt $(: $clt$(+$dlt)*)? ),+>(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {
             match self{
                Self::Web3TesterBlockchain(tester)=>tester.$name::<$( $lt),+ >($($arg_name),*).await,
                Self::Web3HttpGanacheBlockchain(ganache)=>ganache.$name::<$( $lt),+ >($($arg_name),*).await,
            }
        }
    };
}

// // #[samotop_async_trait::async_trait]
// impl<P: ZkayProverInterface+ std::clone::Clone+ std::marker::Send+ std::marker::Sync> ZkayBlockchainInterface<P> for BlockchainClass<P> {
//         class_fn!(fn _pki_contract(&self) -> ARcCell<Option<BTreeMap<String, Address>>>);
//         class_fn!(fn lib_addresses(&self) -> ARcCell<Option<BTreeMap<String, Address>>>);
//     class_fn!(fn _connect_libraries(&self) -> eyre::Result<BTreeMap<String, Address>>);
//     class_fn!(async fn default_address(&self) -> Option<Value<String,AddressValue>>);
//     class_fn!(fn create_test_accounts(&self, _count: i32) -> Vec<String>);
//     class_fn!(async fn get_special_variables(&self,sender: &String,wei_amount: i32) -> (MsgStruct, BlockStruct, TxStruct));
//      class_fn!(fn prover(&self) -> ARcCell<P>);
//    class_fn!(fn deploy_solidity_contract<T: Clone + Default, V: Clone + Default>(&self,sol_filename: &str,contract_name: Option<String>,sender: &str) -> eyre::Result<Address>);
//     class_fn!(fn _verify_contract_integrity(&self,address: &Address,sol_filename: &PathBuf,libraries: Option<&BTreeMap<String, Address>>,contract_name: Option<String>,is_library: bool,cwd: Option<PathBuf>) -> eyre::Result<Address>);
//  class_fn!(fn _verify_library_integrity(&self,libraries: BTreeMap<String, PathBuf>,contract_with_libs_addr: &String,sol_with_libs_filename: &PathBuf) -> eyre::Result<BTreeMap<String, Address>>);
//       class_fn!(fn _verify_zkay_contract_integrity(&self,address: &Address,project_dir: &PathBuf,pki_verifier_addresses: &BTreeMap<String, Address>));
//   class_fn!(async fn _default_address(&self) -> Option<String>);
//      class_fn!(async fn _get_balance(&self, address: &str) -> String);
//      class_fn!(fn _deploy_dependencies(&self,sender: &str,project_dir: &PathBuf,verifier_names: Vec<String>) -> eyre::Result<BTreeMap<String, Address>>);
//      class_fn!(fn _req_public_key(&self,address: &String,crypto_params: &CryptoParams) -> eyre::Result<Value<String, PublicKeyValue>>);
//      class_fn!(fn _announce_public_key(&self,address: &str,pk: &Value<String, PublicKeyValue>,crypto_params: &CryptoParams) ->eyre::Result<String>);
//      class_fn!(fn _call(&self,contract_handle: &Address,sender: &String,name: &str,args: &Vec<DataType>) -> eyre::Result<String>);
//      class_fn!(fn  _req_state_var(&self,contract_handle: &Address,name: &str,indices: Vec<String>) -> eyre::Result<String>);
//      class_fn!(fn _transact(&self,contract_handle: &Address,sender: &str,function: &str,actual_args: &Vec<DataType>,wei_amount: Option<i32>)-> eyre::Result<String>);
//      class_fn!(fn _deploy(&self,project_dir: &PathBuf,sender: &str,contract: &str,actual_args: Vec<String>,wei_amount: Option<i32>) -> eyre::Result<Address>);
//      class_fn!(fn _connect(&self,project_dir: &str,contract: &str,address: Address) -> eyre::Result<Address>) ;

// }

pub fn _blockchain_classes<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
>(
    blockchain_backend: &str,
    prover: ARcCell<P>,
    web3tx: Web3Tx,
) -> BlockchainClass<P> {
    match blockchain_backend {
        "w3-eth-tester" => BlockchainClass::Web3TesterBlockchain(Web3BlockchainBase::<
            P,
            Web3TesterBlockchain,
        >::new(prover, web3tx)),
        "w3-ganache" => BlockchainClass::Web3HttpGanacheBlockchain(Web3BlockchainBase::<
            P,
            Web3HttpGanacheBlockchain,
        >::new(prover, web3tx)),
        // "w3-ipc" => Web3IpcBlockchain,
        // "w3-websocket" => Web3WebsocketBlockchain,
        // "w3-http" => Web3HttpBlockchain,
        // "w3-custom" => Web3CustomBlockchain,
        _ => panic!("unknown {blockchain_backend}"),
    }
}

// class Runtime:
//     """
//     Provides global access to singleton runtime API backend instances.
//     See interface.py for more information.

//     The global configuration in config.py determines which backends are made available via the Runtime class.
//     """
#[derive(Clone)]
pub struct Runtime<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    __blockchain: ARcCell<B>,
    __crypto: ARcCell<BTreeMap<String, ARcCell<CryptoClass<P, B, K>>>>,
    __keystore: ARcCell<BTreeMap<String, ARcCell<K>>>,
    __prover: ARcCell<P>,
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> Runtime<P, B, K>
{
    pub fn new(
        __blockchain: ARcCell<B>,
        __crypto: ARcCell<BTreeMap<String, ARcCell<CryptoClass<P, B, K>>>>,
        __keystore: ARcCell<BTreeMap<String, ARcCell<K>>>,
        __prover: ARcCell<P>,
    ) -> Self {
        Self {
            __blockchain,
            __crypto,
            __keystore,
            __prover,
        }
    }
    //     @staticmethod
    // """
    // Reboot the runtime.

    // When a new backend is selected in the configuration, it will only be loaded after a runtime reset.
    // """
    // pub fn reset(&self) {
    //     // *self.__blockchain.borrow_mut() = None;
    //     // *self.__crypto.borrow_mut() = BTreeMap::new();
    //     // *self.__keystore.borrow_mut() = BTreeMap::new();
    //     // *self.__prover.borrow_mut() = None;
    // }

    //     @staticmethod
    // """Return singleton object which implements ZkayBlockchainInterface."""
    pub fn blockchain(&self) -> ARcCell<B> {
        // if self.__blockchain.lock().is_none() {
        //     // *self.__blockchain.borrow_mut() = Some(_blockchain_classes(&CFG.lock().unwrap().blockchain_backend(),self.prover().clone()));
        //     // from zkay.transaction.types import AddressValue
        //     // AddressValue.get_balance = Runtime.__blockchain.get_balance
        // }
        self.__blockchain.clone()
    }

    //     @staticmethod
    //         """Return object which implements ZkayKeystoreInterface for given homomorphism."""
    pub fn keystore(&self, crypto_params: &CryptoParams) -> ARcCell<K> {
        let crypto_backend = crypto_params.crypto_name.clone();
        // if !self.__keystore.lock().contains_key(&crypto_backend) {
        //     // let k=SimpleKeystore::<P,BlockchainClass<P>>::new(blockchain.clone(), crypto_params.clone());
        //     self.__keystore
        //         .borrow_mut()
        //         .insert(crypto_backend.clone(), f(crypto_params));
        // }
        self.__keystore.lock()[&crypto_backend].clone()
    }

    //     @staticmethod
    //         """Return object which implements ZkayCryptoInterface for given homomorphism."""
    pub fn crypto(&self, crypto_params: &CryptoParams) -> ARcCell<CryptoClass<P, B, K>> {
        let crypto_backend = crypto_params.crypto_name.clone();
        // if !self.__crypto.lock().contains_key(&crypto_backend) {
        //     let keystore = self.keystore(crypto_params, f).clone();
        //     self.__crypto.borrow_mut().insert(
        //         crypto_backend.clone(),
        //         ARcCell::new(_crypto_classes::<P, B, K>(&crypto_backend, keystore)),
        //     );
        // }
        self.__crypto.lock()[&crypto_backend].clone()
    }
    //     @staticmethod
    // """Return singleton object which implements ZkayProverInterface."""
    pub fn prover(&self) -> ARcCell<P> {
        // if self.__prover.lock().is_none() {
        //     *self.__prover.borrow_mut() =
        //         Some(_prover_classes(&CFG.lock().unwrap().snark_backend()));
        // }
        self.__prover.clone()
    }
}
