#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::InstanceOf;
use crate::circuit::StructNameConfig;
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::assert_basic_op::{AssertBasicOp, new_assert};
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::wire_label_instruction::LabelType;
use crate::circuit::operations::wire_label_instruction::WireLabelInstruction;
use crate::circuit::structure::constant_wire::{ConstantWire, new_constant};
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::{VariableWire, new_variable};
use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;

use crate::arc_cell_new;
use crate::util::util::ARcCell;
use crate::util::{
    run_command::run_command,
    util::{BigInteger, Util},
};
use dyn_clone::DynClone;
use lazy_static::lazy_static;
use rccell::RcCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::ops::{Add, Mul, Neg, Rem, Sub};
use std::sync::{LazyLock, Mutex};
lazy_static! {
    static ref active_circuit_generators: Mutex<HashMap<String, Box<dyn CGConfig + Send + Sync>>> =
        Mutex::new(HashMap::<String, Box<dyn CGConfig + Send + Sync>>::new());
}
//  ConcurrentHashMap<Long, CircuitGenerator> activeCircuitGenerators = new ConcurrentHashMap<>();
// 	  CircuitGenerator instance;
static CG_NAME: Mutex<&str> = Mutex::new("CGBase");

#[derive(Debug, Clone)]
pub struct CGBase;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone)]
pub struct CircuitGenerator<T> {
    pub currentWireId: ARcCell<i32>,
    pub evaluationQueue:
        ARcCell<HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>>>,

    pub zeroWire: ARcCell<Option<WireType>>,
    pub oneWire: Option<WireType>,

    pub inWires: ARcCell<Vec<Option<WireType>>>,
    pub outWires: ARcCell<Vec<Option<WireType>>>,
    pub proverWitnessWires: ARcCell<Vec<Option<WireType>>>,

    pub circuitName: String,

    pub knownConstantWires: ARcCell<HashMap<BigInteger, WireType>>,

    pub numOfConstraints: ARcCell<i32>,
    pub circuitEvaluator: ARcCell<Option<CircuitEvaluator>>,
    pub t: T,
}

pub trait CGConfigFields {
    fn current_wire_id(&self) -> ARcCell<i32>;
    fn evaluation_queue(
        &self,
    ) -> ARcCell<HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>>>;
    fn zero_wire(&self) -> ARcCell<Option<WireType>>;
    fn one_wire(&self) -> Option<WireType>;
    fn in_wires(&self) -> ARcCell<Vec<Option<WireType>>>;
    fn out_wires(&self) -> ARcCell<Vec<Option<WireType>>>;
    fn prover_witness_wires(&self) -> ARcCell<Vec<Option<WireType>>>;
    fn circuit_name(&self) -> String;
    fn known_constant_wires(&self) -> ARcCell<HashMap<BigInteger, WireType>>;
    fn num_of_constraints(&self) -> ARcCell<i32>;
    fn circuit_evaluator(&self) -> ARcCell<Option<CircuitEvaluator>>;
}

