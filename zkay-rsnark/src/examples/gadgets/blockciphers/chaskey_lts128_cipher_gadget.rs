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
        auxiliary::long_element::LongElement,
        config::config::CONFIGS,
        eval::instruction::Instruction,
        operations::{
            gadget::{Gadget, GadgetConfig},
            wire_label_instruction,
            wire_label_instruction::LabelType,
        },
        structure::{
            circuit_generator::CGConfigFields,
            circuit_generator::CGInstance,
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator,
            },
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{ARcCell, BigInteger, Util},
};

use rccell::RcCell;
use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::Add,
};
use zkay_derive::ImplStructNameConfig;

//  * Implements the light weight cipher Chaskey128, the LTS version with 16 rounds
//  * https://eprint.iacr.org/2014/386.pdf.
//  *
//  * The gadget follows the reference implementation from this project:
//  * https://www.nist.gov/sites/default/files/documents/2016/10/18/perrin-paper-lwc2016.pdf
//  * https://www.cryptolux.org/index.php/FELICS

// crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct ChaskeyLTS128CipherGadget {
    pub plaintext: Vec<Option<WireType>>,  // 4 32-bit words
    pub key: Vec<Option<WireType>>,        // 4 32-bit words
    pub ciphertext: Vec<Option<WireType>>, // 4 32-bit words
}
impl ChaskeyLTS128CipherGadget {
    #[inline]
    pub fn new(
        inputs: Vec<Option<WireType>>,
        key: Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        Self::new_with_option(inputs, key, &None, generator)
    }

    pub fn new_with_option(
        inputs: Vec<Option<WireType>>,
        key: Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(inputs.len() == 4 && key.len() == 4, "Invalid Input");
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plaintext: inputs,
                ciphertext: vec![],
                key,
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<ChaskeyLTS128CipherGadget> {
    fn build_circuit(&mut self) {
        let mut v: Vec<_> = (0..4)
            .map(|i| {
                self.t.plaintext[i]
                    .as_ref()
                    .unwrap()
                    .xor_bitwises(self.t.key[i].as_ref().unwrap(), 32)
            })
            .collect();

        for i in 0..16 {
            v[0] = v[0].clone().add(&v[1]);
            v[0] = v[0].trim_bits(33, 32);
            v[1] = v[1].rotate_left(32, 5).xor_bitwises(&v[0], 32);
            v[0] = v[0].rotate_left(32, 16);

            v[2] = v[2].clone().add(&v[3]).trim_bits(33, 32);
            v[3] = v[3].rotate_left(32, 8).xor_bitwises(&v[2], 32);

            v[0] = v[0].clone().add(&v[3]).trim_bits(33, 32);
            v[3] = v[3].rotate_left(32, 13).xor_bitwises(&v[0], 32);

            v[2] = v[2].clone().add(&v[1]).trim_bits(33, 32);
            v[1] = v[1].rotate_left(32, 7).xor_bitwises(&v[2], 32);
            v[2] = v[2].rotate_left(32, 16);
        }

        for i in 0..4 {
            v[i] = v[i].xor_bitwises(self.t.key[i].as_ref().unwrap(), 32);
        }
        self.t.ciphertext = v.into_iter().map(|x| Some(x)).collect();
    }
}
impl GadgetConfig for Gadget<ChaskeyLTS128CipherGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
