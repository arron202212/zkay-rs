#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            gadget::GadgetConfig,
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_array;
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{BigInteger, Util};
use crate::examples::gadgets::hash::sha256_gadget::{Base, SHA256Gadget};
use crate::examples::gadgets::math::long_integer_division::LongIntegerDivision;
use crate::examples::gadgets::math::long_integer_division::LongIntegerDivisionConfig;
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Div, Mul, Rem, Shl, Sub};
/**
 * A gadget for RSA encryption according to PKCS#1 v2.2. The gadget assumes a
 * hardcoded pub  exponent of 0x10001, and uses SHA256 as the hash function
 * for mask generation function (mgf).
 * This gadget can accept a hardcoded or a variable RSA modulus. See the
 * corresponding generator example.
 *
 * This gadget is costly in comparison with the PKCS v1.5 RSA encryption gadget
 * due to many SHA256 calls during mask generation.
 *
 * The implementation of this gadget follows the standard specs in:
 * https://www.emc.com/collateral/white-
 * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf
 */
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSAEncryptionOAEPGadget {
    pub modulus: LongElement,

    // every wire represents a byte in the following three arrays
    pub plainText: Vec<Option<WireType>>,
    pub seed: Vec<Option<WireType>>,

    pub ciphertext: Vec<Option<WireType>>,

    pub rsaKeyBitLength: i32, // in bits (assumed to be divisible by 8)
}
impl RSAEncryptionOAEPGadget {
    pub const SHA256_DIGEST_LENGTH: i32 = 32; // in bytes
    pub fn new(
        modulus: LongElement,
        plainText: Vec<Option<WireType>>,
        seed: Vec<Option<WireType>>,
        rsaKeyBitLength: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            rsaKeyBitLength % 8 == 0,
            "RSA Key bit length is assumed to be a multiple of 8"
        );

        assert!(
            plainText.len() as i32 <= rsaKeyBitLength / 8 - 2 * Self::SHA256_DIGEST_LENGTH - 2,
            "Message too long,Invalid message length for RSA Encryption"
        );

