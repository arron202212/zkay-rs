#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::CGConfigFields,
            circuit_generator::{
                CGConfig, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
    zkay::{
        chaskey_lts_cbc::{ChaskeyLtsCbc, KeyParameter},
        chaskey_lts_engine::{ChaskeyLTSEngine, CipherParameters},
        crypto::crypto_backend::{Asymmetric, CIPHER_CHUNK_SIZE, CryptoBackend, Symmetric},
        typed_wire::TypedWire,
        zkay_cbc_symmetric_enc_gadget::CipherType,
        zkay_cbc_symmetric_enc_gadget::ZkayCBCSymmetricEncGadget,
        zkay_type::ZkayType,
        zkay_util::ZkayUtil,
    },
};

use zkay_derive::ImplStructNameConfig;

#[cfg(test)]
mod test {
    use super::*;
    // Chaskey lts test vectors from FELICS
    // https://www.cryptolux.org/index.php/FELICS

    const KEY: [u8; 16] = [
        0x56, 0x09, 0xe9, 0x68, 0x5f, 0x58, 0xe3, 0x29, 0x40, 0xec, 0xec, 0x98, 0xc5, 0x22, 0x98,
        0x2f,
    ];
    const PLAIN: [u8; 16] = [
        0xb8, 0x23, 0x28, 0x26, 0xfd, 0x5e, 0x40, 0x5e, 0x69, 0xa3, 0x01, 0xa9, 0x78, 0xea, 0x7a,
        0xd8,
    ];
    const CIPHER: [u8; 16] = [
        0xd5, 0x60, 0x8d, 0x4d, 0xa2, 0xbf, 0x34, 0x7b, 0xab, 0xf8, 0x77, 0x2f, 0xdf, 0xed, 0xde,
        0x07,
    ];

    #[test]
    pub fn byte_bigint_conversion_test() {
        let mut b = ZkayUtil::unsigned_bytes_to_big_int(&PLAIN);
        let mut o = ZkayUtil::unsigned_bigint_to_bytesi(b, PLAIN.len() as i32);
        assert_eq!(o, PLAIN, "Array bigint conversion does not preserve values");

        b = ZkayUtil::unsigned_bytes_to_big_int(&CIPHER);
        o = ZkayUtil::unsigned_bigint_to_bytesi(b, CIPHER.len() as i32);
        assert_eq!(
            o, CIPHER,
            "Array bigint conversion does not preserve values"
        );

        let zero_arr = vec![0; 16];
        b = ZkayUtil::unsigned_bytes_to_big_int(&zero_arr);
        o = ZkayUtil::unsigned_bigint_to_bytesi(b, zero_arr.len() as i32);
        assert_eq!(
            o, zero_arr,
            "Array bigint conversion does not preserve values"
        );
    }

    #[test]
    pub fn chaskey_lts_test() {
        //MYTODO TEST
        let crypto = ChaskeyLTSEngine::new(true, CipherParameters::new(KEY.to_vec()));

        // Test encrypt
        // crypto.init(true, KeyParameter::new(&KEY));
        let out = vec![0; 16];
        crypto.process_block(&PLAIN.to_vec(), 0, &out, 0);
        assert_eq!(CIPHER.to_vec(), out, "Wrong encryption output");

        // crypto=RcCell::new();

        // Test decrypt
        // crypto.init(false, KeyParameter::new(KEY));
        let crypto = ChaskeyLTSEngine::new(false, CipherParameters::new(KEY.to_vec()));
        crypto.process_block(&out, 0, &out, 0);
        assert_eq!(PLAIN.to_vec(), out, "Wrong decryption output");
    }

    #[test]
    pub fn cbc_chaskey_output_same_as_gadget_test() {
        //MYTODO TEST
        // Define inputs
        let key = Util::parse_big_int_x("b2e21df10a222a69ee1e6a2d60465f4c");
        let iv = Util::parse_big_int_x("f2c605c86352cea9fcaf88f12eba6371");
        let plain = Util::parse_big_int_x(
            "6d60ad00cd9efa16841c842876fd4dc9f0fba1eb9e1ce623a83f45483a221f9",
        );

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub key: BigInteger,
            pub iv: BigInteger,
            pub plain: BigInteger,
        }
        // Compute encryption via jsnark gadget

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let plainwire = TypedWire::new(
                    CircuitGenerator::create_constant_wire(self.cg(), &self.t.plain),
                    ZkayType::zk_uint(256),
                    "plaintext".to_owned(),
                    &vec![],
                    self.cg(),
                );
                let ivwire = CircuitGenerator::create_constant_wire(self.cg(), &self.t.iv);
                let keywire = CircuitGenerator::create_constant_wire(self.cg(), &self.t.key);

                CircuitGenerator::make_output_array(
                    self.cg(),
                    ZkayCBCSymmetricEncGadget::new(
                        plainwire,
                        keywire,
                        ivwire,
                        CipherType::Chaskey,
                        self.cg(),
                    )
                    .get_output_wires(),
                );
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {}
        };
        let t = CGTest {
            key: key.clone(),
            iv: iv.clone(),
            plain: plain.clone(),
        };
        let mut cgen = CircuitGeneratorExtend::<CGTest>::new("cbcchaskey", t);
        cgen.generate_circuit();
        cgen.eval_circuit();
        let mut evaluator = CircuitEvaluator::new("cbcchaskey", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
        let outwires = cgen.get_out_wires();
        let outs: Vec<_> = outwires
            .iter()
            .map(|ow| evaluator.get_wire_value(ow.as_ref().unwrap()))
            .collect();

        // Compute encryption via CbcChaskey implementation
        let iv_bytes = ZkayUtil::unsigned_bigint_to_bytesi(iv, 16);
        let result = ChaskeyLtsCbc::crypt(
            true,
            &ZkayUtil::unsigned_bigint_to_bytesi(key, 16),
            &iv_bytes,
            &ZkayUtil::unsigned_bigint_to_bytesi(plain, 32),
        );

        // Convert output to format produced by gadget (iv included, packed 248bit values in reverse order)
        let mut iv_cipher = vec![0; 16 + result.len()];
        iv_cipher[0..iv_bytes.len()].clone_from_slice(&iv_bytes);
        iv_cipher[iv_bytes.len()..result.len()].clone_from_slice(&result);

        let chunk_size = CIPHER_CHUNK_SIZE as usize / 8;
        let first_chunk_size = iv_cipher.len() % chunk_size;
        let mut bigints = vec![];
        if first_chunk_size != 0 {
            bigints.push(ZkayUtil::unsigned_bytes_to_big_int(
                &iv_cipher[0..first_chunk_size],
            ));
        }
        for i in first_chunk_size..iv_cipher.len() - first_chunk_size {
            bigints.push(ZkayUtil::unsigned_bytes_to_big_int(
                &iv_cipher[i..i + chunk_size],
            ));
        }
        bigints.reverse();

        // Check if both are equal
        assert_eq!(outs, bigints);
    }
}
