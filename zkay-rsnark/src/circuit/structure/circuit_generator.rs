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
use crate::circuit::operations::primitive::assert_basic_op::{AssertBasicOp, new_assert};
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::wire_label_instruction::LabelType;
use crate::circuit::operations::wire_label_instruction::WireLabelInstruction;
use crate::circuit::structure::constant_wire::ConstantWire;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;

use crate::util::{
    run_command::run_command,
    util::{BigInteger, Util},
};
use rccell::RcCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::ops::{Add, Mul, Neg, Rem, Sub};
//  ConcurrentHashMap<Long, CircuitGenerator> activeCircuitGenerators = new ConcurrentHashMap<>();
// 	  CircuitGenerator instance;

use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, PartialEq)]
pub struct CircuitGenerator {
    pub currentWireId:RcCell<i32>,
    pub evaluationQueue: RcCell<HashMap<Box<dyn Instruction>, Box<dyn Instruction>>>,

    pub zeroWire: RcCell<Option<WireType>>,
    pub oneWire: Option<WireType>,

    pub inWires: RcCell<Vec<Option<WireType>>>,
    pub outWires: RcCell<Vec<Option<WireType>>>,
    pub proverWitnessWires: RcCell<Vec<Option<WireType>>>,

    pub circuitName: String,

    pub knownConstantWires: RcCell<HashMap<BigInteger, WireType>>,

