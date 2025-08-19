#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CGInstance;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;

use crate::circuit::structure::circuit_generator::CGConfigFields;
use crate::zkay::crypto::crypto_backend::CIPHER_CHUNK_SIZE;
use crate::zkay::crypto::crypto_backend::{Asymmetric, CryptoBackend, Symmetric};
use crate::zkay::zkay_cbc_symmetric_enc_gadget::ZkayCBCSymmetricEncGadget;

use crate::zkay::chaskey_lts_cbc::{ChaskeyLtsCbc, KeyParameter};
use crate::zkay::chaskey_lts_engine::{ChaskeyLTSEngine, CipherParameters};

use crate::util::util::BigInteger;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_cbc_symmetric_enc_gadget::CipherType;
use crate::zkay::zkay_type::ZkayType;
use crate::zkay::zkay_util::ZkayUtil;
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
    pub fn byteBigintConversionTest() {
        let mut b = ZkayUtil::unsignedBytesToBigInt(&PLAIN);
        let mut o = ZkayUtil::unsignedBigintToBytesi(b, PLAIN.len() as i32);
        assert_eq!(o, PLAIN, "Array bigint conversion does not preserve values");

        b = ZkayUtil::unsignedBytesToBigInt(&CIPHER);
        o = ZkayUtil::unsignedBigintToBytesi(b, CIPHER.len() as i32);
        assert_eq!(
            o, CIPHER,
            "Array bigint conversion does not preserve values"
        );

        let zero_arr = vec![0; 16];
        b = ZkayUtil::unsignedBytesToBigInt(&zero_arr);
        o = ZkayUtil::unsignedBigintToBytesi(b, zero_arr.len() as i32);
        assert_eq!(
            o, zero_arr,
            "Array bigint conversion does not preserve values"
        );
    }

    #[test]
    pub fn chaskeyLtsTest() {
        let crypto = ChaskeyLTSEngine::new(true, CipherParameters::new(KEY.to_vec()));

        // Test encrypt
        // crypto.init(true, KeyParameter::new(&KEY));
        let out = vec![0; 16];
        crypto.processBlock(&PLAIN.to_vec(), 0, &out, 0);
        assert_eq!(CIPHER.to_vec(), out, "Wrong encryption output");

        // crypto.reset();

        // Test decrypt
        // crypto.init(false, KeyParameter::new(KEY));
        let crypto = ChaskeyLTSEngine::new(false, CipherParameters::new(KEY.to_vec()));
        crypto.processBlock(&out, 0, &out, 0);
        assert_eq!(PLAIN.to_vec(), out, "Wrong decryption output");
    }

    #[test]
    pub fn cbcChaskeyOutputSameAsGadgetTest() {
        // Define inputs
        let key = BigInteger::parse_bytes(b"b2e21df10a222a69ee1e6a2d60465f4c", 16).unwrap();
        let iv = BigInteger::parse_bytes(b"f2c605c86352cea9fcaf88f12eba6371", 16).unwrap();
        let plain = BigInteger::parse_bytes(
            b"6d60ad00cd9efa16841c842876fd4dc9f0fba1eb9e1ce623a83f45483a221f9",
            16,
        )
        .unwrap();

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub key: BigInteger,
            pub iv: BigInteger,
            pub plain: BigInteger,
        }
        // Compute encryption via jsnark gadget

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let plainwire = TypedWire::new(
                    self.createConstantWire(&self.t.plain, &None),
                    ZkayType::ZkUint(256),
                    "plaintext".to_owned(),
                    &vec![],
                    self.cg(),
                );
                let ivwire = self.createConstantWire(&self.t.iv, &None);
                let keywire = self.createConstantWire(&self.t.key, &None);

                CircuitGenerator::makeOutputArray(
                    self.cg(),
                    ZkayCBCSymmetricEncGadget::new(
                        plainwire,
                        keywire,
                        ivwire,
                        CipherType::CHASKEY,
                        &None,
                        self.cg(),
                    )
                    .getOutputWires(),
                    &None,
                );
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {}
        };
        let t = CGTest {
            key: key.clone(),
            iv: iv.clone(),
            plain: plain.clone(),
        };
        let mut cgen = CircuitGeneratorExtend::<CGTest>::new("cbcchaskey", t);
        cgen.generateCircuit();
        cgen.evalCircuit();
        let mut evaluator = CircuitEvaluator::new("cbcchaskey", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
        let outwires = cgen.get_out_wires();
        let outs: Vec<_> = outwires
            .iter()
            .map(|ow| evaluator.getWireValue(ow.as_ref().unwrap()))
            .collect();

        // Compute encryption via CbcChaskey implementation
        let iv_bytes = ZkayUtil::unsignedBigintToBytesi(iv, 16);
        let result = ChaskeyLtsCbc::crypt(
            true,
            &ZkayUtil::unsignedBigintToBytesi(key, 16),
            &iv_bytes,
            &ZkayUtil::unsignedBigintToBytesi(plain, 32),
        );

        // Convert output to format produced by gadget (iv included, packed 248bit values in reverse order)
        let mut iv_cipher = vec![0; 16 + result.len()];
        iv_cipher[0..iv_bytes.len()].clone_from_slice(&iv_bytes);
        iv_cipher[iv_bytes.len()..result.len()].clone_from_slice(&result);

        let chunk_size = CIPHER_CHUNK_SIZE as usize / 8;
        let first_chunk_size = iv_cipher.len() % chunk_size;
        let mut bigints = vec![];
        if first_chunk_size != 0 {
            bigints.push(ZkayUtil::unsignedBytesToBigInt(
                &iv_cipher[0..first_chunk_size],
            ));
        }
        for i in first_chunk_size..iv_cipher.len() - first_chunk_size {
            bigints.push(ZkayUtil::unsignedBytesToBigInt(
                &iv_cipher[i..i + chunk_size],
            ));
        }
        bigints.reverse();

        // Check if both are equal
        assert_eq!(outs, bigints);
    }
}
