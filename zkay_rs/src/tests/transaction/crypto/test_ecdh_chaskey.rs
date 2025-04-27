// from zkay.transaction.crypto import babyjubjub
// from zkay.tests.zkay_unit_test import ZkayTestCase
use alloy_primitives::Address;
use ark_ec::AffineRepr;
use ark_ed_on_bn254::{EdwardsAffine as BabyJubJub, EdwardsConfig, Fq, Fr};
use ark_ff::{AdditiveGroup, BigInt, Field, MontFp, PrimeField, Zero};
use std::str::FromStr;
use zkay_config::{
    config::{CFG, zk_print_banner},
    config_user::UserConfig,
    with_context_block,
};
use zkay_transaction::crypto::ecdh_chaskey::EcdhChaskeyCrypto;
use zkay_transaction::interface::ZkayCryptoInterface;
use zkay_transaction::interface::ZkayHomomorphicCryptoInterface;
use zkay_transaction::{
    arc_cell_new,
    blockchain::web3::Web3Tx,
    crypto::elgamal::ElgamalCrypto,
    keystore::simple::SimpleKeystore,
    prover::jsnark::JsnarkProver,
    runtime::{
        _blockchain_classes, _crypto_classes, _prover_classes, BlockchainClass, CryptoClass,
    },
    types::{CipherValue, KeyPair},
};
use zkay_transaction_crypto_params::params::CryptoParams;
// #[macro_export]
// macro_rules! lc_vec_s {
//     () => { Vec::<String>::new() };
//     ( $( $x:expr ),* ) => {
//       {
//        let mut temp= vec![ ];
//         $(
//         temp.push(String::from( $x ));
//         )*
//         temp
//       }
//   };
// }
use crate::lc_vec_s;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_pair() {
        let eg = new_crypto();
        let random = "0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0";
        let cipher = eg._generate_or_load_key_pair(random);

        assert_eq!(cipher, KeyPair::default());
        // ntents: ["d00dfc1214717b49ebe72c40136d6cb8571b5dbc2e664d3ef19fee33fcd4c0f"], value: PublicKeyValue, params: Some(CryptoParams { crypto_name: "ecdh-chaskey" }), crypto_backend: None }, sk: Value { contents: ["17fa748693b2db75163e4b14dc6ff98b39d69b693d342b303b037e4649f088a8"], value: PrivateKeyValue, params: None, crypto_backend: None } }
    }
    #[test]
    fn test_chaskey_encrypt() {
        let eg = new_crypto();
        let plain = "1".to_owned();
        let sk = "7807958133264893761130188013303081620200691321518948604674435021509426660496";
        let pk = "17062064039002105071145928897299270168691005552724292186063894056227694073491";
        let (cipher, _rnd0) = eg._enc(plain.clone(), sk.to_owned(), pk.to_owned());
        let expected = lc_vec_s![
            "1304937986831259537780138834860496713257166540978891612467",
            "3661633562656262"
        ];
        assert_eq!(cipher, expected);
    }
    #[test]
    fn test_chaskey_decrypt() {
        let eg = new_crypto();
// ["838227053584047264855775463953954911643067566649899892978", 
// "2360826162599823589122331983964513739291523196512983359764"]

// ["838227053584047264855775463953954911643067566649899892978",
//  "1416033229769302398294307406363319587040642908780742548101691863815360303443",
//  "1416033229769302398294307406363319587040642908780742548101691863815360303443"]====
// =======10423779789206546051940304242832878301875363611584204385359672593908121289392
        let cipher = lc_vec_s![
            "1380979123654664939337427204872441023736578747935978184807", 
            "1272196659180395035355053465087644716065085215707741474258",
            "17062064039002105071145928897299270168691005552724292186063894056227694073491"
        ];

        let sk = "7807958133264893761130188013303081620200691321518948604674435021509426660496"
            .to_owned();
        let (plain, _) = eg._dec(cipher, &sk);
        let expected = "1".to_owned();
        assert_eq!(plain, expected);
    }
    #[test]
    fn test_ecdh_chaskey_decrypt() {
        let eg = new_crypto();
        let cipher = lc_vec_s![
            "529029680686925620170725866212555604956265071766216907128",
            "2875631731928657496681846026579680987016256009824881239363",
            "2637953654555208202388985563308512819498406696099766487329015329625951809310"
        ];
        let sk = "13962273717504073667787532377331036317483212298219620449628564526533982924744"
            .to_owned();
        let (plain, _) = eg._dec(cipher, &sk);
        let expected = "1".to_owned();
        assert_eq!(plain, expected);
    }
}

pub type BlockchainClassType = BlockchainClass<JsnarkProver>;
pub type KeystoreType = SimpleKeystore<JsnarkProver, BlockchainClassType>;
pub type CryptoClassType = CryptoClass<JsnarkProver, BlockchainClassType, KeystoreType>;

pub fn new_crypto() -> EcdhChaskeyCrypto<
    JsnarkProver,
    BlockchainClass<JsnarkProver>,
    SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
> {
    let web3tx = Web3Tx::default();
    let __prover = arc_cell_new!(_prover_classes(&CFG.lock().unwrap().snark_backend()));
    let __blockchain = arc_cell_new!(_blockchain_classes(
        &CFG.lock().unwrap().blockchain_backend(),
        __prover.clone(),
        web3tx,
    ));
    let crypto_param = CryptoParams::new("ecdh-chaskey".to_owned());
    let key_store = arc_cell_new!(
        SimpleKeystore::<JsnarkProver, BlockchainClass<JsnarkProver>>::new(
            __blockchain.clone(),
            crypto_param.clone(),
        )
    );
    EcdhChaskeyCrypto::<
        JsnarkProver,
        BlockchainClass<JsnarkProver>,
        SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
    >::new(key_store)
}
