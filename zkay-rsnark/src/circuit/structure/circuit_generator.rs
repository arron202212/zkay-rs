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
use crate::util::{
    run_command::run_command,
    util::{BigInteger, Util},
};
use dyn_clone::DynClone;
use rccell::RcCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::ops::{Add, Mul, Neg, Rem, Sub};
use std::sync::LazyLock;
static active_circuit_generators: LazyLock<HashMap<String, Box<dyn CGConfig + Send + Sync>>> =
    LazyLock::new(|| HashMap::<String, Box<dyn CGConfig + Send + Sync>>::new());
//  ConcurrentHashMap<Long, CircuitGenerator> activeCircuitGenerators = new ConcurrentHashMap<>();
// 	  CircuitGenerator instance;
#[derive(Debug, Clone)]
pub struct CGBase;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, PartialEq)]
pub struct CircuitGenerator<T> {
    pub currentWireId: RcCell<i32>,
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
    pub t: T,
}

pub trait CGConfigFields {
    fn current_wire_id(&self) -> RcCell<i32>;
    fn evaluation_queue(&self) -> RcCell<HashMap<Box<dyn Instruction>, Box<dyn Instruction>>>;
    fn zero_wire(&self) -> RcCell<Option<WireType>>;
    fn one_wire(&self) -> Option<WireType>;
    fn in_wires(&self) -> RcCell<Vec<Option<WireType>>>;
    fn out_wires(&self) -> RcCell<Vec<Option<WireType>>>;
    fn prover_witness_wires(&self) -> RcCell<Vec<Option<WireType>>>;
    fn circuit_name(&self) -> String;
    fn known_constant_wires(&self) -> RcCell<HashMap<BigInteger, WireType>>;
    fn num_of_constraints(&self) -> RcCell<i32>;
    fn circuit_evaluator(&self) -> RcCell<Option<CircuitEvaluator>>;
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
// fn   specifyProverWitnessComputation(&self, instruction: Box<dyn Instruction>) ;
// fn   getZeroWire(&self) -> Option<WireType> ;
// fn   getOneWire(&self) -> Option<WireType> ;
// fn   getEvaluationQueue(&self) -> HashMap<Box<dyn Instruction>, Box<dyn Instruction>> ;
// fn   getNumWires(&self) -> i32 ;
// fn   addToEvaluationQueue(&self, e: Box<dyn Instruction>) -> Option<Vec<Option<WireType>>> ;
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

pub fn getActiveCircuitGenerator(name: &str) -> eyre::Result<Box<dyn CGConfig + Send + Sync>> {
    // if !Configs.get().unwrap().runningMultiGenerators {
    //     return Ok(instance);
    // }

    // let threadId = Thread.currentThread().getId();
    // let currentGenerator = activeCircuitGenerators.get(threadId);

    // currentGenerator.ok_or(eyre::eyre!(
    //     "The current thread does not have any active circuit generators"
    // ))
    // eyre::bail!("The current thread does not have any active circuit generators")
    active_circuit_generators
        .get(name)
        .cloned()
        .ok_or(eyre::eyre!(
            "The current thread does not have any active circuit generators"
        ))
}

impl<T> CircuitGenerator<T> {
    pub fn new(circuitName: &str, t: T) -> CircuitGenerator<T> {
        if Configs.get().unwrap().running_multi_generators {
            // activeCircuitGenerators.put(Thread.currentThread().getId(), this);
        }
        CircuitGenerator::<T> {
            circuitName: circuitName.to_owned(),
            inWires: RcCell::new(vec![]),
            outWires: RcCell::new(vec![]),
            zeroWire: RcCell::new(None),
            oneWire: None,
            proverWitnessWires: RcCell::new(vec![]),
            evaluationQueue: RcCell::new(HashMap::new()),
            knownConstantWires: RcCell::new(HashMap::new()),
            currentWireId: RcCell::new(0),
            numOfConstraints: RcCell::new(0),
            circuitEvaluator: RcCell::new(None),
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
        let newInputWire = WireType::Variable(new_variable(*self.getCurrentWireId().borrow()));
        *self.getCurrentWireId().borrow_mut() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            newInputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        self.in_wires()
            .borrow_mut()
            .push(Some(newInputWire.clone()));
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
        let wire = WireType::Variable(new_variable(*self.getCurrentWireId().borrow()));
        *self.getCurrentWireId().borrow_mut() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::nizkinput,
            wire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        self.prover_witness_wires()
            .borrow_mut()
            .push(Some(wire.clone()));
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
        let zeroWires = vec![self.zero_wire().borrow().clone(); n];
        return zeroWires;
    }

    fn generateOneWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        let oneWires = vec![self.one_wire().clone(); n];
        return oneWires;
    }

