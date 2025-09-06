#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_array;
use crate::examples::gadgets::blockciphers::speck128_cipher_gadget::Speck128CipherGadget;
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
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
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{BigInteger, Util};
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use zkay_derive::ImplStructNameConfig;

//  * Performs symmetric encryption in the CBC mode.
//  * Only supports one cipher (speck128) as an example at the moment. Other ciphers will be integrated soon.

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SymmetricEncryptionCBCGadget {
    pub ciphertext: Vec<Option<WireType>>,
    pub cipherName: String,
    pub keyBits: Vec<Option<WireType>>,
    pub plaintextBits: Vec<Option<WireType>>,
    pub ivBits: Vec<Option<WireType>>,
}
impl SymmetricEncryptionCBCGadget {
    const keysize: i32 = 128;
    pub fn new(
        plaintextBits: Vec<Option<WireType>>,
        keyBits: Vec<Option<WireType>>,
        ivBits: Vec<Option<WireType>>,
        cipherName: String,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            keyBits.len() as i32 == Self::keysize && ivBits.len() as i32 == Self::keysize,
            "Key and IV bit vectors should be of length 128"
        );
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plaintextBits,
                ivBits,
                keyBits,
                cipherName,
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
        let numBlocks =
            (self.t.plaintextBits.len() as f64 * 1.0 / Self::blocksize as f64).ceil() as i32;
        let mut plaintextBits = WireArray::new(
            self.t.plaintextBits.clone(),
            self.generator.clone().downgrade(),
        )
        .adjust_length(None, (numBlocks * Self::blocksize) as usize)
        .as_array()
        .clone();

        let preparedKey = self.prepareKey();
        let mut prevCipher =
            WireArray::new(self.t.ivBits.clone(), self.generator.clone().downgrade());

        let mut ciphertext = vec![];
        for i in 0..numBlocks as usize {
            let msgBlock = WireArray::new(
                plaintextBits[i * Self::blocksize as usize..(i + 1) * Self::blocksize as usize]
                    .to_vec(),
                self.generator.clone().downgrade(),
            );
            let xored = msgBlock
                .xor_wire_arrayi(&prevCipher, &None)
                .as_array()
                .clone();
            assert!(
                &self.t.cipherName != "speck128",
                "Other Ciphers not supported in this version!"
            );
            let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                .pack_bits_into_words(64, &None);
            let gadget =
                Speck128CipherGadget::new(tmp, preparedKey.clone(), &None, self.generator.clone());
            let outputs = gadget.get_output_wires();
            prevCipher = WireArray::new(outputs.clone(), self.generator.clone().downgrade())
                .get_bits(64, &None);

            ciphertext = Util::concat(&ciphertext, &prevCipher.pack_bits_into_words(64, &None));
        }
    }

    fn prepareKey(&self) -> Vec<Option<WireType>> {
        assert!(
            &self.t.cipherName != "speck128",
            "Other Ciphers not supported in this version!"
        );

        let packedKey = WireArray::new(self.t.keyBits.clone(), self.generator.clone().downgrade())
            .pack_bits_into_words(64, &None);
        let preparedKey = Gadget::<Speck128CipherGadget>::expandKey(&packedKey, &self.generator);

        preparedKey
    }
}
impl GadgetConfig for Gadget<SymmetricEncryptionCBCGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
