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
use crate::types::{
    ARcCell, AddressValue, CipherValue, DataType, KeyPair, PrivateKeyValue, PublicKeyValue, Value,
};
use ark_ec::AffineRepr;
// use ark_ed_on_bn254::{EdwardsAffine as BabyJubJub, EdwardsConfig, Fq, Fr};
use ark_ec::twisted_edwards::{Affine, Projective};
use ark_ec::CurveGroup;
use ark_ec::twisted_edwards::TECurveConfig;
use ark_ff::BigInteger256;
use ark_ff::{
    AdditiveGroup, BigInt, MontFp, One, PrimeField, Zero,
    biginteger::BigInteger256 as BigInteger,
    fields::{Field, LegendreSymbol::*},
};
use ark_std::UniformRand;
// use babyjubjub_rs::{Point, PrivateKey};
use rccell::{RcCell, WeakCell};
use std::io::{Read, Write};
use std::ops::Mul;
use std::ops::Neg;
use std::path::PathBuf;
use std::str::FromStr;
use zkay_config::config_user::UserConfig;
use zkay_config::{config::CFG, zk_print};
use zkay_transaction_crypto_params::params::CryptoParams;
// use babygiant::baby_giant;

use ark_std::rand;
// use ark_ec::{AffineCurve, ProjectiveCurve};
use hex;
use std::fs::File;
use zkay_utils::timer::time_measure;
use super::babyjubjub::{Fr,Fq,Point,CURVE_ORDER};
use num_bigint::{BigUint, RandBigInt, RandomBits};

fn to_le_32_hex_bytes(num: BigUint) -> String {
    let hx = format!("{0:01$X}", num, 32 * 2);
    hx.into_bytes().chunks_exact(2).rev().map(|c| format!("{}{}",c[0] as char,c[1] as char)).collect()
    // let b = "".join(reversed(["".join(x) for x in zip(*[iter(hx)] * 2)]));
    // b
}