// pub trait CGConfig:DynClone {
//     fn buildCircuit(&self) {}
//     fn generateSampleInput(&self, evaluator: CircuitEvaluator) {}
//     fn generateCircuit(&self);
// fn   getName(&self) -> String ;
// fn   createInputWire(&self, desc: &Option<String>) -> WireType ;
// fn   createInputWireArray(&self, n: usize, desc: &Option<String>) -> Vec<Option<WireType>> ;
// fn   createLongElementInput(&self, totalBitwidth: i32, desc: &Option<String>) -> LongElement ;
// fn   createLongElementProverWitness( &self,
//         totalBitwidth: i32,
//         desc: &Option<String>,
//     ) -> LongElement ;
// fn   createProverWitnessWire(&self, desc: &Option<String>) -> WireType ;
// fn   createProverWitnessWireArray(     &self,
//         n: usize,
//         desc: &Option<String>,
//     ) -> Vec<Option<WireType>> ;
// fn   generateZeroWireArray(&self, n: usize) -> Vec<Option<WireType>> ;
// fn   generateOneWireArray(&self, n: usize) -> Vec<Option<WireType>> ;
// fn   makeOutput(&self, wire: WireType, desc: &Option<String>) -> WireType ;
// fn  makeVariable(&self, wire: WireType, desc: &Option<String>) -> WireType ;
// fn   makeOutputArray(   &self,
//         wires: Vec<Option<WireType>>,
//         desc: &Option<String>,
//     ) -> Vec<Option<WireType>> ;
// fn   addDebugInstruction(&self, w: WireType, desc: &Option<String>) ;
// fn   addDebugInstructiona(&self, wires: Vec<Option<WireType>>, desc: &Option<String>) ;
// fn   writeCircuitFile(&self) ;
// fn   printCircuit(&self) ;
// fn  initCircuitConstruction(&self) ;
// fn   createConstantWire(&self, x: BigInteger, desc: &Option<String>) -> WireType ;
// fn   createConstantWireArray( &self,
//         a: Vec<BigInteger>,
//         desc: &Option<String>,
//     ) -> Vec<Option<WireType>> ;
// fn   createConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType ;
// fn   createConstantWireArrayi(     &self,
//         a: Vec<i64>,
//         desc: &Option<String>,
//     ) -> Vec<Option<WireType>> ;
// fn   createNegConstantWire(&self, x: BigInteger, desc: &Option<String>) -> WireType ;
// fn   createNegConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType ;
// fn   specifyProverWitnessComputation(&self, instruction: Box<dyn Instruction+Send+Sync>) ;
// fn   getZeroWire(&self) -> Option<WireType> ;
// fn   getOneWire(&self) -> Option<WireType> ;
// fn   getEvaluationQueue(&self) -> HashMap<Box<dyn Instruction+Send+Sync>, Box<dyn Instruction+Send+Sync>> ;
// fn   getNumWires(&self) -> i32 ;
// fn   addToEvaluationQueue(&self, e: Box<dyn Instruction+Send+Sync>) -> Option<Vec<Option<WireType>>> ;
// fn   printState(&self, message: String) ;
// fn   getNumOfConstraints(&self) -> i32 ;
// fn   getInWires(&self) -> Vec<Option<WireType>> ;
// fn   getOutWires(&self) -> Vec<Option<WireType>> ;
// fn   getProverWitnessWires(&self) -> Vec<Option<WireType>> ;
// fn   addAssertion(&self, w1: WireType, w2: WireType, w3: WireType, desc: &Option<String>) ;
// fn   addZeroAssertion(&self, w: WireType, desc: &Option<String>) ;
// fn   addOneAssertion(&self, w: WireType, desc: &Option<String>) ;
// fn   addBinaryAssertion(&self, w: WireType, desc: &Option<String>) ;
// fn   addEqualityAssertion(&self, w1: WireType, w2: WireType, desc: &Option<String>) ;
// fn   addEqualityAssertionb(&self, w1: WireType, b: BigInteger, desc: &Option<String>) ;
// fn   evalCircuit(&self) ;
// fn   prepFiles(&self) ;
// fn   runLibsnark(&self) ;
// }
dyn_clone::clone_trait_object!(CGConfig);

pub fn getActiveCircuitGenerator() -> eyre::Result<Box<dyn CGConfig + Send + Sync>> {
    // if !Configs.runningMultiGenerators {
    //     return Ok(instance);
    // }

    // let threadId = Thread.currentThread().getId();
    // let currentGenerator = activeCircuitGenerators.get(threadId);

    // currentGenerator.ok_or(eyre::eyre!(
    //     "The current thread does not have any active circuit generators"
    // ))
    // eyre::bail!("The current thread does not have any active circuit generators")

    let cg_name: String = CG_NAME.lock().unwrap().to_owned();

    active_circuit_generators
        .lock()
        .unwrap()
        .get(&cg_name)
        .cloned()
        .ok_or(eyre::eyre!(
            "The current thread does not have any active circuit generators"
        ))
}
pub fn put_active_circuit_generator(name: &str, cg: Box<dyn CGConfig + Send + Sync>) {
    active_circuit_generators
        .lock()
        .unwrap()
        .insert(name.to_owned(), cg);
}

