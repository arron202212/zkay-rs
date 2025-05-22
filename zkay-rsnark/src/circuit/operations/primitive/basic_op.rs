#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Op<T> {
    pub inputs: Vec<Option<WireType>>,
    pub outputs: Vec<Option<WireType>>,
    pub desc: String,
    pub t: T,
}
impl<T> Op<T> {
    fn new(
        inputs: Vec<Option<WireType>>,
        outputs: Vec<Option<WireType>>,
        desc: Vec<String>,
        t: T,
    ) -> eyre::Result<Self> {
        let desc = if desc.len() > 0 {
            desc[0].clone()
        } else {
            String::new()
        };

        for w in &inputs {
            if w.is_none() {
                println!("One of the input wires is null: {inputs:?}");
                eyre::bail!("A null wire");
            } else if w.as_ref().unwrap().getWireId() == -1 {
                println!("One of the input wires is not packed: {inputs:?}");
                eyre::bail!("A wire with a negative id");
            }
        }
        for w in &outputs {
            if w.is_none() {
                println!("One of the output wires is null {outputs:?}");
                eyre::bail!("A null wire");
            }
        }
        Ok(Self {
            inputs,
            outputs,
            desc,
            t,
        })
    }
}
pub trait BasicOp: Instruction + Debug + PartialEq {
    fn checkInputs(&self, assignment: Vec<Option<BigInteger>>) {
        for w in self.getInputs() {
            if assignment[w.as_ref().unwrap().getWireId() as usize].is_none() {
                println!("Error - The inWire {w:? } has not been assigned {self:?}\n");
                panic!("Error During Evaluation");
            }
        }
    }

    fn compute(&self, assignment: Vec<Option<BigInteger>>);

    fn checkOutputs(&self, assignment: Vec<Option<BigInteger>>) {
        for w in self.getOutputs() {
            if assignment[w.as_ref().unwrap().getWireId() as usize].is_some() {
                println!("Error - The outWire {w:?} has already been assigned {self:?}\n");
                panic!("Error During Evaluation");
            }
        }
    }

    fn getOpcode(&self) -> String;
    fn getNumMulGates(&self) -> i32;

    fn toString(&self) -> String {
        format!(
            "{} in {} <{}> out  <{}> {} {}",
            self.getOpcode(),
            self.getInputs().len(),
            Util::arrayToString(self.getInputs(), " ".to_owned()),
            self.getOutputs().len(),
            Util::arrayToString(self.getOutputs(), " ".to_owned()),
            if self.desc().len() > 0 {
                " \t\t# ".to_owned() + &self.desc()
            } else {
                String::new()
            }
        )
    }

    fn getInputs(&self) -> Vec<Option<WireType>> {
        vec![]
    }

    // fn getOutputs(&self) -> Vec<Option<WireType>> {
    //     vec![]
    // }

    fn doneWithinCircuit(&self) -> bool {
        true
    }
    fn desc(&self) -> String {
        String::new()
    }
    fn hashCode(&self) -> u64 {
        // this method should be overriden when a subclass can have more than one opcode, or have other arguments
        let mut hasher = DefaultHasher::new();
        self.getOpcode().hash(&mut hasher);
        let mut h = hasher.finish();
        for i in self.getInputs() {
            h += i.as_ref().unwrap().hashCode();
        }
        h
    }

    fn equals(&self, rhs: &Self) -> bool {
        self == rhs
        // logic moved to subclasses
    }
}
#[macro_export]
macro_rules! impl_instruction_for {
    ($impl_type:ty) => {
        impl $crate::circuit::eval::instruction::Instruction for $impl_type {
            fn evaluate(
                &self,
                evaluator: $crate::circuit::eval::circuit_evaluator::CircuitEvaluator,
            ) {
                let assignment = evaluator.getAssignment();
                self.checkInputs(assignment.clone());
                self.checkOutputs(assignment.clone());
                self.compute(assignment.clone());
            }
        }
    };
}
