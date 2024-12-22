#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
//::os
// use typing::Tuple, List, Any, Union

// use Crypto.Random.random::randrange

use crate::interface::{
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayHomomorphicCryptoInterface,
    ZkayKeystoreInterface, ZkayProverInterface,
};
use crate::types::{AddressValue, CipherValue, KeyPair, PrivateKeyValue, PublicKeyValue, Value};
use ark_ec::twisted_edwards::TECurveConfig;
use ark_std::UniformRand;
use babyjubjub_rs::{Point, PrivateKey};
use std::io::{Read, Write};
use std::ops::Mul;
use std::ops::Neg;
use std::path::PathBuf;
use std::str::FromStr;
use zkay_config::config_user::UserConfig;
use zkay_config::{config::CFG, zk_print};
use zkay_transaction_crypto_params::params::CryptoParams;
// use babygiant::baby_giant;
use ark_ec::AffineRepr;
use ark_ed_on_bn254::{EdwardsAffine as BabyJubJub, EdwardsConfig, Fq, Fr};
use ark_ff::{AdditiveGroup, BigInt, Field, MontFp, PrimeField, Zero};
use ark_ff::{BigInteger, BigInteger256};
use ark_std::rand;
// use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ec::twisted_edwards::{Affine, Projective};
use hex;
use std::fs::File;
use zkay_utils::timer::time_measure;

fn to_le_32_hex_bytes(num: Fq) -> Vec<u8> {
    let hx = format!("{0:01$X}", num.into_bigint(), 32 * 2);
    let b: Vec<_> = hx.into_bytes().chunks(2).rev().flatten().cloned().collect();
    // let b = "".join(reversed(["".join(x) for x in zip(*[iter(hx)] * 2)]));
    b
}

