#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    arc_cell_new,
    circuit::{
        StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::instruction::Instruction,
        operations::{wire_label_instruction, wire_label_instruction::LabelType},
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
use num_bigint::Sign;
use rccell::{RcCell, WeakCell};
// use crate::util::util::ARcCell;
use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader, Error, Write},
    marker::PhantomData,
    ops::{Add, Mul, Rem, Shl, Sub},
    path::Path,
};
#[derive(Debug, Clone)]
pub struct CircuitEvaluator {
    pub valueAssignment: Vec<Option<BigInteger>>,
    pub cg_name: String,
}

impl CircuitEvaluator {
    pub fn new<T: CGConfig>(cg_name: &str, generator: &RcCell<T>) -> Self {
        let mut valueAssignment = vec![None; generator.get_num_wires() as usize];
        valueAssignment[generator.get_one_wire().unwrap().getWireId() as usize] = Some(Util::one());
        Self {
            valueAssignment,
            cg_name: cg_name.to_owned(),
        }
    }

    pub fn setWireValue(&mut self, w: &WireType, v: &BigInteger) {
        assert!(
            v.sign() != Sign::Minus && v < &Configs.field_prime,
            "Only positive values that are less than the modulus are allowed for this method.{:?},{},{}",
            w,
            w.getWireId(),
            v
        );
        if *v
            == Util::parse_big_int(
                "4791453329370480208338917599763329258431616614496986402522529110684255984585",
            )
            || *v
                == Util::parse_big_int(
                    "13970443093117176793321745021511292434216648861635796193749801964876617418268",
                )
            || w.getWireId() == 4
            || w.getWireId() == 48124
        {
            println!(
                "==wireid====setwv========={}======{}======",
                w.getWireId(),
                w.name()
            );
        }

        self.valueAssignment[w.getWireId() as usize] = Some(v.clone());
    }

    pub fn getWireValue(&self, w: &WireType) -> BigInteger {
        let mut v = &self.valueAssignment[w.getWireId() as usize];
        if let Some(v) = v {
            // println!("==wireid==getWireValue==some========={}============",w.getWireId());
            return v.clone();
        }
        let Some(bits) = w.getBitWiresIfExistAlready() else {
            // println!("==wireid==getWireValue==getBitWiresIfExistAlready==none======={}============",w.getWireId());
            return BigInteger::ZERO;
        };
        // println!("==wireid==getWireValue==getBitWiresIfExistAlready==some======={}============",w.getWireId());

        bits.array
            .iter()
            .enumerate()
            .fold(BigInteger::ZERO, |sum, (i, b)| {
                sum.add(
                    self.valueAssignment[b.as_ref().unwrap().getWireId() as usize]
                        .clone()
                        .unwrap()
                        .shl(i),
                )
            })
    }

    pub fn getWiresValues(&self, w: &Vec<Option<WireType>>) -> Vec<BigInteger> {
        w.iter()
            .map(|v| self.getWireValue(v.as_ref().unwrap()))
            .collect()
    }

    pub fn getWireValuei(&self, e: &LongElement, bitwidthPerChunk: i32) -> BigInteger {
        Util::combine(&self.valueAssignment, e.getArray(), bitwidthPerChunk)
    }

    pub fn setWireValuebi(&mut self, e: &LongElement, value: &BigInteger, bitwidthPerChunk: i32) {
        self.setWireValuea(&e.getArray(), &Util::split(value, bitwidthPerChunk));
    }

    pub fn setWireValuei(&mut self, wire: &WireType, v: i64) {
        assert!(
            v >= 0,
            "Only positive values that are less than the modulus are allowed for this method."
        );
        self.setWireValue(wire, &BigInteger::from(v));
    }

    pub fn setWireValuea(&mut self, wires: &Vec<Option<WireType>>, v: &Vec<BigInteger>) {
        for i in 0..v.len() {
            self.setWireValue(wires[i].as_ref().unwrap(), &v[i]);
        }
        for i in v.len()..wires.len() {
            self.setWireValue(wires[i].as_ref().unwrap(), &BigInteger::ZERO);
        }
    }

    pub fn evaluate<T: CGConfig>(&mut self, generator: &RcCell<T>) -> eyre::Result<()> {
        println!(
            "==evaluate===evaluator.getAssignment().len()============{}",
            self.getAssignment().len()
        );
        println!("Running Circuit Evaluator for < {} >", generator.get_name());
        let evalSequence = generator.get_evaluation_queue();

        for e in evalSequence.values() {
            e.evaluate(self)?;
            e.emit(self);
        }
        // check that each wire has been assigned a value
        for i in 0..self.valueAssignment.len() {
            assert!(
                self.valueAssignment[i].is_some(),
                "WireType# {i}is without value"
            );
        }
        println!(
            "Circuit Evaluation Done for < {} >\n\n",
            generator.get_name()
        );
        Ok(())
    }

