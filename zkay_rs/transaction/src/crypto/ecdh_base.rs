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
use num_bigint::{BigInt, BigUint};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use zkay_config::config_user::UserConfig;
use zkay_config::{config::CFG, zk_print};
use zkay_utils::run_command::run_command;
// class EcdhBase(ZkayCryptoInterface):
pub trait EcdhBase<
    P: ZkayProverInterface + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P>,
    K: ZkayKeystoreInterface<P, B>,
>: ZkayCryptoInterface<P, B, K>
{
    // @staticmethod
    fn _gen_keypair(rnd: &str) -> (String, String) {
        println!("==_gen_keypair=======rnd================{rnd}");
        let (keys, _) = run_command(
            vec![
                "java",
                "-Xms4096m",
                "-Xmx16384m",
                "-cp",
                &format!("{}:.", (JARS_DIR.clone() + "/" + CIRCUIT_BUILDER_JAR)),
                "zkay.ZkayECDHGenerator",
                rnd,
            ],
            None,
            false,
        );
        let keys = keys.unwrap();
        let keys = keys.trim();
        let keys: Vec<_> = keys.split("\n").collect();
        println!(
            "==keys, keys.len()=================={:?},{}",
            keys,
            keys.len()
        );

        let keys = &keys[keys.len() - 2..];
        //  i32(keys[0], 16), int(keys[1], 16)
        println!(
            "==keys[0], keys[1]=================={},{}",
            keys[0], keys[1]
        );
        (
            BigUint::parse_bytes(keys[0].as_bytes(), 16)
                .unwrap()
                .to_string(),
            BigUint::parse_bytes(keys[1].as_bytes(), 16)
                .unwrap()
                .to_string(),
        )
    }

    // @staticmethod
    fn _ecdh_sha256(other_pk: String, my_sk: String) -> Vec<u8> {
        println!(
            "======_ecdh_sha256======**********===============other_pk: , my_sk========={other_pk}========== {my_sk}======={:?}=====",
            alloy_primitives::U256::from_str(&other_pk.trim_matches(&['[', ']', '"', ' ', '\n']))
        );
        let my_sk = BigUint::parse_bytes(my_sk.as_bytes(), 10)
            .unwrap()
            .to_str_radix(16);
        let other_pk = format!("{:x}", alloy_primitives::U256::from_str(&other_pk).unwrap());
        println!(
            "=======_ecdh_sha256======**********=====other_pk=====my_sk===={other_pk}======={my_sk}====="
        );
        let (ret, _) = run_command(
            vec![
                "java",
                "-Xms4096m",
                "-Xmx16384m",
                "-cp",
                &format!("{}", (JARS_DIR.clone() + "/" + CIRCUIT_BUILDER_JAR)),
                "zkay.ZkayECDHGenerator",
                &my_sk,
                &other_pk,
            ],
            None,
            false,
        );
        //  int(key, 16).to_bytes(16, byteorder="big")
        println!(
            "====_ecdh_sha256======**********=ret========={}",
            ret.as_ref().unwrap().trim().split("\n").last().unwrap()
        );
        let v = alloy_primitives::U256::from_str(
            &("0x".to_owned() + &ret.unwrap().trim().split("\n").last().unwrap()),
        )
        .unwrap();
        println!("====_ecdh_sha256======**********=v========={}", v);
        let v: [u8; 32] = v.to_be_bytes::<32>();
        println!(
            "====_ecdh_sha256======**********====_ecdh_sha256===v========={:?}",
            v
        );
        let ret = v[16..].to_vec();
        println!(
            "===_ecdh_sha256======**********==ret=====_ecdh_sha256===={:?}",
            ret
        );
        ret
    }
    fn _generate_or_load_key_pairs(&self, address: &str) -> KeyPair {
        let key_file = PathBuf::from(CFG.lock().unwrap().data_dir())
            .join("keys")
            .join(format!("ec_{address}.bin"));
        let _ = std::fs::create_dir_all(key_file.parent().unwrap());
        let rnd;
        if key_file.try_exists().map_or(true, |x| !x) {
            // # Generate fresh randomness for ec private key
            zk_print!("Key pair not found, generating new EC secret...");
            rnd = rand::thread_rng()
                .r#gen::<[u8; 32]>()
                .iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>(); //secrets.token_bytes(32);

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
        let (pk, sk) = Self::_gen_keypair(&rnd);

        KeyPair::new(
            Value::<String, PublicKeyValue>::new(vec![pk], Some(self.params()), None),
            Value::<String, PrivateKeyValue>::new(vec![sk], None, None),
        )
    }
}
