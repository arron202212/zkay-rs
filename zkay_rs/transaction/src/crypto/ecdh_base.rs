#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::interface::{
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayKeystoreInterface, ZkayProverInterface,
};
use crate::types::{KeyPair, PrivateKeyValue, PublicKeyValue, Value};
use ark_std::rand;
use ark_std::rand::Rng;
use jsnark_interface::jsnark_interface::CIRCUIT_BUILDER_JAR;
use jsnark_interface::jsnark_interface::JARS_DIR;
use std::fs;
use std::path::PathBuf;
use zkay_config::config_user::UserConfig;
use zkay_config::{config::CFG, zk_print};
use zkay_utils::run_command::run_command;
// class EcdhBase(ZkayCryptoInterface):
pub trait EcdhBase<
    P: ZkayProverInterface,
    B: ZkayBlockchainInterface<P>,
    K: ZkayKeystoreInterface<P, B>,
>: ZkayCryptoInterface<P, B, K>
{
    // @staticmethod
    fn _gen_keypair(rnd: &[u8]) -> (i32, i32) {
        let (keys, _) = run_command(
            vec![
                "java",
                "-Xms4096m",
                "-Xmx16384m",
                "-cp",
                &format!("{CIRCUIT_BUILDER_JAR}"),
                "zkay.ZkayECDHGenerator",
                &hex::encode(rnd),
            ],
            None,
            false,
        );
        let keys: Vec<_> = keys.unwrap().split("\n").map(|s| s.to_owned()).collect();
        let _keys = &keys[keys.len() - 2..];
        //  i32(keys[0], 16), int(keys[1], 16)
        (0, 0)
    }

    // @staticmethod
    fn _ecdh_sha256(other_pk: String, my_sk: Vec<u8>) -> Vec<u8> {
        let (ret, _) = run_command(
            vec![
                "java",
                "-Xms4096m",
                "-Xmx16384m",
                "-cp",
                &format!("{CIRCUIT_BUILDER_JAR}"),
                "zkay.ZkayECDHGenerator",
                &hex::encode(my_sk),
                &hex::encode(other_pk),
            ],
            None,
            false,
        );
        let _key = ret.unwrap().split("\n").last().unwrap();
        //  int(key, 16).to_bytes(16, byteorder="big")
        vec![]
    }
    fn _generate_or_load_key_pair(&self, address: &str) -> KeyPair {
        let key_file = PathBuf::from(CFG.lock().unwrap().data_dir())
            .join("keys")
            .join(format!("ec_{address}.bin"));
        let _ = std::fs::create_dir_all(key_file.parent().unwrap());
        let rnd;
        if key_file.try_exists().map_or(true, |x| !x) {
            // # Generate fresh randomness for ec private key
            zk_print!("Key pair not found, generating new EC secret...");
            rnd = hex::encode(&rand::thread_rng().gen::<[u8; 32]>()); //secrets.token_bytes(32);

            // # Store randomness so that address will have the same key every time
            // with open(key_file, "wb") as f:
            let _ = fs::write(key_file, rnd.clone());
            zk_print!("done");
        } else {
            // # Restore saved randomness
            zk_print!("EC secret found, loading from file {key_file:?}");
            // with open(key_file, "rb") as f:
            rnd = fs::read_to_string(key_file).unwrap();
        }
        // # Derive keys from randomness
        let (pk, sk) = Self::_gen_keypair(&rnd.into_bytes());

        KeyPair::new(pk.to_string(), sk.to_string())
    }
}
