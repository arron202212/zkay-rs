#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    gadgetlib2::{
        constraint::PrintOptions,
        pp::Fp,
        variable::{
            FElem, FElemInterface, FieldType, LinearCombination, LinearTerm, ProtoboardPtr,
            R1P_Elem, Variable, VariablePtr,
        },
    },
    jsnark_interface::{
        circuit_parser::{Gate, WireEntry, read_field_element_from_hex},
        util::readIds,
    },
};
use ff_curves::{Fr, default_ec_pp};
use ffec::common::profiling::{enter_block, leave_block, start_profiling};
use rccell::RcCell;
use regex::Regex;
use tracing::{span, Level};
use sscanf::sscanf;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
    ops::{Add, Mul, Neg, Sub},
    process,
};

type Wire = u32;

pub type FieldT = Fr<default_ec_pp>;
type LinearCombinationPtr = RcCell<LinearCombination>;
type WireMap = BTreeMap<Wire, u32>;

fn lcp() -> LinearCombinationPtr {
    RcCell::new(LinearCombination::default())
}

enum OpCode {
    Zero,
    Add,
    Mul,
    Split,
    NonZeroCheck,
    Pack,
    MulConst,
    Xor,
    Or,
    Constraint,
}

pub struct CircuitReader {
    pb: ProtoboardPtr,

    variables: Vec<VariablePtr>,
    wireLinearCombinations: Vec<LinearCombinationPtr>,
    zeroPwires: Vec<LinearCombinationPtr>,

    variableMap: WireMap,
    zeropMap: WireMap,

    wireUseCounters: Vec<u32>,
    wireValues: Vec<FieldT>,

    toClean: Vec<Wire>,

    inputWireIds: Vec<Wire>,
    nizkWireIds: Vec<Wire>,
    outputWireIds: Vec<Wire>,

    numWires: u32,
    numOutputs: u32,
    numInputs: u32,
    numNizkInputs: u32,

    currentLinearCombinationIdx: u32,
    currentVariableIdx: u32,
}

impl CircuitReader {
    pub fn new(arithFilepath: &str, inputsFilepath: &str, pb: ProtoboardPtr) -> Self {
        let mut _self = Self {
            pb,
            variables: vec![],
            wireLinearCombinations: vec![],
            zeroPwires: vec![],
            variableMap: WireMap::new(),
            zeropMap: WireMap::new(),
            wireUseCounters: vec![],
            wireValues: vec![],
            toClean: vec![],
            inputWireIds: vec![],
            nizkWireIds: vec![],
            outputWireIds: vec![],
            numWires: 0,
            numOutputs: 0,
            numInputs: 0,
            numNizkInputs: 0,
            currentLinearCombinationIdx: 0,
            currentVariableIdx: 0,
        };
        let mut input = std::fs::read_to_string(arithFilepath).expect(arithFilepath);
        let gates = super::circuit_parser::parse(&input);
        _self.parse_and_eval(&gates, inputsFilepath);
        _self.construct_circuit(&gates);
        _self.mapValuesToProtoboard();

        _self.wireLinearCombinations.clear();
        _self.wireValues.clear();
        _self.variables.clear();
        _self.variableMap.clear();
        _self.zeropMap.clear();
        _self.zeroPwires.clear();
        _self
    }
    pub fn getNumInputs(&self) -> u32 {
        self.numInputs
    }
    pub fn getNumOutputs(&self) -> u32 {
        self.numOutputs
    }
    pub fn getInputWireIds(&self) -> &Vec<Wire> {
        &self.inputWireIds
    }
    pub fn getOutputWireIds(&self) -> &Vec<Wire> {
        &self.outputWireIds
    }