    pub fn writeInputFile<T: CGConfig>(&self, generator: RcCell<T>) {
        // let generator=generator.upgrade().unwrap();
        let evalSequence = generator.borrow().get_evaluation_queue();
        let mut printWriter = File::create(generator.borrow().get_name() + ".in").unwrap();
        for e in evalSequence.values() {
            if e.wire_label().is_some()
                && (e.wire_label().as_ref().unwrap().getType() == LabelType::input
                    || e.wire_label().as_ref().unwrap().getType() == LabelType::nizkinput)
            {
                let id = e.wire_label().as_ref().unwrap().getWire().getWireId();
                let _ = write!(
                    printWriter,
                    "{} {:x}",
                    id.to_string(),
                    self.valueAssignment[id as usize].as_ref().unwrap()
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
                continue;
            }
            if line.starts_with("DEBUG ") {
                line = line.replace("DEBUG ", "");
                let mut scanner = line.split_whitespace();
                let id = scanner.next().unwrap().parse::<i32>().unwrap();
                println!(
                    "{id}::{:x} >> {}",
                    assignment[id as usize].as_ref().unwrap(),
                    scanner.next().unwrap().split("\n").next().unwrap()
                );
                continue;
            }
            let ins = self.getInputs(&line);
            for &i in &ins {
                if assignment[i as usize].is_none() {
                    println!("Undefined value for a wire:used , at line {line}");
                }
            }
            let outs = self.getOutputs(&line);
            match line {
                _ if line.starts_with("mul ") => {
                    let mut out = Util::one();
                    for w in ins {
                        out = out.mul(assignment[w as usize].clone().unwrap());
                    }
                    wiresToReport.push(outs[0]);
                    assignment[outs[0] as usize] = Some(out.rem(&prime));
                }
                _ if line.starts_with("add ") => {
                    let mut out = BigInteger::ZERO;
                    for w in ins {
                        out = out.add(assignment[w as usize].clone().unwrap());
                    }
                    assignment[outs[0] as usize] = Some(out.rem(&prime));
                }
                _ if line.starts_with("xor ") => {
                    let out = if assignment[ins[0] as usize] == assignment[ins[0] as usize] {
                        BigInteger::ZERO
                    } else {
                        Util::one()
                    };
                    assignment[outs[0] as usize] = Some(out);
                    wiresToReport.push(outs[0]);
                }
                _ if line.starts_with("zerop ") => {
                    ignoreWires.insert(outs[0]);
                    if assignment[ins[0] as usize].as_ref().unwrap().sign() == Sign::NoSign {
                        assignment[outs[1] as usize] = Some(BigInteger::ZERO);
                    } else {
                        assignment[outs[1] as usize] = Some(Util::one());
                    }
                    wiresToReport.push(outs[1]);
                }
                _ if line.starts_with("split ") => {
                    if outs.len() < assignment[ins[0] as usize].as_ref().unwrap().bits() as usize {
                        //println!("Error in Split");
                        //println!("{:x}", assignment[ins[0] as usize].as_ref().unwrap());
                        //println!("{line}");
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
                }
                _ if line.starts_with("pack ") => {
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
                }
                _ if line.starts_with("const-mul-neg-") => {
                    let constantStr = &line["const-mul-neg-".len()..line.find(" ").unwrap()];
                    let constant = prime
                        .clone()
                        .sub(BigInteger::parse_bytes(constantStr.as_bytes(), 16).unwrap());
                    assignment[outs[0] as usize] = Some(
                        assignment[ins[0] as usize]
                            .clone()
                            .unwrap()
                            .mul(&constant)
                            .rem(&prime),
                    );
                }
                _ if line.starts_with("const-mul-") => {
                    let constantStr = &line["const-mul-".len()..line.find(" ").unwrap()];
                    let constant = BigInteger::parse_bytes(constantStr.as_bytes(), 16).unwrap();
                    assignment[outs[0] as usize] = Some(
                        assignment[ins[0] as usize]
                            .clone()
                            .unwrap()
                            .mul(constant)
                            .rem(&prime),
                    );
                }
                _ => {
                    println!("Unknown Circuit Statement");
                }
            }
        }

        for i in 0..totalWires {
            if assignment[i as usize].is_none() && !ignoreWires.contains(&i) {
                println!("WireType {i } is Null");
            }
        }

        let mut printWriter = File::create(inFilePath.clone() + ".full.2").unwrap();
        for id in wiresToReport {
            let _ = write!(
                printWriter,
                "{id} {:x}",
                assignment[id as usize].as_ref().unwrap()
            );
        }
    }

    fn getOutputs(&self, line: &String) -> Vec<i32> {
        // //println!(line);
        let scanner = &line[line.rfind("<").unwrap() + 1..line.rfind(">").unwrap()];
        let mut outs = vec![];
        for v in scanner.split_whitespace() {
            // //println!(v);
            outs.push(v.parse::<i32>().unwrap());
        }
        outs
    }

    fn getInputs(&self, line: &String) -> Vec<i32> {
        line[line.find("<").unwrap() + 1..line.find(">").unwrap()]
            .split_whitespace()
            .filter_map(|v| v.parse::<i32>().ok())
            .collect()
    }

    pub fn getAssignment(&self) -> &Vec<Option<BigInteger>> {
        &self.valueAssignment
    }
    pub fn get_assignment_mut(&mut self) -> &mut Vec<Option<BigInteger>> {
        &mut self.valueAssignment
    }
}