impl<T: StructNameConfig> CircuitGenerator<T> {
    pub fn new(circuitName: &str, t: T) -> CircuitGenerator<T> {
        if Configs.running_multi_generators {
            // activeCircuitGenerators.put(Thread.currentThread().getId(), this);
        }
        CircuitGenerator::<T> {
            circuitName: circuitName.to_owned(),
            inWires: arc_cell_new!(vec![]),
            outWires: arc_cell_new!(vec![]),
            zeroWire: arc_cell_new!(None),
            oneWire: None,
            proverWitnessWires: arc_cell_new!(vec![]),
            evaluationQueue: arc_cell_new!(HashMap::new()),
            knownConstantWires: arc_cell_new!(HashMap::new()),
            currentWireId: arc_cell_new!(0),
            numOfConstraints: arc_cell_new!(0),
            circuitEvaluator: arc_cell_new!(None),
            t,
        }
    }
}
//+ CreateConstantWire + CreateConstantWireArray + CreateNegConstantWire
pub trait CGConfig: DynClone + CGConfigFields + StructNameConfig {
    fn buildCircuit(&self) {}
    fn generateSampleInput(&self, evaluator: CircuitEvaluator) {}
    fn generateCircuit(&self) {
        println!(
            "Running Circuit Generator for <  {}  >",
            self.circuit_name()
        );

        self.initCircuitConstruction();
        self.buildCircuit();

        println!(
            "Circuit Generation Done for < {} > \n \t Total Number of Constraints : {} \n \t Total Number of Wires : {}",
            self.circuit_name(),
            self.getNumOfConstraints(),
            self.getNumWires()
        );
    }

    fn getName(&self) -> String {
        return self.circuit_name().clone();
    }

    fn createInputWire(&self, desc: &Option<String>) -> WireType {
        let newInputWire = WireType::Variable(new_variable(*self.getCurrentWireId().lock()));
        *self.getCurrentWireId().lock() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            newInputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        self.in_wires().lock().push(Some(newInputWire.clone()));
        return newInputWire;
    }

    fn createInputWireArray(&self, n: usize, desc: &Option<String>) -> Vec<Option<WireType>> {
        let mut list = vec![None; n];
        for i in 0..n {
            list[i] = Some(self.createInputWire(&desc.as_ref().map(|d| format!("{} {i}", d))));
        }
        return list;
    }