    fn makeOutput(&self, wire: WireType, desc: &Option<String>) -> WireType {
        let mut outputWire = wire.clone();
        if !(wire.instance_of("VariableWire") || wire.instance_of("VariableBitWire"))
            || self.in_wires().borrow().contains(&Some(wire.clone()))
        {
            wire.packIfNeeded(&None);
            outputWire = self.makeVariable(wire.clone(), desc);
        } else if self.in_wires().borrow().contains(&Some(wire.clone()))
            || self
                .prover_witness_wires()
                .borrow()
                .contains(&Some(wire.clone()))
        {
            outputWire = self.makeVariable(wire.clone(), desc);
        } else {
            wire.packIfNeeded(&None);
        }

        self.out_wires().borrow_mut().push(Some(outputWire.clone()));
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::output,
            outputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        return outputWire;
    }

    fn makeVariable(&self, wire: WireType, desc: &Option<String>) -> WireType {
        let mut outputWire = WireType::Variable(new_variable(*self.getCurrentWireId().borrow()));
        *self.getCurrentWireId().borrow_mut() += 1;
        let op = new_mul(
            wire,
            self.one_wire().clone().unwrap(),
            outputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let cachedOutputs = self.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.getCurrentWireId().borrow_mut() -= 1;
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
        write!(printWriter, "total {}", *self.getCurrentWireId().borrow());
        for e in self.evaluation_queue().borrow().keys() {
            if e.doneWithinCircuit() {
                let _ = write!(printWriter, "{e:?} \n");
            }
        }
    }

    fn printCircuit(&self) {
        for e in self.evaluation_queue().borrow().keys() {
            if e.doneWithinCircuit() {
                println!("{e:?}");
            }
        }
    }

    fn initCircuitConstruction(&self) {
        let oneWire =
            WireType::Constant(new_constant(*self.getCurrentWireId().borrow(), Util::one()));
        *self.getCurrentWireId().borrow_mut() += 1;
        self.known_constant_wires()
            .borrow_mut()
            .insert(Util::one(), oneWire.clone());
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            oneWire.clone(),
            "The one-input wire.".to_owned(),
        )));
        self.in_wires().borrow_mut().push(Some(oneWire.clone()));
        *self.zero_wire().borrow_mut() = Some(oneWire.muli(0, &None));
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
    fn specifyProverWitnessComputation(&self, instruction: Box<dyn Instruction>) {
        self.addToEvaluationQueue(instruction);
    }

    fn getZeroWire(&self) -> Option<WireType> {
        return self.zero_wire().borrow().clone();
    }

    fn getOneWire(&self) -> Option<WireType> {
        return self.one_wire().clone();
    }

    fn getEvaluationQueue(&self) -> HashMap<Box<dyn Instruction>, Box<dyn Instruction>> {
        return self.evaluation_queue().borrow().clone();
    }

