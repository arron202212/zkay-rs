// // from zkay.transaction.crypto import babyjubjub
// // from zkay.tests.zkay_unit_test import ZkayTestCase
// use alloy_primitives::Address;
// use ark_ec::AffineRepr;
// // use ark_ed_on_bn254::{EdwardsAffine as BabyJubJub, EdwardsConfig, Fq, Fr};
// use ark_ff::{AdditiveGroup, BigInt, Field, MontFp, PrimeField, Zero};
// use std::str::FromStr;
// use zkay_config::{
//     config::{CFG, zk_print_banner},
//     config_user::UserConfig,
//     with_context_block,
// };
// use zkay_transaction::interface::ZkayCryptoInterface;
// use zkay_transaction::interface::ZkayHomomorphicCryptoInterface;
// use zkay_transaction::{
//     arc_cell_new,
//     blockchain::web3::Web3Tx,
//     crypto::{elgamal::ElgamalCrypto,babyjubjub::{Fq,Fr,Point}},
//     keystore::simple::SimpleKeystore,
//     prover::jsnark::JsnarkProver,
//     runtime::{
//         _blockchain_classes, _crypto_classes, _prover_classes, BlockchainClass, CryptoClass,
//     },
//     types::CipherValue,
// };
// use zkay_transaction_crypto_params::params::CryptoParams;
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // class TestElgamal(ZkayTestCase)
//     #[test]
//     fn test_enc_with_rand() {
//         let eg = new_crypto();
//         let plain = "42".to_owned();
//         let random = Fr::new(
//             "4992017890738015216991440853823451346783754228142718316135811893930821210517",
//         )
//         ;
//         let pk = lc_vec_s![
//             "2543111965495064707612623550577403881714453669184859408922451773306175031318",
//             "20927827475527585117296730644692999944545060105133073020125343132211068382185"
//         ];
//         let cipher = eg._enc_with_rand(plain, random, pk);
//         let expected = lc_vec_s![
//             "4456281047741802786903502148815403505617014016895297192805304294758675919927",
//             "12789615932003720413289072351590236965415129719743141075654679223915282715496",
//             "18468046541385007143319885849846815206679689889442812945909193876326467244985",
//             "18277252404455744966371819766268305745003464230538782669018935165305166489246"
//         ];
//         assert_eq!(cipher, expected);
//     }
//     #[test]
//     fn test_enc_with_zero() {
//         let eg = new_crypto();
//         let plain = "0".to_owned();
//         let random = Fr::new("0");
//         let pk = lc_vec_s![
//             "2543111965495064707612623550577403881714453669184859408922451773306175031318",
//             "20927827475527585117296730644692999944545060105133073020125343132211068382185"
//         ];
//         let cipher = eg._enc_with_rand(plain, random, pk);
//         let expected = lc_vec_s!["0", "1", "0", "1"];
//         assert_eq!(cipher, expected);
//     }
//     #[test]
//     fn test_de_embed_0() {
//         let eg = new_crypto();
//         let plain = eg._de_embed(Point::zero());
//         let expected = 0;
//         assert_eq!(plain, expected);
//     }
//     #[test]
//     fn test_de_embed_1() {
//         let eg = new_crypto();
//         let plain = eg._de_embed(Point::generator());
//         let expected = 1;
//         assert_eq!(plain, expected);
//     }
//     #[test]
//     fn test_de_embed_other() {
//         let eg = new_crypto();
//         let embedded = [
//             "141579968252753561777903806704988380915591798817413028638954837858390837201",
//             "8211442360329077616485844356105856211290554633036363698328149195845491718472",
//         ];
//         let plain = eg._de_embed(Point::new(
//             Fq::new(embedded[0]),
//             Fq::new(embedded[1]),
//         ));
//         let expected = 42;
//         assert_eq!(plain, expected);
//     }
//     #[test]
//     fn test_decrypt() {
//         let eg = new_crypto();
//         let cipher = lc_vec_s![
//             "17990166387038654353532224054392704246273066434684370089496246721960255371329",
//             "15866190370882469414665095798958204707796441173247149326160843221134574846694",
//             "13578016172019942326633412365679613147103709674318008979748420035774874659858",
//             "15995926508900361671313404296634773295236345482179714831868518062689263430374"
//         ];
//         let sk = "448344687855328518203304384067387474955750326758815542295083498526674852893"
//             .to_owned();
//         let (plain, _) = eg._dec(cipher, &sk);
//         let expected = "42".to_owned();
//         assert_eq!(plain, expected);
//     }
//     #[test]
//     fn test_homomorphic_add() {
//         let eg = new_crypto();
//         let cipher1 = lc_vec_s![
//             "17990166387038654353532224054392704246273066434684370089496246721960255371329",
//             "15866190370882469414665095798958204707796441173247149326160843221134574846694",
//             "13578016172019942326633412365679613147103709674318008979748420035774874659858",
//             "15995926508900361671313404296634773295236345482179714831868518062689263430374"
//         ];
//         let cipher2 = lc_vec_s![
//             "20000451794290380375914691798920385097103434955980148521154607378788339649411",
//             "3379688933589504078077257631396507733503572474143535438012650064116108361323",
//             "19394553866420759826901398082663942344084257999221733532877406304105119931558",
//             "20583024216337563044477284173241746163084488704258522180236559083511927239523"
//         ];
//         let res = eg.do_op("+", vec![], vec![cipher1, cipher2]);
//         let expected = lc_vec_s![
//             "20730086989526828089755101097661965514378771463919358518890676635400766678286",
//             "16799789390858715328555613346600236217774855568279659847118447330575091014058",
//             "14197657705003367760678648101354578739816110086151324339904965365817708908473",
//             "6775926811477809154354189079047560699391344917788796131572409851404624663981"
//         ];
//         assert_eq!(res, expected);
//     }
//     #[test]
//     fn test_homomorphic_add_zero() {
//         let eg = new_crypto();
//         let cipher1 = lc_vec_s![
//             "17990166387038654353532224054392704246273066434684370089496246721960255371329",
//             "15866190370882469414665095798958204707796441173247149326160843221134574846694",
//             "13578016172019942326633412365679613147103709674318008979748420035774874659858",
//             "15995926508900361671313404296634773295236345482179714831868518062689263430374"
//         ];
//         let cipher2 = vec!["0".to_owned(); 4];
//         let res = eg.do_op("+", vec![], vec![cipher1.clone(), cipher2]);
//         assert_eq!(res, cipher1);
//     }
//     #[test]
//     fn test_homomorphic_sub() {
//         let eg = new_crypto();
//         let cipher1 = lc_vec_s![
//             "18885199402227818148211810144232318738102042906622969713112212912459159846007",
//             "11125071952177567933017599368067887482603292954302203070407920687516147981132",
//             "20036470080915178878390944667725801469044803295396841663384258912114611255016",
//             "18986185709423663075397883577572338596028661172318034324882291197251276265727"
//         ];
//         let cipher2 = lc_vec_s![
//             "20000451794290380375914691798920385097103434955980148521154607378788339649411",
//             "3379688933589504078077257631396507733503572474143535438012650064116108361323",
//             "19394553866420759826901398082663942344084257999221733532877406304105119931558",
//             "20583024216337563044477284173241746163084488704258522180236559083511927239523"
//         ];
//         let res = eg.do_op("-", vec![], vec![cipher1, cipher2]);
//         let expected = lc_vec_s![
//             "17453765650212926667159024877673077791194502231133810962604201363016156221214",
//             "17674308389347221388287741682187588441187556119579760637954905303743470691182",
//             "8966062093036566235871236061792057377388266503462348787704300058669700358396",
//             "4633753844574025554800245769804620724497900059105282639989145476546398522237"
//         ];
//         assert_eq!(res, expected)
//     }
//     #[test]
//     fn test_homomorphic_mul() {
//         let eg = new_crypto();
//         let cipher = lc_vec_s![
//             "17990166387038654353532224054392704246273066434684370089496246721960255371329",
//             "15866190370882469414665095798958204707796441173247149326160843221134574846694",
//             "13578016172019942326633412365679613147103709674318008979748420035774874659858",
//             "15995926508900361671313404296634773295236345482179714831868518062689263430374"
//         ];
//         // let cipher = lc_vec_s![
//         //             "4456281047741802786903502148815403505617014016895297192805304294758675919927",
//         //             "12789615932003720413289072351590236965415129719743141075654679223915282715496",
//         //             "18468046541385007143319885849846815206679689889442812945909193876326467244985",
//         //             "18277252404455744966371819766268305745003464230538782669018935165305166489246"
//         //         ];
//         let expected = eg.do_op("+", vec![], vec![cipher.clone(); 2]);
//         let res = eg.do_op("*", vec![], vec![cipher, vec!["int2".to_owned()]]);
//         assert_eq!(res, expected);
//     }
// }

