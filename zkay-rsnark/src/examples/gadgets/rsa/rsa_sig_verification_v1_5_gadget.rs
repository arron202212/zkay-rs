#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
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
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
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
use crate::examples::gadgets::math::long_integer_division::{
    LongIntegerDivision, LongIntegerDivisionConfig,
};
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Div, Mul, Rem, Sub};

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

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSASigVerificationV1_5_Gadget {
    pub modulus: LongElement,
    pub signature: LongElement,
    pub msgHash: Vec<Option<WireType>>, // 32-bit wires (the output of SHA256 gadget)
    pub isValidSignature: Vec<Option<WireType>>,
    pub rsaKeyBitLength: i32, // in bits
}
impl RSASigVerificationV1_5_Gadget {
    pub fn new(
        modulus: LongElement,
        msgHash: Vec<Option<WireType>>,
        signature: LongElement,
        rsaKeyBitLength: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                modulus,
                msgHash,
                signature,
                rsaKeyBitLength,
                isValidSignature: vec![],
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
                self.t.rsaKeyBitLength,
                false,
                &None,
                self.generator.clone(),
            )
            .getRemainder()
            .clone();
        }
        s = s.mul(&self.t.signature);
        s = LongIntegerModGadget::new(
            s,
            self.t.modulus.clone(),
            self.t.rsaKeyBitLength,
            true,
            &None,
            self.generator.clone(),
        )
        .getRemainder()
        .clone();
        let sChunks = s.get_array();

        // note that the following can be improved, but for simplicity we
        // are going to compare byte by byte

        // get byte arrays
        let mut sBytes = WireArray::new(sChunks.clone(), self.generator.clone().downgrade())
            .get_bits(LongElement::CHUNK_BITWIDTH as usize, &None)
            .pack_bits_into_words(8, &None);
        let mut msgHashBytes =
            WireArray::new(self.t.msgHash.clone(), self.generator.clone().downgrade())
                .get_bits(32, &None)
                .pack_bits_into_words(8, &None);

        // reverse the byte array representation of each word of the digest to
        // be compatiable with the endianess
        for i in 0..8 {
            msgHashBytes.swap(4 * i, 4 * i + 3);
            msgHashBytes.swap(4 * i + 1, 4 * i + 2);
        }

        let lengthInBytes = (self.t.rsaKeyBitLength as f64 / 8.0).ceil() as usize;
        let mut sumChecks = self.generator.get_zero_wire().unwrap();
        sumChecks = sumChecks.add(
            sBytes[lengthInBytes - 1]
                .as_ref()
                .unwrap()
                .is_equal_toi(0, &None),
        );
        sumChecks = sumChecks.add(
            sBytes[lengthInBytes - 2]
                .as_ref()
                .unwrap()
                .is_equal_toi(1, &None),
        );
        for i in 3..lengthInBytes - Self::SHA256_DIGEST_LENGTH - Self::SHA256_IDENTIFIER.len() {
            sumChecks = sumChecks.add(
                sBytes[lengthInBytes - i]
                    .as_ref()
                    .unwrap()
                    .is_equal_toi(0xff, &None),
            );
        }
        sumChecks = sumChecks.add(
            sBytes[Self::SHA256_DIGEST_LENGTH + Self::SHA256_IDENTIFIER.len()]
                .as_ref()
                .unwrap()
                .is_equal_toi(0, &None),
        );

        for i in 0..Self::SHA256_IDENTIFIER.len() {
            sumChecks = sumChecks.add(
                sBytes[Self::SHA256_IDENTIFIER.len() + Self::SHA256_DIGEST_LENGTH - 1 - i]
                    .as_ref()
                    .unwrap()
                    .is_equal_toi((Self::SHA256_IDENTIFIER[i] as i64 + 256) % 256, &None),
            );
        }
        for i in (0..=Self::SHA256_DIGEST_LENGTH - 1).rev() {
            sumChecks = sumChecks.add(
                sBytes[Self::SHA256_DIGEST_LENGTH - 1 - i]
                    .as_ref()
                    .unwrap()
                    .is_equal_tos(msgHashBytes[i].as_ref().unwrap(), &None),
            );
        }

        self.t.isValidSignature = vec![Some(sumChecks.is_equal_toi(lengthInBytes as i64, &None))];
    }
}
impl GadgetConfig for Gadget<RSASigVerificationV1_5_Gadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.isValidSignature
    }
}