fn get_dlog(x: BigUint, y: BigUint) -> u64 {
    zk_print!("Fetching discrete log for {x:?}, {y:?}...");
    let xb =  x.to_string(); //to_le_32_hex_bytes(x.clone());//
    let yb = y.to_string(); //to_le_32_hex_bytes(y.clone());//
    zk_print!(
        "Running babygiant with arguments {xb:?}, {yb:?}...==={},======{}",
        x,
        y
    );

    super::babygiant::do_compute_dlog(
        &xb, &yb,
        // 1,
    )
}
use std::marker::PhantomData;
#[derive(Clone)]
pub struct ElgamalCrypto<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    pub key_store: ARcCell<K>,
    pub params: CryptoParams,
    _prover: PhantomData<P>,
    _bc: PhantomData<B>,
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ElgamalCrypto<P, B, K>
{
    pub fn new(key_store: ARcCell<K>) -> Self {
        Self {
            params: CryptoParams::new("elgamal".to_owned()),
            key_store,
            _prover: PhantomData,
            _bc: PhantomData,
        }
    }
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ZkayCryptoInterface<P, B, K> for ElgamalCrypto<P, B, K>
{
    // params = CryptoParams("elgamal")
    fn keystore(&self) -> ARcCell<K> {
        self.key_store.clone()
    }
    fn params(&self) -> CryptoParams {
        CryptoParams::new("elgamal".to_owned())
    }
    fn _generate_or_load_key_pair(&self, address: &str) -> KeyPair {
        let key_file = PathBuf::from(CFG.lock().unwrap().data_dir())
            .join("keys")
            .join(format!("elgamal_{}_{address}.bin", self.params.key_bits()));
        let _ = std::fs::create_dir_all(key_file.parent().unwrap());
        let (pkx, pky, sk);
        if key_file.try_exists().map_or(true, |x| !x) {
            zk_print!("Key pair not found, generating new ElGamal secret...");
            (pkx, pky, sk) = self._generate_key_pair();
            self._write_key_pair(&key_file, pkx.clone(), pky.clone(), sk.clone());
            zk_print!("Done");
        } else {
            // # Restore saved key pair
            zk_print!("ElGamal secret found, loading use file {key_file:?}");
            (pkx, pky, sk) = self._read_key_pair(&key_file);
        }
        KeyPair::new(
            Value::<String, PublicKeyValue>::new(vec![pkx, pky], Some(self.params()), None),
            Value::<String, PrivateKeyValue>::new(vec![sk], None, None),
        )
    }
    fn _enc(&self, plain: String, _my_sk: String, target_pk: String) -> (Vec<String>, Vec<String>) {
        println!("====elgamal==========_enc======================={target_pk}====");
        let pk: Vec<_> = target_pk.split(",").map(String::from).collect(); //self.serialize_pk(target_pk, self.params.key_bytes());
        println!("====elgamal==========_enc====serialize_pk==================={pk:?}===");
        let mut rng = rand::thread_rng();
        // let r = Fr::rand(&mut rng);
        let curve_order=BigUint::parse_bytes(CURVE_ORDER.as_bytes(),10).unwrap();
        let r: BigUint = rng.gen_biguint_below(&curve_order);
        let r=Fr::new(&r.to_string());
        let cipher_chunks: Vec<_> = self
            ._enc_with_rand(plain, r.clone(), pk)
            .into_iter()
            .map(|v| v.to_string())
            .collect();
        (cipher_chunks, vec![r.s.to_string()])
    }
    fn _dec(&self, cipher: Vec<String>, sk: &String) -> (String, Vec<String>) {
        // with time_measure("elgamal_decrypt"):
        let c1 = Point::new(
            Fq::new(&cipher[0]),
            Fq::new(&cipher[1]),
        );
        let c2 = Point::new(
            Fq::new(&cipher[2]),
            Fq::new(&cipher[3]),
        );
        let shared_secret = c1.mul(Fr::new(&sk));
        let plain_embedded = c2 + shared_secret.negate();
        let plain = self._de_embed(plain_embedded.into());

        // # TODO randomness misused for the secret key, which is an extremely ugly hack...
        (plain.to_string(), vec![sk.clone()])
    }
}
impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ElgamalCrypto<P, B, K>
{
    fn _write_key_pair(&self, key_file: &PathBuf, pkx: String, pky: String, sk: String) {
        // with open(key_file, "wb") as f:
        let mut f = File::create(key_file).unwrap();
        // for p in [pkx, pky] {
        //     let _ = f.write(&p); //self.params.cipher_chunk_size()
        // }
        // let _ = f.write(&sk); //self.params.cipher_chunk_size()
        let _ = f.write_all(format!("{},{},{}", pkx, pky, sk).as_bytes());
    }
    fn _read_key_pair(&self, key_file: &PathBuf) -> (String, String, String) {
        // with open(key_file, "rb") as f:
        // let mut f = std::fs::File::open(key_file).unwrap();
        // let mut buff = vec![0; self.params().cipher_chunk_size() as usize];
        // let _ = f.read(&mut buff[..]);
        // let pkx = buff;
        // let mut buff = vec![0; self.params().cipher_chunk_size() as usize];
        // let _ = f.read(&mut buff[..]);
        // let pky = buff;
        // let mut buff = vec![0; self.params().cipher_chunk_size() as usize];
        // let _ = f.read(&mut buff[..]);
        // let sk = buff;
        let s = std::fs::read_to_string(key_file).unwrap();
        let msg: Vec<_> = s.split(",").collect();
        (msg[0].to_owned(), msg[1].to_owned(), msg[2].to_owned())
    }

    fn _generate_key_pair(&self) -> (String, String, String) {
        let mut rng = rand::thread_rng();

        // let sk = Fr::rand(&mut rng);
        // let pk = EdwardsConfig::GENERATOR * sk;
        // println!(
        //     "==elgamal===_generate_key_pair====pkxy========={}==={}========",
        //     pk.x.into_bigint().to_string(),
        //     pk.y.into_bigint().to_string()
        // );
        // (
        //     pk.x.into_bigint().to_string(),
        //     pk.y.into_bigint().to_string(),
        //     sk.into_bigint().to_string(),
        // )
        let curve_order=BigUint::parse_bytes(CURVE_ORDER.as_bytes(),10).unwrap();
        let sk: BigUint = rng.gen_biguint_below(&curve_order);
        let pk=Point::generator().mul(Fr::new(&sk.to_string()));
        (
            pk.u.s.to_string(),
            pk.v.s.to_string(),
            sk.to_string(),
        )
    }
    pub fn _de_embed(&self, plain_embedded: Point) -> u64 {
        // # handle basic special cases without expensive discrete log computation
        if plain_embedded == Point::zero() {
            return 0;
        }
        if plain_embedded == Point::generator() {
            return 1;
        }
        get_dlog(plain_embedded.u.s.clone(), plain_embedded.v.s.clone())
    }
    pub fn _enc_with_rand(&self, plain: String, random: Fr, pk: Vec<String>) -> Vec<String> {
        let plain_embedded = Point::generator().mul(Fr::new(&plain));
        // let random = Fr::from(random);
        println!("==_enc_with_rand===={pk:?}======={random:?}========================"); //.as_bytes().chunks(2).map(|c| c[1] as char).collect::<String>()
        let (pk0, pk1) = (&pk[0], &pk[1]);
        println!("==_enc_with_rand=pk01==={pk0:?}======={pk1:?}========================");
        let shared_secret =
            Point::new(Fq::new(pk0), Fq::new(pk1)).mul(random.clone());
        let c1 = Point::generator().mul(random.clone());
        let c2 = plain_embedded + shared_secret;

        vec![
            c1.u.s.to_string(),
            c1.v.s.to_string(),
            c2.u.s.to_string(),
            c2.v.s.to_string(),
        ]
    }
}

impl<
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ZkayHomomorphicCryptoInterface<P, B, K> for ElgamalCrypto<P, B, K>
{
    fn do_op(&self, op: &str, _public_key: Vec<String>, args: Vec<Vec<String>>) -> Vec<String> {
        // fn deserialize(operand: &DataType) -> (Option<(BabyJubJub, BabyJubJub)>, Option<u128>) {
        //     // # if ciphertext is 0, return (Point.ZERO, Point.ZERO) == Enc(0, 0)
        //     if let DataType::CipherValue(operand) = operand {
        //         if &operand.contents == &vec![0.to_string(); 4] {
        //             (Some((BabyJubJub::zero(), BabyJubJub::zero())), None)
        //         } else {
        //             let c1 = BabyJubJub::new(
        //                 Fq::from_str(&operand[0]).unwrap(),
        //                 Fq::from_str(&operand[1]).unwrap(),
        //             );
        //             let c2 = BabyJubJub::new(
        //                 Fq::from_str(&operand[2]).unwrap(),
        //                 Fq::from_str(&operand[3]).unwrap(),
        //             );
        //             (Some((c1, c2)), None)
        //         }
        //     } else if let DataType::Int(operand) = operand {
        //         (None, Some(*operand))
        //     } else {
        //         (None, None)
        //     }
        // }
        fn deserialize_str(operand: &Vec<String>) -> Vec<Point> {
            // # if ciphertext is 0, return (Point.ZERO, Point.ZERO) == Enc(0, 0)
            if operand == &vec!["0".to_owned(); 4] {
                return vec![Point::zero(); 2];
            }
            let c1 = Point::new(
                Fq::new(&operand[0]),
                Fq::new(&operand[1]),
            );
            let c2 = Point::new(
                Fq::new(&operand[2]),
                Fq::new(&operand[3]),
            );
            vec![c1, c2]
        }
        // fn add<'a>(curr: BabyJubJub, other: &'a BabyJubJub) -> BabyJubJub {
        //     let y1y2 = curr.y * &other.y;
        //     let x1x2 = curr.x * &other.x;
        //     let a = Fq::from_str("168700");
        //     let d = Fq::from_str("168696");
        //     let dx1x2y1y2 = d * &y1y2 * &x1x2;

        //     let d1 = Fq::one() + &dx1x2y1y2;
        //     let d2 = Fq::one() - &dx1x2y1y2;

        //     let x1y2 = curr.x * &other.y;
        //     let y1x2 = curr.y * &other.x;

        //     let x = (x1y2 + &y1x2) / &d1;
        //     let y = (y1y2 - a * &x1x2) / &d2;

        //     BabyJubJub::new(x, y)
        // }
        let (e1, e2) = match op {
            "+" => {
                let args: Vec<_> = args.iter().map(|arg| deserialize_str(arg)).collect();

                (args[0][0].clone() + args[1][0].clone(), args[0][1].clone() + args[1][1].clone())
                // (add(args[0][0].clone(),&args[1][0]),add(args[0][1].clone(),&args[1][1]))
            }
            "-" => {
                let args: Vec<_> = args.iter().map(|arg| deserialize_str(arg)).collect();
                (args[0][0].clone() + args[1][0].clone().negate(), args[0][1].clone() + args[1][1].clone().negate())
                // (add(args[0][0] ,& args[1][0].neg()), add(args[0][1] ,& args[1][1].neg()))
            }
            "*" if &args[1][0][..3] == "int" => {
                // let int = args[1][0][3..].parse::<u32>().unwrap();
                // let a1 = Fr::new(&args[1][0][3..]);
                let arg1 = Fr::new(&args[1][0][3..]);
                // assert!(a1 == arg1, "a1===arg1");
                // println!("============{a1:?}============{arg1:?}");
                let arg0 = deserialize_str(&args[0]);
                (arg0[0].clone().mul(arg1.clone()), arg0[1].clone().mul(arg1))
            }
            "*" if &args[0][0][..3] == "int" => {
                let arg0 = Fr::new(&args[0][0][3..]);
                let arg1 = deserialize_str(&args[1]);
                (arg1[0].clone().mul(arg0.clone()), arg1[1].clone().mul(arg0))
            }
            _ => {
                panic!("Unsupported operation {op}");
            }
        };
        vec![
            e1.u.s.to_string(),
            e1.v.s.to_string(),
            e2.u.s.to_string(),
            e2.v.s.to_string(),
        ]
    }
    fn do_rerand(
        &self,
        arg: Value<String, CipherValue>,
        public_key: Vec<String>,
    ) -> (Vec<String>, Vec<u8>) {
        // # homomorphically add encryption of zero to re-randomize
        // let r = randrange(babyjubjub.CURVE_ORDER);
        let mut rng = rand::thread_rng();
        // let r = Fr::rand(&mut rng);
        let curve_order=BigUint::parse_bytes(CURVE_ORDER.as_bytes(),10).unwrap();
        let r: BigUint = rng.gen_biguint_below(&curve_order);
        let r=Fr::new(&r.to_string());
        let _enc_zero = self._enc_with_rand(0.to_string(), r.clone(), public_key.clone()); //.into_iter().map(|s| DataType::String(s))
        (
            self.do_op("+", public_key, vec![arg.contents.clone(), _enc_zero]),
            r.s.to_string().into_bytes(),
        )
    }
}