// pub type BlockchainClassType = BlockchainClass<JsnarkProver>;
// pub type KeystoreType = SimpleKeystore<JsnarkProver, BlockchainClassType>;
// pub type CryptoClassType = CryptoClass<JsnarkProver, BlockchainClassType, KeystoreType>;

// pub fn new_crypto() -> ElgamalCrypto<
//     JsnarkProver,
//     BlockchainClass<JsnarkProver>,
//     SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
// > {
//     let web3tx = Web3Tx::default();
//     let __prover = arc_cell_new!(_prover_classes(&CFG.lock().unwrap().snark_backend()));
//     let __blockchain = arc_cell_new!(_blockchain_classes(
//         &CFG.lock().unwrap().blockchain_backend(),
//         __prover.clone(),
//         web3tx,
//     ));
//     let crypto_param = CryptoParams::new("elgamal".to_owned());
//     let key_store = arc_cell_new!(
//         SimpleKeystore::<JsnarkProver, BlockchainClass<JsnarkProver>>::new(
//             __blockchain.clone(),
//             crypto_param.clone(),
//         )
//     );
//     ElgamalCrypto::<
//         JsnarkProver,
//         BlockchainClass<JsnarkProver>,
//         SimpleKeystore<JsnarkProver, BlockchainClass<JsnarkProver>>,
//     >::new(key_store)
// }
