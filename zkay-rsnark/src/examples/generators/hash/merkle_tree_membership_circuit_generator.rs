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
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            },
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
// use crate::circuit::config::config::Configs;
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::structure::circuit_generator::{
//     CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
//     get_active_circuit_generator,
// };
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{BigInteger, Util};
use crate::examples::gadgets::hash::merkle_tree_path_gadget::MerkleTreePathGadget;
use crate::examples::gadgets::hash::subset_sum_hash_gadget::SubsetSumHashGadget;
use std::ops::{Add, Div, Mul, Rem, Sub};
use zkay_derive::ImplStructNameConfig;

crate::impl_struct_name_for!(CircuitGeneratorExtend<MerkleTreeMembershipCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct MerkleTreeMembershipCircuitGenerator {
    pub publicRootWires: Vec<Option<WireType>>,
    pub intermediateHasheWires: Vec<Option<WireType>>,
    pub directionSelector: Option<WireType>,
    pub leafWires: Vec<Option<WireType>>,

    pub treeHeight: i32,

    pub merkleTreeGadget: Option<Gadget<MerkleTreePathGadget>>,
}
impl MerkleTreeMembershipCircuitGenerator {
    const leafNumOfWords: i32 = 10;
    const leafWordBitWidth: i32 = 32;
    const hashDigestDimension: i32 = SubsetSumHashGadget::DIMENSION;
    pub fn new(circuit_name: &str, treeHeight: i32) -> CircuitGeneratorExtend<Self> {
        //super(circuitName);
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                publicRootWires: vec![],
                intermediateHasheWires: vec![],
                directionSelector: None,
                leafWires: vec![],
                treeHeight,
                merkleTreeGadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<MerkleTreeMembershipCircuitGenerator> {
    fn build_circuit(&mut self) {
        let hashDigestDimension = MerkleTreeMembershipCircuitGenerator::hashDigestDimension;
        //  declare inputs
        let publicRootWires = CircuitGenerator::create_input_wire_array(
            self.cg(),
            hashDigestDimension as usize,
            &Some("Input Merkle Tree Root".to_owned()),
        );
        let intermediateHasheWires = CircuitGenerator::create_prover_witness_wire_array(
            self.cg(),
            (hashDigestDimension * self.t.treeHeight) as usize,
            &Some("Intermediate Hashes".to_owned()),
        );
        let directionSelector = CircuitGenerator::create_prover_witness_wire(
            self.cg(),
            &Some("Direction selector".to_owned()),
        );
        let leafWires = CircuitGenerator::create_prover_witness_wire_array(
            self.cg(),
            MerkleTreeMembershipCircuitGenerator::leafNumOfWords as usize,
            &Some("Secret Leaf".to_owned()),
        );

        // connect gadget
        let merkleTreeGadget = MerkleTreePathGadget::new(
            directionSelector.clone(),
            leafWires.clone(),
            intermediateHasheWires.clone(),
            MerkleTreeMembershipCircuitGenerator::leafWordBitWidth,
            self.t.treeHeight,
            &None,
            self.cg(),
        );
        let actualRoot = merkleTreeGadget.get_output_wires();

        //  Now compare the actual root with the pub  known root **/
        let mut errorAccumulator = self.get_zero_wire().unwrap();
        for i in 0..hashDigestDimension as usize {
            let diff = actualRoot[i]
                .clone()
                .unwrap()
                .sub(publicRootWires[i].as_ref().unwrap());
            let check = diff.check_non_zero(&None);
            errorAccumulator = errorAccumulator.add(check);
        }

        CircuitGenerator::make_output_array(
            self.cg(),
            &actualRoot,
            &Some("Computed Root".to_owned()),
        );

        //  Expected mismatch here if the sample input below is tried**/
        CircuitGenerator::make_output(
            self.cg(),
            &errorAccumulator.check_non_zero(&None),
            &Some("Error if NON-zero".to_owned()),
        );
        (
            self.t.publicRootWires,
            self.t.intermediateHasheWires,
            self.t.directionSelector,
            self.t.leafWires,
            self.t.merkleTreeGadget,
        ) = (
            publicRootWires,
            intermediateHasheWires,
            Some(directionSelector),
            leafWires,
            Some(merkleTreeGadget),
        );
    }

    fn generate_sample_input(&self, circuit_evaluator: &mut CircuitEvaluator) {
        for i in 0..MerkleTreeMembershipCircuitGenerator::hashDigestDimension as usize {
            circuit_evaluator.set_wire_value(
                self.t.publicRootWires[i].as_ref().unwrap(),
                &Util::nextRandomBigInteger(&Configs.field_prime),
            );
        }

        circuit_evaluator.set_wire_value(
            self.t.directionSelector.as_ref().unwrap(),
            &Util::nextRandomBigIntegeri(self.t.treeHeight as u64),
        );
        for i in 0..(MerkleTreeMembershipCircuitGenerator::hashDigestDimension * self.t.treeHeight)
            as usize
        {
            circuit_evaluator.set_wire_value(
                self.t.intermediateHasheWires[i].as_ref().unwrap(),
                &Util::nextRandomBigInteger(&Configs.field_prime),
            );
        }

        for i in 0..MerkleTreeMembershipCircuitGenerator::leafNumOfWords as usize {
            circuit_evaluator
                .set_wire_valuei(self.t.leafWires[i].as_ref().unwrap(), i32::MAX as i64);
        }
    }
}
pub fn main(args: Vec<String>) {
    let mut generator = MerkleTreeMembershipCircuitGenerator::new("tree_64", 64);
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