    pub fn parse_and_eval(&mut self, gates: &[Gate], inputsFilepath: &str) {
        let span = span!(Level::TRACE, "Parsing and Evaluating the circuit").entered();

        let Gate::Total(ret) = gates[0] else {
            panic!("total failed")
        }; //scan_fmt!(&line.unwrap(), "total {d}", u32);

        self.numWires = ret;
        self.wireValues
            .resize(self.numWires as usize, FieldT::zero());
        self.wireUseCounters.resize(self.numWires as usize, 0);
        self.wireLinearCombinations.resize(
            self.numWires as usize,
            RcCell::new(LinearCombination::default()),
        );
        let mut input = std::fs::read_to_string(inputsFilepath).expect(inputsFilepath);
        let Ok((_, inputfs)) = super::circuit_parser::parse_all_inputs(&input) else {
            println!("Unable to open input file {} \n", inputsFilepath);
            process::exit(-1);
        };

        for WireEntry { id, value } in inputfs {
            self.wireValues[id as usize] = value.into(); // read_field_element_from_hex(&inputStr);
        }

        if self.wireValues[0] != FieldT::one() {
            println!(
                ">> Warning: when using jsnark circuit generator, the first input wire (#0) must have the value of 1.\n"
            );
            println!(
                "\t If the circuit was generated using Pinocchio *without modification*, you can ignore this warning. Pinocchio uses a different indexing for the one-wire input. \n"
            );
        }

        let mut inputStr: &str;
        let mut outputStr: &str;
        let (mut numGateInputs, mut numGateOutputs) = (0, 0);

        let oneElement = FieldT::one();
        let zeroElement = FieldT::zero();
        let negOneElement = FieldT::from(-1);

        // Parse the circuit: few lines were imported from Pinocchio's code.
        use std::time::{Duration, Instant};
        let start = Instant::now();
        for line in &gates[..] {
            match line {
                Gate::Input(wireId) => {
                    self.numInputs += 1;
                    self.inputWireIds.push(*wireId);
                }
                Gate::NizkInput(wireId) => {
                    self.numNizkInputs += 1;
                    self.nizkWireIds.push(*wireId);
                }
                Gate::Output(wireId) => {
                    self.numOutputs += 1;
                    self.outputWireIds.push(*wireId);
                    self.wireUseCounters[*wireId as usize] += 1;
                }
                Gate::Complex {
                    typ,
                    inputs,
                    outputs,
                } => {
                    let (types, numGateInputs, inputStr, numGateOutputs, outputStr) =
                        (*typ, inputs.len(), inputs, outputs.len(), outputs);

                    let mut inValues = vec![];
                    let mut outWires = vec![];

                    for inWireId in inputStr {
                        self.wireUseCounters[*inWireId as usize] += 1;
                        inValues.push(self.wireValues[*inWireId as usize]);
                    }
                    outWires = outputStr.clone();
                    let mut opcode = OpCode::Zero;
                    let mut constant = FieldT::zero();
                    match types {
                        "add" => {
                            opcode = OpCode::Add;
                        }
                        "mul" => {
                            opcode = OpCode::Mul;
                        }
                        "xor" => {
                            opcode = OpCode::Xor;
                        }
                        "or" => {
                            opcode = OpCode::Or;
                        }
                        "assert" => {
                            self.wireUseCounters[outWires[0] as usize] += 1;
                            opcode = OpCode::Constraint;
                        }
                        "pack" => {
                            opcode = OpCode::Pack;
                        }
                        "zerop" => {
                            opcode = OpCode::NonZeroCheck;
                        }
                        "split" => {
                            opcode = OpCode::Split;
                        }
                        _ if types.starts_with("const-mul-neg-") => {
                            opcode = OpCode::MulConst;
                            let constStr = &types["const-mul-neg-".len()..];
                            constant = negOneElement * read_field_element_from_hex(constStr);
                        }
                        _ if types.starts_with("const-mul-") => {
                            opcode = OpCode::MulConst;
                            let constStr = &types["const-mul-".len()..];
                            constant = read_field_element_from_hex(constStr).into();
                        }
                        _ => {
                            panic!("Error:types unrecognized line: {line:?}\n");
                        }
                    }
                    // TODO: separate evaluation from parsing completely to get accurate evaluation cost
                    //	 Calling  get_nsec_time(); repetitively as in the old version adds much overhead
                    // TODO 2: change circuit format to enable skipping some lines during evaluation
                    //       Not all intermediate wire values need to be computed in this phase
                    // TODO 3: change circuit format to make common constants defined once

                    //begin = get_nsec_time();
                    match opcode {
                        OpCode::Add => {
                            self.wireValues[outWires[0] as usize] =
                                inValues.iter().fold(Fp::default(), |s, c| s + c);
                        }
                        OpCode::Mul => {
                            self.wireValues[outWires[0] as usize] = inValues[0] * inValues[1];
                        }
                        OpCode::Xor => {
                            self.wireValues[outWires[0] as usize] = if inValues[0] == inValues[1] {
                                zeroElement
                            } else {
                                oneElement
                            };
                        }
                        OpCode::Or => {
                            self.wireValues[outWires[0] as usize] =
                                if inValues[0] == zeroElement && inValues[1] == zeroElement {
                                    zeroElement
                                } else {
                                    oneElement
                                };
                        }
                        OpCode::NonZeroCheck => {
                            self.wireValues[outWires[1] as usize] = if inValues[0] == zeroElement {
                                zeroElement
                            } else {
                                oneElement
                            };
                        }
                        OpCode::Pack => {
                            let (mut sum, mut coeff) = (FieldT::zero(), FieldT::zero());
                            let mut two = oneElement;
                            for &v in &inValues {
                                sum += two * v;
                                two += two;
                            }
                            self.wireValues[outWires[0] as usize] = sum;
                        }
                        OpCode::Split => {
                            let size = outWires.len();
                            let inVal = FElem::from(R1P_Elem::froms(inValues[0].clone()));
                            for i in 0..size {
                                self.wireValues[outWires[i] as usize] =
                                    inVal.getBits(i as u32, &FieldType::R1P).into();
                            }
                        }
                        OpCode::MulConst => {
                            self.wireValues[outWires[0] as usize] = constant * inValues[0];
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        println!("===eval==start=={:?}==", start.elapsed());
        span.exit();
    }

    pub fn construct_circuit(&mut self, gates: &[Gate]) {
        println!("Translating Constraints ... ");

        let (mut currentVariableIdx, mut currentLinearCombinationIdx) = (0, 0);
        for i in 0..self.numInputs as usize {
            self.variables.push(RcCell::new(Variable::from("input")));
            self.variableMap
                .insert(self.inputWireIds[i], currentVariableIdx);
            currentVariableIdx += 1;
        }
        for i in 0..self.numOutputs as usize {
            self.variables.push(RcCell::new(Variable::from("output")));
            self.variableMap
                .insert(self.outputWireIds[i], currentVariableIdx);
            currentVariableIdx += 1;
        }
        for i in 0..self.numNizkInputs as usize {
            self.variables
                .push(RcCell::new(Variable::from("nizk input")));
            self.variableMap
                .insert(self.nizkWireIds[i], currentVariableIdx);
            currentVariableIdx += 1;
        }

        // char types[200];
        // inputStr:&str;
        // outputStr:&str;
        // string line;
        let (mut numGateInputs, mut numGateOutputs) = (0, 0);

        // let mut ifs2 = fs::read_to_string(arithFilepath);

        // if ifs2.is_err() {
        //     println!("Unable to open circuit file:\n");
        //     process::exit(5);
        // }

        // // Parse the circuit: few lines were imported from Pinocchio's code.
        // let Ok(ifs2) = ifs2 else { return };
        // let mut ifs2 = ifs2.lines();
        // let mut line = ifs2.next().unwrap();
        let Gate::Total(numWires) = gates[0] else {
            panic!("total failed")
        };
        // let re = Regex::new(r"(\S+) in (\d+) <([^>]+)> out (\d+) <([^>]+)>").unwrap();
        let mut lineCount = 0;
        use std::time::{Duration, Instant};
        let start = Instant::now();
        for line in &gates[1..] {
            lineCount += 1;
            //		if lineCount % 100000 == 0 {
            //			println!("At Line:: {}\n", lineCount);
            //		}

            // if line.is_empty() {
            //     continue;
            // }

            // let Ok((types, numGateInputs, inputStr, numGateOutputs, outputStr)) = scan_fmt!(
            //     line,
            //     "{} in {} <{:/[^>]+/}> out {} <{:/[^>]+/}>",
            //     String,
            //     i32,
            //     String,
            //     i32,
            //     String
            // )
            let Gate::Complex {
                typ,
                inputs,
                outputs,
            } = line
            else {
                continue;
            };
            let (types, numGateInputs, inputStr, numGateOutputs, outputStr) =
                (*typ, inputs.len(), inputs, outputs.len(), outputs);

            match types {
                "add" => {
                    assert!(numGateOutputs == 1);
                    self.handleAddition(&inputStr, &outputStr);
                }
                "mul" => {
                    assert!(numGateInputs == 2 && numGateOutputs == 1);
                    self.addMulConstraint(&inputStr, &outputStr);
                }
                "xor" => {
                    assert!(numGateInputs == 2 && numGateOutputs == 1);
                    self.addXorConstraint(&inputStr, &outputStr);
                }
                "or" => {
                    assert!(numGateInputs == 2 && numGateOutputs == 1);
                    self.addOrConstraint(&inputStr, &outputStr);
                }
                "assert" => {
                    assert!(numGateInputs == 2 && numGateOutputs == 1);
                    self.addAssertionConstraint(&inputStr, &outputStr);
                }
                _ if types.starts_with("const-mul-neg-") => {
                    assert!(numGateInputs == 1 && numGateOutputs == 1);
                    self.handleMulNegConst(&types, &inputStr, &outputStr);
                }
                _ if types.starts_with("const-mul-") => {
                    assert!(numGateInputs == 1 && numGateOutputs == 1);
                    self.handleMulConst(&types, &inputStr, &outputStr);
                }
                "zerop" => {
                    assert!(numGateInputs == 1 && numGateOutputs == 2);
                    self.addNonzeroCheckConstraint(&inputStr, &outputStr);
                }
                _ if types.starts_with("split") => {
                    assert!(numGateInputs == 1);
                    self.addSplitConstraint(&inputStr, &outputStr, numGateOutputs as u16);
                }
                _ if types.starts_with("pack") => {
                    assert!(numGateOutputs == 1);
                    // addPackConstraint(inputStr, outputStr, numGateInputs);
                    self.handlePackOperation(&inputStr, &outputStr, numGateInputs as u16);
                }
                _ => {}
            }

            self.clean();
        }
        println!("===start===={:?}==", start.elapsed());
        println!("\tConstraint translation done\n");

        //
        // look_up_our_self(&usage2);
        // u64 diff = usage2.vsize - usage1.vsize;
        // println!("\tMemory usage for constraint translation: %lu MB\n", diff >> 20);
        //
    }

    pub fn mapValuesToProtoboard(&self) {
        let mut zeropGateIndex = 0;
        for (&wireId, &v) in &self.variableMap {
            *self
                .pb
                .as_ref()
                .unwrap()
                .borrow_mut()
                .val(&self.variables[v as usize].borrow()) =
                R1P_Elem::froms(self.wireValues[wireId as usize].clone()).into();
        }
        for (&wireId, &z) in &self.zeropMap {
            let l = self.zeroPwires[zeropGateIndex].clone();
            zeropGateIndex += 1;
            if self.pb.as_ref().unwrap().borrow().val_lc(&l.borrow())
                == FElem::from(R1P_Elem::froms(FieldT::zero()))
            {
                *self
                    .pb
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .val(&self.variables[z as usize].borrow()) =
                    R1P_Elem::froms(FieldT::zero()).into();
            } else {
                *self
                    .pb
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .val(&self.variables[z as usize].borrow()) = self
                    .pb
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .val_lc(&l.borrow())
                    .inverses(&self.pb.as_ref().unwrap().borrow().fieldType_);
            }
        }

        if !self
            .pb
            .as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        {
            println!("Note: Protoboard Not Satisfied .. \n");
            // assert!(false);
        }
        println!("Assignment of values done .. \n");
    }

    pub fn find(&mut self, wireId: Wire, lc: &mut LinearCombinationPtr, intentionToEdit: bool) {
        if self.wireLinearCombinations[wireId as usize]
            .borrow()
            .clone()
            == LinearCombination::default()
        {
            assert!(
                self.variableMap.contains_key(&wireId),
                "{wireId} not in variableMap"
            );
            self.wireLinearCombinations[wireId as usize] = RcCell::new(LinearCombination::from(
                self.variables[self.variableMap[&wireId] as usize]
                    .borrow()
                    .clone(),
            ));
        }
        self.wireUseCounters[wireId as usize] -= 1;
        if self.wireUseCounters[wireId as usize] == 0 {
            self.toClean.push(wireId);
            *lc = self.wireLinearCombinations[wireId as usize].clone();
        } else {
            if intentionToEdit {
                *lc = self.wireLinearCombinations[wireId as usize].clone();
            } else {
                *lc = RcCell::new(
                    self.wireLinearCombinations[wireId as usize]
                        .borrow()
                        .clone(),
                );
            }
        }
    }

    pub fn clean(&mut self) {
        for &wireId in &self.toClean {
            self.wireLinearCombinations[wireId as usize] =
                RcCell::new(LinearCombination::default());
        }
        self.toClean.clear();
    }

    pub fn addMulConstraint(&mut self, inputs: &[u32], outputs: &[u32]) {
        let (outputWireId, inWireId1, inWireId2) = (outputs[0], inputs[0], inputs[1]);
        let (mut l1, mut l2) = (lcp(), lcp());
        self.find(inWireId1, &mut l1, false);
        self.find(inWireId2, &mut l2, false);

        if let Some(&v) = self.variableMap.get(&outputWireId) {
            self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
                l1.borrow().clone(),
                l2.borrow().clone(),
                self.variables[v as usize].borrow().clone().into(),
                "Mul ..",
            );
        } else {
            self.variables.push(RcCell::new(Variable::from("mul out")));
            self.variableMap
                .insert(outputWireId, self.currentVariableIdx);
            self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
                l1.borrow().clone(),
                l2.borrow().clone(),
                self.variables[self.currentVariableIdx as usize]
                    .borrow()
                    .clone()
                    .into(),
                "Mul ..",
            );
            self.currentVariableIdx += 1;
        }
    }

    pub fn addXorConstraint(&mut self, inputs: &[u32], outputs: &[u32]) {
        let (outputWireId, inWireId1, inWireId2) = (outputs[0], inputs[0], inputs[1]);

        let (mut lp1, mut lp2) = (lcp(), lcp());
        self.find(inWireId1, &mut lp1, false);
        self.find(inWireId2, &mut lp2, false);
        let (l1, l2): (LinearCombination, LinearCombination) =
            (lp1.borrow().clone(), lp2.borrow().clone());

        if let Some(v) = self.variableMap.get(&outputWireId) {
            self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
                l1.clone() * 2,
                l2.clone(),
                l1.clone() + &l2
                    - &self.variables[self.variableMap[&outputWireId] as usize]
                        .borrow()
                        .clone(),
                "XOR ..",
            );
        } else {
            self.variables.push(RcCell::new(Variable::from("xor out")));
            self.variableMap
                .insert(outputWireId, self.currentVariableIdx);
            self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
                l1.clone() * 2,
                l2.clone(),
                l1.clone() + &l2
                    - &self.variables[self.currentVariableIdx as usize]
                        .borrow()
                        .clone(),
                "XOR ..",
            );
            self.currentVariableIdx += 1;
        }
    }

    pub fn addOrConstraint(&mut self, inputs: &[u32], outputs: &[u32]) {
        let (outputWireId, inWireId1, inWireId2) = (outputs[0], inputs[0], inputs[1]);

        let (mut lp1, mut lp2) = (lcp(), lcp());
        self.find(inWireId1, &mut lp1, false);
        self.find(inWireId2, &mut lp2, false);
        let (l1, l2): (LinearCombination, LinearCombination) =
            (lp1.borrow().clone(), lp2.borrow().clone());

        if let Some(&v) = self.variableMap.get(&outputWireId) {
            self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
                l1.clone(),
                l2.clone(),
                l1.clone() + &l2 - &self.variables[v as usize].borrow().clone(),
                "OR ..",
            );
        } else {
            self.variables.push(RcCell::new(Variable::from("or out")));
            self.variableMap
                .insert(outputWireId, self.currentVariableIdx);
            self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
                l1.clone(),
                l2.clone(),
                l1.clone() + &l2
                    - &self.variables[self.currentVariableIdx as usize]
                        .borrow()
                        .clone(),
                "OR ..",
            );
            self.currentVariableIdx += 1;
        }
    }

    pub fn addAssertionConstraint(&mut self, inputs: &[u32], outputs: &[u32]) {
        let (outputWireId, inWireId1, inWireId2) = (outputs[0], inputs[0], inputs[1]);

        let (mut lp1, mut lp2, mut lp3) = (lcp(), lcp(), lcp());
        self.find(inWireId1, &mut lp1, false);
        self.find(inWireId2, &mut lp2, false);
        self.find(outputWireId, &mut lp3, false);

        let (l1, l2, l3): (LinearCombination, LinearCombination, LinearCombination) = (
            lp1.borrow().clone(),
            lp2.borrow().clone(),
            lp3.borrow().clone(),
        );
        self.pb
            .as_ref()
            .unwrap()
            .borrow_mut()
            .addRank1Constraint(l1, l2, l3, "Assertion ..");
    }

    pub fn addSplitConstraint(&mut self, inputs: &[u32], outputs: &[u32], n: u16) {
        let inWireId = inputs[0];

        let mut l = lcp();
        self.find(inWireId, &mut l, false);

        let mut sum = lcp();
        let mut two_i = Fr::<default_ec_pp>::from("1");

        for i in 0..n {
            let bitWireId = outputs[i as usize];
            let mut vptr;
            if let Some(&v) = self.variableMap.get(&bitWireId) {
                vptr = self.variables[v as usize].clone();
            } else {
                self.variables.push(RcCell::new(Variable::from("bit out")));
                self.variableMap.insert(bitWireId, self.currentVariableIdx);
                vptr = self.variables[self.currentVariableIdx as usize].clone();
                self.currentVariableIdx += 1;
            }
            self.pb
                .as_ref()
                .unwrap()
                .borrow_mut()
                .enforceBooleanity(&vptr.borrow());
            let v = sum.borrow().clone()
                + &LinearTerm::new(
                    vptr.borrow().clone(),
                    FElem::from(R1P_Elem::froms(two_i.clone())),
                );
            *sum.borrow_mut() = v;
            two_i += &two_i.clone();
        }

        self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
            l.borrow().clone(),
            1.into(),
            sum.borrow().clone(),
            "Split Constraint",
        );
    }

    pub fn addNonzeroCheckConstraint(&mut self, inputs: &[u32], outputs: &[u32]) {
        // let mut auxConditionInverse_;
        let (mut outputWireId, mut inWireId) = (outputs[1], inputs[0]);
        let mut l = lcp();

        self.find(inWireId, &mut l, false);
        let mut vptr;
        if let Some(&v) = self.variableMap.get(&outputWireId) {
            vptr = self.variables[v as usize].clone();
        } else {
            self.variables
                .push(RcCell::new(Variable::from("zerop out")));
            self.variableMap
                .insert(outputWireId, self.currentVariableIdx);
            vptr = self.variables[self.currentVariableIdx as usize].clone();
            self.currentVariableIdx += 1;
        }
        self.variables
            .push(RcCell::new(Variable::from("zerop aux")));
        self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
            l.borrow().clone(),
            -vptr.borrow().clone() + 1,
            0.into(),
            "condition * not(output) = 0",
        );
        self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
            l.borrow().clone(),
            self.variables[self.currentVariableIdx as usize]
                .borrow()
                .clone()
                .into(),
            vptr.borrow().clone().into(),
            "condition * auxConditionInverse = output",
        );

        self.zeroPwires.push(l.clone());
        self.zeropMap.insert(outputWireId, self.currentVariableIdx);
        self.currentVariableIdx += 1;
    }

    pub fn handlePackOperation(&mut self, inputs: &[u32], outputs: &[u32], n: u16) {
        let outputWireId = outputs[0];

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of a pack operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }

        let mut sum = lcp();
        let mut bitWireId = inputs[0];

        self.find(bitWireId, &mut sum, true);
        let mut two_i = Fr::<default_ec_pp>::from("1");
        for i in 1..n {
            bitWireId = inputs[i as usize];
            let mut l = lcp();
            self.find(bitWireId, &mut l, false);
            two_i += two_i;
            let lt = (l.borrow().clone() * &FElem::from(R1P_Elem::froms(two_i.clone())));
            *sum.borrow_mut() += &lt;
        }
        self.wireLinearCombinations[outputWireId as usize] = sum;
    }

    pub fn handleAddition(&mut self, inputs: &[u32], outputs: &[u32]) {
        let (inWireId, outputWireId) = (inputs[0], outputs[0]);

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of an add operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }

        let (mut s, mut l) = (lcp(), lcp());
        self.find(inWireId, &mut l, true);
        s = l.clone();
        for &inWireId in &inputs[1..] {
            self.find(inWireId, &mut l, false);
            let ll = l.borrow().clone();
            *s.borrow_mut() += &ll;
        }
        self.wireLinearCombinations[outputWireId as usize] = s;
    }

    pub fn handleMulConst(&mut self, types: &str, inputs: &[u32], outputs: &[u32]) {
        let constStr = &types["const-mul-".len()..];
        let (outputWireId, inWireId) = (outputs[0], inputs[0]);

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of a const-mul operation was either defined before, or is declared directly as a circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }

        let mut l = lcp();
        self.find(inWireId, &mut l, true);
        self.wireLinearCombinations[outputWireId as usize] = l;
        let v = FElem::from(R1P_Elem::froms(
            read_field_element_from_hex(constStr).into(),
        ));
        *self.wireLinearCombinations[outputWireId as usize].borrow_mut() *= &v;
    }

    pub fn handleMulNegConst(&mut self, types: &str, inputs: &[u32], outputs: &[u32]) {
        let constStr = &types["const-mul-neg-".len()..];
        let (mut outputWireId, mut inWireId) = (outputs[0], inputs[0]);

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of a const-mul-neg operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }
        let mut l = lcp();
        self.find(inWireId, &mut l, true);

        self.wireLinearCombinations[outputWireId as usize] = l;
        *self.wireLinearCombinations[outputWireId as usize].borrow_mut() *= &FElem::from(
            R1P_Elem::froms(read_field_element_from_hex(constStr).into()),
        );
        *self.wireLinearCombinations[outputWireId as usize].borrow_mut() *=
            &FElem::from(R1P_Elem::froms(FieldT::from(-1))); //TODO: make shared FieldT constants
    }
}
