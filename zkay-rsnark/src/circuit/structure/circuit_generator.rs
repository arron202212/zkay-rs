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
 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct CircuitGenerator {
    currentWireId: i32,
    evaluationQueue: HashMap<Box<dyn Instruction>, Box<dyn Instruction>>,

    zeroWire: WireType,
    oneWire: WireType,

    inWires: Vec<WireType>,
    outWires: Vec<WireType>,
    proverWitnessWires: Vec<WireType>,

    circuitName: String,

    knownConstantWires: HashMap<BigInteger, WireType>,

    numOfConstraints: i32,
    circuitEvaluator: CircuitEvaluator,
}
pub trait CGConfig {
    fn buildCircuit();
    fn generateSampleInput(evaluator: CircuitEvaluator);
}
impl CircuitGenerator {
    pub fn new(circuitName: String) -> Self {
        if Configs.get().unwrap().runningMultiGenerators {
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

        initCircuitConstruction();
        buildCircuit();

        println!(
            "Circuit Generation Done for < {} > \n \t Total Number of Constraints : {} \n \t Total Number of Wires : {}",self.circuitName,
            getNumOfConstraints(),
            getNumWires()
        );
    }

    pub fn getName(&self,) -> String {
        return self.circuitName.clone();
    }

    pub fn createInputWire(&self,desc: Vec<String>) -> WireType {
        let newInputWire = VariableWire::new(self.currentWireId += 1);
        addToEvaluationQueue(WireLabelInstruction::new(LabelType::input, newInputWire, desc));
        self.inWires.add(newInputWire);
        return newInputWire;
    }

    pub fn createInputWireArray(&self,n: i32, desc: Vec<String>) -> Vec<WireType> {
        let mut list = vec![WireType::default(); n];
        for i in 0..n {
            if desc.length == 0 {
                list[i] = createInputWire("");
            } else {
                list[i] = createInputWire(desc[0] + " " + i);
            }
        }
        return list;
    }

    pub fn createLongElementInput(&self,totalBitwidth: i32, desc: Vec<String>) -> LongElement {
        let numWires = (totalBitwidth * 1.0 / LongElement::CHUNK_BITWIDTH).ceil() as i32;
        let w = createInputWireArray(numWires, desc);
        let bitwidths = vec![LongElement::CHUNK_BITWIDTH; numWires];
        if numWires * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = totalBitwidth % LongElement::CHUNK_BITWIDTH;
        }
        return LongElement::new(w, bitwidths);
    }

