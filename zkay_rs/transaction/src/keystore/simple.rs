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
use rccell::RcCell;
use std::collections::BTreeMap;
use zkay_transaction_crypto_params::params::CryptoParams;
// class SimpleKeystore(ZkayKeystoreInterface):
//     pass
use std::marker::PhantomData;
#[derive(Clone)]
pub struct SimpleKeystore<P: ZkayProverInterface + Clone, B: ZkayBlockchainInterface<P> + Clone> {
    conn: RcCell<B>,
    crypto_params: CryptoParams,
    local_pk_store: RcCell<BTreeMap<String, Value<String, PublicKeyValue>>>,
    local_key_pairs: RcCell<BTreeMap<String, KeyPair>>,
    _prover: PhantomData<P>,
}
impl<P: ZkayProverInterface + Clone, B: ZkayBlockchainInterface<P> + Clone> SimpleKeystore<P, B> {
    pub fn new(conn: RcCell<B>, crypto_params: CryptoParams) -> SimpleKeystore<P, B> {
        SimpleKeystore {
            conn,
            crypto_params,
            local_pk_store: RcCell::new(BTreeMap::new()),
            local_key_pairs: RcCell::new(BTreeMap::new()),
            _prover: PhantomData,
        }
    }
}
impl<P: ZkayProverInterface + Clone> ZkayKeystoreInterface<P, BlockchainClass<P>>
    for SimpleKeystore<P, BlockchainClass<P>>
{
    fn conn(&self) -> RcCell<BlockchainClass<P>> {
        self.conn.clone()
    }
    fn local_key_pairs(&self) -> RcCell<BTreeMap<String, KeyPair>> {
        self.local_key_pairs.clone()
    }
    fn local_pk_store(&self) -> RcCell<BTreeMap<String, Value<String, PublicKeyValue>>> {
        self.local_pk_store.clone()
    }

    fn crypto_params(&self) -> &CryptoParams {
        &self.crypto_params
    }
}
