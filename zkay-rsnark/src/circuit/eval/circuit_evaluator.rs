#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        auxiliary::long_element::LongElement,
        config::config::CONFIGS,
        operations::wire_label_instruction::LabelType,
        structure::{
            circuit_generator::CGConfig,
            circuit_generator::CGConfigFields,
            wire::{GetWireId, WireConfig},
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};
use num_bigint::Sign;
use rccell::RcCell;
use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Write},
    ops::{Add, Mul, Rem, Shl, Sub},
};
#[derive(Debug, Clone)]
pub struct CircuitEvaluator {
    pub value_assignment: Vec<Option<BigInteger>>,
    pub cg_name: String,
}

impl CircuitEvaluator {
    pub fn new<T: CGConfig>(cg_name: &str, generator: &RcCell<T>) -> Self {
        let mut value_assignment = vec![None; generator.get_num_wires() as usize];
        value_assignment[generator.get_one_wire().unwrap().get_wire_id() as usize] =
            Some(Util::one());
        Self {
            value_assignment,
            cg_name: cg_name.to_owned(),
        }
    }

    pub fn set_wire_value(&mut self, w: &WireType, v: &BigInteger) {
        assert!(
            v.sign() != Sign::Minus && v < &CONFIGS.field_prime,
            "Only positive values that are less than the modulus are allowed for this method.{:?},{},{}",
            w,
            w.get_wire_id(),
            v
        );

        self.value_assignment[w.get_wire_id() as usize] = Some(v.clone());
    }

    pub fn get_wire_value(&self, w: &WireType) -> BigInteger {
        let mut v = &self.value_assignment[w.get_wire_id() as usize];
        if let Some(v) = v {
            return v.clone();
        }
        let Some(bits) = w.get_bit_wires_if_exist_already() else {
            return BigInteger::ZERO;
        };

        bits.array
            .iter()
            .enumerate()
            .fold(BigInteger::ZERO, |sum, (i, b)| {
                sum.add(
                    self.value_assignment[b.as_ref().unwrap().get_wire_id() as usize]
                        .clone()
                        .unwrap()
                        .shl(i),
                )
            })
    }

    pub fn get_wires_values(&self, w: &Vec<Option<WireType>>) -> Vec<BigInteger> {
        w.iter()
            .map(|v| self.get_wire_value(v.as_ref().unwrap()))
            .collect()
    }

    pub fn get_wire_valuei(&self, e: &LongElement, bitwidth_per_chunk: i32) -> BigInteger {
        Util::combine(&self.value_assignment, e.get_array(), bitwidth_per_chunk)
    }

    pub fn set_wire_valuebi(
        &mut self,
        e: &LongElement,
        value: &BigInteger,
        bitwidth_per_chunk: i32,
    ) {
        self.set_wire_valuea(&e.get_array(), &Util::split(value, bitwidth_per_chunk));
    }

    pub fn set_wire_valuei(&mut self, wire: &WireType, v: i64) {
        assert!(
            v >= 0,
            "Only positive values that are less than the modulus are allowed for this method."
        );
        self.set_wire_value(wire, &BigInteger::from(v));
    }

    pub fn set_wire_valuea(&mut self, wires: &Vec<Option<WireType>>, v: &Vec<BigInteger>) {
        for i in 0..v.len() {
            self.set_wire_value(wires[i].as_ref().unwrap(), &v[i]);
        }
        for i in v.len()..wires.len() {
            self.set_wire_value(wires[i].as_ref().unwrap(), &BigInteger::ZERO);
        }
    }

    pub fn evaluate<T: CGConfig>(&mut self, generator: &RcCell<T>) -> eyre::Result<()> {
        println!("Running Circuit Evaluator for < {} >", generator.get_name());
        let eval_sequence = generator.get_evaluation_queue();

        for e in eval_sequence.values() {
            e.evaluate(self)?;
            e.emit(self);
        }
        // check that each wire has been assigned a value
        for i in 0..self.value_assignment.len() {
            assert!(
                self.value_assignment[i].is_some(),
                "WireType# {i}is without value"
            );
        }
        println!(
            "Circuit Evaluation Done for < {} >\n\n",
            generator.get_name()
        );
        Ok(())
    }

    pub fn write_input_file<T: CGConfig>(&self, generator: RcCell<T>) {
        // let generator=generator.upgrade().unwrap();
        let eval_sequence = generator.borrow().get_evaluation_queue();
        let mut print_writer = File::create(generator.borrow().get_name() + ".in").unwrap();
        for e in eval_sequence.values() {
            if e.wire_label().is_some()
                && (e.wire_label().as_ref().unwrap().get_type() == LabelType::input
                    || e.wire_label().as_ref().unwrap().get_type() == LabelType::nizkinput)
            {
                let id = e.wire_label().as_ref().unwrap().get_wire().get_wire_id();
                let _ = write!(
                    print_writer,
                    "{} {:x}",
                    id.to_string(),
                    self.value_assignment[id as usize].as_ref().unwrap()
                );
            }
        }
    }

