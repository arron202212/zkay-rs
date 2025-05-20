#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::assert_basic_op::AssertBasicOp;
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::mul_basic_op::MulBasicOp;
use crate::circuit::operations::wire_label_instruction::WireLabelInstruction;
use crate::circuit::operations::wire_label_instruction::LabelType;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::constant_wire::ConstantWire;
 use crate::util::{run_command::run_command,util::{Util,BigInteger}};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
//  ConcurrentHashMap<Long, CircuitGenerator> activeCircuitGenerators = new ConcurrentHashMap<>();
// 	  CircuitGenerator instance;

use std::hash::{DefaultHasher, Hash, Hasher};
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash,PartialEq)]
pub struct CircuitGenerator {
pub currentWireId: i32,
pub evaluationQueue: HashMap<Box<dyn Instruction>, Box<dyn Instruction>>,

pub zeroWire: WireType,
pub oneWire: WireType,

pub inWires: Vec<Option<WireType>>,
pub outWires: Vec<Option<WireType>>,
pub proverWitnessWires: Vec<Option<WireType>>,

pub circuitName: String,

pub knownConstantWires: HashMap<BigInteger, WireType>,

pub numOfConstraints: i32,
pub circuitEvaluator: CircuitEvaluator,
}
pub trait CGConfig {
    fn buildCircuit();
    fn generateSampleInput(evaluator: CircuitEvaluator);
}
impl CircuitGenerator {

pub fn getActiveCircuitGenerator() -> eyre::Result<CircuitGenerator> {
    // if !Configs.get().unwrap().runningMultiGenerators {
    //     return Ok(instance);
    // }

    // let threadId = Thread.currentThread().getId();
    // let currentGenerator = activeCircuitGenerators.get(threadId);

    // currentGenerator.ok_or(eyre::eyre!(
    //     "The current thread does not have any active circuit generators"
    // ))
    eyre::eyre!(
        "The current thread does not have any active circuit generators"
    )
}
    pub fn new(circuitName: String) -> Self {
        if Configs.get().unwrap().running_multi_generators {
            // activeCircuitGenerators.put(Thread.currentThread().getId(), this);
        }
        Self {
            circuitName,
            inWires: vec![],
            outWires: vec![],
            proverWitnessWires: vec![],
            evaluationQueue: HashMap::new(),
            knownConstantWires: HashMap::new(),
            currentWireId: 0,
            numOfConstraints: 0,
        }
    }

    pub fn generateCircuit(&self,) {
        println!("Running Circuit Generator for <  {}  >",self.circuitName);

        self.initCircuitConstruction();
       self.buildCircuit();

        println!(
            "Circuit Generation Done for < {} > \n \t Total Number of Constraints : {} \n \t Total Number of Wires : {}",self.circuitName,
            self.getNumOfConstraints(),
            self.getNumWires()
        );
    }

    pub fn getName(&self,) -> String {
        return self.circuitName.clone();
    }

    pub fn createInputWire(&mut self,desc: Vec<String>) -> WireType {
        let newInputWire = Wiretype::Variable(VariableWire::new(self.currentWireId));
        self.currentWireId += 1;
        self.addToEvaluationQueue(WireLabelInstruction::new(LabelType::input, newInputWire, desc));
        self.inWires.push(newInputWire);
        return newInputWire;
    }

    pub fn createInputWireArray(&self,n: i32, desc: Vec<String>) -> Vec<Option<WireType>> {
        let mut list = vec![None; n];
        for i in 0..n {
                list[i] = self.createInputWire(desc.get(0).map_or_else(||String::new(),|d|d.clone()+ " " + &i.to_string()) );
        }
        return list;
    }

    pub fn createLongElementInput(&self,totalBitwidth: i32, desc: Vec<String>) -> LongElement {
        let numWires = (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as i32;
        let w = self.createInputWireArray(numWires, desc);
        let bitwidths = vec![LongElement::CHUNK_BITWIDTH; numWires];
        if numWires * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = totalBitwidth % LongElement::CHUNK_BITWIDTH;
        }
        return LongElement::new(w, bitwidths);
    }

    pub fn createLongElementProverWitness(&self,totalBitwidth: i32, desc: Vec<String>) -> LongElement {
        let numWires = (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as i32;
        let w = self.createProverWitnessWireArray(numWires, desc);
        let bitwidths = vec![LongElement::CHUNK_BITWIDTH; numWires];
        if numWires * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = totalBitwidth % LongElement::CHUNK_BITWIDTH;
        }
        return LongElement::new(w, bitwidths);
    }

    pub fn createProverWitnessWire(&mut self,desc: Vec<String>) -> WireType {
        let wire = WireType::Variable(VariableWire::new(self.currentWireId));
        self.currentWireId += 1;
        self.addToEvaluationQueue(WireLabelInstruction::new(LabelType::nizkinput, wire, desc));
        self.proverWitnessWires.push(wire);
        return wire;
    }

