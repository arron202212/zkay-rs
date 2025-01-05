#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use foundry_compilers::{
    artifacts::{CompactBytecode, CompactDeployedBytecode, Settings},
    cache::{CacheEntry, CompilerCache},
    utils::read_json_file,
    Artifact, ProjectCompileOutput,
};
use alloy_json_abi::JsonAbi;
use foundry_compilers::ArtifactId;
use foundry_cli::opts::{RpcOpts,EthereumOpts};
use foundry_compilers::Project;
 use alloy_primitives::Address;

// use alloy_sol_types::sol_data::Address;
use crate::blockchain::web3rs::{
    Web3Blockchain, Web3BlockchainBase, Web3HttpGanacheBlockchain, Web3TesterBlockchain,
};
use crate::types::{
    AddressValue, BlockStruct, CipherValue, DataType, KeyPair, MsgStruct, PublicKeyValue,
    RandomnessValue, TxStruct, Value,
};
use proving_scheme::proving_scheme::ProvingScheme;
use rccell::{RcCell, WeakCell};

use serde_json::{json, Map, Result, Value as JsonValue};
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
    P: ZkayProverInterface + Clone,
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
    P: ZkayProverInterface + Clone,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>(
    crypto_backend: &str,
    key_store: RcCell<K>,
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

#[enum_dispatch(ZkayBlockchainInterface<P>,Web3Blockchain)]
#[derive(Clone)]
pub enum BlockchainClass<P: ZkayProverInterface + Clone> {
    Web3TesterBlockchain(Web3BlockchainBase<P, Web3TesterBlockchain>),
    Web3HttpGanacheBlockchain(Web3BlockchainBase<P, Web3HttpGanacheBlockchain>),
}
// impl<P:ZkayProverInterface>  ZkayBlockchainInterface<P> for BlockchainClass<P>{
// // pub fn new(blockchain_class:B)->Self{
// //     Self{blockchain_class,
// // _prover:PhantomData,
// // }
// // }
// }

pub fn _blockchain_classes<P: ZkayProverInterface + Clone>(
    blockchain_backend: &str,
    prover: RcCell<P>,
    eth: Option<EthereumOpts>,
    rpc: Option<RpcOpts>,
) -> BlockchainClass<P> {
    match blockchain_backend {
        "w3-eth-tester" => BlockchainClass::Web3TesterBlockchain(Web3BlockchainBase::<
            P,
            Web3TesterBlockchain,
        >::new(prover,eth,rpc)),
        "w3-ganache" => BlockchainClass::Web3HttpGanacheBlockchain(Web3BlockchainBase::<
            P,
            Web3HttpGanacheBlockchain,
        >::new(prover,eth,rpc)),
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
    P: ZkayProverInterface + Clone,
    B: Web3Blockchain + ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    __blockchain: RcCell<B>,
    __crypto: RcCell<BTreeMap<String, RcCell<CryptoClass<P, B, K>>>>,
    __keystore: RcCell<BTreeMap<String, RcCell<K>>>,
    __prover: RcCell<P>,
}
impl<
        P: ZkayProverInterface + Clone,
        B: Web3Blockchain + ZkayBlockchainInterface<P> + Clone,
        K: ZkayKeystoreInterface<P, B> + Clone,
    > Runtime<P, B, K>
{
    pub fn new(
        __blockchain: RcCell<B>,
        __crypto: RcCell<BTreeMap<String, RcCell<CryptoClass<P, B, K>>>>,
        __keystore: RcCell<BTreeMap<String, RcCell<K>>>,
        __prover: RcCell<P>,
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
    pub fn blockchain(&self) -> RcCell<B> {
        // if self.__blockchain.borrow().is_none() {
        //     // *self.__blockchain.borrow_mut() = Some(_blockchain_classes(&CFG.lock().unwrap().blockchain_backend(),self.prover().clone()));
        //     // from zkay.transaction.types import AddressValue
        //     // AddressValue.get_balance = Runtime.__blockchain.get_balance
        // }
        self.__blockchain.clone()
    }

    //     @staticmethod
    //         """Return object which implements ZkayKeystoreInterface for given homomorphism."""
    pub fn keystore(&self, crypto_params: &CryptoParams) -> RcCell<K> {
        let crypto_backend = crypto_params.crypto_name.clone();
        // if !self.__keystore.borrow().contains_key(&crypto_backend) {
        //     // let k=SimpleKeystore::<P,BlockchainClass<P>>::new(blockchain.clone(), crypto_params.clone());
        //     self.__keystore
        //         .borrow_mut()
        //         .insert(crypto_backend.clone(), f(crypto_params));
        // }
        self.__keystore.borrow()[&crypto_backend].clone()
    }

    //     @staticmethod
    //         """Return object which implements ZkayCryptoInterface for given homomorphism."""
    pub fn crypto(&self, crypto_params: &CryptoParams) -> RcCell<CryptoClass<P, B, K>> {
        let crypto_backend = crypto_params.crypto_name.clone();
        // if !self.__crypto.borrow().contains_key(&crypto_backend) {
        //     let keystore = self.keystore(crypto_params, f).clone();
        //     self.__crypto.borrow_mut().insert(
        //         crypto_backend.clone(),
        //         RcCell::new(_crypto_classes::<P, B, K>(&crypto_backend, keystore)),
        //     );
        // }
        self.__crypto.borrow()[&crypto_backend].clone()
    }
    //     @staticmethod
    // """Return singleton object which implements ZkayProverInterface."""
    pub fn prover(&self) -> RcCell<P> {
        // if self.__prover.borrow().is_none() {
        //     *self.__prover.borrow_mut() =
        //         Some(_prover_classes(&CFG.lock().unwrap().snark_backend()));
        // }
        self.__prover.clone()
    }
}
