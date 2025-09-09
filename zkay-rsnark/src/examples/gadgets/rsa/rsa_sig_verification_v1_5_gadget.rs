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
    examples::gadgets::math::{
        long_integer_division::{LongIntegerDivision, LongIntegerDivisionConfig},
        long_integer_mod_gadget::LongIntegerModGadget,
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
    ops::{Add, Div, Mul, Rem, Sub},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

//  * A gadget to check if an RSA signature is valid according to PKCS 1 v1.5 (A
//  * gadget based on the latest standard (PSS) will be added in the future).
//  * This gadget assumes SHA256 for the message hash, and a pub  exponent of
//  * 0x10001.
//  * This gadget can accept a hardcoded or a variable RSA modulus. See the
//  * corresponding generator example.
//  *
//  * Implemented according to the standard specs here:
//  * https://www.emc.com/collateral/white-
//  * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSASigVerificationV1_5_Gadget {
    pub modulus: LongElement,
    pub signature: LongElement,
    pub msg_hash: Vec<Option<WireType>>, // 32-bit wires (the output of SHA256 gadget)
    pub is_valid_signature: Vec<Option<WireType>>,
    pub rsa_key_bit_length: i32, // in bits
}
impl RSASigVerificationV1_5_Gadget {
    #[inline]
    pub fn new(
        modulus: LongElement,
        msg_hash: Vec<Option<WireType>>,
        signature: LongElement,
        rsa_key_bit_length: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        Self::new_with_option(
            modulus,
            msg_hash,
            signature,
            rsa_key_bit_length,
            &None,
            generator,
        )
    }
    pub fn new_with_option(
        modulus: LongElement,
        msg_hash: Vec<Option<WireType>>,
        signature: LongElement,
        rsa_key_bit_length: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                modulus,
                msg_hash,
                signature,
                rsa_key_bit_length,
                is_valid_signature: vec![],
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<RSASigVerificationV1_5_Gadget> {
    pub const SHA256_IDENTIFIER: [u8; 19] = [
        0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01,
        0x05, 0x00, 0x04, 0x20,
    ];

    pub const SHA256_DIGEST_LENGTH: usize = 32; // in bytes
    fn build_circuit(&mut self) {
        let mut s = self.t.signature.clone();

        for i in 0..16 {
            s = s.clone().mul(&s);
            s = LongIntegerModGadget::new(
                s,
                self.t.modulus.clone(),
                self.t.rsa_key_bit_length,
                false,
                self.generator.clone(),
            )
            .get_remainder()
            .clone();
        }
        s = s.mul(&self.t.signature);
        s = LongIntegerModGadget::new(
            s,
            self.t.modulus.clone(),
            self.t.rsa_key_bit_length,
            true,
            self.generator.clone(),
        )
        .get_remainder()
        .clone();
        let s_chunks = s.get_array();

        // note that the following can be improved, but for simplicity we
        // are going to compare byte by byte

        // get byte arrays
        let mut s_bytes = WireArray::new(s_chunks.clone(), self.generator.clone().downgrade())
            .get_bits(LongElement::CHUNK_BITWIDTH as usize)
            .pack_bits_into_words(8);
        let mut msg_hash_bytes =
            WireArray::new(self.t.msg_hash.clone(), self.generator.clone().downgrade())
                .get_bits(32)
                .pack_bits_into_words(8);

        // reverse the byte array representation of each word of the digest to
        // be compatiable with the endianess
        for i in 0..8 {
            msg_hash_bytes.swap(4 * i, 4 * i + 3);
            msg_hash_bytes.swap(4 * i + 1, 4 * i + 2);
        }

        let length_in_bytes = (self.t.rsa_key_bit_length as f64 / 8.0).ceil() as usize;
        let mut sum_checks = self.generator.get_zero_wire().unwrap();
        sum_checks = sum_checks.add(
            s_bytes[length_in_bytes - 1]
                .as_ref()
                .unwrap()
                .is_equal_toi(0),
        );
        sum_checks = sum_checks.add(
            s_bytes[length_in_bytes - 2]
                .as_ref()
                .unwrap()
                .is_equal_toi(1),
        );
        for i in 3..length_in_bytes - Self::SHA256_DIGEST_LENGTH - Self::SHA256_IDENTIFIER.len() {
            sum_checks = sum_checks.add(
                s_bytes[length_in_bytes - i]
                    .as_ref()
                    .unwrap()
                    .is_equal_toi(0xff),
            );
        }
        sum_checks = sum_checks.add(
            s_bytes[Self::SHA256_DIGEST_LENGTH + Self::SHA256_IDENTIFIER.len()]
                .as_ref()
                .unwrap()
                .is_equal_toi(0),
        );

        for i in 0..Self::SHA256_IDENTIFIER.len() {
            sum_checks = sum_checks.add(
                s_bytes[Self::SHA256_IDENTIFIER.len() + Self::SHA256_DIGEST_LENGTH - 1 - i]
                    .as_ref()
                    .unwrap()
                    .is_equal_toi((Self::SHA256_IDENTIFIER[i] as i64 + 256) % 256),
            );
        }
        for i in (0..=Self::SHA256_DIGEST_LENGTH - 1).rev() {
            sum_checks = sum_checks.add(
                s_bytes[Self::SHA256_DIGEST_LENGTH - 1 - i]
                    .as_ref()
                    .unwrap()
                    .is_equal_tos(msg_hash_bytes[i].as_ref().unwrap()),
            );
        }

        self.t.is_valid_signature = vec![Some(sum_checks.is_equal_toi(length_in_bytes as i64))];
    }
}
impl GadgetConfig for Gadget<RSASigVerificationV1_5_Gadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.is_valid_signature
    }
}
