#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::{
    config::config::Configs,
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

// impl std::fmt::Display for LabelType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 _ => "",
//             }
//         )
//     }
// }
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
    fn getWire(&self) -> WireType;

    fn getType(&self) -> LabelType;
}
impl WireLabel for WireLabelInstruction {
    fn getWire(&self) -> WireType {
        self.w.clone()
    }

    fn getType(&self) -> LabelType {
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

    pub fn getWire(&self) -> WireType {
        self.w.clone()
    }

    // pub fn toString(&self) -> String {
    //     format!(
    //         "{} {}{}",
    //         self.label_type,
    //         self.w,
    //         &(if self.desc.is_empty() {
    //             self.desc.clone()
    //         } else {
    //             format!("\t\t\t # {}", self.desc)
    //         })
    //     )
    // }
    pub fn getType(&self) -> LabelType {
        self.label_type.clone()
    }
}

impl Instruction for WireLabelInstruction {
    fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
        // nothing to do.
    }

    fn emit(&self, evaluator: &CircuitEvaluator) {
        if self.label_type == LabelType::output && Configs.output_verbose
            || self.label_type == LabelType::debug && Configs.debug_verbose
        {
            println!(
                "\t[ {} ] Value of WireType# {} {} :: {}",
                self.label_type,
                self.w,
                if self.desc.is_empty() {
                    self.desc.clone()
                } else {
                    format!(" ({}) ", self.desc)
                },
                if Configs.hex_output_enabled {
                    format!("{:x}", evaluator.getWireValue(self.w.clone()))
                } else {
                    format!("{}", evaluator.getWireValue(self.w.clone()))
                }
            );
        }
    }

    fn doneWithinCircuit(&self) -> bool {
        self.label_type != LabelType::debug
    }

    // fn name(&self) -> &str {
    //      ""
    //  }
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