    fn createLongElementInput(&self, totalBitwidth: i32, desc: &Option<String>) -> LongElement {
        let numWires =
            (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as usize;
        let w = self.createInputWireArray(numWires, desc);
        let mut bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; numWires];
        if numWires as i32 * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = (totalBitwidth % LongElement::CHUNK_BITWIDTH) as u64;
        }
        return LongElement::new(w, bitwidths);
    }

    fn createLongElementProverWitness(
        &self,
        totalBitwidth: i32,
        desc: &Option<String>,
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

    fn createProverWitnessWire(&self, desc: &Option<String>) -> WireType {
        let wire = WireType::Variable(new_variable(*self.getCurrentWireId().lock()));
        *self.getCurrentWireId().lock() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::nizkinput,
            wire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        self.prover_witness_wires().lock().push(Some(wire.clone()));
        return wire;
    }

    fn createProverWitnessWireArray(
        &self,
        n: usize,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut ws = vec![None; n];
        for k in 0..n {
            ws[k] =
                Some(self.createProverWitnessWire(&desc.as_ref().map(|d| format!("{} {k}", d))));
        }
        return ws;
    }

    fn generateZeroWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        let zeroWires = vec![self.zero_wire().lock().clone(); n];
        return zeroWires;
    }

    fn generateOneWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        let oneWires = vec![self.one_wire().clone(); n];
        return oneWires;
    }

    fn makeOutput(&self, wire: WireType, desc: &Option<String>) -> WireType {
        let mut outputWire = wire.clone();
        if !(wire.instance_of("VariableWire") || wire.instance_of("VariableBitWire"))
            || self.in_wires().lock().contains(&Some(wire.clone()))
        {
            wire.packIfNeeded(&None);
            outputWire = self.makeVariable(wire.clone(), desc);
        } else if self.in_wires().lock().contains(&Some(wire.clone()))
            || self
                .prover_witness_wires()
                .lock()
                .contains(&Some(wire.clone()))
        {
            outputWire = self.makeVariable(wire.clone(), desc);
        } else {
            wire.packIfNeeded(&None);
        }

        self.out_wires().lock().push(Some(outputWire.clone()));
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::output,
            outputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        return outputWire;
    }

    fn makeVariable(&self, wire: WireType, desc: &Option<String>) -> WireType {
        let mut outputWire = WireType::Variable(new_variable(*self.getCurrentWireId().lock()));
        *self.getCurrentWireId().lock() += 1;
        let op = new_mul(
            wire,
            self.one_wire().clone().unwrap(),
            outputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let cachedOutputs = self.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.getCurrentWireId().lock() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        outputWire
    }

    fn makeOutputArray(
        &self,
        wires: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut outs = vec![None; wires.len()];
        for i in 0..wires.len() {
            outs[i] = wires[i]
                .as_ref()
                .map(|w| self.makeOutput(w.clone(), &desc.as_ref().map(|d| format!("{}[{i}]", d))));
        }
        return outs;
    }

    fn addDebugInstruction(&self, w: WireType, desc: &Option<String>) {
        w.packIfNeeded(&None);
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::debug,
            w,
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
    }

    fn addDebugInstructiona(&self, wires: Vec<Option<WireType>>, desc: &Option<String>) {
        for i in 0..wires.len() {
            wires[i].as_ref().unwrap().packIfNeeded(&None);
            self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
                LabelType::debug,
                wires[i].clone().unwrap(),
                desc.as_ref()
                    .map_or_else(|| String::new(), |d| d.to_owned()),
            )));
        }
    }

    fn writeCircuitFile(&self) {
        let mut printWriter = File::create(self.getName() + ".arith").unwrap();
        write!(printWriter, "total {}", *self.getCurrentWireId().lock());
        for e in self.evaluation_queue().lock().keys() {
            if e.doneWithinCircuit() {
                let _ = write!(printWriter, "{e:?} \n");
            }
        }
    }

    fn printCircuit(&self) {
        for e in self.evaluation_queue().lock().keys() {
            if e.doneWithinCircuit() {
                println!("{e:?}");
            }
        }
    }

    fn initCircuitConstruction(&self) {
        let oneWire =
            WireType::Constant(new_constant(*self.getCurrentWireId().lock(), Util::one()));
        *self.getCurrentWireId().lock() += 1;
        self.known_constant_wires()
            .lock()
            .insert(Util::one(), oneWire.clone());
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            oneWire.clone(),
            "The one-input wire.".to_owned(),
        )));
        self.in_wires().lock().push(Some(oneWire.clone()));
        *self.zero_wire().lock() = Some(oneWire.muli(0, &None));
    }

    fn createConstantWire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        return self.one_wire().clone().unwrap().mulb(x, desc);
    }

    fn createConstantWireArray(
        &self,
        a: Vec<BigInteger>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.createConstantWire(a[i].clone(), desc));
        }
        return w;
    }

    fn createConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType {
        return self.one_wire().clone().unwrap().muli(x, desc);
    }

    fn createConstantWireArrayi(
        &self,
        a: Vec<i64>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.createConstantWirei(a[i], desc));
        }
        return w;
    }

    fn createNegConstantWire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        return self.one_wire().clone().unwrap().mulb(x.neg(), desc);
    }

    fn createNegConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType {
        return self.one_wire().clone().unwrap().muli(-x, desc);
    }

    /**
     * Use to support computation for prover witness values outside of the
     * circuit. See Mod_Gadget and Field_Division gadgets for examples.
     *
     * @param instruction
     */
    fn specifyProverWitnessComputation(&self, instruction: Box<dyn Instruction + Send + Sync>) {
        self.addToEvaluationQueue(instruction);
    }

    fn getZeroWire(&self) -> Option<WireType> {
        return self.zero_wire().lock().clone();
    }

    fn getOneWire(&self) -> Option<WireType> {
        return self.one_wire().clone();
    }

    fn getEvaluationQueue(
        &self,
    ) -> HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>> {
        return self.evaluation_queue().lock().clone();
    }

    fn getNumWires(&self) -> i32 {
        return *self.getCurrentWireId().lock();
    }
    fn getCurrentWireId(&self) -> ARcCell<i32> {
        self.current_wire_id().clone()
    }
    fn addToEvaluationQueue(
        &self,
        e: Box<dyn Instruction + Send + Sync>,
    ) -> Option<Vec<Option<WireType>>> {
        let evaluationQueue = self.evaluation_queue().lock().clone();
        let existingInstruction = evaluationQueue.get(&e);
        self.evaluation_queue()
            .lock()
            .entry(e.clone())
            .or_insert(e.clone());
        if existingInstruction.is_none() {
            if e.instance_of("BasicOp") {
                *self.num_of_constraints().lock() +=
                    e.basic_op().as_ref().unwrap().getNumMulGates();
            }
            return None; // returning null means we have not seen this instruction before
        }
        // have seen this instruction before, but can't de-duplicate
        existingInstruction.and_then(|e| e.basic_op().map(|op| op.getOutputs()))
        // if existingInstruction.unwrap().instance_of("BasicOp") {
        //     return Some(existingInstruction.unwrap().basic_op().unwrap().getOutputs());
        // } else {
        //     return None;
        // }
    }

    fn printState(&self, message: String) {
        println!("\nGenerator State @ {message}");
        println!(
            "\tCurrent Number of Multiplication Gates  .  {}\n",
            *self.num_of_constraints().lock()
        );
    }

    fn getNumOfConstraints(&self) -> i32 {
        return *self.num_of_constraints().lock();
    }

    fn getInWires(&self) -> Vec<Option<WireType>> {
        return self.in_wires().lock().clone();
    }

    fn getOutWires(&self) -> Vec<Option<WireType>> {
        return self.out_wires().lock().clone();
    }

    fn getProverWitnessWires(&self) -> Vec<Option<WireType>> {
        return self.prover_witness_wires().lock().clone();
    }

    /**
     * Asserts an r1cs constraint. w1*w2 = w3
     *
     */
    fn addAssertion(&self, w1: WireType, w2: WireType, w3: WireType, desc: &Option<String>) {
        if w1.instance_of("ConstantWire")
            && w2.instance_of("ConstantWire")
            && w3.instance_of("ConstantWire")
        {
            let const1 = w1.try_as_constant_ref().unwrap().getConstant();
            let const2 = w2.try_as_constant_ref().unwrap().getConstant();
            let const3 = w3.try_as_constant_ref().unwrap().getConstant();
            assert!(
                const3 == const1.mul(const2).rem(Configs.field_prime.clone()),
                "Assertion failed on the provided constant wires .. "
            );
        } else {
            w1.packIfNeeded(&None);
            w2.packIfNeeded(&None);
            w3.packIfNeeded(&None);
            let op = new_assert(
                w1,
                w2,
                w3,
                desc.as_ref()
                    .map_or_else(|| String::new(), |d| d.to_owned()),
            );
            self.addToEvaluationQueue(Box::new(op));
        }
    }

    fn addZeroAssertion(&self, w: WireType, desc: &Option<String>) {
        self.addAssertion(
            w,
            self.one_wire().clone().unwrap(),
            self.zero_wire().lock().clone().unwrap(),
            desc,
        );
    }

    fn addOneAssertion(&self, w: WireType, desc: &Option<String>) {
        self.addAssertion(
            w,
            self.one_wire().clone().unwrap(),
            self.one_wire().clone().unwrap(),
            desc,
        );
    }

    fn addBinaryAssertion(&self, w: WireType, desc: &Option<String>) {
        let inv = w.invAsBit(desc).unwrap();
        self.addAssertion(w, inv, self.zero_wire().lock().clone().unwrap(), desc);
    }

    fn addEqualityAssertion(&self, w1: WireType, w2: WireType, desc: &Option<String>) {
        if w1 != w2 {
            self.addAssertion(w1, self.one_wire().clone().unwrap(), w2, desc);
        }
    }

    fn addEqualityAssertionb(&self, w1: WireType, b: BigInteger, desc: &Option<String>) {
        self.addAssertion(
            w1,
            self.one_wire().clone().unwrap(),
            self.createConstantWire(b, desc),
            desc,
        );
    }

    fn evalCircuit(&self) {
        let circuitEvaluator = CircuitEvaluator::new(&self.name());
        self.generateSampleInput(circuitEvaluator.clone());
        circuitEvaluator.evaluate();
        *self.circuit_evaluator().lock() = Some(circuitEvaluator);
    }

    fn prepFiles(&self) {
        self.writeCircuitFile();
        assert!(
            self.circuit_evaluator().lock().is_some(),
            "evalCircuit() must be called before prepFiles()"
        );
        self.circuit_evaluator()
            .lock()
            .as_ref()
            .unwrap()
            .writeInputFile();
    }

    fn runLibsnark(&self) {
        let p = run_command(
            vec![
                &Configs.libsnark_exec.clone(),
                &(self.circuit_name().clone() + ".arith"),
                &(self.circuit_name().clone() + ".in"),
            ],
            None,
            false,
        );
        println!(
            "\n-----------------------------------RUNNING LIBSNARK -----------------------------------------"
        );
        let inp = p.0.clone().unwrap();
        let input = inp.split_ascii_whitespace();
        let mut buf = String::new();
        for line in input {
            buf.push_str(&(line.to_owned() + "\n"));
        }
        println!("{buf}");
    }

    fn getCircuitEvaluator(&self) -> CircuitEvaluator {
        assert!(
            self.circuit_evaluator().lock().is_some(),
            "evalCircuit() must be called before getCircuitEvaluator()"
        );

        return self.circuit_evaluator().lock().clone().unwrap();
    }
}