    pub fn createProverWitnessWireArray(&self,n: usize, desc: Vec<String>) -> Vec<Option<WireType>> {
        let mut ws = vec![WireType::default(); n];
        for k in 0..n {
        ws[k] = self.createProverWitnessWire(desc.get(0).map_or_else(||String::new(),|d|d.clone()+ " " + &k.to_string()) );
        }
        return ws;
    }

    pub fn generateZeroWireArray(&self,n: i32) -> Vec<Option<WireType>> {
        let zeroWires = vec![Some(self.zeroWire.clone()); n];
        return zeroWires;
    }

    pub fn generateOneWireArray(&self,n: i32) -> Vec<Option<WireType>> {
        let oneWires = vec![Some(self.oneWire.clone());n];
        return oneWires;
    }

    pub fn makeOutput(&self,wire: WireType, desc: Vec<String>) -> WireType {
        let outputWire = wire;
        if !(wire.instance_of("VariableWire") || wire.instance_of("VariableBitWire"))
            || self.inWires.contains(wire)
        {
            wire.packIfNeeded();
            outputWire = self.makeVariable(wire, desc);
        } else if self.inWires.contains(wire) || self.proverWitnessWires.contains(wire) {
            outputWire = self.makeVariable(wire, desc);
        } else {
            wire.packIfNeeded();
        }

        self.outWires.push(outputWire.clone());
        self.addToEvaluationQueue(WireLabelInstruction::new(LabelType::output, outputWire, desc));
        return outputWire;
    }

    fn makeVariable(&self,wire: WireType, desc: Vec<String>) -> WireType {
        let outputWire = WireType::Variable(VariableWire::new(self.currentWireId));
        self.currentWireId += 1;
        let op = new_mul(wire, self.oneWire.clone(), outputWire, desc);
        let cachedOutputs = self.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.currentWireId -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        outputWire
    }

    pub fn makeOutputArray(&self,wires: Vec<Option<WireType>>, desc: Vec<String>) -> Vec<Option<WireType>> {
        let outs = vec![None; wires.len()];
        for i in 0..wires.len() {
                outs[i] = makeOutput(wires[i], desc.get(0).map_or_else(||String::new(),|d|d.clone()+ "[" + &i.to_string() + "]") );
        }
        return outs;
    }

    pub fn addDebugInstruction(&self,w: WireType, desc: Vec<String>) {
        w.packIfNeeded();
        self.addToEvaluationQueue(WireLabelInstruction::new(LabelType::debug, w, desc));
    }

    pub fn addDebugInstructiona(&self,wires: Vec<Option<WireType>>, desc: Vec<String>) {
        for i in 0..wires.len() {
            wires[i].packIfNeeded();
            self.addToEvaluationQueue(WireLabelInstruction::new(
                LabelType::debug,
                wires[i],
                desc.get(0).map_or_else(||String::new(),|d|d.clone()+ " - " + &i.to_string())
               ,
            ));
        }
    }

    pub fn writeCircuitFile(&self,) {
        let printWriter = File::create(getName() + ".arith");
        write!(printWriter, "total {}", self.currentWireId );
        for e in self.evaluationQueue.keys() {
            if e.doneWithinCircuit() {
                write!(printWriter, "{e} \n");
            }
        }
    }

    pub fn printCircuit(&self,) {
        for e in self.evaluationQueue.keys() {
            if e.doneWithinCircuit() {
                println!("{e}");
            }
        }
    }

    fn initCircuitConstruction(&self,) {
        let oneWire = Wiretype::Constant(ConstantWire::new(self.currentWireId, Util::one()));
self.currentWireId += 1;
        self.knownConstantWires.insert(Util::one(), oneWire);
        self.addToEvaluationQueue(WireLabelInstruction::new(
            LabelType::input,
            oneWire,
            "The one-input wire.".to_owned(),
        ));
        self.inWires.push(oneWire);
        self.zeroWire = oneWire.mul(0);
    }

    pub fn createConstantWire(&self,x: BigInteger, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().mul(x, desc);
    }

