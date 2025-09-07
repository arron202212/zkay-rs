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
    examples::gadgets::hash::{
        merkle_tree_path_gadget::MerkleTreePathGadget, subset_sum_hash_gadget::SubsetSumHashGadget,
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use std::ops::{Add, Div, Mul, Rem, Sub};
use zkay_derive::ImplStructNameConfig;

crate::impl_struct_name_for!(CircuitGeneratorExtend<MerkleTreeMembershipCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct MerkleTreeMembershipCircuitGenerator {
    pub public_root_wires: Vec<Option<WireType>>,
    pub intermediate_hashe_wires: Vec<Option<WireType>>,
    pub direction_selector: Option<WireType>,
    pub leaf_wires: Vec<Option<WireType>>,
    pub tree_height: i32,
    pub merkle_tree_gadget: Option<Gadget<MerkleTreePathGadget>>,
}
impl MerkleTreeMembershipCircuitGenerator {
    const leaf_num_of_words: i32 = 10;
    const leaf_word_bit_width: i32 = 32;
    const hash_digest_dimension: i32 = SubsetSumHashGadget::DIMENSION;
    pub fn new(circuit_name: &str, tree_height: i32) -> CircuitGeneratorExtend<Self> {
        //super(circuitName);
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                public_root_wires: vec![],
                intermediate_hashe_wires: vec![],
                direction_selector: None,
                leaf_wires: vec![],
                tree_height,
                merkle_tree_gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<MerkleTreeMembershipCircuitGenerator> {
    fn build_circuit(&mut self) {
        let hash_digest_dimension = MerkleTreeMembershipCircuitGenerator::hash_digest_dimension;
        //  declare inputs
        let public_root_wires = CircuitGenerator::create_input_wire_array_with_str(
            self.cg(),
            hash_digest_dimension as usize,
            "Input Merkle Tree Root",
        );
        let intermediate_hashe_wires = CircuitGenerator::create_prover_witness_wire_array(
            self.cg(),
            (hash_digest_dimension * self.t.tree_height) as usize,
            &Some("Intermediate Hashes".to_owned()),
        );
        let direction_selector = CircuitGenerator::create_prover_witness_wire(
            self.cg(),
            &Some("Direction selector".to_owned()),
        );
        let leaf_wires = CircuitGenerator::create_prover_witness_wire_array(
            self.cg(),
            MerkleTreeMembershipCircuitGenerator::leaf_num_of_words as usize,
            &Some("Secret Leaf".to_owned()),
        );

        // connect gadget
        let merkle_tree_gadget = MerkleTreePathGadget::new(
            direction_selector.clone(),
            leaf_wires.clone(),
            intermediate_hashe_wires.clone(),
            MerkleTreeMembershipCircuitGenerator::leaf_word_bit_width,
            self.t.tree_height,
            &None,
            self.cg(),
        );
        let actual_root = merkle_tree_gadget.get_output_wires();

        //  Now compare the actual root with the pub  known root **/
        let mut error_accumulator = self.get_zero_wire().unwrap();
        for i in 0..hash_digest_dimension as usize {
            let diff = actual_root[i]
                .clone()
                .unwrap()
                .sub(public_root_wires[i].as_ref().unwrap());
            let check = diff.check_non_zero(&None);
            error_accumulator = error_accumulator.add(check);
        }

        CircuitGenerator::make_output_array(
            self.cg(),
            &actual_root,
            &Some("Computed Root".to_owned()),
        );

        //  Expected mismatch here if the sample input below is tried**/
        CircuitGenerator::make_output(
            self.cg(),
            &error_accumulator.check_non_zero(&None),
            &Some("Error if NON-zero".to_owned()),
        );
        (
            self.t.public_root_wires,
            self.t.intermediate_hashe_wires,
            self.t.direction_selector,
            self.t.leaf_wires,
            self.t.merkle_tree_gadget,
        ) = (
            public_root_wires,
            intermediate_hashe_wires,
            Some(direction_selector),
            leaf_wires,
            Some(merkle_tree_gadget),
        );
    }

    fn generate_sample_input(&self, circuit_evaluator: &mut CircuitEvaluator) {
        for i in 0..MerkleTreeMembershipCircuitGenerator::hash_digest_dimension as usize {
            circuit_evaluator.set_wire_value(
                self.t.public_root_wires[i].as_ref().unwrap(),
                &Util::next_random_big_integer(&CONFIGS.field_prime),
            );
        }

        circuit_evaluator.set_wire_value(
            self.t.direction_selector.as_ref().unwrap(),
            &Util::next_random_big_integeri(self.t.tree_height as u64),
        );
        for i in 0..(MerkleTreeMembershipCircuitGenerator::hash_digest_dimension
            * self.t.tree_height) as usize
        {
            circuit_evaluator.set_wire_value(
                self.t.intermediate_hashe_wires[i].as_ref().unwrap(),
                &Util::next_random_big_integer(&CONFIGS.field_prime),
            );
        }

        for i in 0..MerkleTreeMembershipCircuitGenerator::leaf_num_of_words as usize {
            circuit_evaluator
                .set_wire_valuei(self.t.leaf_wires[i].as_ref().unwrap(), i32::MAX as i64);
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