    pub fn createLongElementProverWitness(&self,totalBitwidth: i32, desc: Vec<String>) -> LongElement {
        let numWires = (totalBitwidth * 1.0 / LongElement::CHUNK_BITWIDTH).ceil() as i32;
        let w = createProverWitnessWireArray(numWires, desc);
        let bitwidths = vec![LongElement::CHUNK_BITWIDTH; numWires];
        if numWires * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = totalBitwidth % LongElement::CHUNK_BITWIDTH;
        }
        return LongElement::new(w, bitwidths);
    }

    pub fn createProverWitnessWire(&self,desc: Vec<String>) -> WireType {
        let wire = VariableWire::new(self.currentWireId);
        self.currentWireId += 1;
        addToEvaluationQueue(WireLabelInstruction::new(LabelType::nizkinput, wire, desc));
        self.proverWitnessWires.add(wire);
        return wire;
    }

    pub fn createProverWitnessWireArray(&self,n: i32, desc: Vec<String>) -> Vec<WireType> {
        let mut ws = vec![WireType::default(); n];
        for k in 0..n {
            if desc.length == 0 {
                ws[k] = createProverWitnessWire("");
            } else {
                ws[k] = createProverWitnessWire(desc[0] + " " + k);
            }
        }
        return ws;
    }

    pub fn generateZeroWireArray(&self,n: i32) -> Vec<WireType> {
        let zeroWires = vec![self.zeroWire; n];
        return zeroWires;
    }

    pub fn generateOneWireArray(&self,n: i32) -> Vec<WireType> {
        let oneWires = vec![self.oneWire.clone();n];
        return oneWires;
    }

    pub fn makeOutput(&self,wire: WireType, desc: Vec<String>) -> WireType {
        let outputWire = wire;
        if !(wire.instanceof("VariableWire") || wire.instanceof("VariableBitWire"))
            || self.inWires.contains(wire)
        {
            wire.packIfNeeded();
            outputWire = makeVariable(wire, desc);
        } else if self.inWires.contains(wire) || self.proverWitnessWires.contains(wire) {
            outputWire = makeVariable(wire, desc);
        } else {
            wire.packIfNeeded();
        }

        self.outWires.add(outputWire);
        addToEvaluationQueue(WireLabelInstruction::new(LabelType::output, outputWire, desc));
        return outputWire;
    }

    fn makeVariable(&self,wire: WireType, desc: Vec<String>) -> WireType {
        let outputWire = VariableWire::new(self.currentWireId);
        self.currentWireId += 1;
        let op = MulBasicOp::new(wire, self.oneWire, outputWire, desc);
        let cachedOutputs = addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        outputWire
    }

    pub fn makeOutputArray(&self,wires: Vec<WireType>, desc: Vec<String>) -> Vec<WireType> {
        let outs = vec![WireType::default(); wires.length];
        for i in 0..wires.length {
            if desc.length == 0 {
                outs[i] = makeOutput(wires[i], "");
            } else {
                outs[i] = makeOutput(wires[i], desc[0] + "[" + i + "]");
            }
        }
        return outs;
    }

    pub fn addDebugInstruction(&self,w: WireType, desc: Vec<String>) {
        w.packIfNeeded();
        addToEvaluationQueue(WireLabelInstruction::new(LabelType::debug, w, desc));
    }

    pub fn addDebugInstructiona(&self,wires: Vec<WireType>, desc: Vec<String>) {
        for i in 0..wires.len() {
            wires[i].packIfNeeded();
            addToEvaluationQueue(WireLabelInstruction::new(
                LabelType::debug,
                wires[i],
                if desc.len() > 0 {
                    desc[0].to_owned() + " - " + &i.to_string()
                } else {
                    ""
                },
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
        let oneWire = ConstantWire::new(self.currentWireId, Util::one());
self.currentWireId += 1;
        self.knownConstantWires.put(Util::one(), oneWire);
        addToEvaluationQueue(WireLabelInstruction::new(
            LabelType::input,
            oneWire,
            "The one-input wire.",
        ));
        self.inWires.add(oneWire);
        self.zeroWire = oneWire.mul(0);
    }

    pub fn createConstantWire(&self,x: BigInteger, desc: Vec<String>) -> WireType {
        return self.oneWire.mul(x, desc);
    }

    pub fn createConstantWireArray(&self,a: Vec<BigInteger>, desc: Vec<String>) -> Vec<WireType> {
        let mut w = vec![WireType::default(); a.length];
        for i in 0..a.length {
            w[i] = createConstantWire(a[i], desc);
        }
        return w;
    }

    pub fn createConstantWirei(&self,x: i64, desc: Vec<String>) -> WireType {
        return self.oneWire.mul(x, desc);
    }

    pub fn createConstantWireArrayi(&self,a: Vec<i64>, desc: Vec<String>) -> Vec<WireType> {
        let mut w = vec![WireType::default(); a.length];
        for i in 0..a.length {
            w[i] = createConstantWire(a[i], desc);
        }
        return w;
    }

    pub fn createNegConstantWire(&self,x: BigInteger, desc: Vec<String>) -> WireType {
        return self.oneWire.mul(x.negate(), desc);
    }

    pub fn createNegConstantWirei(&self,x: i64, desc: Vec<String>) -> WireType {
        return self.oneWire.mul(-x, desc);
    }

    /**
     * Use to support computation for prover witness values outside of the
     * circuit. See Mod_Gadget and Field_Division gadgets for examples.
     *
     * @param instruction
     */
    pub fn specifyProverWitnessComputation(&self,instruction: impl Instruction) {
        addToEvaluationQueue(instruction);
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

    pub fn addToEvaluationQueue(&self,e: impl Instruction) -> Option<Vec<WireType>> {
        let existingInstruction = self.evaluationQueue.get(e);
        self.evaluationQueue.entry(e).or_insert(e);
        if existingInstruction.is_none() {
            if "e instanceof BasicOp".is_empty() {
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

    pub fn getInWires(&self,) -> Vec<WireType> {
        return self.inWires.clone();
    }

    pub fn getOutWires(&self,) -> Vec<WireType> {
        return self.outWires.clone();
    }

    pub fn getProverWitnessWires(&self,) -> Vec<WireType> {
        return self.proverWitnessWires.clone();
    }

    /**
     * Asserts an r1cs constraint. w1*w2 = w3
     *
     */
    pub fn addAssertion(&self,w1: WireType, w2: WireType, w3: WireType, desc: Vec<String>) {
        if w1.instance_of("ConstantWire") && w2.instance_of("ConstantWire") && w3.instance_of("ConstantWire")
        {
            let const1 = (w1).getConstant();
            let const2 = (w2).getConstant();
            let const3 = (w3).getConstant();
            assert!(
                const3.equals(const1.multiply(const2).modulo(Configs.get().unwrap().field_prime)),
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
        self.addAssertion(w, self.oneWire.clone(), self.zeroWire, desc);
    }

    pub fn addOneAssertion(&self,w: WireType, desc: Vec<String>) {
        self.addAssertion(w, self.oneWire, self.oneWire, desc);
    }

    pub fn addBinaryAssertion(&self,w: WireType, desc: Vec<String>) {
        let inv = w.invAsBit(desc);
        self.addAssertion(w, inv, self.zeroWire, desc);
    }

    pub fn addEqualityAssertion(&self,w1: WireType, w2: WireType, desc: Vec<String>) {
        if !w1.equals(w2) {
            self.addAssertion(w1, self.oneWire, w2, desc);
        }
    }

    pub fn addEqualityAssertion(&self,w1: WireType, b: BigInteger, desc: Vec<String>) {
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
            Configs.get().unwrap().LIBSNARK_EXEC.clone(),
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
