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
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::instruction::Instruction,
        operations::{gadget::Gadget, wire_label_instruction, wire_label_instruction::LabelType},
        structure::{
            circuit_generator::CGConfigFields,
            circuit_generator::CGInstance,
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator,
            },
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{ARcCell, BigInteger, Util},
};
// use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;
//  use crate::util::util::BigInteger;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Neg, Sub};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct PinocchioGadget {
    pub input_wires: Vec<Option<WireType>>,
    pub prover_witness_wires: Vec<Option<WireType>>,
    pub output_wires: Vec<Option<WireType>>,
}
impl PinocchioGadget {
    pub fn new(
        input_wires: Vec<Option<WireType>>,
        path_to_arith_file: String,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                input_wires,
                prover_witness_wires: vec![],
                output_wires: vec![],
            },
        );
        _self.build_circuit(path_to_arith_file);
        _self
    }
}

impl Gadget<PinocchioGadget> {
    fn build_circuit(&mut self, path: String) {
        let mut prover_witness_wires = Vec::new();
        let mut output_wires = Vec::new();

        let mut scanner = BufReader::new(File::open(path).unwrap()).lines();

        assert!(
            scanner.next().unwrap().unwrap() == "total",
            "Expected total %d in the first line"
        );

        let num_wires = scanner.next().unwrap().unwrap().parse::<i32>().unwrap();
        scanner.next();
        let mut wire_mapping = vec![None; num_wires as usize];

        let mut input_count = 0;
        while let Some(Ok(mut line)) = scanner.next() {
            // let line = scanner.nextLine();
            // remove comments
            if let Some(i) = line.find('#') {
                line = line[..i].to_owned();
            }
            if line.is_empty() {
                continue;
            }
            if line.starts_with("input") {
                let tokens: Vec<_> = line
                    .split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect();
                let wire_index = tokens[1].parse::<i32>().unwrap() as usize;
                assert!(
                    wire_mapping[wire_index].is_none(),
                    "WireType assigned twice!{wire_index} "
                );

                if input_count < self.t.input_wires.len() {
                    wire_mapping[wire_index] = self.t.input_wires[input_count].clone();
                } else {
                    // the last input wire is assumed to be the one wire
                    wire_mapping[wire_index] = self.generator.get_one_wire();
                }
                input_count += 1;
            } else if line.starts_with("output") {
                let tokens: Vec<_> = line
                    .split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect();
                let wire_index = tokens[1].parse::<i32>().unwrap() as usize;
                output_wires.push(wire_mapping[wire_index].clone());
            } else if line.starts_with("nizk") {
                let tokens: Vec<_> = line
                    .split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect();
                let wire_index = tokens[1].parse::<i32>().unwrap() as usize;
                assert!(
                    wire_mapping[wire_index as usize].is_none(),
                    "WireType assigned twice!{wire_index} "
                );

                let w = Some(CircuitGenerator::create_prover_witness_wire(
                    self.generator.clone(),
                    &None,
                ));
                prover_witness_wires.push(w.clone());
                wire_mapping[wire_index] = w;
            } else {
                let ins = Self::get_inputs(&line);
                for i in &ins {
                    assert!(i.is_some(), "Undefined input wire  {i:? } at line{line}",);
                }
                let outs = Self::get_outputs(&line);
                if line.starts_with("mul ") {
                    wire_mapping[*outs[0].as_ref().unwrap()] = Some(
                        wire_mapping[*ins[0].as_ref().unwrap()]
                            .clone()
                            .unwrap()
                            .mul(wire_mapping[*ins[1].as_ref().unwrap()].as_ref().unwrap()),
                    );
                } else if line.starts_with("add ") {
                    let mut result = wire_mapping[*ins[0].as_ref().unwrap()]
                        .as_ref()
                        .unwrap()
                        .clone();
                    for i in 1..ins.len() {
                        result =
                            result.add(wire_mapping[*ins[i].as_ref().unwrap()].as_ref().unwrap());
                    }
                    wire_mapping[*outs[0].as_ref().unwrap()] = Some(result);
                } else if line.starts_with("zerop ") {
                    wire_mapping[*outs[1].as_ref().unwrap()] = Some(
                        wire_mapping[*ins[0].as_ref().unwrap()]
                            .as_ref()
                            .unwrap()
                            .check_non_zero(&None),
                    );
                } else if line.starts_with("split ") {
                    let bits = wire_mapping[*ins[0].as_ref().unwrap()]
                        .clone()
                        .unwrap()
                        .get_bit_wiresi(outs.len() as u64, &None)
                        .as_array()
                        .clone();
                    for i in 0..outs.len() {
                        wire_mapping[*outs[i].as_ref().unwrap()] = bits[i].clone();
                    }
                } else if line.starts_with("const-mul-neg-") {
                    let constant_str = &line["const-mul-neg-".len()..line.find(" ").unwrap()];
                    let constant = BigInteger::parse_bytes(constant_str.as_bytes(), 16).unwrap();
                    wire_mapping[*outs[0].as_ref().unwrap()] = Some(
                        wire_mapping[*ins[0].as_ref().unwrap()]
                            .as_ref()
                            .unwrap()
                            .mulb(&constant.neg(), &None),
                    );
                } else if line.starts_with("const-mul-") {
                    let constant_str = &line["const-mul-".len()..line.find(" ").unwrap()];
                    let constant = BigInteger::parse_bytes(constant_str.as_bytes(), 16).unwrap();
                    wire_mapping[*outs[0].as_ref().unwrap()] = Some(
                        wire_mapping[*ins[0].as_ref().unwrap()]
                            .as_ref()
                            .unwrap()
                            .mulb(&constant, &None),
                    );
                } else {
                    panic!("Unsupport Circuit Line {line} ");
                }
            }
        }

        self.t.prover_witness_wires = prover_witness_wires;
        self.t.output_wires = output_wires;
    }

    fn get_outputs(line: &String) -> Vec<Option<usize>> {
        line[line.rfind("<").unwrap() + 1..line.rfind(">").unwrap()]
            .split('\n')
            .filter_map(|v| (!v.is_empty()).then(|| v.parse::<usize>().ok()))
            .collect()
    }

    fn get_inputs(line: &String) -> Vec<Option<usize>> {
        line[line.rfind("<").unwrap() + 1..line.rfind(">").unwrap()]
            .split('\n')
            .map(|v| v.parse::<usize>().ok())
            .collect()
    }
    fn get_prover_witness_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.prover_witness_wires
    }
}
impl GadgetConfig for Gadget<PinocchioGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output_wires
    }
}
