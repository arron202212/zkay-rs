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
                assert_basic_op::{AssertBasicOp, new_assert},
                basic_op::BasicOp,
                mul_basic_op::{MulBasicOp, new_mul},
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::{ConstantWire, new_constant},
            variable_bit_wire::VariableBitWire,
            variable_wire::{VariableWire, new_variable},
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
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{BigInteger, Util};
use crate::examples::gadgets::math::field_division_gadget::FieldDivisionGadget;
use crate::examples::gadgets::math::long_integer_division::{
    LongIntegerDivision, LongIntegerDivisionConfig,
};
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Div, Mul, Rem, Shl, Sub};
/**
 * A gadget for RSA encryption according to PKCS#1 v1.5. A future version will
 * have the RSA-OAEP method according to PKCS#1 v2.x. The gadget assumes a
 * hardcoded pub  exponent of 0x10001.
 * This gadget can accept a hardcoded or a variable RSA modulus. See the
 * corresponding generator example.
 *
 * Implemented according to the standard specs here:
 * https://www.emc.com/collateral/white-
 * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf
 *
 */
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSAEncryptionV1_5_Gadget {
    pub modulus: LongElement,

    // every wire represents a byte in the following three arrays
    pub plainText: Vec<Option<WireType>>,
    pub randomness: Vec<Option<WireType>>, // (rsaKeyBitLength / 8 - 3 - plainTextLength)
    // non-zero bytes
    pub ciphertext: Vec<Option<WireType>>,

    pub rsaKeyBitLength: i32, // in bits (assumed to be divisible by 8)
}
impl RSAEncryptionV1_5_Gadget {
    pub fn new(
        modulus: LongElement,
        plainText: Vec<Option<WireType>>,
        randomness: Vec<Option<WireType>>,
        rsaKeyBitLength: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            rsaKeyBitLength % 8 == 0,
            "RSA Key bit length is assumed to be a multiple of 8"
        );

        //println!("Check Message & Padding length");
        assert!(
            plainText.len() <= rsaKeyBitLength as usize / 8 - 11
                && plainText.len() + randomness.len() == rsaKeyBitLength as usize / 8 - 3,
            "Invalid Argument Dimensions for RSA Encryption"
        );

        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                randomness,
                plainText,
                modulus,
                ciphertext: vec![],
                rsaKeyBitLength,
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<RSAEncryptionV1_5_Gadget> {
    fn buildCircuit(&mut self) {
        let lengthInBytes = self.t.rsaKeyBitLength as usize / 8;
        let mut paddedPlainText = vec![None; lengthInBytes];
        for i in 0..self.t.plainText.len() {
            paddedPlainText[self.t.plainText.len() - i - 1] = self.t.plainText[i].clone();
        }
        paddedPlainText[self.t.plainText.len()] = self.generator.get_zero_wire();
        for i in 0..self.t.randomness.len() {
            paddedPlainText[self.t.plainText.len() + 1 + (self.t.randomness.len() - 1) - i] =
                self.t.randomness[i].clone();
        }
        paddedPlainText[lengthInBytes - 2] = Some(self.generator.createConstantWirei(2, &None));
        paddedPlainText[lengthInBytes - 1] = self.generator.get_zero_wire();

        /*
         * To proceed with the RSA operations, we need to convert the
         * padddedPlainText array to a long element. Two ways to do that.
         */
        // 1. safest method:
        //		 WireArray allBits = WireArray::new(paddedPlainText).getBits(8);
        //		 LongElement paddedMsg = LongElement::new(allBits);

        // 2. Make multiple long integer constant multiplications (need to be
        // done carefully)
        let mut paddedMsg =
            LongElement::newb(vec![BigInteger::ZERO], self.generator.clone().downgrade());
        for i in 0..paddedPlainText.len() {
            let e = LongElement::new(
                vec![paddedPlainText[i].clone()],
                vec![8],
                self.generator.clone().downgrade(),
            );
            let c = LongElement::newb(
                Util::split(&Util::one().shl(8 * i), LongElement::CHUNK_BITWIDTH),
                self.generator.clone().downgrade(),
            );
            paddedMsg = paddedMsg.add(&e.mul(&c));
        }

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
    pub fn getExpectedRandomnessLength(rsaKeyBitLength: i32, plainTextLength: i32) -> i32 {
        assert!(
            rsaKeyBitLength % 8 == 0,
            "RSA Key bit length is assumed to be a multiple of 8"
        );

        rsaKeyBitLength / 8 - 3 - plainTextLength
    }

    pub fn checkRandomnessCompliance(&self) {
        // assert the randomness vector has non-zero bytes
        for i in 0..self.t.randomness.len() {
            self.t.randomness[i]
                .as_ref()
                .unwrap()
                .restrictBitLength(8, &None);
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
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
