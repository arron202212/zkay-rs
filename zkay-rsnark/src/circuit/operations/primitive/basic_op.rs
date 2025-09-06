#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        eval::instruction::Instruction,
        structure::{wire::GetWireId, wire_type::WireType},
        {OpCodeConfig, StructNameConfig},
    },
    util::util::BigInteger,
};
use std::fmt::Debug;

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
        let desc = desc.clone().unwrap_or(String::new());

        for w in &inputs {
            if w.is_none() {
                //println!("One of the input wires is None: {inputs:?}");
                eyre::bail!("A None wire");
            } else if w.as_ref().unwrap().get_wire_id() == -1 {
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
    fn get_inputs(&self) -> Vec<Option<WireType>> {
        vec![]
    }

    fn get_outputs(&self) -> Vec<Option<WireType>> {
        vec![]
    }
}
impl<T> BasicOpInOut for Op<T> {
    fn get_inputs(&self) -> Vec<Option<WireType>> {
        self.inputs.clone()
    }

    fn get_outputs(&self) -> Vec<Option<WireType>> {
        self.outputs.clone()
    }
}
pub trait BasicOp: Instruction + BasicOpInOut + Debug + crate::circuit::OpCodeConfig {
    fn check_inputs(&self, assignment: &Vec<Option<BigInteger>>) {
        self.super_check_inputs(assignment);
    }
    fn super_check_inputs(&self, assignment: &Vec<Option<BigInteger>>) {
        let inputs = self.get_inputs();
        let n = inputs.len();
        for (_i, w) in inputs.iter().enumerate() {
            // println!(
            //     "===w.as_ref().unwrap().get_wire_id()==={i}===={}==",
            //     w.as_ref().unwrap().get_wire_id()
            // );
            // if assignment[w.as_ref().unwrap().get_wire_id() as usize].is_none() {
            //println!("Error - The inWire {w:? } has not been assigned {self:?}\n");
            assert!(
                assignment[w.as_ref().unwrap().get_wire_id() as usize].is_some(),
                "Error During Evaluation in check_inputs wire id={},{},{}",
                w.as_ref().unwrap().get_wire_id(),
                assignment.len(),
                n
            );
            // }
        }
        assert!(
            self.get_inputs()
                .iter()
                .all(|w| assignment[w.as_ref().unwrap().get_wire_id() as usize].is_some()),
            "Error During Evaluation in check_inputs"
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()>;

    fn check_outputs(&self, assignment: &Vec<Option<BigInteger>>) {
        for w in self.get_outputs() {
            // if assignment[w.as_ref().unwrap().get_wire_id() as usize].is_some() {
            //println!("Error - The outWire {w:?} has already been assigned {self:?}\n");
            assert!(
                assignment[w.as_ref().unwrap().get_wire_id() as usize].is_none(),
                "Error During Evaluation in checkOutputswire id={}",
                w.as_ref().unwrap().get_wire_id()
            );
            // }
        }
        assert!(
            self.get_outputs()
                .iter()
                .all(|w| assignment[w.as_ref().unwrap().get_wire_id() as usize].is_none()),
            "Error During Evaluation in check_outputs"
        );
    }

    fn get_op_code(&self) -> String {
        self.op_code()
    }
    fn get_num_mul_gates(&self) -> i32;

    // fn toString(&self) -> String {
    //     format!(
    //         "{} in {} <{}> out  <{}> {} {}",
    //         self.get_op_code(),
    //         self.get_inputs().len(),
    //         Util::array_to_string(self.get_inputs(), " ".to_owned()),
    //         self.get_outputs().len(),
    //         Util::array_to_string(self.get_outputs(), " ".to_owned()),
    //         if self.desc().len() > 0 {
    //             " \t\t# ".to_owned() + &self.desc()
    //         } else {
    //             String::new()
    //         }
    //     )
    // }

    fn done_within_circuit(&self) -> bool {
        true
    }
    fn desc(&self) -> String {
        String::new()
    }
    // fn hashCode(&self) -> u64 {
    //     // this method should be overriden when a subclass can have more than one opcode, or have other arguments
    //     let mut hasher = DefaultHasher::new();
    //     self.get_op_code().hash(&mut hasher);
    //     let mut h = hasher.finish();
    //     for i in self.get_inputs() {
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
            ) -> eyre::Result<()> {
                let assignment = evaluator.get_assignment();
                self.check_inputs(assignment);
                self.check_outputs(assignment);
                self.compute(evaluator.get_assignment_mut())?;
                Ok(())
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
                self.get_op_code().hash(state);
                let mut inputs = self.get_inputs();
                if self.get_op_code() != "pack".to_owned() {
                    inputs.sort_unstable_by_key(|x| x.as_ref().unwrap().get_wire_id());
                }
                for i in inputs {
                    i.as_ref().unwrap().get_wire_id().hash(state);
                }
                if self.get_op_code() == "assert".to_owned() && !self.outputs.is_empty() {
                    self.outputs[0].as_ref().unwrap().get_wire_id().hash(state);
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
                    self.get_op_code(),
                    self.get_inputs().len(),
                    Util::array_to_string(self.get_inputs(), " ".to_owned()),
                    self.get_outputs().len(),
                    Util::array_to_string(self.get_outputs(), " ".to_owned()),
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
