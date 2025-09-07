#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::{self, LongElement},
        config::config::CONFIGS,
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
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::{
        hash::sha256_gadget::{Base, SHA256Gadget},
        math::{
            long_integer_division::LongIntegerDivision,
            long_integer_division::LongIntegerDivisionConfig,
            long_integer_mod_gadget::LongIntegerModGadget,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Div, Mul, Rem, Shl, Sub},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

//  * A gadget for RSA encryption according to PKCS#1 v2.2. The gadget assumes a
//  * hardcoded pub  exponent of 0x10001, and uses SHA256 as the hash function
//  * for mask generation function (mgf).
//  * This gadget can accept a hardcoded or a variable RSA modulus. See the
//  * corresponding generator example.
//  *
//  * This gadget is costly in comparison with the PKCS v1.5 RSA encryption gadget
//  * due to many SHA256 calls during mask generation.
//  *
//  * The implementation of this gadget follows the standard specs in:
//  * https://www.emc.com/collateral/white-
//  * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSAEncryptionOAEPGadget {
    pub modulus: LongElement,

    // every wire represents a byte in the following three arrays
    pub plain_text: Vec<Option<WireType>>,
    pub seed: Vec<Option<WireType>>,

    pub ciphertext: Vec<Option<WireType>>,

    pub rsa_key_bit_length: i32, // in bits (assumed to be divisible by 8)
}
impl RSAEncryptionOAEPGadget {
    pub const SHA256_DIGEST_LENGTH: i32 = 32; // in bytes
    pub fn new(
        modulus: LongElement,
        plain_text: Vec<Option<WireType>>,
        seed: Vec<Option<WireType>>,
        rsa_key_bit_length: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            rsa_key_bit_length % 8 == 0,
            "RSA Key bit length is assumed to be a multiple of 8"
        );

        assert!(
            plain_text.len() as i32 <= rsa_key_bit_length / 8 - 2 * Self::SHA256_DIGEST_LENGTH - 2,
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
                plain_text,
                modulus,
                ciphertext: vec![],
                rsa_key_bit_length,
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<RSAEncryptionOAEPGadget> {
    pub const lSHA256_HASH: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];
    fn build_circuit(&mut self) {
        let m_len = self.t.plain_text.len();
        let h_len = RSAEncryptionOAEPGadget::SHA256_DIGEST_LENGTH as usize;
        let key_len = self.t.rsa_key_bit_length as usize / 8; // in bytes
        let mut padding_string =
            vec![self.generator.get_zero_wire(); key_len - m_len - 2 * h_len - 2];

        let mut db = vec![None; key_len - h_len - 1];
        for i in 0..key_len - h_len - 1 {
            if i < h_len {
                db[i] = Some(CircuitGenerator::create_constant_wirei(
                    self.generator.clone(),
                    (Self::lSHA256_HASH[i] as i64 + 256) % 256,
                    &None,
                ));
            } else if i < h_len + padding_string.len() {
                db[i] = padding_string[i - h_len].clone();
            } else if i < h_len + padding_string.len() + 1 {
                db[i] = self.generator.get_one_wire();
            } else {
                db[i] = self.t.plain_text[i - (h_len + padding_string.len() + 1)].clone();
            }
        }

        let db_mask = self.mgf1(&self.t.seed, (key_len - h_len - 1) as i32);
        let mut masked_db = vec![None; key_len - h_len - 1];
        for i in 0..key_len - h_len - 1 {
            masked_db[i] = Some(db_mask[i].as_ref().unwrap().xor_bitwises(
                db[i].as_ref().unwrap(),
                8,
                &None,
            ));
        }

        let seeded_mask = self.mgf1(&masked_db, h_len as i32);
        let mut masked_seed = vec![None; h_len];
        for i in 0..h_len {
            masked_seed[i] = Some(seeded_mask[i].as_ref().unwrap().xor_bitwises(
                self.t.seed[i].as_ref().unwrap(),
                8,
                &None,
            ));
        }

        let padded_byte_array = Util::concat(&masked_seed, &masked_db); // Big-Endian

        // The LongElement implementation is LittleEndian, so we will process the array in reverse order

        let mut padded_msg =
            LongElement::newb(vec![BigInteger::ZERO], self.generator.clone().downgrade());
        for i in 0..padded_byte_array.len() {
            let e = LongElement::new(
                vec![padded_byte_array[padded_byte_array.len() - i - 1].clone()],
                vec![8],
                self.generator.clone().downgrade(),
            );
            let c = LongElement::newb(
                Util::split(&Util::one().shl(8 * i), LongElement::CHUNK_BITWIDTH),
                self.generator.clone().downgrade(),
            );
            padded_msg = padded_msg.add(&e.mul(&c));
        }

        // do modular exponentiation
        let mut s = padded_msg.clone();
        for i in 0..16 {
            s = s.clone().mul(&s);
            s = LongIntegerDivision::<LongIntegerModGadget>::new(
                s,
                self.t.modulus.clone(),
                self.t.rsa_key_bit_length,
                false,
                &None,
                self.generator.clone(),
            )
            .get_remainder()
            .clone();
        }
        s = s.mul(&padded_msg);
        s = LongIntegerDivision::<LongIntegerModGadget>::new(
            s,
            self.t.modulus.clone(),
            self.t.rsa_key_bit_length,
            true,
            &None,
            self.generator.clone(),
        )
        .get_remainder()
        .clone();

        // return the cipher text as byte array
        self.t.ciphertext = s
            .get_bitsi(self.t.rsa_key_bit_length)
            .pack_bits_into_words(8, &None);
    }

    pub fn check_seed_compliance(&self) {
        for i in 0..self.t.seed.len() {
            // Verify that the seed wires are bytes
            // This is also checked already by the sha256 gadget in the mgf1 calls, but added here for clarity
            self.t.seed[i]
                .as_ref()
                .unwrap()
                .restrict_bit_length(8, &None);
        }
    }

    fn mgf1(&self, ins: &Vec<Option<WireType>>, length: i32) -> Vec<Option<WireType>> {
        let mut mgf_output_list = vec![];
        for i in 0..=(length as f64 / RSAEncryptionOAEPGadget::SHA256_DIGEST_LENGTH as f64).ceil()
            as i64
            - 1
        {
            // the standard follows a Big Endian format
            let counter = CircuitGenerator::create_constant_wire_arrayi(
                self.generator.clone(),
                &vec![(i >> 24), (i >> 16), (i >> 8), i],
                &None,
            );

            let input_to_hash = Util::concat(&ins, &counter);
            let sha_gadget = SHA256Gadget::new(
                input_to_hash.clone(),
                8,
                input_to_hash.len(),
                false,
                true,
                &None,
                self.generator.clone(),
                Base,
            );
            let digest = sha_gadget.get_output_wires();

            let mut msg_hash_bytes =
                WireArray::new(digest.clone(), self.generator.clone().downgrade())
                    .get_bits(32, &None)
                    .pack_bits_into_words(8, &None);
            // reverse the byte array representation of each word of the digest
            // to
            // be compatible with the endianess
            for j in 0..8 {
                msg_hash_bytes.swap(4 * j, 4 * j + 3);
                msg_hash_bytes.swap(4 * j + 1, 4 * j + 2);
            }
            for j in 0..msg_hash_bytes.len() {
                mgf_output_list.push(msg_hash_bytes[j].clone());
            }
        }
        let out = mgf_output_list; //.toArray(&None);
        out[..length as usize].to_vec()
    }
}
impl GadgetConfig for Gadget<RSAEncryptionOAEPGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
