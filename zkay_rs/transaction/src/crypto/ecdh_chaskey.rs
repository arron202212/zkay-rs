#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
//::secrets
// use typing::Tuple, List, Any
use crate::crypto::ecdh_base::EcdhBase;
use crate::interface::{
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayHomomorphicCryptoInterface,
    ZkayKeystoreInterface, ZkayProverInterface,
};
use crate::types::{
    ARcCell, AddressValue, CipherValue, DataType, KeyPair, PrivateKeyValue, PublicKeyValue, Value,
};
use ark_ff::BigInteger256;
use ark_std::rand;
use jsnark_interface::jsnark_interface::{CIRCUIT_BUILDER_JAR, JARS_DIR};
use rand::RngCore;
use rccell::{RcCell, WeakCell};
use rustc_serialize::hex::ToHex;
use std::str::FromStr;
use zkay_transaction_crypto_params::params::CryptoParams;
use zkay_utils::run_command::run_command;
fn main() {
    // compile-time length, use `vec![0;len]` for runtime
    let mut bytes = [0; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    // demo-ing both crates, either works
    println!("{}", hex::encode(&bytes));
    println!("{}", bytes.to_hex());
}

use rand::Rng;

fn main1() {
    println!("{}", hex::encode(&rand::thread_rng().r#gen::<[u8; 16]>()));
}
use std::marker::PhantomData;
#[derive(Clone)]
pub struct EcdhChaskeyCrypto<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    key_store: ARcCell<K>,
    params: CryptoParams,
    _prover: PhantomData<P>,
    _bc: PhantomData<B>,
}

// class EcdhChaskeyCrypto(EcdhBase):
// params = CryptoParams("ecdh-chaskey")
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> EcdhChaskeyCrypto<P, B, K>
{
    pub fn new(key_store: ARcCell<K>) -> Self {
        Self {
            key_store,
            params: CryptoParams::new("ecdh-chaskey".to_owned()),
            _prover: PhantomData,
            _bc: PhantomData,
        }
    }
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> EcdhBase<P, B, K> for EcdhChaskeyCrypto<P, B, K>
{
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ZkayCryptoInterface<P, B, K> for EcdhChaskeyCrypto<P, B, K>
{
    fn keystore(&self) -> ARcCell<K> {
        self.key_store.clone()
    }

    fn params(&self) -> CryptoParams {
        CryptoParams::new("ecdh-chaskey".to_owned())
    }
    fn _generate_or_load_key_pair(&self, address: &str) -> KeyPair {
        self._generate_or_load_key_pairs(address)
    }

    fn _enc(&self, plain: String, my_sk: String, target_pk: String) -> (Vec<String>, Vec<String>) {
        println!(
            "=_enc==ecdh_chashkey=*****************************************===plain====my_sk===target_pk=={plain}=={my_sk}===={target_pk}"
        );
        // # Compute shared key
        let key = Self::_ecdh_sha256(target_pk, my_sk);
        println!(
            "=_enc=ecdh_chashkey=key======={key:?},===hex::encode(&key)=========={:?}",
            hex::encode(&key)
        );
        let key = key
            .into_iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        println!("=_enc=ecdh_chashkey=key=s======{key}");
        let plain = alloy_primitives::U256::from_str(&plain).unwrap();
        println!("=_enc=ecdh_chashkey=plain========={}", plain);
        let plain: [u8; 32] = plain.to_be_bytes::<32>();
        let plain_bytes = hex::encode(&plain);
        println!("=_enc=ecdh_chashkey=plain_bytes========={}", plain_bytes);
        // # Call java implementation
        let mut iv = ark_std::rand::thread_rng().r#gen::<[u8; 16]>();
        println!(
            "===_enc=ecdh_chashkey===hex::encode(&iv)======={:?}=====",
            hex::encode(&iv)
        );
        let mut ivhex = iv.iter().map(|b| format!("{b:02x}")).collect::<String>();
        println!("==_enc=ecdh_chashkey==iv==={key:?}===={plain_bytes}====={iv:?}");
        let (iv_cipher, _) = run_command(
            vec![
                "java",
                "-Xms4096m",
                "-Xmx16384m",
                "-cp",
                &format!("{}", (JARS_DIR.clone() + "/" + CIRCUIT_BUILDER_JAR)),
                "zkay.ChaskeyLtsCbc",
                "enc",
                &key,
                &ivhex,
                &plain_bytes,
            ],
            None,
            false,
        );
        println!(
            "==_enc==ecdh_chashkey=======iv_cipher===={iv_cipher:?}========={}===",
            iv_cipher
                .as_ref()
                .unwrap()
                .trim()
                .split("\n")
                .last()
                .unwrap()
        );
        let iv_cipher32 = alloy_primitives::U256::from_str(
            &("0x".to_owned() + &iv_cipher.unwrap().trim().split("\n").last().unwrap()),
        )
        .unwrap()
        .to_be_bytes::<32>();
        println!(
            "==_enc==ecdh_chashkey=======iv_cipher32===={iv_cipher32:?}====={}==={}====",
            iv.len(),
            iv_cipher32.len()
        );
        let iv_cipher: Vec<_> = iv.into_iter().chain(iv_cipher32).collect();
        // let iv_cipher: Vec<u8> = iv.into_bytes(); //.into_iter().flat_map(|v|v.to_string().into_bytes()).collect();
        println!(
            "==_enc=ecdh_chashkey==iv+=iv_cipher===={iv_cipher:?}========{}====",
            iv_cipher.len()
        );
        let cipher = self.pack_byte_array(iv_cipher, self.params().cipher_chunk_size() as usize);
        println!(
            "==_enc==ecdh_chaskey=========********************************************===*=return =cipher===={cipher:?}============"
        );
        (cipher, vec![])
    }
    fn _dec(&self, mut cipher: Vec<String>, sk: &String) -> (String, Vec<String>) {
        println!(
            "===_dec==ecdh_chaskey==*****************************************************==cipher=====sk===={cipher:?}==========={sk}====================================="
        );
        // # Extract sender address from cipher metadata and request corresponding public key
        let sender_pk = cipher.pop().unwrap();
        // assert!( cipher.len() == self.params.cipher_payload_len);

        // # Compute shared key
        let key = Self::_ecdh_sha256(sender_pk.clone(), sk.clone());
        println!(
            "===_dec==ecdh_chashkey===key=====sender_pk=====sk======cipher=={}=={}=={:?}==== {}====={}======{:?}=",
            key.to_hex(),
            hex::encode(&key),
            key,
            sender_pk,
            sk,
            cipher
        );
        println!(
            "===_dec==ecdh_chashkey======cipher_chunk_size=======cipher_bytes_payload===== {},{}",
            self.params().cipher_chunk_size(),
            self.params().cipher_bytes_payload(),
        );
        // # Call java implementation
        let iv_cipher = self.unpack_to_byte_array(
            cipher,
            self.params().cipher_chunk_size(),
            self.params().cipher_bytes_payload(),
        );
        println!(
            "=_dec====ecdh_chashkey======iv_cipher=============={:?}==========",
            iv_cipher
        );
        let (iv, cipher_bytes) = iv_cipher.split_at(16);
        println!(
            "=_dec==ecdh_chashkey==iv======iv_cipher========{iv:?}======{cipher_bytes:?}=========="
        );
        let (plain, _) = run_command(
            vec![
                "java",
                "-Xms4096m",
                "-Xmx16384m",
                "-cp",
                &format!("{}", (JARS_DIR.clone() + "/" + CIRCUIT_BUILDER_JAR)),
                "zkay.ChaskeyLtsCbc",
                "dec",
                &key.to_hex(),
                &iv.to_hex(),
                &cipher_bytes.to_hex(),
            ],
            None,
            false,
        );
        println!(
            "==_dec===ecdh_chaskey==*****************************===plain=========={:?}=========={}",
            plain,
            plain.as_ref().unwrap().trim().split("\n").last().unwrap()
        );
        let v = alloy_primitives::U256::from_str(
            &("0x".to_owned() + &plain.unwrap().trim().split("\n").last().unwrap()),
        )
        .unwrap();
        let plain = v.to_string();
        println!(
            "==_dec===ecdh_chaskey==**************************=return==plain================={}",
            plain
        );
        (plain, vec![])
    }
}

impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ZkayHomomorphicCryptoInterface<P, B, K> for EcdhChaskeyCrypto<P, B, K>
{
    fn do_op(&self, _op: &str, _public_key: Vec<String>, _args: Vec<Vec<String>>) -> Vec<String> {
        vec![]
    }
    fn do_rerand(
        &self,
        _arg: Value<String, CipherValue>,
        _public_key: Vec<String>,
    ) -> (Vec<String>, Vec<u8>) {
        (vec![], vec![])
    }
}