    pub numOfConstraints: RcCell<i32>,
    pub circuitEvaluator: RcCell<Option<CircuitEvaluator>>,
}
pub trait CGConfig {
    fn buildCircuit(&self) {}
    fn generateSampleInput(&self, evaluator: CircuitEvaluator) {}
}
impl CGConfig for CircuitGenerator {}
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
        eyre::bail!("The current thread does not have any active circuit generators")
    }
    pub fn new(circuitName: String) -> Self {
        if Configs.get().unwrap().running_multi_generators {
            // activeCircuitGenerators.put(Thread.currentThread().getId(), this);
        }
        Self {
            circuitName,
            inWires: RcCell::new(vec![]),
            outWires: RcCell::new(vec![]),
            zeroWire: RcCell::new(None),
            oneWire: None,
            proverWitnessWires: RcCell::new(vec![]),
            evaluationQueue: RcCell::new(HashMap::new()),
            knownConstantWires: RcCell::new(HashMap::new()),
            currentWireId: RcCell::new(0),
            numOfConstraints:RcCell::new( 0),
            circuitEvaluator: RcCell::new(None),
        }
    }

    pub fn generateCircuit(&self) {
        println!("Running Circuit Generator for <  {}  >", self.circuitName);

        self.initCircuitConstruction();
        self.buildCircuit();

        println!(
            "Circuit Generation Done for < {} > \n \t Total Number of Constraints : {} \n \t Total Number of Wires : {}",
            self.circuitName,
            self.getNumOfConstraints(),
            self.getNumWires()
        );
    }

    pub fn getName(&self) -> String {
        return self.circuitName.clone();
    }

    pub fn createInputWire(&self, desc: Vec<String>) -> WireType {
        let newInputWire = WireType::Variable(VariableWire::new(*self.currentWireId.borrow_mut()));
        *self.currentWireId.borrow_mut() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            newInputWire.clone(),
            desc,
        )));
        self.inWires.borrow_mut().push(Some(newInputWire.clone()));
        return newInputWire;
    }

    pub fn createInputWireArray(&self, n: usize, desc: Vec<String>) -> Vec<Option<WireType>> {
        let mut list = vec![None; n];
        for i in 0..n {
            list[i] = Some(self.createInputWire(
                vec![desc.get(0).map_or_else(||String::new(),|d|d.clone()+ " " + &i.to_string())],
            ));
        }
        return list;
    }

    pub fn createLongElementInput(&self, totalBitwidth: i32, desc: Vec<String>) -> LongElement {
        let numWires =
            (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as usize;
        let w = self.createInputWireArray(numWires, desc);
        let mut bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; numWires];
        if numWires as i32 * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = (totalBitwidth % LongElement::CHUNK_BITWIDTH) as u64;
        }
        return LongElement::new(w, bitwidths);
    }

    pub fn createLongElementProverWitness(
        &self,
        totalBitwidth: i32,
        desc: Vec<String>,
    ) -> LongElement {
        let numWires =
            (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as usize;
        let w = self.createProverWitnessWireArray(numWires, desc);
        let mut bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; numWires];
        if numWires as i32 * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = (totalBitwidth % LongElement::CHUNK_BITWIDTH) as u64;
        }
        return LongElement::new(w, bitwidths);
    }

    pub fn createProverWitnessWire(&self, desc: Vec<String>) -> WireType {
        let wire = WireType::Variable(VariableWire::new(*self.currentWireId.borrow_mut()));
        *self.currentWireId.borrow_mut() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::nizkinput,
            wire.clone(),
            desc,
        )));
        self.proverWitnessWires.borrow_mut().push(Some(wire.clone()));
        return wire;
    }

    pub fn createProverWitnessWireArray(
        &self,
        n: usize,
        desc: Vec<String>,
    ) -> Vec<Option<WireType>> {
        let mut ws = vec![None; n];
        for k in 0..n {
            ws[k] = Some(self.createProverWitnessWire(
                vec![desc.get(0).map_or_else(||String::new(),|d|d.clone()+ " " + &k.to_string())],
            ));
        }
        return ws;
    }

    pub fn generateZeroWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        let zeroWires = vec![self.zeroWire.borrow().clone(); n];
        return zeroWires;
    }

    pub fn generateOneWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        let oneWires = vec![self.oneWire.clone(); n];
        return oneWires;
    }

    pub fn makeOutput(&self, wire: WireType, desc: Vec<String>) -> WireType {
        let mut outputWire = wire.clone();
        if !(wire.instance_of("VariableWire") || wire.instance_of("VariableBitWire"))
            || self.inWires.borrow().contains(&Some(wire.clone()))
        {
            wire.packIfNeeded(vec![]);
            outputWire = self.makeVariable(wire.clone(), desc.clone());
        } else if self.inWires.borrow().contains(&Some(wire.clone()))
            || self.proverWitnessWires.borrow().contains(&Some(wire.clone()))
        {
            outputWire = self.makeVariable(wire.clone(), desc.clone());
        } else {
            wire.packIfNeeded(vec![]);
        }

        self.outWires.borrow_mut().push(Some(outputWire.clone()));
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::output,
            outputWire.clone(),
            desc,
        )));
        return outputWire;
    }

    fn makeVariable(&self, wire: WireType, desc: Vec<String>) -> WireType {
        let mut outputWire = WireType::Variable(VariableWire::new(*self.currentWireId.borrow_mut()));
        *self.currentWireId.borrow_mut() += 1;
        let op = new_mul(wire, self.oneWire.clone().unwrap(), outputWire.clone(), desc);
        let cachedOutputs = self.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        outputWire
    }

    pub fn makeOutputArray(
        &self,
        wires: Vec<Option<WireType>>,
        desc: Vec<String>,
    ) -> Vec<Option<WireType>> {
        let mut outs = vec![None; wires.len()];
        for i in 0..wires.len() {
            outs[i] =
                wires[i].as_ref().map(|w| {
                    self.makeOutput(
                        w.clone(),
                        vec![desc.get(0).map_or_else(
                            || String::new(),
                            |d| d.clone() + "[" + &i.to_string() + "]",
                        )],
                    )
                });
        }
        return outs;
    }

    pub fn addDebugInstruction(&self, w: WireType, desc: Vec<String>) {
        w.packIfNeeded(vec![]);
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::debug,
            w,
            desc,
        )));
    }

    pub fn addDebugInstructiona(&self, wires: Vec<Option<WireType>>, desc: Vec<String>) {
        for i in 0..wires.len() {
            wires[i].as_ref().unwrap().packIfNeeded(vec![]);
            self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
                LabelType::debug,
                wires[i].clone().unwrap(),
                desc.clone(),
            )));
        }
    }

    pub fn writeCircuitFile(&self) {
        let mut printWriter = File::create(self.getName() + ".arith").unwrap();
        write!(printWriter, "total {}", *self.currentWireId.borrow_mut());
        for e in self.evaluationQueue.borrow().keys() {
            if e.doneWithinCircuit() {
                write!(printWriter, "{e:?} \n");
            }
        }
    }

    pub fn printCircuit(&self) {
        for e in self.evaluationQueue.borrow().keys() {
            if e.doneWithinCircuit() {
                println!("{e:?}");
            }
        }
    }

    fn initCircuitConstruction(&self) {
        let oneWire = WireType::Constant(ConstantWire::new(*self.currentWireId.borrow_mut(), Util::one()));
        *self.currentWireId.borrow_mut() += 1;
        self.knownConstantWires.borrow_mut().insert(Util::one(), oneWire.clone());
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            oneWire.clone(),
            vec!["The one-input wire.".to_owned()],
        )));
        self.inWires.borrow_mut().push(Some(oneWire.clone()));
        *self.zeroWire.borrow_mut() = Some(oneWire.muli(0, vec![]));
    }

    pub fn createConstantWire(&self, x: BigInteger, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().unwrap().mulb(x, desc);
    }

    pub fn createConstantWireArray(
        &self,
        a: Vec<BigInteger>,
        desc: Vec<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.createConstantWire(a[i].clone(), desc.clone()));
        }
        return w;
    }

    pub fn createConstantWirei(&self, x: i64, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().unwrap().muli(x, desc);
    }

    pub fn createConstantWireArrayi(
        &self,
        a: Vec<i64>,
        desc: Vec<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.createConstantWirei(a[i], desc.clone()));
        }
        return w;
    }

    pub fn createNegConstantWire(&self, x: BigInteger, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().unwrap().mulb(x.neg(), desc);
    }

    pub fn createNegConstantWirei(&self, x: i64, desc: Vec<String>) -> WireType {
        return self.oneWire.clone().unwrap().muli(-x, desc);
    }

    /**
     * Use to support computation for prover witness values outside of the
     * circuit. See Mod_Gadget and Field_Division gadgets for examples.
     *
     * @param instruction
     */
    pub fn specifyProverWitnessComputation(&self, instruction: Box<dyn Instruction>) {
        self.addToEvaluationQueue(instruction);
    }

    pub fn getZeroWire(&self) -> Option<WireType> {
        return self.zeroWire.borrow().clone();
    }

    pub fn getOneWire(&self) -> Option<WireType> {
        return self.oneWire.clone();
    }

    pub fn getEvaluationQueue(&self) -> HashMap<Box<dyn Instruction>, Box<dyn Instruction>> {
        return self.evaluationQueue.borrow().clone();
    }

    pub fn getNumWires(&self) -> i32 {
        return *self.currentWireId.borrow_mut();
    }

    pub fn addToEvaluationQueue(&self, e: Box<dyn Instruction>) -> Option<Vec<Option<WireType>>> {
        let evaluationQueue=self.evaluationQueue.borrow().clone();
        let existingInstruction = evaluationQueue.get(&e);
        self.evaluationQueue.borrow_mut().entry(e.clone()).or_insert(e.clone());
        if existingInstruction.is_none() {
            if e.instance_of("BasicOp") {
                *self.numOfConstraints.borrow_mut() += (e).getNumMulGates();
            }
            return None; // returning null means we have not seen this instruction before
        }

        if existingInstruction.as_ref().unwrap().instance_of("BasicOp") {
            return Some(existingInstruction.as_ref().unwrap().getOutputs());
        } else {
            return None; // have seen this instruction before, but can't de-duplicate
        }
    }

    pub fn printState(&self, message: String) {
        println!("\nGenerator State @ {message}");
        println!(
            "\tCurrent Number of Multiplication Gates  .  {}\n",
            *self.numOfConstraints.borrow_mut()
        );
    }

    pub fn getNumOfConstraints(&self) -> i32 {
        return *self.numOfConstraints.borrow_mut();
    }

    pub fn getInWires(&self) -> Vec<Option<WireType>> {
        return self.inWires.borrow().clone();
    }

    pub fn getOutWires(&self) -> Vec<Option<WireType>> {
        return self.outWires.borrow().clone();
    }

    pub fn getProverWitnessWires(&self) -> Vec<Option<WireType>> {
        return self.proverWitnessWires.borrow().clone();
    }

    /**
     * Asserts an r1cs constraint. w1*w2 = w3
     *
     */
    pub fn addAssertion(&self, w1: WireType, w2: WireType, w3: WireType, desc: Vec<String>) {
        if w1.instance_of("ConstantWire")
            && w2.instance_of("ConstantWire")
            && w3.instance_of("ConstantWire")
        {
            let const1 = (w1).getConstant();
            let const2 = (w2).getConstant();
            let const3 = (w3).getConstant();
            assert!(
                const3
                    == const1
                        .mul(const2)
                        .rem(Configs.get().unwrap().field_prime.clone()),
                "Assertion failed on the provided constant wires .. "
            );
        } else {
            w1.packIfNeeded(vec![]);
            w2.packIfNeeded(vec![]);
            w3.packIfNeeded(vec![]);
            let op = new_assert(w1, w2, w3, desc);
            self.addToEvaluationQueue(Box::new(op));
        }
    }

    pub fn addZeroAssertion(&self, w: WireType, desc: Vec<String>) {
        self.addAssertion(
            w,
            self.oneWire.clone().unwrap(),
            self.zeroWire.borrow().clone().unwrap(),
            desc,
        );
    }

    pub fn addOneAssertion(&self, w: WireType, desc: Vec<String>) {
        self.addAssertion(
            w,
            self.oneWire.clone().unwrap(),
            self.oneWire.clone().unwrap(),
            desc,
        );
    }

    pub fn addBinaryAssertion(&self, w: WireType, desc: Vec<String>) {
        let inv = w.invAsBit(desc.clone()).unwrap();
        self.addAssertion(w, inv, self.zeroWire.borrow().clone().unwrap(), desc);
    }

    pub fn addEqualityAssertion(&self, w1: WireType, w2: WireType, desc: Vec<String>) {
        if w1 != w2 {
            self.addAssertion(w1, self.oneWire.clone().unwrap(), w2, desc);
        }
    }

    pub fn addEqualityAssertionb(&self, w1: WireType, b: BigInteger, desc: Vec<String>) {
        self.addAssertion(
            w1,
            self.oneWire.clone().unwrap(),
            self.createConstantWire(b, desc.clone()),
            desc,
        );
    }

    pub fn evalCircuit(&self) {
        let circuitEvaluator = CircuitEvaluator::new(self.clone());
        self.generateSampleInput(circuitEvaluator.clone());
        circuitEvaluator.evaluate();
        *self.circuitEvaluator.borrow_mut() = Some(circuitEvaluator);
    }

    pub fn prepFiles(&self) {
        self.writeCircuitFile();
        assert!(
            self.circuitEvaluator.borrow().is_some(),
            "evalCircuit() must be called before prepFiles()"
        );
        self.circuitEvaluator.borrow().as_ref().unwrap().writeInputFile();
    }

    pub fn runLibsnark(&self) {
        let p = run_command(
            vec![
                &Configs.get().unwrap().libsnark_exec.clone(),
                &(self.circuitName.clone() + ".arith"),
                &(self.circuitName.clone() + ".in"),
            ],
            None,
            false,
        );
        println!(
            "\n-----------------------------------RUNNING LIBSNARK -----------------------------------------"
        );
        let inp=p.0.clone().unwrap();
        let input = inp.split_ascii_whitespace();
        let mut buf = String::new();
        for line in input {
            buf.push_str(&(line.to_owned() + "\n"));
        }
        println!("{buf}");
    }

    pub fn getCircuitEvaluator(&self) -> CircuitEvaluator {
        assert!(
            self.circuitEvaluator.borrow().is_some(),
            "evalCircuit() must be called before getCircuitEvaluator()"
        );

        return self.circuitEvaluator.borrow().clone().unwrap();
    }
}
