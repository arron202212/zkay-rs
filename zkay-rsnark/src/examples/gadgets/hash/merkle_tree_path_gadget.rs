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
    examples::gadgets::hash::subset_sum_hash_gadget::SubsetSumHashGadget,
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use zkay_derive::ImplStructNameConfig;
//  * A Merkle tree authentication gadget using the subsetsum hash function

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct MerkleTreePathGadget {
    pub tree_height: i32,
    pub direction_selector_wire: WireType,
    pub direction_selector_bits: Vec<Option<WireType>>,
    pub leaf_wires: Vec<Option<WireType>>,
    pub intermediate_hash_wires: Vec<Option<WireType>>,
    pub out_root: Vec<Option<WireType>>,
    pub leaf_word_bit_width: i32,
}
impl MerkleTreePathGadget {
    pub fn new(
        direction_selector_wire: WireType,
        leaf_wires: Vec<Option<WireType>>,
        intermediate_hash_wires: Vec<Option<WireType>>,
        leaf_word_bit_width: i32,
        tree_height: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                direction_selector_wire,
                tree_height,
                leaf_wires,
                intermediate_hash_wires,
                leaf_word_bit_width,
                direction_selector_bits: vec![],
                out_root: vec![],
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<MerkleTreePathGadget> {
    const digest_width: i32 = SubsetSumHashGadget::DIMENSION;
    fn build_circuit(&mut self) {
        let digest_width = Self::digest_width as usize;
        let mut direction_selector_bits = self
            .t
            .direction_selector_wire
            .get_bit_wiresi(self.t.tree_height as u64)
            .as_array()
            .clone();

        // Apply CRH to leaf data
        let leaf_bits = WireArray::new(
            self.t.leaf_wires.clone(),
            self.generator.clone().downgrade(),
        )
        .get_bits(self.t.leaf_word_bit_width as usize, &None)
        .as_array()
        .clone();
        let mut subset_sum_gadget =
            SubsetSumHashGadget::new(leaf_bits.clone(), false, &None, self.generator.clone());
        let mut current_hash = subset_sum_gadget.get_output_wires();

        // Apply CRH across tree path guided by the direction bits
        for i in 0..self.t.tree_height as usize {
            let mut in_hash = vec![None; 2 * digest_width as usize];
            for j in 0..digest_width {
                let temp = current_hash[j].clone().unwrap().sub(
                    self.t.intermediate_hash_wires[i * digest_width + j]
                        .as_ref()
                        .unwrap(),
                );
                let temp2 = direction_selector_bits[i].clone().unwrap().mul(temp);
                in_hash[j] = Some(
                    self.t.intermediate_hash_wires[i * digest_width + j]
                        .clone()
                        .unwrap()
                        .add(temp2),
                );
            }
            for j in digest_width..2 * digest_width {
                let temp = current_hash[j - digest_width].clone().unwrap().add(
                    self.t.intermediate_hash_wires[i * digest_width + j - digest_width]
                        .as_ref()
                        .unwrap(),
                );
                in_hash[j] = Some(temp.sub(in_hash[j - digest_width].as_ref().unwrap()));
            }

            let next_input_bits = WireArray::new(in_hash, self.generator.clone().downgrade())
                .get_bits(CONFIGS.log2_field_prime as usize, &None)
                .as_array()
                .clone();
            subset_sum_gadget = SubsetSumHashGadget::new(
                next_input_bits.clone(),
                false,
                &None,
                self.generator.clone(),
            );
            current_hash = subset_sum_gadget.get_output_wires();
        }
        self.t.out_root = current_hash.clone();
    }
}
impl GadgetConfig for Gadget<MerkleTreePathGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.out_root
    }
}
