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
        auxiliary::long_element::LongElement,
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::{Gadget, GadgetConfig},
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
            wire_array,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::blockciphers::speck128_cipher_gadget::Speck128CipherGadget,
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use rccell::RcCell;
use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
};
use zkay_derive::ImplStructNameConfig;

//  * Performs symmetric encryption in the CBC mode.
//  * Only supports one cipher (speck128) as an example at the moment. Other ciphers will be integrated soon.

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SymmetricEncryptionCBCGadget {
    pub ciphertext: Vec<Option<WireType>>,
    pub cipher_name: String,
    pub key_bits: Vec<Option<WireType>>,
    pub plaintext_bits: Vec<Option<WireType>>,
    pub iv_bits: Vec<Option<WireType>>,
}
impl SymmetricEncryptionCBCGadget {
    const keysize: i32 = 128;
    #[inline]
    pub fn new(
        plaintext_bits: Vec<Option<WireType>>,
        key_bits: Vec<Option<WireType>>,
        iv_bits: Vec<Option<WireType>>,
        cipher_name: String,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        Self::new_with_option(
            plaintext_bits,
            key_bits,
            iv_bits,
            cipher_name,
            &None,
            generator,
        )
    }
    pub fn new_with_option(
        plaintext_bits: Vec<Option<WireType>>,
        key_bits: Vec<Option<WireType>>,
        iv_bits: Vec<Option<WireType>>,
        cipher_name: String,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            key_bits.len() as i32 == Self::keysize && iv_bits.len() as i32 == Self::keysize,
            "Key and IV bit vectors should be of length 128"
        );
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plaintext_bits,
                iv_bits,
                key_bits,
                cipher_name,
                ciphertext: vec![],
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<SymmetricEncryptionCBCGadget> {
    const blocksize: i32 = 128;

    fn build_circuit(&mut self) {
        let num_blocks =
            (self.t.plaintext_bits.len() as f64 * 1.0 / Self::blocksize as f64).ceil() as i32;
        let mut plaintext_bits = WireArray::new(
            self.t.plaintext_bits.clone(),
            self.generator.clone().downgrade(),
        )
        .adjust_length(None, (num_blocks * Self::blocksize) as usize)
        .as_array()
        .clone();

        let prepared_key = self.prepare_key();
        let mut prev_cipher =
            WireArray::new(self.t.iv_bits.clone(), self.generator.clone().downgrade());

        let mut ciphertext = vec![];
        for i in 0..num_blocks as usize {
            let msg_block = WireArray::new(
                plaintext_bits[i * Self::blocksize as usize..(i + 1) * Self::blocksize as usize]
                    .to_vec(),
                self.generator.clone().downgrade(),
            );
            let xored = msg_block.xor_wire_arrayi(&prev_cipher).as_array().clone();
            assert!(
                &self.t.cipher_name != "speck128",
                "Other Ciphers not supported in this version!"
            );
            let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                .pack_bits_into_words(64);
            let gadget =
                Speck128CipherGadget::new(tmp, prepared_key.clone(), self.generator.clone());
            let outputs = gadget.get_output_wires();
            prev_cipher =
                WireArray::new(outputs.clone(), self.generator.clone().downgrade()).get_bits(64);

            ciphertext = Util::concat(&ciphertext, &prev_cipher.pack_bits_into_words(64));
        }
    }

    fn prepare_key(&self) -> Vec<Option<WireType>> {
        assert!(
            &self.t.cipher_name != "speck128",
            "Other Ciphers not supported in this version!"
        );

        let packed_key =
            WireArray::new(self.t.key_bits.clone(), self.generator.clone().downgrade())
                .pack_bits_into_words(64);
        let prepared_key = Gadget::<Speck128CipherGadget>::expandKey(&packed_key, &self.generator);

        prepared_key
    }
}
impl GadgetConfig for Gadget<SymmetricEncryptionCBCGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
