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
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum LabelType {
    input,
    output,
    nizkinput,
    debug,
}

impl std::fmt::Display for LabelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                _ => "",
            }
        )
    }
}
use std::fmt;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct WireLabelInstruction {
    pub label_type: LabelType,
    pub w: WireType,
    pub desc: String,
}
pub trait WireLabel {
    fn getWire(&self) -> WireType;

    fn getType(&self) -> LabelType;
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
                &("\t\t\t # ".to_owned() + &self.desc)
            })
        )
    }
    pub fn getType(&self) -> LabelType {
        self.label_type.clone()
    }
}

impl Instruction for WireLabelInstruction {
    fn evaluate(&self, evaluator: CircuitEvaluator) {
        // nothing to do.
    }

    fn emit(&self, evaluator: CircuitEvaluator) {
        if self.label_type == LabelType::output && Configs.get().unwrap().output_verbose
            || self.label_type == LabelType::debug && Configs.get().unwrap().debug_verbose
        {
            println!(
                "\t[ {} ] Value of WireType # {} {} :: {}",
                self.label_type,
                self.w,
                if self.desc.len() > 0 {
                    &(" (".to_owned() + &self.desc + ")")
                } else {
                    &self.desc
                },
                if Configs.get().unwrap().hex_output_enabled {
                    format!("{:x}", evaluator.getWireValue(self.w))
                } else {
                    format!("{}", evaluator.getWireValue(self.w))
                }
            );
        }
    }

    fn doneWithinCircuit(&self) -> bool {
        self.label_type != LabelType::debug
    }

    fn name(&self) -> &str {
        ""
    }
}
