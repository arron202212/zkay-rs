#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
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
                assert_basic_op::{AssertBasicOp, new_assert},
                basic_op::BasicOp,
                mul_basic_op::{MulBasicOp, new_mul},
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            },
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
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::structure::circuit_generator::{
//     CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
//     getActiveCircuitGenerator,
// };
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{BigInteger, Util};
use crate::examples::gadgets::augmenter::pinocchio_gadget::PinocchioGadget;
use crate::examples::gadgets::hash::sha256_gadget::{Base, SHA256Gadget};
use zkay_derive::ImplStructNameConfig;
/**
 * This circuit generator augments a second-price auction circuit (produced by Pinocchio's compiler)
 * with SHA-256 gadgets on each input and output value.
 *
 */
crate::impl_struct_name_for!(CircuitGeneratorExtend<AugmentedAuctionCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AugmentedAuctionCircuitGenerator {
    // each value is assumed to be a 64-bit value
    pub secretInputValues: Vec<Option<WireType>>,
    pub secretOutputValues: Vec<Option<WireType>>,

    // randomness vectors for each participant (each random vector is 7 64-bit words)
    pub secretInputRandomness: Vec<Vec<Option<WireType>>>,
    pub secretOutputRandomness: Vec<Vec<Option<WireType>>>,

    pub pathToCompiledCircuit: String,
    pub numParties: i32, // includes the auction manager + the participants
}
impl AugmentedAuctionCircuitGenerator {
    pub fn new(
        circuit_name: &str,
        pathToCompiledCircuit: String,
        numParticipants: i32,
    ) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::<Self>::new(
            circuit_name,
            Self {
                secretInputValues: vec![],
                secretOutputValues: vec![],
                secretInputRandomness: vec![],
                secretOutputRandomness: vec![],
                pathToCompiledCircuit,
                numParties: numParticipants + 1,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<AugmentedAuctionCircuitGenerator> {
    fn buildCircuit(&mut self) {
        let numParties = self.t.numParties as usize;
        let mut secretInputValues = self.createProverWitnessWireArray(numParties - 1, &None); // the manager has a zero input (no need to commit to it)
        let mut secretInputRandomness = vec![vec![]; numParties - 1];
        let mut secretOutputRandomness = vec![vec![]; numParties];
        for i in 0..numParties - 1 {
            secretInputRandomness[i] = self.createProverWitnessWireArray(7, &None);
            secretOutputRandomness[i] = self.createProverWitnessWireArray(7, &None);
        }
        secretOutputRandomness[numParties - 1] = self.createProverWitnessWireArray(7, &None);
        let mut secretInputValuess = secretInputValues.clone();
        secretInputValuess.insert(0, self.get_zero_wire());
        // instantiate a Pinocchio gadget for the auction circuit
        let auctionGagdet = PinocchioGadget::new(
            secretInputValuess,
            self.t.pathToCompiledCircuit.clone(),
            &None,
            self.cg(),
        );
        let outputs = auctionGagdet.getOutputWires();

        // ignore the last output for this circuit which carries the index of the winner (not needed for this example)
        let mut secretOutputValues = outputs[..outputs.len() - 1].to_vec();

        // augment the input side
        for i in 0..numParties - 1 {
            let mut secretInputRandomnessi = secretInputRandomness[i].clone();
            secretInputRandomnessi.insert(0, secretInputValues[i].clone());
            let g = SHA256Gadget::new(
                secretInputRandomnessi,
                64,
                64,
                false,
                false,
                &None,
                self.cg(),
                Base,
            );
            self.makeOutputArray(
                g.getOutputWires(),
                &Some(format!("Commitment for party # {i}'s input balance.")),
            );
        }

        // augment the output side
        for i in 0..numParties {
            // adapt the output values to 64-bit values (adaptation is needed due to the way Pinocchio's compiler handles subtractions)
            secretOutputValues[i] = Some(
                secretOutputValues[i]
                    .as_ref()
                    .unwrap()
                    .getBitWiresi(64 * 2, &None)
                    .packAsBits(None, Some(64), &None),
            );
            let mut secretOutputRandomnessi = secretOutputRandomness[i].clone();
            secretOutputRandomnessi.insert(0, secretOutputValues[i].clone());
            let g = SHA256Gadget::new(
                secretOutputRandomnessi,
                64,
                64,
                false,
                false,
                &None,
                self.cg(),
                Base,
            );
            self.makeOutputArray(
                g.getOutputWires(),
                &Some(format!("Commitment for party # {i}'s output balance.")),
            );
        }
        (
            self.t.secretInputValues,
            self.t.secretOutputValues,
            self.t.secretInputRandomness,
            self.t.secretOutputRandomness,
        ) = (
            secretInputValues,
            secretOutputValues,
            secretInputRandomness,
            secretOutputRandomness,
        );
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        let numParties = self.t.numParties as usize;
        for i in 0..numParties - 1 {
            evaluator.setWireValue(
                self.t.secretInputValues[i].as_ref().unwrap(),
                &Util::nextRandomBigIntegeri(63),
            );
        }

        for i in 0..numParties - 1 {
            for w in &self.t.secretInputRandomness[i] {
                evaluator.setWireValue(w.as_ref().unwrap(), &Util::nextRandomBigIntegeri(64));
            }
        }
        for i in 0..numParties {
            for w in &self.t.secretOutputRandomness[i] {
                evaluator.setWireValue(w.as_ref().unwrap(), &Util::nextRandomBigIntegeri(64));
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
    generator.generateCircuit();
    let mut evaluator = generator.evalCircuit().ok();
    generator.prepFiles(evaluator);
    generator.runLibsnark();
}
