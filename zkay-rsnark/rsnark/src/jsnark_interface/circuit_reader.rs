#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

// use  Util;
// use  crate::gadgetlib2::integration;
// use  crate::gadgetlib2::adapters;
// use  common::profiling;
use crate::gadgetlib2::constraint::PrintOptions;
use crate::gadgetlib2::pp::Fp;
use crate::gadgetlib2::variable::{
    FElem, FElemInterface, FieldType, LinearCombination, LinearTerm, ProtoboardPtr, R1P_Elem,
    Variable, VariablePtr,
};
use crate::jsnark_interface::util::{readFieldElementFromHex, readIds};
use ff_curves::{Fr, default_ec_pp};
use ffec::common::profiling::{enter_block, leave_block, start_profiling};
use rccell::RcCell;

use std::{
    fmt::Debug,
    fs,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
    ops::{Add, Mul, Neg, Sub},
    process,
};

// use  <memory.h>
// use  <iostream>
// use  <sstream>
// use  <fstream>
// use  <list>
//
// use  <set>
// use  <map>
// use  <ctime>

// use  <termios.h>
// use  <unistd.h>
// use  <stdio.h>

// //#ifndef NO_PROCPS
// use  <proc/readproc.h>
// //#endif

//
// using namespace gadgetlib2;
// using namespace std;
use std::collections::HashMap;

type Wire = u32;

pub type FieldT = Fr<default_ec_pp>;
type LinearCombinationPtr = RcCell<LinearCombination>;
type WireMap = HashMap<Wire, u32>;

