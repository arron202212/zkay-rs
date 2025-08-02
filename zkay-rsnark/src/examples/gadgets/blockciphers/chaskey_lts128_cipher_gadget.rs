#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::GadgetConfig;
use crate::{
    arc_cell_new,
    circuit::{
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::instruction::Instruction,
        operations::{gadget::Gadget, wire_label_instruction, wire_label_instruction::LabelType},
        structure::{
            circuit_generator::CGConfigFields,
            circuit_generator::CGInstance,
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
                getActiveCircuitGenerator,
            },
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{ARcCell, BigInteger, Util},
};
// use crate::circuit::structure::wire_type::WireType;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Add;
use zkay_derive::ImplStructNameConfig;
/**
 * Implements the light weight cipher Chaskey128, the LTS version with 16 rounds
 * https://eprint.iacr.org/2014/386.pdf.
 *
 * The gadget follows the reference implementation from this project:
 * https://www.nist.gov/sites/default/files/documents/2016/10/18/perrin-paper-lwc2016.pdf
 * https://www.cryptolux.org/index.php/FELICS
 */
// crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct ChaskeyLTS128CipherGadget {
    plaintext: Vec<Option<WireType>>,  // 4 32-bit words
    key: Vec<Option<WireType>>,        // 4 32-bit words
    ciphertext: Vec<Option<WireType>>, // 4 32-bit words
}
impl ChaskeyLTS128CipherGadget {
    pub fn new(
        inputs: Vec<Option<WireType>>,
        key: Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(inputs.len() == 4 && key.len() == 4, "Invalid Input");
        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                plaintext: inputs,
                ciphertext: vec![],
                key,
            },
        };

        _self.buildCircuit();
        _self
    }
}
impl Gadget<ChaskeyLTS128CipherGadget> {
    fn buildCircuit(&mut self) {
        let mut v: Vec<_> = (0..4)
            .map(|i| {
                self.t.plaintext[i].as_ref().unwrap().xorBitwise(
                    self.t.key[i].as_ref().unwrap(),
                    32,
                    &None,
                )
            })
            .collect();

        for i in 0..16 {
            v[0] = v[0].clone().add(&v[1]);
            v[0] = v[0].trimBits(33, 32, &None);
            v[1] = v[1].rotateLeft(32, 5, &None).xorBitwise(&v[0], 32, &None);
            v[0] = v[0].rotateLeft(32, 16, &None);

            v[2] = v[2].clone().add(&v[3]).trimBits(33, 32, &None);
            v[3] = v[3].rotateLeft(32, 8, &None).xorBitwise(&v[2], 32, &None);

            v[0] = v[0].clone().add(&v[3]).trimBits(33, 32, &None);
            v[3] = v[3].rotateLeft(32, 13, &None).xorBitwise(&v[0], 32, &None);

            v[2] = v[2].clone().add(&v[1]).trimBits(33, 32, &None);
            v[1] = v[1].rotateLeft(32, 7, &None).xorBitwise(&v[2], 32, &None);
            v[2] = v[2].rotateLeft(32, 16, &None);
        }

        for i in 0..4 {
            v[i] = v[i].xorBitwise(self.t.key[i].as_ref().unwrap(), 32, &None);
        }
        self.t.ciphertext = v.into_iter().map(|x| Some(x)).collect();
    }
}
impl GadgetConfig for Gadget<ChaskeyLTS128CipherGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
