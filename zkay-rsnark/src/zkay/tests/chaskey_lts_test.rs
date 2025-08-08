#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;

use crate::zkay::crypto::crypto_backend::{
    Asymmetric, AsymmetricConfig, CryptoBackend, Symmetric, SymmetricConfig,
};
use crate::zkay::*;
#[cfg(test)]
mod test {
    use super::*;
    // Chaskey lts test vectors from FELICS
    // https://www.cryptolux.org/index.php/FELICS

    const key: [u8; 16] = [
        0x56, 0x09, 0xe9, 0x68, 0x5f, 0x58, 0xe3, 0x29, 0x40, 0xec, 0xec, 0x98, 0xc5, 0x22, 0x98,
        0x2f,
    ];
    const plain: [u8; 16] = [
        0xb8, 0x23, 0x28, 0x26, 0xfd, 0x5e, 0x40, 0x5e, 0x69, 0xa3, 0x01, 0xa9, 0x78, 0xea, 0x7a,
        0xd8,
    ];
    const cipher: [u8; 16] = [
        0xd5, 0x60, 0x8d, 0x4d, 0xa2, 0xbf, 0x34, 0x7b, 0xab, 0xf8, 0x77, 0x2f, 0xdf, 0xed, 0xde,
        0x07,
    ];

    #[test]
    pub fn byteBigintConversionTest() {
        let b = ZkayUtil.unsignedBytesToBigInt(plain);
        let o = ZkayUtil.unsignedBigintToBytes(b, plain.len());
        assert_eq!("Array bigint conversion does not preserve values", o, plain);

        b = ZkayUtil.unsignedBytesToBigInt(cipher);
        o = ZkayUtil.unsignedBigintToBytes(b, cipher.len());
        assert_eq!(
            "Array bigint conversion does not preserve values", o,
            cipher
        );

        let zero_arr = vec![0; 16];
        b = ZkayUtil.unsignedBytesToBigInt(zero_arr);
        o = ZkayUtil.unsignedBigintToBytes(b, zero_arr.len());
        assert_eq!(
            "Array bigint conversion does not preserve values", o,
            zero_arr
        );
    }

    #[test]
    pub fn chaskeyLtsTest() {
        let crypto = ChaskeyLTSEngine::new();

        // Test encrypt
        crypto.init(true, KeyParameter::new(key));
        let out = vec![0; 16];
        crypto.processBlock(plain, 0, out, 0);
        assert_eq!("Wrong encryption output", cipher, out);

        crypto.reset();

        // Test decrypt
        crypto.init(false, KeyParameter::new(key));
        crypto.processBlock(out, 0, out, 0);
        assert_eq!("Wrong decryption output", plain, out);
    }

    #[test]
    pub fn cbcChaskeyOutputSameAsGadgetTest() {
        // Define inputs
        let key = BigInteger::new("b2e21df10a222a69ee1e6a2d60465f4c", 16);
        let iv = BigInteger::new("f2c605c86352cea9fcaf88f12eba6371", 16);
        let plain = BigInteger::new(
            "6d60ad00cd9efa16841c842876fd4dc9f0fba1eb9e1ce623a83f45483a221f9",
            16,
        );

        // Compute encryption via jsnark gadget
        let cgen = CircuitGenerator::new("cbcchaskey");
        crate::impl_struct_name_for!(CircuitGeneratorExtend<ElgamalDecCircuitGenerator>);
        impl CGConfig for CircuitGeneratorExtend<ElgamalDecCircuitGenerator> {
            fn buildCircuit(&mut self) {
                let plainwire =
                    TypedWire::new(createConstantWire(plain), ZkayType.ZkUint(256), "plaintext");
                let ivwire = createConstantWire(iv);
                let keywire = createConstantWire(key);

                makeOutputArray(
                    ZkayCBCSymmetricEncGadget::new(
                        plainwire,
                        keywire,
                        ivwire,
                        ZkayCBCSymmetricEncGadget.CipherType.CHASKEY,
                    )
                    .getOutputWires(),
                );
            }

            pub fn generateSampleInput(evaluator: &mut CircuitEvaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        let mut evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate(generator.cg());
        let outwires = cgen.get_out_wires();
        let outs = vec![BigInteger::default(); outwires.size()];
        for i in 0..outs.len() {
            outs[i] = evaluator.getWireValue(outwires.get(i));
        }

        // Compute encryption via CbcChaskey implementation
        let iv_bytes = ZkayUtil.unsignedBigintToBytes(iv, 16);
        let result = ChaskeyLtsCbc.crypt(
            true,
            ZkayUtil.unsignedBigintToBytes(key, 16),
            iv_bytes,
            ZkayUtil.unsignedBigintToBytes(plain, 32),
        );

        // Convert output to format produced by gadget (iv included, packed 248bit values in reverse order)
        let iv_cipher = vec![0; 16 + result.len()];
        iv_cipher[0..iv_bytes.len()].clone_from_slice(&iv_bytes[0..]);
        iv_cipher[iv_bytes.len()..result.len()].clone_from_slice(&result[0..]);

        let chunk_size = CIPHER_CHUNK_SIZE / 8;
        let first_chunk_size = iv_cipher.len() % chunk_size;
        let bigints = vec![];
        if first_chunk_size != 0 {
            let chunk = iv_cipher[0..first_chunk_size].to_vec();
            bigints.add(ZkayUtil.unsignedBytesToBigInt(chunk));
        }
        for i in first_chunk_size..iv_cipher.len() - first_chunk_size {
            let chunk = iv_cipher[i..i + chunk_size].to_vec();
            bigints.add(ZkayUtil.unsignedBytesToBigInt(chunk));
        }
        Collections.reverse(bigints);

        // Check if both are equal
        assert_eq!(outs, bigints.toArray());
    }
}