        assert!(
            seed.len() as i32 == Self::SHA256_DIGEST_LENGTH,
            "Seed must have the same length as the hash function output,Invalid seed dimension for RSA Encryption"
        );
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                seed,
                plainText,
                modulus,
                ciphertext: vec![],
                rsaKeyBitLength,
            },
        );

        _self.buildCircuit();
        _self
    }
}
impl Gadget<RSAEncryptionOAEPGadget> {
    pub const lSHA256_HASH: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];
    fn buildCircuit(&mut self) {
        let mLen = self.t.plainText.len();
        let hLen = RSAEncryptionOAEPGadget::SHA256_DIGEST_LENGTH as usize;
        let keyLen = self.t.rsaKeyBitLength as usize / 8; // in bytes
        let mut paddingString = vec![self.generator.get_zero_wire(); keyLen - mLen - 2 * hLen - 2];

        let mut db = vec![None; keyLen - hLen - 1];
        for i in 0..keyLen - hLen - 1 {
            if i < hLen {
                db[i] = Some(
                    self.generator
                        .createConstantWirei((Self::lSHA256_HASH[i] as i64 + 256) % 256, &None),
                );
            } else if i < hLen + paddingString.len() {
                db[i] = paddingString[i - hLen].clone();
            } else if i < hLen + paddingString.len() + 1 {
                db[i] = self.generator.get_one_wire();
            } else {
                db[i] = self.t.plainText[i - (hLen + paddingString.len() + 1)].clone();
            }
        }

        let dbMask = self.mgf1(&self.t.seed, (keyLen - hLen - 1) as i32);
        let mut maskedDb = vec![None; keyLen - hLen - 1];
        for i in 0..keyLen - hLen - 1 {
            maskedDb[i] = Some(dbMask[i].as_ref().unwrap().xorBitwise(
                db[i].as_ref().unwrap(),
                8,
                &None,
            ));
        }

        let seededMask = self.mgf1(&maskedDb, hLen as i32);
        let mut maskedSeed = vec![None; hLen];
        for i in 0..hLen {
            maskedSeed[i] = Some(seededMask[i].as_ref().unwrap().xorBitwise(
                self.t.seed[i].as_ref().unwrap(),
                8,
                &None,
            ));
        }

        let paddedByteArray = Util::concat(&maskedSeed, &maskedDb); // Big-Endian

        // The LongElement implementation is LittleEndian, so we will process the array in reverse order

        let mut paddedMsg =
            LongElement::newb(vec![BigInteger::ZERO], self.generator.clone().downgrade());
        for i in 0..paddedByteArray.len() {
            let e = LongElement::new(
                vec![paddedByteArray[paddedByteArray.len() - i - 1].clone()],
                vec![8],
                self.generator.clone().downgrade(),
            );
            let c = LongElement::newb(
                Util::split(&Util::one().shl(8 * i), LongElement::CHUNK_BITWIDTH),
                self.generator.clone().downgrade(),
            );
            paddedMsg = paddedMsg.add(&e.mul(&c));
        }

        // do modular exponentiation
        let mut s = paddedMsg.clone();
        for i in 0..16 {
            s = s.clone().mul(&s);
            s = LongIntegerDivision::<LongIntegerModGadget>::new(
                s,
                self.t.modulus.clone(),
                self.t.rsaKeyBitLength,
                false,
                &None,
                self.generator.clone(),
            )
            .getRemainder()
            .clone();
        }
        s = s.mul(&paddedMsg);
        s = LongIntegerDivision::<LongIntegerModGadget>::new(
            s,
            self.t.modulus.clone(),
            self.t.rsaKeyBitLength,
            true,
            &None,
            self.generator.clone(),
        )
        .getRemainder()
        .clone();

        // return the cipher text as byte array
        self.t.ciphertext = s
            .getBitsi(self.t.rsaKeyBitLength)
            .packBitsIntoWords(8, &None);
    }

    pub fn checkSeedCompliance(&self) {
        for i in 0..self.t.seed.len() {
            // Verify that the seed wires are bytes
            // This is also checked already by the sha256 gadget in the mgf1 calls, but added here for clarity
            self.t.seed[i].as_ref().unwrap().restrictBitLength(8, &None);
        }
    }

    fn mgf1(&self, ins: &Vec<Option<WireType>>, length: i32) -> Vec<Option<WireType>> {
        let mut mgfOutputList = vec![];
        for i in 0..=(length as f64 / RSAEncryptionOAEPGadget::SHA256_DIGEST_LENGTH as f64).ceil()
            as i64
            - 1
        {
            // the standard follows a Big Endian format
            let counter = self
                .generator
                .createConstantWireArrayi(&vec![(i >> 24), (i >> 16), (i >> 8), i], &None);

            let inputToHash = Util::concat(&ins, &counter);
            let shaGadget = SHA256Gadget::new(
                inputToHash.clone(),
                8,
                inputToHash.len(),
                false,
                true,
                &None,
                self.generator.clone(),
                Base,
            );
            let digest = shaGadget.getOutputWires();

            let mut msgHashBytes =
                WireArray::new(digest.clone(), self.generator.clone().downgrade())
                    .getBits(32, &None)
                    .packBitsIntoWords(8, &None);
            // reverse the byte array representation of each word of the digest
            // to
            // be compatible with the endianess
            for j in 0..8 {
                msgHashBytes.swap(4 * j, 4 * j + 3);
                msgHashBytes.swap(4 * j + 1, 4 * j + 2);
            }
            for j in 0..msgHashBytes.len() {
                mgfOutputList.push(msgHashBytes[j].clone());
            }
        }
        let out = mgfOutputList; //.toArray(&None);
        out[..length as usize].to_vec()
    }
}
impl GadgetConfig for Gadget<RSAEncryptionOAEPGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