    fn getNumWires(&self) -> i32 {
        return *self.getCurrentWireId().borrow();
    }
    fn getCurrentWireId(&self) -> RcCell<i32> {
        self.current_wire_id().clone()
    }
    fn addToEvaluationQueue(&self, e: Box<dyn Instruction>) -> Option<Vec<Option<WireType>>> {
        let evaluationQueue = self.evaluation_queue().borrow().clone();
        let existingInstruction = evaluationQueue.get(&e);
        self.evaluation_queue()
            .borrow_mut()
            .entry(e.clone())
            .or_insert(e.clone());
        if existingInstruction.is_none() {
            if e.instance_of("BasicOp") {
                *self.num_of_constraints().borrow_mut() +=
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
            *self.num_of_constraints().borrow()
        );
    }

    fn getNumOfConstraints(&self) -> i32 {
        return *self.num_of_constraints().borrow();
    }

    fn getInWires(&self) -> Vec<Option<WireType>> {
        return self.in_wires().borrow().clone();
    }

    fn getOutWires(&self) -> Vec<Option<WireType>> {
        return self.out_wires().borrow().clone();
    }

    fn getProverWitnessWires(&self) -> Vec<Option<WireType>> {
        return self.prover_witness_wires().borrow().clone();
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
                const3
                    == const1
                        .mul(const2)
                        .rem(Configs.get().unwrap().field_prime.clone()),
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
            self.zero_wire().borrow().clone().unwrap(),
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
        self.addAssertion(w, inv, self.zero_wire().borrow().clone().unwrap(), desc);
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
        *self.circuit_evaluator().borrow_mut() = Some(circuitEvaluator);
    }

    fn prepFiles(&self) {
        self.writeCircuitFile();
        assert!(
            self.circuit_evaluator().borrow().is_some(),
            "evalCircuit() must be called before prepFiles()"
        );
        self.circuit_evaluator()
            .borrow()
            .as_ref()
            .unwrap()
            .writeInputFile();
    }

    fn runLibsnark(&self) {
        let p = run_command(
            vec![
                &Configs.get().unwrap().libsnark_exec.clone(),
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
            self.circuit_evaluator().borrow().is_some(),
            "evalCircuit() must be called before getCircuitEvaluator()"
        );

        return self.circuit_evaluator().borrow().clone().unwrap();
    }
}

#[macro_export]
macro_rules! impl_circuit_generator_config_fields_for {
    ($impl_type:ty) => {
        impl crate::circuit::structure::circuit_generator::CGConfigFields for $impl_type {
            fn current_wire_id(&self) -> RcCell<i32> {
                self.currentWireId.clone()
            }
            fn evaluation_queue(
                &self,
            ) -> RcCell<HashMap<Box<dyn Instruction>, Box<dyn Instruction>>> {
                self.evaluationQueue.clone()
            }

            fn zero_wire(&self) -> RcCell<Option<WireType>> {
                self.zeroWire.clone()
            }
            fn one_wire(&self) -> Option<WireType> {
                self.oneWire.clone()
            }
            fn in_wires(&self) -> RcCell<Vec<Option<WireType>>> {
                self.inWires.clone()
            }
            fn out_wires(&self) -> RcCell<Vec<Option<WireType>>> {
                self.outWires.clone()
            }
            fn prover_witness_wires(&self) -> RcCell<Vec<Option<WireType>>> {
                self.prover_witness_wires().clone()
            }
            fn circuit_name(&self) -> String {
                self.circuitName.clone()
            }
            fn known_constant_wires(&self) -> RcCell<HashMap<BigInteger, WireType>> {
                self.knownConstantWires.clone()
            }
            fn num_of_constraints(&self) -> RcCell<i32> {
                self.numOfConstraints.clone()
            }
            fn circuit_evaluator(&self) -> RcCell<Option<CircuitEvaluator>> {
                self.circuitEvaluator.clone()
            }
        }
    };
}
// impl_circuit_generator_config_fields_for!(CircuitGenerator<CGBase>);

impl<T> crate::circuit::structure::circuit_generator::CGConfigFields for CircuitGenerator<T> {
    fn current_wire_id(&self) -> RcCell<i32> {
        self.currentWireId.clone()
    }
    fn evaluation_queue(&self) -> RcCell<HashMap<Box<dyn Instruction>, Box<dyn Instruction>>> {
        self.evaluationQueue.clone()
    }

    fn zero_wire(&self) -> RcCell<Option<WireType>> {
        self.zeroWire.clone()
    }
    fn one_wire(&self) -> Option<WireType> {
        self.oneWire.clone()
    }
    fn in_wires(&self) -> RcCell<Vec<Option<WireType>>> {
        self.inWires.clone()
    }
    fn out_wires(&self) -> RcCell<Vec<Option<WireType>>> {
        self.outWires.clone()
    }
    fn prover_witness_wires(&self) -> RcCell<Vec<Option<WireType>>> {
        self.prover_witness_wires().clone()
    }
    fn circuit_name(&self) -> String {
        self.circuitName.clone()
    }
    fn known_constant_wires(&self) -> RcCell<HashMap<BigInteger, WireType>> {
        self.knownConstantWires.clone()
    }
    fn num_of_constraints(&self) -> RcCell<i32> {
        self.numOfConstraints.clone()
    }
    fn circuit_evaluator(&self) -> RcCell<Option<CircuitEvaluator>> {
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