// #[macro_export]
// macro_rules! impl_circuit_generator_config_fields_for {
//     ($impl_type:ty) => {
//         impl crate::circuit::structure::circuit_generator::CGConfigFields for $impl_type {
//             fn current_wire_id(&self) -> ARcCell<i32> {
//                 self.currentWireId.clone()
//             }
//             fn evaluation_queue(
//                 &self,
//             ) -> ARcCell<HashMap<Box<dyn Instruction+Send+Sync>, Box<dyn Instruction+Send+Sync>>> {
//                 self.evaluationQueue.clone()
//             }

//             fn zero_wire(&self) -> ARcCell<Option<WireType>> {
//                 self.zeroWire.clone()
//             }
//             fn one_wire(&self) -> Option<WireType> {
//                 self.oneWire.clone()
//             }
//             fn in_wires(&self) -> ARcCell<Vec<Option<WireType>>> {
//                 self.inWires.clone()
//             }
//             fn out_wires(&self) -> ARcCell<Vec<Option<WireType>>> {
//                 self.outWires.clone()
//             }
//             fn prover_witness_wires(&self) -> ARcCell<Vec<Option<WireType>>> {
//                 self.prover_witness_wires().clone()
//             }
//             fn circuit_name(&self) -> String {
//                 self.circuitName.clone()
//             }
//             fn known_constant_wires(&self) -> ARcCell<HashMap<BigInteger, WireType>> {
//                 self.knownConstantWires.clone()
//             }
//             fn num_of_constraints(&self) -> ARcCell<i32> {
//                 self.numOfConstraints.clone()
//             }
//             fn circuit_evaluator(&self) -> ARcCell<Option<CircuitEvaluator>> {
//                 self.circuitEvaluator.clone()
//             }
//         }
//     };
// }
// impl_circuit_generator_config_fields_for!(CircuitGenerator<CGBase>);