    pub fn createConstantWireArray(&self,a: Vec<BigInteger>, desc: Vec<String>) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = self.createConstantWire(a[i], desc);
        }
        return w;
    }

    pub fn createConstantWirei(&self,x: i64, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().mul(x, desc);
    }

    pub fn createConstantWireArrayi(&self,a: Vec<i64>, desc: Vec<String>) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = self.createConstantWire(a[i], desc);
        }
        return w;
    }

    pub fn createNegConstantWire(&self,x: BigInteger, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().mul(x.neg(), desc);
    }

    pub fn createNegConstantWirei(&self,x: i64, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().mul(-x, desc);
    }

    /**
     * Use to support computation for prover witness values outside of the
     * circuit. See Mod_Gadget and Field_Division gadgets for examples.
     *
     * @param instruction
     */
    pub fn specifyProverWitnessComputation(&self,instruction: impl Instruction) {
       self.addToEvaluationQueue(instruction);
    }

    pub fn getZeroWire(&self,) -> WireType {
        return self.zeroWire.clone();
    }

    pub fn getOneWire(&self,) -> WireType {
        return self.oneWire.clone();
    }

    pub fn getEvaluationQueue(&self,) -> HashMap<Box<dyn Instruction>, Box<dyn Instruction>> {
        return self.evaluationQueue.clone();
    }

    pub fn getNumWires(&self,) -> i32 {
        return self.currentWireId;
    }

    pub fn addToEvaluationQueue(&self,e: impl Instruction) -> Option<Vec<Option<WireType>>> {
        let existingInstruction = self.evaluationQueue.get(e);
        self.evaluationQueue.entry(e).or_insert(e);
        if existingInstruction.is_none() {
            if e.instance_of("BasicOp") {
                self.numOfConstraints += (e).getNumMulGates();
            }
            return None; // returning null means we have not seen this instruction before
        }

        if existingInstruction.instance_of("BasicOp"){
            return existingInstruction.getOutputs();
        } else {
            return None; // have seen this instruction before, but can't de-duplicate
        }
    }

    pub fn printState(&self,message: String) {
        println!("\nGenerator State @ {message}");
        println!("\tCurrent Number of Multiplication Gates  .  {}\n",self.numOfConstraints);
    }

    pub fn getNumOfConstraints(&self,) -> i32 {
        return self.numOfConstraints;
    }

    pub fn getInWires(&self,) -> Vec<Option<WireType>> {
        return self.inWires.clone();
    }

    pub fn getOutWires(&self,) -> Vec<Option<WireType>> {
        return self.outWires.clone();
    }

    pub fn getProverWitnessWires(&self,) -> Vec<Option<WireType>> {
        return self.proverWitnessWires.clone();
    }

    /**
     * Asserts an r1cs constraint. w1*w2 = w3
     *
     */
    pub fn addAssertion(&self,w1: WireType, w2: WireType, w3: WireType, desc: Vec<String>) {
        if w1.instance_of("ConstantWire") && w2.instance_of("ConstantWire") && w3.instance_of("ConstantWire"){
            let const1 = (w1).getConstant();
            let const2 = (w2).getConstant();
            let const3 = (w3).getConstant();
            assert!(
                const3.equals(const1.mul(const2).modulo(Configs.get().unwrap().field_prime)),
                "Assertion failed on the provided constant wires .. "
            );
        } else {
            w1.packIfNeeded();
            w2.packIfNeeded();
            w3.packIfNeeded();
            let op = AssertBasicOp::new(w1, w2, w3, desc);
            self.addToEvaluationQueue(op);
        }
    }

    pub fn addZeroAssertion(&self,w: WireType, desc: Vec<String>) {
        self.addAssertion(w, self.oneWire.clone(), self.zeroWire.clone(), desc);
    }

    pub fn addOneAssertion(&self,w: WireType, desc: Vec<String>) {
        self.addAssertion(w, self.oneWire.clone(), self.oneWire.clone(), desc);
    }

    pub fn addBinaryAssertion(&self,w: WireType, desc: Vec<String>) {
        let inv = w.invAsBit(desc.clone());
        self.addAssertion(w, inv, self.zeroWire.clone(), desc);
    }

    pub fn addEqualityAssertion(&self,w1: WireType, w2: WireType, desc: Vec<String>) {
        if w1 != w2 {
            self.addAssertion(w1, self.oneWire.clone(), w2, desc);
        }
    }

    pub fn addEqualityAssertionb(&self,w1: WireType, b: BigInteger, desc: Vec<String>) {
        self.addAssertion(w1, self.oneWire, createConstantWire(b, desc), desc);
    }

    pub fn evalCircuit(&self,) {
        self.circuitEvaluator = CircuitEvaluator::new(self);
        self.generateSampleInput(self.circuitEvaluator);
        self.circuitEvaluator.evaluate();
    }

    pub fn prepFiles(&self,) {
        self.writeCircuitFile();
        assert!(
            self.circuitEvaluator.is_some(),
            "evalCircuit() must be called before prepFiles()"
        );
        self.circuitEvaluator.writeInputFile();
    }

    pub fn runLibsnark(&self,) {
        let p = run_command(vec![
            Configs.get().unwrap().libsnark_exec.clone(),
            self.circuitName.clone() + ".arith",
            self.circuitName.clone() + ".in",
        ]);
        println!(
            "\n-----------------------------------RUNNING LIBSNARK -----------------------------------------"
        );
        let mut line;
        let input = BufReader::new(p.getInputStream());
        let buf = String::new();
        for line in input.lines() {
            buf.push_str(&(line + "\n"));
        }
        println!("{buf}");
    }

    pub fn getCircuitEvaluator(&self,) -> CircuitEvaluator {
        assert!(
            self.circuitEvaluator.is_some(),
            "evalCircuit() must be called before getCircuitEvaluator()"
        );

        return self.circuitEvaluator;
    }
}
