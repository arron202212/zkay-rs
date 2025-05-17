#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;

use crate::circuit::eval::instruction::Instruction;
use crate::circuit::structure::wire_type::WireType;
pub enum LabelType {
    input,
    output,
    nizkinput,
    debug,
}
 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct WireLabelInstruction {
    label_type: LabelType,
    w: WireType,
    desc: String,
}
impl WireLabelInstruction {
    pub fn new(label_type: LabelType, w: WireType, desc: Vec<String>) -> Self {
        Self {
            label_type,
            w,
            desc: desc.get(0).unwrap_or(&String::new()).clone(),
        }
    }

    pub fn getWire(&self) -> WireType {
        self.w.clone()
    }

    pub fn toString(&self) -> String {
        format!(
            "{} {}{}",
            self.label_type,
            self.w,
            (if self.desc.len() == 0 {
               &self.desc
            } else {
               &( "\t\t\t # ".to_owned() +&self.desc)
            })
        )
    }
    pub fn getType(&self) -> LabelType {
        self.label_type.clone()
    }
}

impl Instruction for WireLabelInstruction {
    fn evaluate(&self,evaluator: CircuitEvaluator) {
        // nothing to do.
    }

    fn emit(&self,evaluator: CircuitEvaluator) {
        if self.label_type == LabelType::output && Configs.get().unwrap().outputVerbose
            || self.label_type == LabelType::debug && Configs.get().unwrap().debugVerbose
        {
            println!("\t[ {} ] Value of WireType # {} {} :: {:bin$}",self.label_type,self.w,  if self.desc.len() > 0  { &(" (".to_owned() + &self.desc + ")") }else { &self.desc}
					, evaluator.getWireValue(self.w),bin= if Configs.get().unwrap().hexOutputEnabled  { "x" }else { ""});
        }
    }

    fn doneWithinCircuit(&self) -> bool {
        self.labe_type != LabelType::debug
    }

    fn name(&self)->&str{
        ""
    }
}