fn lcp() -> LinearCombinationPtr {
    RcCell::new(LinearCombination::default())
}
const ADD_OPCODE: u32 = 1;
const MUL_OPCODE: u32 = 2;
const SPLIT_OPCODE: u32 = 3;
const NONZEROCHECK_OPCODE: u32 = 4;
const PACK_OPCODE: u32 = 5;
const MULCONST_OPCODE: u32 = 6;
const XOR_OPCODE: u32 = 7;
const OR_OPCODE: u32 = 8;
const CONSTRAINT_OPCODE: u32 = 9;

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
    // pub fn  parseAndEval(arithFilepath:&str, inputsFilepath:&str);
    // pub fn  constructCircuit(const char*);  // Second Pass:
    // pub fn  mapValuesToProtoboard();

    // pub fn  find(unsigned int, LinearCombinationPtr&, bool intentionToEdit = false);
    // pub fn  clean();

    // pub fn  addMulConstraint(char*, char*);
    // pub fn  addXorConstraint(char*, char*);

    // pub fn  addOrConstraint(char*, char*);
    // pub fn  addAssertionConstraint(char*, char*);

    // pub fn  addSplitConstraint(char*, char*, unsigned short);
    // // pub fn  addPackConstraint(char*, char*, unsigned short);
    // pub fn  addNonzeroCheckConstraint(char*, char*);

    // pub fn  handleAddition(char*, char*);
    // pub fn  handlePackOperation(char*, char*, unsigned short);
    // pub fn  handleMulConst(char*, char*, char*);
    // pub fn  handleMulNegConst(char*, char*, char*);
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

        _self.parseAndEval(arithFilepath, inputsFilepath);
        _self.constructCircuit(arithFilepath);
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
    pub fn parseAndEval(&mut self, arithFilepath: &str, inputsFilepath: &str) {
        enter_block("Parsing and Evaluating the circuit", false);
        let mut arithfs = fs::read_to_string(arithFilepath);
        let mut inputfs = fs::read_to_string(inputsFilepath);

        let mut line;

        if arithfs.is_err() {
            println!("Unable to open circuit file {} \n", arithFilepath);
            process::exit(-1);
        }
        let Ok(arithfs) = arithfs else { return };
        let mut arithfs = arithfs.lines();
        line = arithfs.next().unwrap();
        let ret = scan_fmt!(line, "total {d}", u32);

        if ret.is_err() {
            println!("File Format Does not Match\n");
            process::exit(-1);
        }
        self.numWires = ret.unwrap();
        self.wireValues
            .resize(self.numWires as usize, FieldT::zero());
        self.wireUseCounters.resize(self.numWires as usize, 0);
        self.wireLinearCombinations.resize(
            self.numWires as usize,
            RcCell::new(LinearCombination::default()),
        );

        if inputfs.is_err() {
            println!("Unable to open input file {} \n", inputsFilepath);
            process::exit(-1);
        }
        let Ok(inputfs) = inputfs else { return };
        let mut inputfs = inputfs.lines();
        // let mut inputStr;
        for line in inputfs {
            if line.is_empty() {
                continue;
            }
            // let mut wireId;
            if let Ok((wireId, inputStr)) = scan_fmt!(line, "{} {}", u32, String) {
                self.wireValues[wireId as usize] = readFieldElementFromHex(&inputStr);
            } else {
                println!("Error in Input\n");
                process::exit(-1);
            }
        }

        if self.wireValues[0] != FieldT::one() {
            println!(
                ">> Warning: when using jsnark circuit generator, the first input wire (#0) must have the value of 1.\n"
            );
            println!(
                "\t If the circuit was generated using Pinocchio *without modification*, you can ignore this warning. Pinocchio uses a different indexing for the one-wire input. \n"
            );
        }

        // char types[200];
        let mut inputStr: &str;
        let mut outputStr: &str;
        let (mut numGateInputs, mut numGateOutputs) = (0, 0);

        // let mut wireId;

        let oneElement = FieldT::one();
        let zeroElement = FieldT::zero();
        let negOneElement = FieldT::from(-1);

        // i64 evalTime;
        // i64 begin, end;
        // evalTime = 0;

        // Parse the circuit: few lines were imported from Pinocchio's code.

        for line in arithfs {
            if line.is_empty() {
                continue;
            }

            if line.as_bytes()[0] == b'#' {
                continue;
            }
            if let Ok((wireId)) = scan_fmt!(line, "input {}", u32) {
                self.numInputs += 1;
                self.inputWireIds.push(wireId);
            } else if let Ok((wireId)) = scan_fmt!(line, "nizkinput {}", u32) {
                self.numNizkInputs += 1;
                self.nizkWireIds.push(wireId);
            } else if let Ok((wireId)) = scan_fmt!(line, "output {}", u32) {
                self.numOutputs += 1;
                self.outputWireIds.push(wireId);
                self.wireUseCounters[wireId as usize] += 1;
            } else if let Ok((types, numGateInputs, inputStr, numGateOutputs, outputStr)) = scan_fmt!(
                line,
                "{} in {} <{:/[^>]+/}> out {} <{:/[^>]+/}>",
                String,
                u32,
                String,
                u32,
                String
            ) {
                let mut iss_i = inputStr.lines();
                let mut inValues = vec![];
                let mut outWires = vec![];

                for s in inputStr.split_ascii_whitespace() {
                    let inWireId = s.parse::<u32>().unwrap() as usize;
                    self.wireUseCounters[inWireId as usize] += 1;
                    inValues.push(self.wireValues[inWireId]);
                }
                readIds(&outputStr, &mut outWires);

                let mut opcode = 0;
                let mut constant = FieldT::zero();
                match types.as_str() {
                    "add" => {
                        opcode = ADD_OPCODE;
                    }
                    "mul" => {
                        opcode = MUL_OPCODE;
                    }
                    "xor" => {
                        opcode = XOR_OPCODE;
                    }
                    "or" => {
                        opcode = OR_OPCODE;
                    }
                    "assert" => {
                        self.wireUseCounters[outWires[0] as usize] += 1;
                        opcode = CONSTRAINT_OPCODE;
                    }
                    "pack" => {
                        opcode = PACK_OPCODE;
                    }
                    "zerop" => {
                        opcode = NONZEROCHECK_OPCODE;
                    }
                    "split" => {
                        opcode = SPLIT_OPCODE;
                    }
                    _ if types.contains("const-mul-neg-") => {
                        opcode = MULCONST_OPCODE;
                        let constStr = &types["const-mul-neg-".len() - 1..];
                        constant = readFieldElementFromHex(constStr) * negOneElement;
                    }
                    _ if types.contains("const-mul-") => {
                        opcode = MULCONST_OPCODE;
                        let constStr = &types["const-mul-".len() - 1..];
                        constant = readFieldElementFromHex(constStr);
                    }
                    _ => {
                        println!("Error: unrecognized line: {line}\n");
                        panic!("0");
                    }
                }
                // TODO: separate evaluation from parsing completely to get accurate evaluation cost
                //	 Calling  get_nsec_time(); repetitively as in the old version adds much overhead
                // TODO 2: change circuit format to enable skipping some lines during evaluation
                //       Not all intermediate wire values need to be computed in this phase
                // TODO 3: change circuit format to make common constants defined once

                //begin = get_nsec_time();
                if opcode == ADD_OPCODE {
                    self.wireValues[outWires[0] as usize] =
                        inValues.iter().fold(Fp::default(), |s, c| s + c);
                } else if opcode == MUL_OPCODE {
                    self.wireValues[outWires[0] as usize] = inValues[0] * inValues[1];
                } else if opcode == XOR_OPCODE {
                    self.wireValues[outWires[0] as usize] = if inValues[0] == inValues[1] {
                        zeroElement
                    } else {
                        oneElement
                    };
                } else if opcode == OR_OPCODE {
                    self.wireValues[outWires[0] as usize] =
                        if inValues[0] == zeroElement && inValues[1] == zeroElement {
                            zeroElement
                        } else {
                            oneElement
                        };
                } else if opcode == NONZEROCHECK_OPCODE {
                    self.wireValues[outWires[1] as usize] = if inValues[0] == zeroElement {
                        zeroElement
                    } else {
                        oneElement
                    };
                } else if opcode == PACK_OPCODE {
                    let (mut sum, mut coeff) = (FieldT::zero(), FieldT::zero());
                    let mut two = oneElement;
                    for &v in &inValues {
                        sum += two * v;
                        two += two;
                    }
                    self.wireValues[outWires[0] as usize] = sum;
                } else if opcode == SPLIT_OPCODE {
                    let size = outWires.len();
                    let inVal = FElem::from(R1P_Elem::new(inValues[0].clone()));
                    for i in 0..size {
                        self.wireValues[outWires[i] as usize] =
                            inVal.getBits(i as u32, &FieldType::R1P).into();
                    }
                } else if opcode == MULCONST_OPCODE {
                    self.wireValues[outWires[0] as usize] = constant * inValues[0];
                }
                //end =  get_nsec_time();
                //evalTime += (end - begin);
            } else {
                println!("Error: unrecognized line: {line}\n");
                panic!("0");
            }
        }

        // println!("\t Evaluation Done in %lf seconds \n", (double) (evalTime) * 1e-9);
        leave_block("Parsing and Evaluating the circuit", false);
    }

    pub fn constructCircuit(&mut self, arithFilepath: &str) {
        println!("Translating Constraints ... ");

        // //#ifndef NO_PROCPS
        // struct proc_t usage1, usage2;
        // look_up_our_self(&usage1);
        //     //#endif

        let (mut currentVariableIdx, mut currentLinearCombinationIdx) = (0, 0);
        for i in 0..self.numInputs as usize {
            self.variables.push(RcCell::new(Variable::new("input")));
            self.variableMap
                .insert(self.inputWireIds[i], currentVariableIdx);
            currentVariableIdx += 1;
        }
        for i in 0..self.numOutputs as usize {
            self.variables.push(RcCell::new(Variable::new("output")));
            self.variableMap
                .insert(self.outputWireIds[i], currentVariableIdx);
            currentVariableIdx += 1;
        }
        for i in 0..self.numNizkInputs as usize {
            self.variables
                .push(RcCell::new(Variable::new("nizk input")));
            self.variableMap
                .insert(self.nizkWireIds[i], currentVariableIdx);
            currentVariableIdx += 1;
        }

        // char types[200];
        // inputStr:&str;
        // outputStr:&str;
        // string line;
        let (mut numGateInputs, mut numGateOutputs) = (0, 0);

        let mut ifs2 = fs::read_to_string(arithFilepath);

        if ifs2.is_err() {
            println!("Unable to open circuit file:\n");
            process::exit(5);
        }

        // Parse the circuit: few lines were imported from Pinocchio's code.
        let Ok(ifs2) = ifs2 else { return };
        let mut ifs2 = ifs2.lines();
        let mut line = ifs2.next().unwrap();
        let Ok(numWires) = scan_fmt!(line, "total {}", i32) else {
            eprintln!("=======================");
            return;
        };

        let mut lineCount = 0;
        for line in ifs2 {
            lineCount += 1;
            //		if lineCount % 100000 == 0 {
            //			println!("At Line:: {}\n", lineCount);
            //		}

            if line.is_empty() {
                continue;
            }

            if let Ok((types, numGateInputs, inputStr, numGateOutputs, outputStr)) = scan_fmt!(
                line,
                "{} in {} <{:/[^>]+/}> out {} <{:/[^>]+/}>",
                String,
                i32,
                String,
                i32,
                String
            ) {
                match types.as_str() {
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
                    _ if types.contains("const-mul-neg-") => {
                        assert!(numGateInputs == 1 && numGateOutputs == 1);
                        self.handleMulNegConst(&types, &inputStr, &outputStr);
                    }
                    _ if types.contains("const-mul-") => {
                        assert!(numGateInputs == 1 && numGateOutputs == 1);
                        self.handleMulConst(&types, &inputStr, &outputStr);
                    }
                    "zerop" => {
                        assert!(numGateInputs == 1 && numGateOutputs == 2);
                        self.addNonzeroCheckConstraint(&inputStr, &outputStr);
                    }
                    _ if types.contains("split") => {
                        assert!(numGateInputs == 1);
                        self.addSplitConstraint(&inputStr, &outputStr, numGateOutputs as u16);
                    }
                    _ if types.contains("pack") => {
                        assert!(numGateOutputs == 1);
                        // addPackConstraint(inputStr, outputStr, numGateInputs);
                        self.handlePackOperation(&inputStr, &outputStr, numGateInputs as u16);
                    }
                    _ => {}
                }
            } else {
                //			assert!(0);
            }

            self.clean();
        }

        println!("\tConstraint translation done\n");

        // //#ifndef NO_PROCPS
        // look_up_our_self(&usage2);
        // u64 diff = usage2.vsize - usage1.vsize;
        // println!("\tMemory usage for constraint translation: %lu MB\n", diff >> 20);
        //     //#endif
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
                R1P_Elem::new(self.wireValues[wireId as usize].clone()).into();
            if let Some(&z) = self.zeropMap.get(&wireId) {
                let l = self.zeroPwires[zeropGateIndex].clone();
                zeropGateIndex += 1;
                if self.pb.as_ref().unwrap().borrow().val_lc(&l.borrow())
                    == FElem::from(R1P_Elem::new(FieldT::zero()))
                {
                    *self
                        .pb
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .val(&self.variables[z as usize].borrow()) =
                        R1P_Elem::new(FieldT::zero()).into();
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
            != LinearCombination::default()
        {
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

    pub fn addMulConstraint(&mut self, inputStr: &str, outputStr: &str) {
        let (outputWireId, inWireId1, inWireId2);

        let mut iss_i = inputStr.lines();
        inWireId1 = iss_i.next().unwrap().parse::<u32>().unwrap();
        inWireId2 = iss_i.next().unwrap().parse::<u32>().unwrap();
        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

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
            self.variables.push(RcCell::new(Variable::new("mul out")));
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

    pub fn addXorConstraint(&mut self, inputStr: &str, outputStr: &str) {
        let (outputWireId, inWireId1, inWireId2);

        let mut iss_i = inputStr.lines();
        inWireId1 = iss_i.next().unwrap().parse::<u32>().unwrap();
        inWireId2 = iss_i.next().unwrap().parse::<u32>().unwrap();
        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

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
            self.variables.push(RcCell::new(Variable::new("xor out")));
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

    pub fn addOrConstraint(&mut self, inputStr: &str, outputStr: &str) {
        let (outputWireId, inWireId1, inWireId2);

        let mut iss_i = inputStr.lines();
        inWireId1 = iss_i.next().unwrap().parse::<u32>().unwrap();
        inWireId2 = iss_i.next().unwrap().parse::<u32>().unwrap();
        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

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
            self.variables.push(RcCell::new(Variable::new("or out")));
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

    pub fn addAssertionConstraint(&mut self, inputStr: &str, outputStr: &str) {
        let (outputWireId, inWireId1, inWireId2);

        let mut iss_i = inputStr.lines();
        inWireId1 = iss_i.next().unwrap().parse::<u32>().unwrap();
        inWireId2 = iss_i.next().unwrap().parse::<u32>().unwrap();
        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

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

    pub fn addSplitConstraint(&mut self, inputStr: &str, outputStr: &str, n: u16) {
        let mut iss_i = inputStr.lines();
        let inWireId = iss_i.next().unwrap().parse::<u32>().unwrap();

        let mut l = lcp();
        self.find(inWireId, &mut l, false);

        let mut iss_o = outputStr.lines();

        let mut sum = lcp();
        let mut two_i = Fr::<default_ec_pp>::from("1");

        for i in 0..n {
            let bitWireId = iss_o.next().unwrap().parse::<u32>().unwrap();
            let mut vptr;
            if let Some(&v) = self.variableMap.get(&bitWireId) {
                vptr = self.variables[v as usize].clone();
            } else {
                self.variables.push(RcCell::new(Variable::new("bit out")));
                self.variableMap.insert(bitWireId, self.currentVariableIdx);
                vptr = self.variables[self.currentVariableIdx as usize].clone();
                self.currentVariableIdx += 1;
            }
            self.pb
                .as_ref()
                .unwrap()
                .borrow_mut()
                .enforceBooleanity(&vptr.borrow());
            *sum.borrow_mut() = sum.borrow().clone()
                + &LinearTerm::new(
                    vptr.borrow().clone(),
                    FElem::from(R1P_Elem::new(two_i.clone())),
                );
            two_i += &two_i.clone();
        }

        self.pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
            l.borrow().clone(),
            1.into(),
            sum.borrow().clone(),
            "Split Constraint",
        );
    }

    pub fn addNonzeroCheckConstraint(&mut self, inputStr: &str, outputStr: &str) {
        // let mut auxConditionInverse_;
        let (mut outputWireId, mut inWireId) = (0, 0);

        let mut iss_i = inputStr.lines();
        inWireId = iss_i.next().unwrap().parse::<u32>().unwrap();
        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();
        let mut l = lcp();

        self.find(inWireId, &mut l, false);
        let mut vptr;
        if let Some(&v) = self.variableMap.get(&outputWireId) {
            vptr = self.variables[v as usize].clone();
        } else {
            self.variables.push(RcCell::new(Variable::new("zerop out")));
            self.variableMap
                .insert(outputWireId, self.currentVariableIdx);
            vptr = self.variables[self.currentVariableIdx as usize].clone();
            self.currentVariableIdx += 1;
        }
        self.variables.push(RcCell::new(Variable::new("zerop aux")));
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

    pub fn handlePackOperation(&mut self, inputStr: &str, outputStr: &str, n: u16) {
        let mut iss_o = outputStr.lines();
        let outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of a pack operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }

        let mut iss_i = inputStr.lines();
        let mut sum = lcp();
        let mut bitWireId = iss_i.next().unwrap().parse::<u32>().unwrap();

        self.find(bitWireId, &mut sum, true);
        let mut two_i = Fr::<default_ec_pp>::from("1");
        for i in 1..n {
            bitWireId = iss_i.next().unwrap().parse::<u32>().unwrap();
            let mut l = lcp();
            self.find(bitWireId, &mut l, false);
            two_i += two_i;
            *sum.borrow_mut() += &(l.borrow().clone() * &FElem::from(R1P_Elem::new(two_i.clone())));
        }
        self.wireLinearCombinations[outputWireId as usize] = sum;
    }

    pub fn handleAddition(&mut self, inputStr: &str, outputStr: &str) {
        let (inWireId, outputWireId);
        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of an add operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }

        let mut iss_i = inputStr.lines();
        let (mut s, mut l) = (lcp(), lcp());
        inWireId = iss_i.next().unwrap().parse::<u32>().unwrap();
        self.find(inWireId, &mut l, true);
        s = l.clone();
        for inWireId in iss_i {
            self.find(inWireId.parse::<u32>().unwrap(), &mut l, false);
            *s.borrow_mut() += &l.borrow();
        }
        self.wireLinearCombinations[outputWireId as usize] = s;
    }

    pub fn handleMulConst(&mut self, types: &str, inputStr: &str, outputStr: &str) {
        let constStr = &types["const-mul-".len() - 1..];
        let (outputWireId, inWireId);

        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of a const-mul operation was either defined before, or is declared directly as a circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }

        let mut iss_i = inputStr.lines();
        inWireId = iss_i.next().unwrap().parse::<u32>().unwrap();
        let mut l = lcp();
        self.find(inWireId, &mut l, true);
        self.wireLinearCombinations[outputWireId as usize] = l;
        *self.wireLinearCombinations[outputWireId as usize].borrow_mut() *=
            &FElem::from(R1P_Elem::new(readFieldElementFromHex(constStr)));
    }

    pub fn handleMulNegConst(&mut self, types: &str, inputStr: &str, outputStr: &str) {
        let constStr = &types["const-mul-neg-".len() - 1..];
        let (mut outputWireId, mut inWireId) = (0, 0);
        let mut iss_o = outputStr.lines();
        outputWireId = iss_o.next().unwrap().parse::<u32>().unwrap();

        if self.variableMap.contains_key(&outputWireId) {
            println!(
                "An output of a const-mul-neg operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n"
            );
            println!(
                "\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n"
            );
            process::exit(-1);
        }

        let mut iss_i = inputStr.lines();
        inWireId = iss_i.next().unwrap().parse::<u32>().unwrap();

        let mut l = lcp();
        self.find(inWireId, &mut l, true);

        self.wireLinearCombinations[outputWireId as usize] = l;
        *self.wireLinearCombinations[outputWireId as usize].borrow_mut() *=
            &FElem::from(R1P_Elem::new(readFieldElementFromHex(constStr)));
        *self.wireLinearCombinations[outputWireId as usize].borrow_mut() *=
            &FElem::from(R1P_Elem::new(FieldT::from(-1))); //TODO: make shared FieldT constants
    }
}