impl<T> crate::circuit::structure::circuit_generator::CGConfigFields for CircuitGenerator<T> {
    fn current_wire_id(&self) -> ARcCell<i32> {
        self.currentWireId.clone()
    }
    fn evaluation_queue(
        &self,
    ) -> ARcCell<HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>>>
    {
        self.evaluationQueue.clone()
    }

    fn zero_wire(&self) -> ARcCell<Option<WireType>> {
        self.zeroWire.clone()
    }
    fn one_wire(&self) -> Option<WireType> {
        self.oneWire.clone()
    }
    fn in_wires(&self) -> ARcCell<Vec<Option<WireType>>> {
        self.inWires.clone()
    }
    fn out_wires(&self) -> ARcCell<Vec<Option<WireType>>> {
        self.outWires.clone()
    }
    fn prover_witness_wires(&self) -> ARcCell<Vec<Option<WireType>>> {
        self.prover_witness_wires().clone()
    }
    fn circuit_name(&self) -> String {
        self.circuitName.clone()
    }
    fn known_constant_wires(&self) -> ARcCell<HashMap<BigInteger, WireType>> {
        self.knownConstantWires.clone()
    }
    fn num_of_constraints(&self) -> ARcCell<i32> {
        self.numOfConstraints.clone()
    }
    fn circuit_evaluator(&self) -> ARcCell<Option<CircuitEvaluator>> {
        self.circuitEvaluator.clone()
    }
}

