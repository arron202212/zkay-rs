#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::config::config::Configs;
use crate::circuit::operations::wire_label_instruction;
use crate::circuit::operations::wire_label_instruction::LabelType;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use num_bigint::Sign;
use rccell::RcCell;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::ops::{Add, Mul, Rem, Shl, Sub};
use std::path::Path;
// circuitGenerator: CircuitGenerator,
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct CircuitEvaluator {
    pub valueAssignment: RcCell<Vec<Option<BigInteger>>>,
}

impl CircuitEvaluator {
    pub fn new(circuitGenerator: CircuitGenerator) -> Self {
        let mut valueAssignment = vec![None; circuitGenerator.getNumWires() as usize];
        valueAssignment[circuitGenerator.getOneWire().unwrap().getWireId() as usize] =
            Some(Util::one());
        Self {
            valueAssignment: RcCell::new(valueAssignment),
        }
    }

    pub fn setWireValue(&self, w: WireType, v: BigInteger) {
        assert!(
            v.sign() != Sign::Minus && v < Configs.get().unwrap().field_prime,
            "Only positive values that are less than the modulus are allowed for this method."
        );
        self.valueAssignment.borrow_mut()[w.getWireId() as usize] = Some(v);
    }

    pub fn getWireValue(&self, w: WireType) -> BigInteger {
        let mut v = self.valueAssignment.borrow()[w.getWireId() as usize].clone();
        if v.is_none() {
            let bits = w.getBitWiresIfExistAlready();
            if let Some(bits) = bits {
                let mut sum = BigInteger::ZERO;
                for i in 0..bits.size() {
                    sum = sum.add(
                        self.valueAssignment.borrow()[bits.get(i).getWireId() as usize]
                            .clone()
                            .unwrap()
                            .shl(i),
                    );
                }
                v = Some(sum);
            }
        }
        v.unwrap()
    }

    pub fn getWiresValues(&self, w: Vec<Option<WireType>>) -> Vec<BigInteger> {
        let mut values = vec![BigInteger::ZERO; w.len()];
        for i in 0..w.len() {
            values[i] = self.getWireValue(w[i].clone().unwrap());
        }
        values
    }

    pub fn getWireValuei(&self, e: LongElement, bitwidthPerChunk: i32) -> BigInteger {
        Util::combine(
            self.valueAssignment.borrow().clone(),
            e.getArray(),
            bitwidthPerChunk,
        )
    }

    pub fn setWireValuebi(&self, e: LongElement, value: BigInteger, bitwidthPerChunk: i32) {
        let array: Vec<_> = e.getArray();
        self.setWireValuea(array, Util::split(value, bitwidthPerChunk));
    }

    pub fn setWireValuei(&self, wire: WireType, v: i64) {
        assert!(
            v >= 0,
            "Only positive values that are less than the modulus are allowed for this method."
        );
        self.setWireValue(wire, BigInteger::from(v));
    }

    pub fn setWireValuea(&self, wires: Vec<Option<WireType>>, v: Vec<BigInteger>) {
        for i in 0..v.len() {
            self.setWireValue(wires[i].clone().unwrap(), v[i].clone());
        }
        for i in v.len()..wires.len() {
            self.setWireValue(wires[i].clone().unwrap(), BigInteger::ZERO);
        }
    }

    pub fn evaluate(&self) {
        let circuitGenerator = CircuitGenerator::getActiveCircuitGenerator()
            .unwrap()
            .clone();
        println!(
            "Running Circuit Evaluator for < {} >",
            circuitGenerator.getName()
        );
        let evalSequence = circuitGenerator.getEvaluationQueue();

        for e in evalSequence.keys() {
            e.evaluate(self.clone());
            e.emit(self.clone());
        }
        // check that each wire has been assigned a value
        for i in 0..self.valueAssignment.borrow().len() {
            assert!(
                self.valueAssignment.borrow()[i].is_some(),
                "WireType# {i}is without value"
            );
        }
        println!(
            "Circuit Evaluation Done for < {} >\n\n",
            circuitGenerator.getName()
        );
    }

