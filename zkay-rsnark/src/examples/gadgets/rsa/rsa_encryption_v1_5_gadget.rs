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
        field_division_gadget::FieldDivisionGadget,
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
    ops::{Add, Div, Mul, Rem, Shl, Sub},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

//  * A gadget for RSA encryption according to PKCS#1 v1.5. A future version will
//  * have the RSA-Oaep method according to PKCS#1 v2.x. The gadget assumes a
//  * hardcoded pub  exponent of 0x10001.
//  * This gadget can accept a hardcoded or a variable RSA modulus. See the
//  * corresponding generator example.
//  *
//  * Implemented according to the standard specs here:
//  * https://www.emc.com/collateral/white-
//  * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSAEncryptionV1_5_Gadget {
    pub modulus: LongElement,

    // every wire represents a byte in the following three arrays
    pub plain_text: Vec<Option<WireType>>,
    pub randomness: Vec<Option<WireType>>, // (rsa_key_bit_length / 8 - 3 - plain_text_length)
    // non-zero bytes
    pub ciphertext: Vec<Option<WireType>>,

    pub rsa_key_bit_length: i32, // in bits (assumed to be divisible by 8)
}
impl RSAEncryptionV1_5_Gadget {
    pub fn new(
        modulus: LongElement,
        plain_text: Vec<Option<WireType>>,
        randomness: Vec<Option<WireType>>,
        rsa_key_bit_length: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            rsa_key_bit_length % 8 == 0,
            "RSA Key bit length is assumed to be a multiple of 8"
        );

        //println!("Check Message & Padding length");
        assert!(
            plain_text.len() <= rsa_key_bit_length as usize / 8 - 11
                && plain_text.len() + randomness.len() == rsa_key_bit_length as usize / 8 - 3,
            "Invalid Argument Dimensions for RSA Encryption"
        );

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                randomness,
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
impl Gadget<RSAEncryptionV1_5_Gadget> {
    fn build_circuit(&mut self) {
        let length_in_bytes = self.t.rsa_key_bit_length as usize / 8;
        let mut padded_plain_text = vec![None; length_in_bytes];
        for i in 0..self.t.plain_text.len() {
            padded_plain_text[self.t.plain_text.len() - i - 1] = self.t.plain_text[i].clone();
        }
        padded_plain_text[self.t.plain_text.len()] = self.generator.get_zero_wire();
        for i in 0..self.t.randomness.len() {
            padded_plain_text[self.t.plain_text.len() + 1 + (self.t.randomness.len() - 1) - i] =
                self.t.randomness[i].clone();
        }
        padded_plain_text[length_in_bytes - 2] = Some(CircuitGenerator::create_constant_wirei(
            self.generator.clone(),
            2,
        ));
        padded_plain_text[length_in_bytes - 1] = self.generator.get_zero_wire();

        //To proceed with the RSA operations, we need to convert the
        //padddedPlainText array to a long element. Two ways to do that.

        // 1. safest method:
        //		 WireArray allBits = WireArray::new(padded_plain_text).get_bits(8);
        //		 LongElement padded_msg = LongElement::new(allBits);

        // 2. Make multiple long integer constant multiplications (need to be
        // done carefully)
        let mut padded_msg =
            LongElement::newb(vec![BigInteger::ZERO], self.generator.clone().downgrade());
        for i in 0..padded_plain_text.len() {
            let e = LongElement::new(
                vec![padded_plain_text[i].clone()],
                vec![8],
                self.generator.clone().downgrade(),
            );
            let c = LongElement::newb(
                Util::split(&Util::one().shl(8 * i), LongElement::CHUNK_BITWIDTH),
                self.generator.clone().downgrade(),
            );
            padded_msg = padded_msg.add(&e.mul(&c));
        }

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
    pub fn get_expected_randomness_length(rsa_key_bit_length: i32, plain_text_length: i32) -> i32 {
        assert!(
            rsa_key_bit_length % 8 == 0,
            "RSA Key bit length is assumed to be a multiple of 8"
        );

        rsa_key_bit_length / 8 - 3 - plain_text_length
    }

    pub fn check_randomness_compliance(&self) {
        // assert the randomness vector has non-zero bytes
        for i in 0..self.t.randomness.len() {
            self.t.randomness[i]
                .as_ref()
                .unwrap()
                .restrict_bit_length(8);
            // verify that each element has a multiplicative inverse
            FieldDivisionGadget::new(
                self.generator.get_one_wire().unwrap(),
                self.t.randomness[i].clone().unwrap(),
                &None,
                self.generator.clone(),
            );
        }
    }
}
impl GadgetConfig for Gadget<RSAEncryptionV1_5_Gadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