fn get_dlog(x: Fq, y: Fq) -> u64 {
    zk_print!("Fetching discrete log for {x:?}, {y:?}...");
    let xb = to_le_32_hex_bytes(x);
    let yb = to_le_32_hex_bytes(y);
    zk_print!("Running babygiant with arguments {xb:?}, {yb:?}...");

    babygiant_alt_bn128::do_compute_dlog(
        &String::from_utf8(xb).unwrap(),
        &String::from_utf8(yb).unwrap(),
        1,
    )
}
use std::marker::PhantomData;
#[derive(Clone)]
pub struct ElgamalCrypto<
    P: ZkayProverInterface + Clone,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    pub key_store: K,
    pub params: CryptoParams,
    _prover: PhantomData<P>,
    _bc: PhantomData<B>,
}
impl<
        P: ZkayProverInterface + Clone,
        B: ZkayBlockchainInterface<P> + Clone,
        K: ZkayKeystoreInterface<P, B> + Clone,
    > ElgamalCrypto<P, B, K>
{
    pub fn new(key_store: K) -> Self {
        Self {
            params: CryptoParams::new("elgamal".to_owned()),
            key_store,
            _prover: PhantomData,
            _bc: PhantomData,
        }
    }
}
impl<
        P: ZkayProverInterface + Clone,
        B: ZkayBlockchainInterface<P> + Clone,
        K: ZkayKeystoreInterface<P, B> + Clone,
    > ZkayCryptoInterface<P, B, K> for ElgamalCrypto<P, B, K>
{
    // params = CryptoParams("elgamal")
    fn keystore(&self) -> &K {
        &self.key_store
    }
    fn keystore_mut(&mut self) -> &mut K {
        &mut self.key_store
    }
    fn params(&self) -> CryptoParams {
        CryptoParams::new("elgamal".to_owned())
    }
    fn _generate_or_load_key_pair(&self, address: &String) -> KeyPair {
        let key_file = PathBuf::from(CFG.lock().unwrap().data_dir())
            .join("keys")
            .join(format!("elgamal_{}_{address}.bin", self.params.key_bits()));
        let _ = std::fs::create_dir_all(key_file.parent().unwrap());
        let (pk, sk);
        if key_file.try_exists().map_or(true, |x| !x) {
            zk_print!("Key pair not found, generating new ElGamal secret...");
            (pk, sk) = self._generate_key_pair();
            self._write_key_pair(&key_file, pk.clone(), sk.clone());
            zk_print!("Done");
        } else {
            // # Restore saved key pair
            zk_print!("ElGamal secret found, loading use file {key_file:?}");
            (pk, sk) = self._read_key_pair(&key_file);
        }

        KeyPair::new(
            String::from_utf8(pk).unwrap(),
            String::from_utf8(sk).unwrap(),
        )
    }
    fn _enc(&self, plain: String, _my_sk: Vec<u8>, target_pk: String) -> (Vec<String>, Vec<u8>) {
        let pk = self.serialize_pk(target_pk, self.params.key_bytes());
        let mut rng = rand::thread_rng();
        let r = Fr::rand(&mut rng);
        let cipher_chunks: Vec<_> = self
            ._enc_with_rand(plain, r, pk)
            .into_iter()
            .map(|v| v.to_string())
            .collect();
        (cipher_chunks, r.into_bigint().to_string().into_bytes())
    }
    fn _dec(&self, cipher: Vec<String>, sk: &Vec<u8>) -> (u64, Vec<u8>) {
        // with time_measure("elgamal_decrypt"):
        let c1 = BabyJubJub::new(
            Fq::from_str(&cipher[0]).unwrap(),
            Fq::from_str(&cipher[1]).unwrap(),
        );
        let c2 = BabyJubJub::new(
            Fq::from_str(&cipher[2]).unwrap(),
            Fq::from_str(&cipher[3]).unwrap(),
        );
        let shared_secret = c1 * Fr::from_str(&String::from_utf8(sk.clone()).unwrap()).unwrap();
        let plain_embedded = c2 + shared_secret.neg();
        let plain = self._de_embed(plain_embedded.into());

        // # TODO randomness misused for the secret key, which is an extremely ugly hack...
        (plain, sk.clone())
    }
}
impl<
        P: ZkayProverInterface + Clone,
        B: ZkayBlockchainInterface<P> + Clone,
        K: ZkayKeystoreInterface<P, B> + Clone,
    > ElgamalCrypto<P, B, K>
{
    fn _write_key_pair(&self, key_file: &PathBuf, pk: Vec<u8>, sk: Vec<u8>) {
        // with open(key_file, "wb") as f:
        let mut f = File::create(key_file).unwrap();
        for p in pk {
            let _ = f.write(&p.to_be_bytes()); //self.params.cipher_chunk_size()
        }
        let _ = f.write(&sk); //self.params.cipher_chunk_size()
    }
    fn _read_key_pair(&self, key_file: &PathBuf) -> (Vec<u8>, Vec<u8>) {
        // with open(key_file, "rb") as f:
        let mut f = std::fs::File::open(key_file).unwrap();
        let mut buff = vec![0; self.params().cipher_chunk_size() as usize];
        let _ = f.read(&mut buff[..]);
        let pkx = buff;
        let mut buff = vec![0; self.params().cipher_chunk_size() as usize];
        let _ = f.read(&mut buff[..]);
        let pky = buff;
        let mut buff = vec![0; self.params().cipher_chunk_size() as usize];
        let _ = f.read(&mut buff[..]);
        let sk = buff;
        (pkx.into_iter().chain(pky).collect(), sk)
    }

    fn _generate_key_pair(&self) -> (Vec<u8>, Vec<u8>) {
        let mut rng = rand::thread_rng();
        let sk = Fr::rand(&mut rng);
        let pk = EdwardsConfig::GENERATOR * sk;
        (
            pk.x.into_bigint()
                .to_string()
                .into_bytes()
                .into_iter()
                .chain(pk.y.into_bigint().to_string().into_bytes().into_iter())
                .collect(),
            sk.into_bigint().to_string().into_bytes(),
        )
    }
    fn _de_embed(&self, plain_embedded: BabyJubJub) -> u64 {
        // # handle basic special cases without expensive discrete log computation
        if plain_embedded == BabyJubJub::zero() {
            return 0;
        }
        if plain_embedded == BabyJubJub::generator() {
            return 1;
        }
        get_dlog(plain_embedded.x, plain_embedded.y)
    }
    fn _enc_with_rand(&self, plain: String, random: Fr, pk: Vec<String>) -> Vec<String> {
        let plain_embedded = EdwardsConfig::GENERATOR.mul(Fr::from_str(&plain).unwrap());
        // let random = Fr::from(random);
        let shared_secret =
            BabyJubJub::new(Fq::from_str(&pk[0]).unwrap(), Fq::from_str(&pk[1]).unwrap()) * &random;
        let c1 = EdwardsConfig::GENERATOR * &random;
        let c2 = plain_embedded + shared_secret;

        vec![
            c1.x.into_bigint().to_string(),
            c1.y.into_bigint().to_string(),
            c2.x.into_bigint().to_string(),
            c2.y.into_bigint().to_string(),
        ]
    }
}

