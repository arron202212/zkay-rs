#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::interface::{ZkayBlockchainInterface, ZkayKeystoreInterface, ZkayProverInterface};
use crate::runtime::BlockchainClass;
use crate::types::{AddressValue, KeyPair, PublicKeyValue, Value};
use std::collections::BTreeMap;
use zkay_transaction_crypto_params::params::CryptoParams;
// class SimpleKeystore(ZkayKeystoreInterface):
//     pass
use std::marker::PhantomData;
#[derive(Clone)]
pub struct SimpleKeystore<P: ZkayProverInterface + Clone, B: ZkayBlockchainInterface<P> + Clone> {
    conn: B,
    crypto_params: CryptoParams,
    local_pk_store: BTreeMap<String, Value<u8, PublicKeyValue>>,
    local_key_pairs: BTreeMap<String, KeyPair>,
    _prover: PhantomData<P>,
}
impl<P: ZkayProverInterface + Clone, B: ZkayBlockchainInterface<P> + Clone> SimpleKeystore<P, B> {
    pub fn new(conn: B, crypto_params: CryptoParams) -> SimpleKeystore<P, B> {
        SimpleKeystore {
            conn,
            crypto_params,
            local_pk_store: BTreeMap::new(),
            local_key_pairs: BTreeMap::new(),
            _prover: PhantomData,
        }
    }
}
impl<P: ZkayProverInterface + Clone> ZkayKeystoreInterface<P, BlockchainClass<P>>
    for SimpleKeystore<P, BlockchainClass<P>>
{
    fn conn(&mut self) -> &mut BlockchainClass<P> {
        &mut self.conn
    }
    fn local_key_pairs(&self) -> &BTreeMap<String, KeyPair> {
        &self.local_key_pairs
    }
    fn local_key_pairs_mut(&mut self) -> &mut BTreeMap<String, KeyPair> {
        &mut self.local_key_pairs
    }
    fn local_pk_store(&self) -> &BTreeMap<String, Value<u8, PublicKeyValue>> {
        &self.local_pk_store
    }
    fn local_pk_store_mut(&mut self) -> &mut BTreeMap<String, Value<u8, PublicKeyValue>> {
        &mut self.local_pk_store
    }
    fn crypto_params(&self) -> &CryptoParams {
        &self.crypto_params
    }
}
