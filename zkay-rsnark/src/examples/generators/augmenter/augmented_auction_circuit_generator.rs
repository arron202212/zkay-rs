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
    examples::gadgets::{
        augmenter::pinocchio_gadget::PinocchioGadget,
        hash::sha256_gadget::{Base, SHA256Gadget},
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use zkay_derive::ImplStructNameConfig;

//  * This circuit generator augments a second-price auction circuit (produced by Pinocchio's compiler)
//  * with SHA-256 gadgets on each input and output value.

crate::impl_struct_name_for!(CircuitGeneratorExtend<AugmentedAuctionCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AugmentedAuctionCircuitGenerator {
    // each value is assumed to be a 64-bit value
    pub secret_input_values: Vec<Option<WireType>>,
    pub secret_output_values: Vec<Option<WireType>>,

    // randomness vectors for each participant (each random vector is 7 64-bit words)
    pub secret_input_randomness: Vec<Vec<Option<WireType>>>,
    pub secret_output_randomness: Vec<Vec<Option<WireType>>>,

    pub path_to_compiled_circuit: String,
    pub num_parties: i32, // includes the auction manager + the participants
}
impl AugmentedAuctionCircuitGenerator {
    pub fn new(
        circuit_name: &str,
        path_to_compiled_circuit: String,
        num_participants: i32,
    ) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::<Self>::new(
            circuit_name,
            Self {
                secret_input_values: vec![],
                secret_output_values: vec![],
                secret_input_randomness: vec![],
                secret_output_randomness: vec![],
                path_to_compiled_circuit,
                num_parties: num_participants + 1,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<AugmentedAuctionCircuitGenerator> {
    fn build_circuit(&mut self) {
        let num_parties = self.t.num_parties as usize;
        let mut secret_input_values =
            CircuitGenerator::create_prover_witness_wire_array(self.cg(), num_parties - 1, &None); // the manager has a zero input (no need to commit to it)
        let mut secret_input_randomness = vec![vec![]; num_parties - 1];
        let mut secret_output_randomness = vec![vec![]; num_parties];
        for i in 0..num_parties - 1 {
            secret_input_randomness[i] =
                CircuitGenerator::create_prover_witness_wire_array(self.cg(), 7, &None);
            secret_output_randomness[i] =
                CircuitGenerator::create_prover_witness_wire_array(self.cg(), 7, &None);
        }
        secret_output_randomness[num_parties - 1] =
            CircuitGenerator::create_prover_witness_wire_array(self.cg(), 7, &None);
        let mut secret_input_valuess = secret_input_values.clone();
        secret_input_valuess.insert(0, self.get_zero_wire());
        // instantiate a Pinocchio gadget for the auction circuit
        let auction_gagdet = PinocchioGadget::new(
            secret_input_valuess,
            self.t.path_to_compiled_circuit.clone(),
            &None,
            self.cg(),
        );
        let outputs = auction_gagdet.get_output_wires();

        // ignore the last output for this circuit which carries the index of the winner (not needed for this example)
        let mut secret_output_values = outputs[..outputs.len() - 1].to_vec();

        // augment the input side
        for i in 0..num_parties - 1 {
            let mut secret_input_randomnessi = secret_input_randomness[i].clone();
            secret_input_randomnessi.insert(0, secret_input_values[i].clone());
            let g = SHA256Gadget::new(
                secret_input_randomnessi,
                64,
                64,
                false,
                false,
                &None,
                self.cg(),
                Base,
            );
            CircuitGenerator::make_output_array(
                self.cg(),
                g.get_output_wires(),
                &Some(format!("Commitment for party # {i}'s input balance.")),
            );
        }

        // augment the output side
        for i in 0..num_parties {
            // adapt the output values to 64-bit values (adaptation is needed due to the way Pinocchio's compiler handles subtractions)
            secret_output_values[i] = Some(
                secret_output_values[i]
                    .as_ref()
                    .unwrap()
                    .get_bit_wiresi(64 * 2, &None)
                    .pack_as_bits(None, Some(64), &None),
            );
            let mut secret_output_randomnessi = secret_output_randomness[i].clone();
            secret_output_randomnessi.insert(0, secret_output_values[i].clone());
            let g = SHA256Gadget::new(
                secret_output_randomnessi,
                64,
                64,
                false,
                false,
                &None,
                self.cg(),
                Base,
            );
            CircuitGenerator::make_output_array(
                self.cg(),
                g.get_output_wires(),
                &Some(format!("Commitment for party # {i}'s output balance.")),
            );
        }
        (
            self.t.secret_input_values,
            self.t.secret_output_values,
            self.t.secret_input_randomness,
            self.t.secret_output_randomness,
        ) = (
            secret_input_values,
            secret_output_values,
            secret_input_randomness,
            secret_output_randomness,
        );
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        let num_parties = self.t.num_parties as usize;
        for i in 0..num_parties - 1 {
            evaluator.set_wire_value(
                self.t.secret_input_values[i].as_ref().unwrap(),
                &Util::next_random_big_integeri(63),
            );
        }

        for i in 0..num_parties - 1 {
            for w in &self.t.secret_input_randomness[i] {
                evaluator.set_wire_value(w.as_ref().unwrap(), &Util::next_random_big_integeri(64));
            }
        }
        for i in 0..num_parties {
            for w in &self.t.secret_output_randomness[i] {
                evaluator.set_wire_value(w.as_ref().unwrap(), &Util::next_random_big_integeri(64));
            }
        }
    }
}

pub fn main(args: Vec<String>) {
    let mut generator = AugmentedAuctionCircuitGenerator::new(
        "augmented_auction_10",
        "auction_10.arith".to_owned(),
        10,
    );
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
