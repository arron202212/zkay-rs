#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        eval::circuit_evaluator::CircuitEvaluator,
        eval::instruction::Instruction,
        structure::{
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_type::WireType,
        },
        {InstanceOf, OpCodeConfig, StructNameConfig},
    },
    util::util::{BigInteger, Util},
};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone)]
pub struct Op<T> {
    pub inputs: Vec<Option<WireType>>,
    pub outputs: Vec<Option<WireType>>,
    pub desc: String,
    pub t: T,
}
impl<T: StructNameConfig> crate::circuit::StructNameConfig for Op<T> {
    fn name(&self) -> String {
        self.t.name()
    }
}
impl<T: StructNameConfig> crate::circuit::InstanceOf for Op<T> {}
impl<T: OpCodeConfig> crate::circuit::OpCodeConfig for Op<T> {
    fn op_code(&self) -> String {
        self.t.op_code()
    }
}

// impl<T:OpCodeConfig> Hash for Op<T> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.t.op_code().hash(state);
//         for i in &self.inputs {
//             i.as_ref().unwrap().hash(state);
//         }
//     }
// }

impl<T> Op<T> {
    fn new(
        inputs: Vec<Option<WireType>>,
        outputs: Vec<Option<WireType>>,
        desc: &Option<String>,
        t: T,
    ) -> eyre::Result<Self> {
        let desc = desc
            .as_ref()
            .map_or_else(|| String::new(), |d| d.to_owned());

        for w in &inputs {
            if w.is_none() {
                //println!("One of the input wires is None: {inputs:?}");
                eyre::bail!("A None wire");
            } else if w.as_ref().unwrap().getWireId() == -1 {
                //println!("One of the input wires is not packed: {inputs:?}");
                eyre::bail!("A wire with a negative id");
            }
        }
        for w in &outputs {
            if w.is_none() {
                //println!("One of the output wires is None {outputs:?}");
                eyre::bail!("A None wire");
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
pub trait BasicOpInOut {
    fn getInputs(&self) -> Vec<Option<WireType>> {
        vec![]
    }

    fn getOutputs(&self) -> Vec<Option<WireType>> {
        vec![]
    }
}
impl<T> BasicOpInOut for Op<T> {
    fn getInputs(&self) -> Vec<Option<WireType>> {
        self.inputs.clone()
    }

    fn getOutputs(&self) -> Vec<Option<WireType>> {
        self.outputs.clone()
    }
}
pub trait BasicOp: Instruction + BasicOpInOut + Debug + crate::circuit::OpCodeConfig {
    fn checkInputs(&self, assignment: &Vec<Option<BigInteger>>) {
        self.super_checkInputs(assignment);
    }
    fn super_checkInputs(&self, assignment: &Vec<Option<BigInteger>>) {
        let inputs = self.getInputs();
        let n = inputs.len();
        for (_i, w) in inputs.iter().enumerate() {
            // println!(
            //     "===w.as_ref().unwrap().getWireId()==={i}===={}==",
            //     w.as_ref().unwrap().getWireId()
            // );
            // if assignment[w.as_ref().unwrap().getWireId() as usize].is_none() {
            //println!("Error - The inWire {w:? } has not been assigned {self:?}\n");
            assert!(
                assignment[w.as_ref().unwrap().getWireId() as usize].is_some(),
                "Error During Evaluation in checkInputs wire id={},{:?},{}",
                w.as_ref().unwrap().getWireId(),
                assignment.len(),
                n
            );
            // }
        }
        assert!(
            self.getInputs()
                .iter()
                .all(|w| assignment[w.as_ref().unwrap().getWireId() as usize].is_some()),
            "Error During Evaluation in checkInputs"
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>);

    fn checkOutputs(&self, assignment: &Vec<Option<BigInteger>>) {
        for w in self.getOutputs() {
            // if assignment[w.as_ref().unwrap().getWireId() as usize].is_some() {
            //println!("Error - The outWire {w:?} has already been assigned {self:?}\n");
            assert!(
                assignment[w.as_ref().unwrap().getWireId() as usize].is_none(),
                "Error During Evaluation in checkOutputswire id={}",
                w.as_ref().unwrap().getWireId()
            );
            // }
        }
        assert!(
            self.getOutputs()
                .iter()
                .all(|w| assignment[w.as_ref().unwrap().getWireId() as usize].is_none()),
            "Error During Evaluation in checkOutputs"
        );
    }

    fn getOpcode(&self) -> String {
        self.op_code()
    }
    fn getNumMulGates(&self) -> i32;

    // fn toString(&self) -> String {
    //     format!(
    //         "{} in {} <{}> out  <{}> {} {}",
    //         self.getOpcode(),
    //         self.getInputs().len(),
    //         Util::arrayToString(self.getInputs(), " ".to_owned()),
    //         self.getOutputs().len(),
    //         Util::arrayToString(self.getOutputs(), " ".to_owned()),
    //         if self.desc().len() > 0 {
    //             " \t\t# ".to_owned() + &self.desc()
    //         } else {
    //             String::new()
    //         }
    //     )
    // }

    fn doneWithinCircuit(&self) -> bool {
        true
    }
    fn desc(&self) -> String {
        String::new()
    }
    // fn hashCode(&self) -> u64 {
    //     // this method should be overriden when a subclass can have more than one opcode, or have other arguments
    //     let mut hasher = DefaultHasher::new();
    //     self.getOpcode().hash(&mut hasher);
    //     let mut h = hasher.finish();
    //     for i in self.getInputs() {
    //         h += i.as_ref().unwrap().hashCode();
    //     }
    //     h
    // }

    // fn equals(&self, rhs: &Self) -> bool {
    //     // self == rhs
    //     // logic moved to subclasses
    //     false
    // }
}

#[macro_export]
macro_rules! impl_instruction_for {
    ($impl_type:ty) => {
        impl $crate::circuit::eval::instruction::Instruction for $impl_type {
            fn evaluate(
                &self,
                evaluator: &mut $crate::circuit::eval::circuit_evaluator::CircuitEvaluator,
            ) {
                let assignment = evaluator.getAssignment();
                self.checkInputs(assignment);
                self.checkOutputs(assignment);
                self.compute(evaluator.get_assignment_mut());
            }
            fn basic_op(&self) -> Option<Box<dyn BasicOp>> {
                Some(Box::new(self.clone()))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_hash_code_for {
    ($impl_type:ty) => {
        impl std::hash::Hash for $impl_type {
            fn hash<H: Hasher>(&self, state: &mut H) {
                // this method should be overriden when a subclass can have more than one opcode, or have other arguments
                self.getOpcode().hash(state);
                let mut inputs = self.getInputs();
                if self.getOpcode() != "pack".to_owned() {
                    inputs.sort_unstable_by_key(|x| x.as_ref().unwrap().getWireId());
                }
                for i in inputs {
                    i.as_ref().unwrap().getWireId().hash(state);
                }
                if self.getOpcode() == "assert".to_owned() && !self.outputs.is_empty() {
                    self.outputs[0].as_ref().unwrap().getWireId().hash(state);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display_of_op_for {
    ($impl_type:ty) => {
        impl std::fmt::Display for $impl_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
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
        }
    };
}