    pub fn writeInputFile(&self) {
        let circuitGenerator = CircuitGenerator::getActiveCircuitGenerator()
            .unwrap()
            .clone();
        let evalSequence = circuitGenerator.getEvaluationQueue();
        let mut printWriter = File::create(circuitGenerator.getName() + ".in").unwrap();
        for e in evalSequence.keys() {
            if e.wire_label().is_some()
                && (e.wire_label().as_ref().unwrap().getType() == LabelType::input
                    || e.wire_label().as_ref().unwrap().getType() == LabelType::nizkinput)
            {
                let id = e.wire_label().as_ref().unwrap().getWire().getWireId();
                let _=write!(
                    printWriter,
                    "{} {:x}",
                    id.to_string(),
                    self.valueAssignment.borrow()[id as usize].clone().unwrap()
                );
            }
        }
    }

    /**
     * An independent old method for testing.
     *
     * @param circuitFilePath
     * @param inFilePath
     * @
     */

    pub fn eval(&self, circuitFilePath: String, inFilePath: String) {
        let mut circuitScanner = BufReader::new(File::open(circuitFilePath).unwrap()).lines();
        let mut inFileScanner = BufReader::new(File::open(&inFilePath).unwrap()).lines();

        let totalWires = circuitScanner
            .next()
            .unwrap()
            .unwrap()
            .replace("total ", "")
            .parse::<i32>()
            .unwrap();

        let mut assignment = vec![None; totalWires as usize];

        let mut wiresToReport = vec![];
        let mut ignoreWires = HashSet::new();

        // Hashtable<Integer, BigInteger> assignment = new Hashtable<>();
        while let Some(Ok(wireNumber)) = inFileScanner.next() {
            let wireNumber = wireNumber.parse::<i32>().unwrap();
            let num = inFileScanner.next().unwrap().unwrap();
            assignment[wireNumber as usize] = BigInteger::parse_bytes(num.as_bytes(), 16);
            wiresToReport.push(wireNumber);
            // assignment.put(wireNumber, BigInteger::new(num));
        }

        let prime = BigInteger::parse_bytes(
            b"21888242871839275222246405745257275088548364400416034343698204186575808495617",
            10,
        )
        .unwrap();

        circuitScanner.next();
        while let Some(Ok(mut line)) = circuitScanner.next() {
            if line.contains("#") {
                line = line[..line.find("#").unwrap()].to_string();
                line = line.trim().to_string();
            }
            if line.starts_with("input") || line.starts_with("nizkinput") {
                continue;
            }
            if line.starts_with("output ") {
                let line = line.replace("output ", "").parse::<i32>().unwrap();
                println!(
                    "{}::{:x}",
                    line,
                    assignment[line as usize].as_ref().unwrap()
                );
                wiresToReport.push(line);
            } else if line.starts_with("DEBUG ") {
                line = line.replace("DEBUG ", "");
                let mut scanner = line.split_whitespace();
                let id = scanner.next().unwrap().parse::<i32>().unwrap();
                println!(
                    "{id}::{:x} >> {}",
                    assignment[id as usize].as_ref().unwrap(),
                    scanner.next().unwrap().split("\n").next().unwrap()
                );
            } else {
                let ins = self.getInputs(line.clone());
                for &i in &ins {
                    if assignment[i as usize].is_none() {
                        println!("Undefined value for a wire:used , at line {line}");
                    }
                }
                let outs = self.getOutputs(line.clone());
                if line.starts_with("mul ") {
                    let mut out = Util::one();
                    for w in ins {
                        out = out.mul(assignment[w as usize].clone().unwrap());
                    }
                    wiresToReport.push(outs[0]);
                    assignment[outs[0] as usize] = Some(out.rem(prime.clone()));
                } else if line.starts_with("add ") {
                    let mut out = BigInteger::ZERO;
                    for w in ins {
                        out = out.add(assignment[w as usize].clone().unwrap());
                    }
                    assignment[outs[0] as usize] = Some(out.rem(prime.clone()));
                } else if line.starts_with("xor ") {
                    let out = if assignment[ins[0] as usize] == assignment[ins[0] as usize] {
                        BigInteger::ZERO
                    } else {
                        Util::one()
                    };
                    assignment[outs[0] as usize] = Some(out);
                    wiresToReport.push(outs[0]);
                } else if line.starts_with("zerop ") {
                    ignoreWires.insert(outs[0]);
                    if assignment[ins[0] as usize].as_ref().unwrap().sign() == Sign::NoSign {
                        assignment[outs[1] as usize] = Some(BigInteger::ZERO);
                    } else {
                        assignment[outs[1] as usize] = Some(Util::one());
                    }
                    wiresToReport.push(outs[1]);
                } else if line.starts_with("split ") {
                    if outs.len() < assignment[ins[0] as usize].as_ref().unwrap().bits() as usize {
                        println!("Error in Split");
                        println!("{:x}", assignment[ins[0] as usize].as_ref().unwrap());
                        println!("{line}");
                    }
                    for i in 0..outs.len() {
                        assignment[outs[i] as usize] =
                            if assignment[ins[0] as usize].as_ref().unwrap().bit(i as u64) {
                                Some(Util::one())
                            } else {
                                Some(BigInteger::ZERO)
                            };
                        wiresToReport.push(outs[i]);
                    }
                } else if line.starts_with("pack ") {
                    let mut sum = BigInteger::ZERO;
                    for i in 0..ins.len() {
                        sum = sum.add(
                            assignment[ins[i] as usize]
                                .clone()
                                .unwrap()
                                .mul(BigInteger::from(2u8).pow(i as u32)),
                        );
                    }
                    wiresToReport.push(outs[0]);
                    assignment[outs[0] as usize] = Some(sum);
                } else if line.starts_with("const-mul-neg-") {
                    let constantStr = &line["const-mul-neg-".len()..line.find(" ").unwrap()];
                    let constant = prime
                        .clone()
                        .sub(BigInteger::parse_bytes(constantStr.as_bytes(), 16).unwrap());
                    assignment[outs[0] as usize] = Some(
                        assignment[ins[0] as usize]
                            .clone()
                            .unwrap()
                            .mul(constant.clone())
                            .rem(prime.clone()),
                    );
                } else if line.starts_with("const-mul-") {
                    let constantStr = &line["const-mul-".len()..line.find(" ").unwrap()];
                    let constant = BigInteger::parse_bytes(constantStr.as_bytes(), 16).unwrap();
                    assignment[outs[0] as usize] = Some(
                        assignment[ins[0] as usize]
                            .clone()
                            .unwrap()
                            .mul(constant)
                            .rem(prime.clone()),
                    );
                } else {
                    println!("Unknown Circuit Statement");
                }
            }
        }

        for i in 0..totalWires {
            if assignment[i as usize].is_none() && !ignoreWires.contains(&i) {
                println!("WireType  {i } is Null");
            }
        }

        let mut printWriter = File::create(inFilePath.clone() + ".full.2").unwrap();
        for id in wiresToReport {
            let _=write!(
                printWriter,
                "{id} {:x}",
                assignment[id as usize].clone().unwrap()
            );
        }
    }

    fn getOutputs(&self, line: String) -> Vec<i32> {
        // println!(line);
        let scanner = &line[line.rfind("<").unwrap() + 1..line.rfind(">").unwrap()];
        let mut outs = vec![];
        for v in scanner.split_whitespace() {
            // println!(v);
            outs.push(v.parse::<i32>().unwrap());
        }
        outs
    }

    fn getInputs(&self, line: String) -> Vec<i32> {
        line[line.find("<").unwrap() + 1..line.find(">").unwrap()]
            .split_whitespace()
            .filter_map(|v| v.parse::<i32>().ok())
            .collect()
    }

    pub fn getAssignment(&self) -> Vec<Option<BigInteger>> {
        self.valueAssignment.borrow().clone()
    }
}
