#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::{
    config::config::CONFIGS,
    eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
    structure::wire_type::WireType,
};
use strum::{Display, EnumString};
use zkay_derive::ImplStructNameConfig;
#[derive(Clone, Debug, Hash, PartialEq, Display)]
pub enum LabelType {
    input,
    output,
    nizkinput,
    debug,
}

use std::fmt;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct WireLabelInstruction {
    pub label_type: LabelType,
    pub w: WireType,
    pub desc: String,
}
pub trait WireLabel {
    fn get_wire(&self) -> WireType;

    fn get_type(&self) -> LabelType;
}
impl WireLabel for WireLabelInstruction {
    fn get_wire(&self) -> WireType {
        self.w.clone()
    }

    fn get_type(&self) -> LabelType {
        self.label_type.clone()
    }
}
impl WireLabelInstruction {
    pub fn new(label_type: LabelType, w: WireType, desc: String) -> Self {
        Self {
            label_type,
            w,
            desc,
        }
    }

    pub fn get_wire(&self) -> WireType {
        self.w.clone()
    }

    pub fn get_type(&self) -> LabelType {
        self.label_type.clone()
    }
}

impl Instruction for WireLabelInstruction {
    fn evaluate(&self, evaluator: &mut CircuitEvaluator) -> eyre::Result<()> {
        // nothing to do.
        Ok(())
    }

    fn emit(&self, evaluator: &CircuitEvaluator) {
        if self.label_type == LabelType::output && CONFIGS.output_verbose
            || self.label_type == LabelType::debug && CONFIGS.debug_verbose
        {
            use std::sync::atomic::{self, AtomicBool, Ordering};
            println!(
                "\t[ {} ] Value of WireType# {} {} :: {}",
                self.label_type,
                self.w,
                if self.desc.is_empty() {
                    self.desc.clone()
                } else {
                    format!(" ({}) ", self.desc)
                },
                if crate::circuit::config::config::ATOMIC_HEX_OUTPUT_ENABLED.load(Ordering::Relaxed)
                {
                    format!("{:x}", evaluator.get_wire_value(&self.w))
                } else {
                    format!("{}", evaluator.get_wire_value(&self.w))
                }
            );
        }
    }

    fn done_within_circuit(&self) -> bool {
        self.label_type != LabelType::debug
    }

    fn wire_label(&self) -> Option<Box<dyn WireLabel>> {
        Some(Box::new(self.clone()))
    }
}

impl std::fmt::Display for WireLabelInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.label_type,
            self.w,
            &(if self.desc.is_empty() {
                self.desc.clone()
            } else {
                format!("\t\t\t # {}", self.desc)
            })
        )
    }
}
