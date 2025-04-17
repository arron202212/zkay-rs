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
    types::CipherValue,
};
use zkay_transaction_crypto_params::params::CryptoParams;
#[macro_export]
macro_rules! lc_vec_s {
    () => { Vec::<String>::new() };
    ( $( $x:expr ),* ) => {
      {
       let mut temp= vec![ ];
        $(
        temp.push(String::from( $x ));
        )*
        temp
      }
  };
}

#[cfg(test)]
mod tests {
    use super::*;
    // class TestElgamal(ZkayTestCase)
    #[test]
    fn test_enc_with_rand() {
        let eg = new_crypto();
        let plain = "42".to_owned();
        let random = Fr::from_str(
            "4992017890738015216991440853823451346783754228142718316135811893930821210517",
        )
        .unwrap();
        let pk = lc_vec_s![
            "2543111965495064707612623550577403881714453669184859408922451773306175031318",
            "20927827475527585117296730644692999944545060105133073020125343132211068382185"
        ];
        let cipher = eg._enc_with_rand(plain, random, pk);
        let expected = lc_vec_s![
            "6731113522826802802383657036169607184484076765294939980621432336381893864836",
            "78204964903378981476715499932364206947352790269463727976367552660149130136",
            "18420844891664608832534029863885766414506540483764708081391447166848875816735",
            "15913825326024518269824330264558672677559558287176295933391901890454211811997"
        ];
        assert_eq!(cipher, expected);
    }
    #[test]
    fn test_enc_with_zero() {
        let eg = new_crypto();
        let plain = "0".to_owned();
        let random = Fr::from_str("0").unwrap();
        let pk = lc_vec_s![
            "2543111965495064707612623550577403881714453669184859408922451773306175031318",
            "20927827475527585117296730644692999944545060105133073020125343132211068382185"
        ];
        let cipher = eg._enc_with_rand(plain, random, pk);
        let expected = lc_vec_s!["0", "1", "0", "1"];
        assert_eq!(cipher, expected);
    }
    #[test]
    fn test_de_embed_0() {
        let eg = new_crypto();
        let plain = eg._de_embed(BabyJubJub::zero());
        let expected = 0;
        assert_eq!(plain, expected);
    }
    #[test]
    fn test_de_embed_1() {
        let eg = new_crypto();
        let plain = eg._de_embed(BabyJubJub::generator());
        let expected = 1;
        assert_eq!(plain, expected);
    }
    #[test]
    fn test_de_embed_other() {
        let eg = new_crypto();
        let embedded = [
            "141579968252753561777903806704988380915591798817413028638954837858390837201",
            "8211442360329077616485844356105856211290554633036363698328149195845491718472",
        ];
        let plain = eg._de_embed(BabyJubJub::new(
            Fq::from_str(embedded[0]).unwrap(),
            Fq::from_str(embedded[1]).unwrap(),
        ));
        let expected = 42;
        assert_eq!(plain, expected);
    }
    #[test]
    fn test_decrypt() {
        let eg = new_crypto();
        let cipher = lc_vec_s![
            "17990166387038654353532224054392704246273066434684370089496246721960255371329",
            "15866190370882469414665095798958204707796441173247149326160843221134574846694",
            "13578016172019942326633412365679613147103709674318008979748420035774874659858",
            "15995926508900361671313404296634773295236345482179714831868518062689263430374"
        ];
        let sk = "448344687855328518203304384067387474955750326758815542295083498526674852893"
            .to_owned();
        let (plain, _) = eg._dec(cipher, &sk);
        let expected = "42".to_owned();
        assert_eq!(plain, expected);
    }
    #[test]
    fn test_homomorphic_add() {
        let eg = new_crypto();
        let cipher1 = lc_vec_s![
            "17990166387038654353532224054392704246273066434684370089496246721960255371329",
            "15866190370882469414665095798958204707796441173247149326160843221134574846694",
            "13578016172019942326633412365679613147103709674318008979748420035774874659858",
            "15995926508900361671313404296634773295236345482179714831868518062689263430374"
        ];
        let cipher2 = lc_vec_s![
            "20000451794290380375914691798920385097103434955980148521154607378788339649411",
            "3379688933589504078077257631396507733503572474143535438012650064116108361323",
            "19394553866420759826901398082663942344084257999221733532877406304105119931558",
            "20583024216337563044477284173241746163084488704258522180236559083511927239523"
        ];
        let res = eg.do_op("+", vec![], vec![cipher1, cipher2]);
        let expected = lc_vec_s![
            "18885199402227818148211810144232318738102042906622969713112212912459159846007",
            "11125071952177567933017599368067887482603292954302203070407920687516147981132",
            "20036470080915178878390944667725801469044803295396841663384258912114611255016",
            "18986185709423663075397883577572338596028661172318034324882291197251276265727"
        ];
        assert_eq!(res, expected);
    }
    #[test]
    fn test_homomorphic_add_zero() {
        let eg = new_crypto();
        let cipher1 = lc_vec_s![
            "17990166387038654353532224054392704246273066434684370089496246721960255371329",
            "15866190370882469414665095798958204707796441173247149326160843221134574846694",
            "13578016172019942326633412365679613147103709674318008979748420035774874659858",
            "15995926508900361671313404296634773295236345482179714831868518062689263430374"
        ];
        let cipher2 = vec!["0".to_owned(); 4];
        let res = eg.do_op("+", vec![], vec![cipher1.clone(), cipher2]);
        assert_eq!(res, cipher1);
    }
    #[test]
    fn test_homomorphic_sub() {
        let eg = new_crypto();
        let cipher1 = lc_vec_s![
            "18885199402227818148211810144232318738102042906622969713112212912459159846007",
            "11125071952177567933017599368067887482603292954302203070407920687516147981132",
            "20036470080915178878390944667725801469044803295396841663384258912114611255016",
            "18986185709423663075397883577572338596028661172318034324882291197251276265727"
        ];
        let cipher2 = lc_vec_s![
            "20000451794290380375914691798920385097103434955980148521154607378788339649411",
            "3379688933589504078077257631396507733503572474143535438012650064116108361323",
            "19394553866420759826901398082663942344084257999221733532877406304105119931558",
            "20583024216337563044477284173241746163084488704258522180236559083511927239523"
        ];
        let res = eg.do_op("-", vec![], vec![cipher1, cipher2]);
        let expected = lc_vec_s![
            "17990166387038654353532224054392704246273066434684370089496246721960255371329",
            "15866190370882469414665095798958204707796441173247149326160843221134574846694",
            "13578016172019942326633412365679613147103709674318008979748420035774874659858",
            "15995926508900361671313404296634773295236345482179714831868518062689263430374"
        ];
        assert_eq!(res, expected)
    }
    #[test]
    fn test_homomorphic_mul() {
        let eg = new_crypto();
        let cipher = lc_vec_s![
            "17990166387038654353532224054392704246273066434684370089496246721960255371329",
            "15866190370882469414665095798958204707796441173247149326160843221134574846694",
            "13578016172019942326633412365679613147103709674318008979748420035774874659858",
            "15995926508900361671313404296634773295236345482179714831868518062689263430374"
        ];
        let expected = eg.do_op("+", vec![], vec![cipher.clone(); 2]);
        let res = eg.do_op("*", vec![], vec![cipher, vec!["int2".to_owned()]]);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_order() {
        assert_eq!(
            num_bigint::BigUint::parse_bytes(
                "4683099829275206785211452249446732022186240477850348096696166878914424525639"
                    .as_bytes(),
                10
            )
            .unwrap()
                % num_bigint::BigUint::parse_bytes(
                    "21888242871839275222246405745257275088548364400416034343698204186575808495617"
                        .as_bytes(),
                    10
                )
                .unwrap(),
            num_bigint::BigUint::zero()
        );
    }
}

pub type BlockchainClassType = BlockchainClass<JsnarkProver>;
pub type KeystoreType = SimpleKeystore<JsnarkProver, BlockchainClassType>;
pub type CryptoClassType = CryptoClass<JsnarkProver, BlockchainClassType, KeystoreType>;

pub fn new_crypto() -> ElgamalCrypto<
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
    let crypto_param = CryptoParams::new("elgamal".to_owned());
    let key_store = arc_cell_new!(
        SimpleKeystore::<JsnarkProver, BlockchainClass<JsnarkProver>>::new(
            __blockchain.clone(),
            crypto_param.clone(),
        )
    );
    ElgamalCrypto::<
        JsnarkProver,
        BlockchainClass<JsnarkProver>,
        SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
    >::new(key_store)
}
