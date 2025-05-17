#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{Util,BigInteger};

 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct Op<T> {
    inputs: Vec<WireType>,
    outputs: Vec<WireType>,
    desc: String,
    t: T,
}
impl<T> Op<T> {
    fn new(inputs: Vec<WireType>, outputs: Vec<WireType>, desc: Vec<String>) -> eyre::Result<Self> {

       let desc= if desc.len() > 0 {
            desc[0].clone()
        } else {
           String::new()
        };

        for w in &inputs {
            if w.is_none() {
                println!("One of the input wires is null: {inputs:?}");
                eyre::bail!("A null wire");
            } else if w.getWireId() == -1 {
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
        Ok(Self {inputs,outputs,desc})
    }
}
pub trait BasicOp: Instruction {
    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        for w in self.inputs {
            if assignment[w.getWireId()].is_none() {
                println!("Error - The inWire {w } has not been assigned {self:?}\n");
                panic!("Error During Evaluation");
            }
        }
    }

    fn compute(&self, assignment: Vec<BigInteger>);

    fn checkOutputs(&self,assignment: Vec<BigInteger>) {
        for w in self.outputs {
            if assignment[w.getWireId()].is_some() {
                println!("Error - The outWire {w} has already been assigned {self:?}\n");
                panic!("Error During Evaluation");
            }
        }
    }

    fn getOpcode(&self) -> String;
    fn getNumMulGates(&self) -> i32;

    fn toString(&self) -> String {
        format!("{} in {} <{}> out  <{}> {} {}",getOpcode(),self.inputs.len(),Util::arrayToString(self.inputs, " "),self.outputs.len(),Util::arrayToString(self.outputs, " "),if self.desc.len() > 0  { " \t\t# ".to_owned() + &self.desc }else {String::new()} )
    }

    fn getInputs(&self) -> Vec<WireType> {
        self.inputs.clone()
    }

    fn getOutputs(&self) -> Vec<WireType> {
        self.outputs.clone()
    }

    fn doneWithinCircuit(&self) -> bool {
        true
    }

    fn hashCode(&self) -> i32 {
        // this method should be overriden when a subclass can have more than one opcode, or have other arguments
        let mut h = getOpcode().hashCode();
        for i in self.inputs {
            h += i.hashCode();
        }
        h
    }

    fn equals(&self, rhs: &Self) -> bool {
        self == rhs
        // logic moved to subclasses
    }
}
impl<T: BasicOp> Instruction for T {
    fn evaluate(&self, evaluator: CircuitEvaluator) {
        let assignment = evaluator.getAssignment();
        self.checkInputs(assignment.clone());
        self.checkOutputs(assignment.clone());
        self.compute(assignment.clone());
    }
}
