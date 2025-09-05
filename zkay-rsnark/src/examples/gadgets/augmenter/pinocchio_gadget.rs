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
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
                getActiveCircuitGenerator,
            },
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
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
    pub inputWires: Vec<Option<WireType>>,
    pub proverWitnessWires: Vec<Option<WireType>>,
    pub outputWires: Vec<Option<WireType>>,
}
impl PinocchioGadget {
    pub fn new(
        inputWires: Vec<Option<WireType>>,
        pathToArithFile: String,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                inputWires,
                proverWitnessWires: vec![],
                outputWires: vec![],
            },
        );
        _self.buildCircuit(pathToArithFile);
        _self
    }
}

impl Gadget<PinocchioGadget> {
    fn buildCircuit(&mut self, path: String) {
        let mut proverWitnessWires = Vec::new();
        let mut outputWires = Vec::new();

        let mut scanner = BufReader::new(File::open(path).unwrap()).lines();

        assert!(
            scanner.next().unwrap().unwrap() == "total",
            "Expected total %d in the first line"
        );

        let numWires = scanner.next().unwrap().unwrap().parse::<i32>().unwrap();
        scanner.next();
        let mut wireMapping = vec![None; numWires as usize];

        let mut inputCount = 0;
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
                let wireIndex = tokens[1].parse::<i32>().unwrap() as usize;
                assert!(
                    wireMapping[wireIndex].is_none(),
                    "WireType assigned twice!{wireIndex} "
                );

                if inputCount < self.t.inputWires.len() {
                    wireMapping[wireIndex] = self.t.inputWires[inputCount].clone();
                } else {
                    // the last input wire is assumed to be the one wire
                    wireMapping[wireIndex] = self.generator.get_one_wire();
                }
                inputCount += 1;
            } else if line.starts_with("output") {
                let tokens: Vec<_> = line
                    .split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect();
                let wireIndex = tokens[1].parse::<i32>().unwrap() as usize;
                outputWires.push(wireMapping[wireIndex].clone());
            } else if line.starts_with("nizk") {
                let tokens: Vec<_> = line
                    .split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect();
                let wireIndex = tokens[1].parse::<i32>().unwrap() as usize;
                assert!(
                    wireMapping[wireIndex as usize].is_none(),
                    "WireType assigned twice!{wireIndex} "
                );

                let w = Some(CircuitGenerator::createProverWitnessWire(
                    self.generator.clone(),
                    &None,
                ));
                proverWitnessWires.push(w.clone());
                wireMapping[wireIndex] = w;
            } else {
                let ins = Self::getInputs(&line);
                for i in &ins {
                    assert!(i.is_some(), "Undefined input wire  {i:? } at line{line}",);
                }
                let outs = Self::getOutputs(&line);
                if line.starts_with("mul ") {
                    wireMapping[*outs[0].as_ref().unwrap()] = Some(
                        wireMapping[*ins[0].as_ref().unwrap()]
                            .clone()
                            .unwrap()
                            .mul(wireMapping[*ins[1].as_ref().unwrap()].as_ref().unwrap()),
                    );
                } else if line.starts_with("add ") {
                    let mut result = wireMapping[*ins[0].as_ref().unwrap()]
                        .as_ref()
                        .unwrap()
                        .clone();
                    for i in 1..ins.len() {
                        result =
                            result.add(wireMapping[*ins[i].as_ref().unwrap()].as_ref().unwrap());
                    }
                    wireMapping[*outs[0].as_ref().unwrap()] = Some(result);
                } else if line.starts_with("zerop ") {
                    wireMapping[*outs[1].as_ref().unwrap()] = Some(
                        wireMapping[*ins[0].as_ref().unwrap()]
                            .as_ref()
                            .unwrap()
                            .checkNonZero(&None),
                    );
                } else if line.starts_with("split ") {
                    let bits = wireMapping[*ins[0].as_ref().unwrap()]
                        .clone()
                        .unwrap()
                        .getBitWiresi(outs.len() as u64, &None)
                        .asArray()
                        .clone();
                    for i in 0..outs.len() {
                        wireMapping[*outs[i].as_ref().unwrap()] = bits[i].clone();
                    }
                } else if line.starts_with("const-mul-neg-") {
                    let constantStr = &line["const-mul-neg-".len()..line.find(" ").unwrap()];
                    let constant = BigInteger::parse_bytes(constantStr.as_bytes(), 16).unwrap();
                    wireMapping[*outs[0].as_ref().unwrap()] = Some(
                        wireMapping[*ins[0].as_ref().unwrap()]
                            .as_ref()
                            .unwrap()
                            .mulb(&constant.neg(), &None),
                    );
                } else if line.starts_with("const-mul-") {
                    let constantStr = &line["const-mul-".len()..line.find(" ").unwrap()];
                    let constant = BigInteger::parse_bytes(constantStr.as_bytes(), 16).unwrap();
                    wireMapping[*outs[0].as_ref().unwrap()] = Some(
                        wireMapping[*ins[0].as_ref().unwrap()]
                            .as_ref()
                            .unwrap()
                            .mulb(&constant, &None),
                    );
                } else {
                    panic!("Unsupport Circuit Line {line} ");
                }
            }
        }

        self.t.proverWitnessWires = proverWitnessWires;
        self.t.outputWires = outputWires;
    }

    fn getOutputs(line: &String) -> Vec<Option<usize>> {
        line[line.rfind("<").unwrap() + 1..line.rfind(">").unwrap()]
            .split('\n')
            .filter_map(|v| (!v.is_empty()).then(|| v.parse::<usize>().ok()))
            .collect()
    }

    fn getInputs(line: &String) -> Vec<Option<usize>> {
        line[line.rfind("<").unwrap() + 1..line.rfind(">").unwrap()]
            .split('\n')
            .map(|v| v.parse::<usize>().ok())
            .collect()
    }
    fn get_prover_witness_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.proverWitnessWires
    }
}
impl GadgetConfig for Gadget<PinocchioGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.outputWires
    }
}