    //An independent old method for testing.
    //  *
    //@param circuit_file_path
    //@param in_file_path
    //@

    pub fn eval(&self, circuit_file_path: String, in_file_path: String) {
        let mut circuit_scanner = BufReader::new(File::open(circuit_file_path).unwrap()).lines();
        let mut in_file_scanner = BufReader::new(File::open(&in_file_path).unwrap()).lines();

        let total_wires = circuit_scanner
            .next()
            .unwrap()
            .unwrap()
            .replace("total ", "")
            .parse::<i32>()
            .unwrap();

        let mut assignment = vec![None; total_wires as usize];

        let mut wires_to_report = vec![];
        let mut ignore_wires = HashSet::new();

        // Hashtable<Integer, BigInteger> assignment = new Hashtable<>();
        while let Some(Ok(wire_number)) = in_file_scanner.next() {
            let wire_number = wire_number.parse::<i32>().unwrap();
            let num = in_file_scanner.next().unwrap().unwrap();
            assignment[wire_number as usize] = BigInteger::parse_bytes(num.as_bytes(), 16);
            wires_to_report.push(wire_number);
            // assignment.put(wire_number, BigInteger::new(num));
        }

        let prime = BigInteger::parse_bytes(
            b"21888242871839275222246405745257275088548364400416034343698204186575808495617",
            10,
        )
        .unwrap();

        circuit_scanner.next();
        while let Some(Ok(mut line)) = circuit_scanner.next() {
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
                wires_to_report.push(line);
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
            let ins = self.get_inputs(&line);
            for &i in &ins {
                if assignment[i as usize].is_none() {
                    println!("Undefined value for a wire:used , at line {line}");
                }
            }
            let outs = self.get_outputs(&line);
            match line {
                _ if line.starts_with("mul ") => {
                    let mut out = Util::one();
                    for w in ins {
                        out = out.mul(assignment[w as usize].clone().unwrap());
                    }
                    wires_to_report.push(outs[0]);
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
                    wires_to_report.push(outs[0]);
                }
                _ if line.starts_with("zerop ") => {
                    ignore_wires.insert(outs[0]);
                    if assignment[ins[0] as usize].as_ref().unwrap().sign() == Sign::NoSign {
                        assignment[outs[1] as usize] = Some(BigInteger::ZERO);
                    } else {
                        assignment[outs[1] as usize] = Some(Util::one());
                    }
                    wires_to_report.push(outs[1]);
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
                        wires_to_report.push(outs[i]);
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
                    wires_to_report.push(outs[0]);
                    assignment[outs[0] as usize] = Some(sum);
                }
                _ if line.starts_with("const-mul-neg-") => {
                    let constant_str = &line["const-mul-neg-".len()..line.find(" ").unwrap()];
                    let constant = prime
                        .clone()
                        .sub(BigInteger::parse_bytes(constant_str.as_bytes(), 16).unwrap());
                    assignment[outs[0] as usize] = Some(
                        assignment[ins[0] as usize]
                            .clone()
                            .unwrap()
                            .mul(&constant)
                            .rem(&prime),
                    );
                }
                _ if line.starts_with("const-mul-") => {
                    let constant_str = &line["const-mul-".len()..line.find(" ").unwrap()];
                    let constant = BigInteger::parse_bytes(constant_str.as_bytes(), 16).unwrap();
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

        for i in 0..total_wires {
            if assignment[i as usize].is_none() && !ignore_wires.contains(&i) {
                println!("WireType {i } is Null");
            }
        }

        let mut print_writer = File::create(in_file_path.clone() + ".full.2").unwrap();
        for id in wires_to_report {
            let _ = write!(
                print_writer,
                "{id} {:x}",
                assignment[id as usize].as_ref().unwrap()
            );
        }
    }

    fn get_outputs(&self, line: &String) -> Vec<i32> {
        let scanner = &line[line.rfind("<").unwrap() + 1..line.rfind(">").unwrap()];
        let mut outs = vec![];
        for v in scanner.split_whitespace() {
            outs.push(v.parse::<i32>().unwrap());
        }
        outs
    }

    fn get_inputs(&self, line: &String) -> Vec<i32> {
        line[line.find("<").unwrap() + 1..line.find(">").unwrap()]
            .split_whitespace()
            .filter_map(|v| v.parse::<i32>().ok())
            .collect()
    }

    pub fn get_assignment(&self) -> &Vec<Option<BigInteger>> {
        &self.value_assignment
    }
    pub fn get_assignment_mut(&mut self) -> &mut Vec<Option<BigInteger>> {
        &mut self.value_assignment
    }
}