pub trait CreateConstantWire<T = WireType> {
    fn create_constant_wire(&self, x: T, desc: &Option<String>) -> WireType;
}

impl<T> CreateConstantWire<BigInteger> for CircuitGenerator<T> {
    fn create_constant_wire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        return self.oneWire.clone().unwrap().mulb(x, desc);
    }
}
impl<T> CreateConstantWire<i64> for CircuitGenerator<T> {
    fn create_constant_wire(&self, x: i64, desc: &Option<String>) -> WireType {
        return self.oneWire.clone().unwrap().muli(x, desc);
    }
}
pub trait CreateConstantWireArray<T = WireType> {
    fn create_constant_wire_array(&self, a: T, desc: &Option<String>) -> Vec<Option<WireType>>;
}
impl<T> CreateConstantWireArray<Vec<BigInteger>> for CircuitGenerator<T> {
    fn create_constant_wire_array(
        &self,
        a: Vec<BigInteger>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.create_constant_wire(a[i].clone(), desc));
        }
        return w;
    }
}
impl<T> CreateConstantWireArray<Vec<i64>> for CircuitGenerator<T> {
    fn create_constant_wire_array(
        &self,
        a: Vec<i64>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.create_constant_wire(a[i], desc));
        }
        return w;
    }
}

pub trait CreateNegConstantWire<T = WireType> {
    fn create_neg_constant_wire(&self, x: T, desc: &Option<String>) -> WireType;
}
impl<T> CreateNegConstantWire<BigInteger> for CircuitGenerator<T> {
    fn create_neg_constant_wire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        return self.oneWire.clone().unwrap().mulb(x.neg(), desc);
    }
}
impl<T> CreateNegConstantWire<i64> for CircuitGenerator<T> {
    fn create_neg_constant_wire(&self, x: i64, desc: &Option<String>) -> WireType {
        return self.oneWire.clone().unwrap().muli(-x, desc);
    }
}