impl<
        P: ZkayProverInterface + Clone,
        B: ZkayBlockchainInterface<P> + Clone,
        K: ZkayKeystoreInterface<P, B> + Clone,
    > ZkayHomomorphicCryptoInterface<P, B, K> for ElgamalCrypto<P, B, K>
{
    fn do_op(&self, op: &str, _public_key: Vec<String>, args: Vec<String>) -> Vec<u8> {
        fn deserialize(operand: &[String]) -> (BabyJubJub, BabyJubJub) {
            // # if ciphertext is 0, return (Point.ZERO, Point.ZERO) == Enc(0, 0)
            if operand == &vec![0.to_string(); 4] {
                (BabyJubJub::zero(), BabyJubJub::zero())
            } else {
                let c1 = BabyJubJub::new(
                    Fq::from_str(&operand[..4].concat()).unwrap(),
                    Fq::from_str(&operand[4..8].concat()).unwrap(),
                );
                let c2 = BabyJubJub::new(
                    Fq::from_str(&operand[8..16].concat()).unwrap(),
                    Fq::from_str(&operand[16..].concat()).unwrap(),
                );
                (c1, c2)
            }
        }
        let args: Vec<_> = args.chunks(4).map(|arg| deserialize(arg)).collect();
        let (e1, e2);
        if op == "+" {
            e1 = args[0].0 + args[1].0;
            e2 = args[0].1 + args[1].1;
        } else if op == "-" {
            e1 = args[0].0 + args[1].0.neg();
            e2 = args[0].1 + args[1].1.neg();
        // } else if op == "*" || args[1] != 0 {
        //     // && isinstance(args[1], int)
        //     e1 = args[0].0 * Fr::from(args[1]);
        //     e2 = args[0].1 * Fr::from(args[1]);
        // } else if op == "*" || args[0] != 0 {
        //     //&& isinstance(args[0], int)
        //     e1 = args[1].0 * Fr::from(args[0]);
        //     e2 = args[1].1 * Fr::from(args[0]);
        } else {
            panic!("Unsupported operation {op}");
        }

        e1.x.into_bigint()
            .to_string()
            .into_bytes()
            .into_iter()
            .chain(e1.y.into_bigint().to_string().into_bytes().into_iter())
            .chain(e2.x.into_bigint().to_string().into_bytes().into_iter())
            .chain(e2.y.into_bigint().to_string().into_bytes().into_iter())
            .collect()
    }
    fn do_rerand(&self, arg: Vec<String>, public_key: Vec<String>) -> (Vec<u8>, Vec<u8>) {
        // # homomorphically add encryption of zero to re-randomize
        // let r = randrange(babyjubjub.CURVE_ORDER);
        let mut rng = rand::thread_rng();
        let r = Fr::rand(&mut rng);
        let _enc_zero = self._enc_with_rand(0.to_string(), r, public_key.clone());
        (
            self.do_op("+", public_key, arg),
            r.into_bigint().to_string().into_bytes(),
        )
    }
}
