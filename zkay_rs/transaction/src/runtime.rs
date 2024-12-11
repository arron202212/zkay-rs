#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::blockchain::web3rs::{Web3BlockchainBase,Web3Blockchain,Web3TesterBlockchain,Web3HttpGanacheBlockchain};
// use crate::crypto::dummy::DummyCrypto;
// use crate::crypto::dummy_hom::DummyHomCrypto;
// use crate::crypto::ecdh_aes::EcdhAesCrypto;
use zkay_config::config_user::UserConfig;
use crate::crypto::ecdh_chaskey::EcdhChaskeyCrypto;
use crate::crypto::elgamal::ElgamalCrypto;
// use crate::crypto::paillier::PaillierCrypto;
// use crate::crypto::params::CryptoParams;
// use crate::crypto::rsa_oaep::RSAOAEPCrypto;
// use crate::crypto::rsa_pkcs15::RSAPKCS15Crypto;
use crate::keystore::simple::SimpleKeystore;
use crate::prover::jsnark::*;
use std::collections::BTreeMap;
use zkay_config::config::CFG;
use zkay_transaction_crypto_params::params::CryptoParams;
use crate::interface::{ZkayHomomorphicCryptoInterface,
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayKeystoreInterface, ZkayProverInterface,
};
use std::marker::PhantomData;
use enum_dispatch::enum_dispatch;
#[enum_dispatch(ZkayCryptoInterface<P,B,K>,ZkayHomomorphicCryptoInterface<P,B,K>)]
#[derive(Clone)]
pub enum CryptoClass<P:ZkayProverInterface+Clone,B:ZkayBlockchainInterface<P>+Clone,K: ZkayKeystoreInterface<P,B>+Clone>{
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
fn _crypto_classes<P:ZkayProverInterface+Clone,B:ZkayBlockchainInterface<P>+Clone,K: ZkayKeystoreInterface<P,B>+Clone>(crypto_backend: &str,key_store:K) -> CryptoClass<P,B,K> {
    match crypto_backend {
        // "dummy" => DummyCrypto,
        // "dummy-hom" => DummyHomCrypto,
        // "rsa-pkcs1.5" => RSAPKCS15Crypto,
        // "rsa-oaep" => RSAOAEPCrypto,
        // "ecdh-aes" => EcdhAesCrypto,
        "ecdh-chaskey" => CryptoClass::EcdhChaskeyCrypto(EcdhChaskeyCrypto::<P, B, K>::new(key_store)),
        // "paillier" => PaillierCrypto,
        "elgamal" => CryptoClass::ElgamalCrypto(ElgamalCrypto::<P, B, K>::new(key_store)),
        _ => panic!("unknown {crypto_backend}"),
    }
}

fn _prover_classes(snark_backend: &str) -> JsnarkProver {
    match snark_backend {
        "jsnark" => JsnarkProver,
        _ => panic!("unknown {snark_backend}"),
    }
}

#[enum_dispatch(ZkayBlockchainInterface<P>)]
#[derive(Clone)]
pub enum BlockchainClass<P:ZkayProverInterface+Clone>{
    Web3TesterBlockchain(Web3BlockchainBase<P,Web3TesterBlockchain>),
    Web3HttpGanacheBlockchain(Web3BlockchainBase<P,Web3HttpGanacheBlockchain>),
}
// impl<P:ZkayProverInterface>  ZkayBlockchainInterface<P> for BlockchainClass<P>{
// // pub fn new(blockchain_class:B)->Self{
// //     Self{blockchain_class,
// // _prover:PhantomData,
// // }
// // }
// }

fn _blockchain_classes<P:ZkayProverInterface+Clone>(blockchain_backend: &str,prover:P) -> BlockchainClass<P> {
    match blockchain_backend {
        "w3-eth-tester" => BlockchainClass::Web3TesterBlockchain(Web3BlockchainBase::<P,Web3TesterBlockchain>::new(prover)),
        "w3-ganache" => BlockchainClass::Web3HttpGanacheBlockchain(Web3BlockchainBase::<P,Web3HttpGanacheBlockchain>::new(prover)),
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
    P: ZkayProverInterface+Clone,
    B: Web3Blockchain+ZkayBlockchainInterface<P>+Clone,
    K: ZkayKeystoreInterface<P,B>+Clone,
> {
    __blockchain: Option<BlockchainClass<P>>,
    __crypto: BTreeMap<String, CryptoClass<P,B,K>>,
    __keystore: BTreeMap<String, K>,
    __prover: Option<JsnarkProver>,
}
impl<
        P: ZkayProverInterface+Clone,
        B: Web3Blockchain+ZkayBlockchainInterface<P>+Clone,
        K: ZkayKeystoreInterface<P,B>+Clone,
    > Runtime<P,B,  K> 
{
    pub fn new()->Self{
        Self{
  __blockchain : None,
        __crypto : BTreeMap::new(),
        __keystore : BTreeMap::new(),
        __prover : None,
        }
    }
    //     @staticmethod
    // """
    // Reboot the runtime.

    // When a new backend is selected in the configuration, it will only be loaded after a runtime reset.
    // """
    pub fn reset(&mut self) {
        self.__blockchain = None;
        self.__crypto = BTreeMap::new();
        self.__keystore = BTreeMap::new();
        self.__prover = None;
    }

    //     @staticmethod
    // """Return singleton object which implements ZkayBlockchainInterface."""
    pub fn blockchain(&mut self) -> &BlockchainClass<P> {
        // if self.__blockchain.is_none() {
        //     self.__blockchain = _blockchain_classes(CFG.lock().unwrap().blockchain_backend());
        //     // from zkay.transaction.types import AddressValue
        //     // AddressValue.get_balance = Runtime.__blockchain.get_balance
        // }
        self.__blockchain.as_ref().unwrap()
    }

    //     @staticmethod
    //         """Return object which implements ZkayKeystoreInterface for given homomorphism."""
   pub fn keystore(&mut self, crypto_params: &CryptoParams,k:K) -> &K {
        let crypto_backend = crypto_params.crypto_name.clone();
        let blockchain:BlockchainClass<P> = self.blockchain().clone();
        if !self.__keystore.contains_key(&crypto_backend) {
            // let k=SimpleKeystore::<P,BlockchainClass<P>>::new(blockchain.clone(), crypto_params.clone());
             self.__keystore.insert(
                crypto_backend.clone(),
                k
                ,
            );
        }
        &self.__keystore[&crypto_backend]
    }

    //     @staticmethod
    //         """Return object which implements ZkayCryptoInterface for given homomorphism."""
   pub fn crypto(&mut self, crypto_params: &CryptoParams,k:K) -> &CryptoClass<P,B,K> {
        let crypto_backend = crypto_params.crypto_name.clone();
        if !self.__crypto.contains_key(&crypto_backend) {
            let keystore = (*self.keystore(crypto_params,k)).clone();
            self.__crypto
                .insert(crypto_backend.clone(), _crypto_classes::<P,B,K>(&crypto_backend, keystore));
        }
        &self.__crypto[&crypto_backend]
    }
    //     @staticmethod
            // """Return singleton object which implements ZkayProverInterface."""
        pub fn prover(&mut self) -> &JsnarkProver
         {   if self.__prover.is_none()
               { self.__prover = Some(_prover_classes(&CFG.lock().unwrap().snark_backend()));}
             self.__prover.as_ref().unwrap()
        }
}

