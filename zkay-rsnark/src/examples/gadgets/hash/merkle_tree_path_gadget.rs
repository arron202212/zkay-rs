#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
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
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::config::config::Configs;
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;
// use crate::circuit::structure::wire_array;
use crate::examples::gadgets::hash::subset_sum_hash_gadget::SubsetSumHashGadget;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use zkay_derive::ImplStructNameConfig;

//  * A Merkle tree authentication gadget using the subsetsum hash function

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct MerkleTreePathGadget {
    pub treeHeight: i32,
    pub directionSelectorWire: WireType,
    pub directionSelectorBits: Vec<Option<WireType>>,
    pub leafWires: Vec<Option<WireType>>,
    pub intermediateHashWires: Vec<Option<WireType>>,
    pub outRoot: Vec<Option<WireType>>,

    pub leafWordBitWidth: i32,
}
impl MerkleTreePathGadget {
    pub fn new(
        directionSelectorWire: WireType,
        leafWires: Vec<Option<WireType>>,
        intermediateHashWires: Vec<Option<WireType>>,
        leafWordBitWidth: i32,
        treeHeight: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                directionSelectorWire,
                treeHeight,
                leafWires,
                intermediateHashWires,
                leafWordBitWidth,
                directionSelectorBits: vec![],
                outRoot: vec![],
            },
        );

        _self.buildCircuit();
        _self
    }
}
impl Gadget<MerkleTreePathGadget> {
    const digestWidth: i32 = SubsetSumHashGadget::DIMENSION;
    fn buildCircuit(&mut self) {
        let digestWidth = Self::digestWidth as usize;
        let mut directionSelectorBits = self
            .t
            .directionSelectorWire
            .getBitWiresi(self.t.treeHeight as u64, &None)
            .asArray()
            .clone();

        // Apply CRH to leaf data
        let leafBits = WireArray::new(self.t.leafWires.clone(), self.generator.clone().downgrade())
            .getBits(self.t.leafWordBitWidth as usize, &None)
            .asArray()
            .clone();
        let mut subsetSumGadget =
            SubsetSumHashGadget::new(leafBits.clone(), false, &None, self.generator.clone());
        let mut currentHash = subsetSumGadget.getOutputWires();

        // Apply CRH across tree path guided by the direction bits
        for i in 0..self.t.treeHeight as usize {
            let mut inHash = vec![None; 2 * digestWidth as usize];
            for j in 0..digestWidth {
                let temp = currentHash[j].clone().unwrap().sub(
                    self.t.intermediateHashWires[i * digestWidth + j]
                        .as_ref()
                        .unwrap(),
                );
                let temp2 = directionSelectorBits[i].clone().unwrap().mul(temp);
                inHash[j] = Some(
                    self.t.intermediateHashWires[i * digestWidth + j]
                        .clone()
                        .unwrap()
                        .add(temp2),
                );
            }
            for j in digestWidth..2 * digestWidth {
                let temp = currentHash[j - digestWidth].clone().unwrap().add(
                    self.t.intermediateHashWires[i * digestWidth + j - digestWidth]
                        .as_ref()
                        .unwrap(),
                );
                inHash[j] = Some(temp.sub(inHash[j - digestWidth].as_ref().unwrap()));
            }

            let nextInputBits = WireArray::new(inHash, self.generator.clone().downgrade())
                .getBits(Configs.log2_field_prime as usize, &None)
                .asArray()
                .clone();
            subsetSumGadget = SubsetSumHashGadget::new(
                nextInputBits.clone(),
                false,
                &None,
                self.generator.clone(),
            );
            currentHash = subsetSumGadget.getOutputWires();
        }
        self.t.outRoot = currentHash.clone();
    }
}
impl GadgetConfig for Gadget<MerkleTreePathGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.outRoot
    }
}
