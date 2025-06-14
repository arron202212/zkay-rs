#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::arc_cell_new;
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
use crate::util::util::ARcCell;
use crate::util::{
    run_command::run_command,
    util::{BigInteger, Util},
};
use dyn_clone::DynClone;
use lazy_static::lazy_static;
use rccell::RcCell;
// use crate::util::util::ARcCell;
use rccell::WeakCell;
use serde::{Serialize, de::DeserializeOwned};
use serde_closure::{Fn, FnMut, FnOnce};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg, Rem, Sub};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::{LazyLock, Mutex};
use std::{fmt::Debug, mem::size_of};
use zkay_derive::ImplStructNameConfig;
lazy_static! {
    static ref active_circuit_generators: ARcCell<HashMap<String, ARcCell<dyn CGConfig + Send + Sync>>> =
        arc_cell_new!(HashMap::<String, ARcCell<dyn CGConfig + Send + Sync>>::new());
    static ref CG_NAME: ARcCell<String> = arc_cell_new!("CGBase".to_owned());
}
//  ConcurrentHashMap<Long, CircuitGenerator> activeCircuitGenerators = new ConcurrentHashMap<>();
// 	  CircuitGenerator instance;

#[derive(Debug, Clone)]
pub struct CGBase;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone)]
pub struct CircuitGenerator<T: Debug> {
    pub current_wire_id: i32,
    pub evaluation_queue:
        HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>>,

    pub zero_wire: Option<WireType>,
    pub one_wire: Option<WireType>,

    pub in_wires: Vec<Option<WireType>>,
    pub out_wires: Vec<Option<WireType>>,
    pub prover_witness_wires: Vec<Option<WireType>>,

    pub circuit_name: String,

    pub known_constant_wires: HashMap<BigInteger, WireType>,

    pub num_of_constraints: i32,
    // pub circuitEvaluator: Option<CircuitEvaluator>,
    pub t: T,
    // me: Option<WeakCell<Self>>,
}

pub trait CGConfigFields: Debug {
    fn current_wire_id(&mut self) -> &mut i32;
    fn evaluation_queue(
        &mut self,
    ) -> &mut HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>>;
    fn zero_wire(&mut self) -> &mut Option<WireType>;
    fn one_wire(&mut self) -> &mut Option<WireType>;
    fn in_wires(&mut self) -> &mut Vec<Option<WireType>>;
    fn out_wires(&mut self) -> &mut Vec<Option<WireType>>;
    fn prover_witness_wires(&mut self) -> &mut Vec<Option<WireType>>;
    fn circuit_name(&mut self) -> &mut String;
    fn known_constant_wires(&mut self) -> &mut HashMap<BigInteger, WireType>;
    fn num_of_constraints(&mut self) -> &mut i32;
    // fn circuit_evaluator(&self) -> Option<CircuitEvaluator>;
    fn get_name(&self) -> String;
    fn get_zero_wire(&self) -> Option<WireType>;
    fn get_one_wire(&self) -> Option<WireType>;

    fn get_evaluation_queue(
        &self,
    ) -> &HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>>;

    fn get_num_wires(&self) -> i32;
    fn get_current_wire_id(&self) -> i32;

    fn get_num_of_constraints(&self) -> i32;

    fn get_in_wires(&self) -> &Vec<Option<WireType>>;

    fn get_out_wires(&self) -> &Vec<Option<WireType>>;

    fn get_prover_witness_wires(&self) -> &Vec<Option<WireType>>;
}

dyn_clone::clone_trait_object!(CGConfig);

pub fn getActiveCircuitGenerator() -> eyre::Result<ARcCell<dyn CGConfig + Send + Sync>> {
    // if !Configs.runningMultiGenerators {
    //     return Ok(instance);
    // }

    // let threadId = Thread.currentThread().getId();
    // let currentGenerator = activeCircuitGenerators.get(threadId);

    // currentGenerator.ok_or(eyre::eyre!(
    //     "The current thread does not have any active circuit generators"
    // ))
    // eyre::bail!("The current thread does not have any active circuit generators")

    let cg_name: String = CG_NAME.lock().to_owned();
    //println!("====cg_name=========={cg_name}");
    active_circuit_generators
        .lock()
        .get(&cg_name)
        .cloned()
        .ok_or(eyre::eyre!(
            "The current thread does not have any active circuit generators"
        ))
}
pub fn put_active_circuit_generator(name: &str, cg: ARcCell<dyn CGConfig + Send + Sync>) {
    *CG_NAME.lock() = name.to_owned();
    active_circuit_generators.lock().insert(name.to_owned(), cg);
}

impl<T: StructNameConfig + Debug> CircuitGenerator<T> {
    pub fn new(circuit_name: &str, t: T) -> Self {
        if Configs.running_multi_generators {
            // activeCircuitGenerators.put(Thread.currentThread().getId(), this);
        }
        CircuitGenerator::<T> {
            circuit_name: circuit_name.to_owned(),
            in_wires: vec![],
            out_wires: vec![],
            zero_wire: None,
            one_wire: None,
            prover_witness_wires: vec![],
            evaluation_queue: HashMap::new(),
            known_constant_wires: HashMap::new(),
            current_wire_id: 0,
            num_of_constraints: 0,
            // circuitEvaluator: None,
            t,
        }
        // let mut selfs = RcCell(Rc::new_cyclic(|_me| {
        //     RefCell::new(Self {
        //         circuit_name: circuit_name.to_owned(),
        //         in_wires: vec![],
        //         out_wires: vec![],
        //         zero_wire: None,
        //         one_wire: None,
        //         prover_witness_wires: vec![],
        //         evaluation_queue: HashMap::new(),
        //         known_constant_wires: HashMap::new(),
        //         current_wire_id: 0,
        //         num_of_constraints: 0,
        //         // circuitEvaluator: None,
        //         t,
        //         me: None,
        //     })
        // }));
        // let weakselfs = selfs.downgrade();
        // selfs.borrow_mut().me = Some(weakselfs.clone());
        // selfs
    }
}

//+ CreateConstantWire + CreateConstantWireArray + CreateNegConstantWire
pub trait CGConfig: DynClone + CGConfigFields + StructNameConfig {
    fn buildCircuit(&mut self) {}
    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {}
    fn generateCircuit(&mut self) {
        println!("Running Circuit Generator for <  {}  >", self.get_name());

        self.initCircuitConstruction();
        //println!("{},{}",file!(),line!());
        self.buildCircuit();

        println!(
            "Circuit Generation Done for < {} > \n \t Total Number of Constraints : {} \n \t Total Number of Wires : {}",
            self.get_name(),
            self.get_num_of_constraints(),
            self.get_num_wires()
        );
    }

    fn createInputWire(&mut self, desc: &Option<String>) -> WireType {
        let newInputWire = WireType::Variable(new_variable(self.get_current_wire_id()));
        *self.current_wire_id() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            newInputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        self.in_wires().push(Some(newInputWire.clone()));
        newInputWire
    }

    fn createInputWireArray(&mut self, n: usize, desc: &Option<String>) -> Vec<Option<WireType>> {
        let mut list = vec![None; n];
        for i in 0..n {
            list[i] = Some(self.createInputWire(&desc.as_ref().map(|d| format!("{} {i}", d))));
        }
        list
    }

    fn createLongElementInput(&mut self, totalBitwidth: i32, desc: &Option<String>) -> LongElement {
        let numWires =
            (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as usize;
        let w = self.createInputWireArray(numWires, desc);
        let mut bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; numWires];
        if numWires as i32 * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = (totalBitwidth % LongElement::CHUNK_BITWIDTH) as u64;
        }
        LongElement::new(w, bitwidths)
    }

    fn createLongElementProverWitness(
        &mut self,
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
        LongElement::new(w, bitwidths)
    }

    fn createProverWitnessWire(&mut self, desc: &Option<String>) -> WireType {
        let wire = WireType::Variable(new_variable(self.get_current_wire_id()));
        *self.current_wire_id() += 1;
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::nizkinput,
            wire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        self.prover_witness_wires().push(Some(wire.clone()));
        wire
    }

    fn createProverWitnessWireArray(
        &mut self,
        n: usize,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut ws = vec![None; n];
        for k in 0..n {
            ws[k] =
                Some(self.createProverWitnessWire(&desc.as_ref().map(|d| format!("{} {k}", d))));
        }
        ws
    }

    fn generateZeroWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        vec![self.get_zero_wire().clone(); n]
    }

    fn generateOneWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        vec![self.get_one_wire().clone(); n]
    }

    fn makeOutput(&mut self, wire: WireType, desc: &Option<String>) -> WireType {
        let mut outputWire = wire.clone();
        if !(wire.instance_of("VariableWire") || wire.instance_of("VariableBitWire"))
            || self.get_in_wires().contains(&Some(wire.clone()))
        {
            wire.packIfNeeded(&None);
            outputWire = self.makeVariable(wire.clone(), desc);
        } else if self.get_in_wires().contains(&Some(wire.clone()))
            || self
                .get_prover_witness_wires()
                .contains(&Some(wire.clone()))
        {
            outputWire = self.makeVariable(wire.clone(), desc);
        } else {
            wire.packIfNeeded(&None);
        }

        self.out_wires().push(Some(outputWire.clone()));
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::output,
            outputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
        outputWire
    }

    fn makeVariable(&mut self, wire: WireType, desc: &Option<String>) -> WireType {
        let mut outputWire = WireType::Variable(new_variable(self.get_current_wire_id()));
        *self.current_wire_id() += 1;
        let op = new_mul(
            wire,
            self.get_one_wire().clone().unwrap(),
            outputWire.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let cachedOutputs = self.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.current_wire_id() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        outputWire
    }

    fn makeOutputArray(
        &mut self,
        wires: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut outs = vec![None; wires.len()];
        for i in 0..wires.len() {
            outs[i] = Some(self.makeOutput(
                wires[i].clone().unwrap(),
                &desc.as_ref().map(|d| format!("{}[{i}]", d)),
            ));
        }
        outs
    }

    fn addDebugInstruction(&mut self, w: WireType, desc: &Option<String>) {
        w.packIfNeeded(&None);
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::debug,
            w,
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        )));
    }

    fn addDebugInstructiona(&mut self, wires: Vec<Option<WireType>>, desc: &Option<String>) {
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
        let mut printWriter = File::create(self.get_name() + ".arith").unwrap();
        write!(printWriter, "total {}", self.get_current_wire_id());
        let keys = self.get_evaluation_queue().keys().clone();
        for e in keys {
            if e.doneWithinCircuit() {
                let _ = write!(printWriter, "{e:?} \n");
            }
        }
    }

    fn printCircuit(&self) {
        let keys = self.get_evaluation_queue().keys().clone();
        for e in keys {
            if e.doneWithinCircuit() {
                //println!("{e:?}");
            }
        }
    }

    fn initCircuitConstruction(&mut self) {
        let one_wire = WireType::Constant(new_constant(self.get_current_wire_id(), Util::one()));
        //println!("{},{}",file!(),line!());
        *self.one_wire() = Some(one_wire.clone());
        println!(
            "==**********initCircuitConstruction************=={:?}====*self.one_wire() ========{:?}",
            self.name(),
            self.get_one_wire()
        );
        *self.current_wire_id() += 1;
        self.known_constant_wires()
            .insert(Util::one(), one_wire.clone());
        //println!("{},{}",file!(),line!());
        self.addToEvaluationQueue(Box::new(WireLabelInstruction::new(
            LabelType::input,
            one_wire.clone(),
            "The one-input wire.".to_owned(),
        )));
        //println!("{},{}",file!(),line!());
        self.in_wires().push(Some(one_wire.clone()));
        *self.zero_wire() = Some(one_wire.muli(0, &None));
    }

    fn createConstantWire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().mulb(x, desc)
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
        w
    }

    fn createConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().muli(x, desc)
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
        w
    }

    fn createNegConstantWire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().mulb(x.neg(), desc)
    }

    fn createNegConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().muli(-x, desc)
    }

    /**
     * Use to support computation for prover witness values outside of the
     * circuit. See Mod_Gadget and Field_Division gadgets for examples.
     *
     * @param instruction
     */
    fn specifyProverWitnessComputation(&mut self, e: Box<dyn Instruction + Send + Sync>) {
        // serde_json::to_string(&f).unwrap()
        // let f: FnOnce( &mut CircuitEvaluator)
        // 			+ Serialize
        // 			+ DeserializeOwned
        // 			+ PartialEq
        // 			+ Eq
        // 			+ Clone
        // 			+ Debug+Hash = serde_json::from_str(&f).unwrap();
        //         // let f= Fn!(move &|evaluator: &mut CircuitEvaluator| f(evaluator));
        //         // let k=format!("{f}");
        //         // self.addToEvaluationQueues();
        //     //     let fff=move |evaluator: &mut CircuitEvaluator|{ ff(evaluator)};
        //     //    let f=FnOnce!(move |evaluator: &mut CircuitEvaluator|{ fff(evaluator)});
        //          #[derive(Hash, Clone, Debug)]
        //             struct Prover<F: FnOnce( &mut CircuitEvaluator)
        // 			+ Serialize
        // 			+ DeserializeOwned
        // 			+ PartialEq
        // 			+ Eq
        // 			+ Clone
        // 			+ Debug+Hash> {
        //                 pub f:F,
        //             }
        //              impl<F: FnOnce( &mut CircuitEvaluator)
        // 			+ Serialize
        // 			+ DeserializeOwned
        // 			+ PartialEq
        // 			+ Eq
        // 			+ Clone
        // 			+ Debug+Hash> Instruction for Prover<F>{
        //                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
        //                     (self.f).clone()(evaluator);
        //                 }
        //             }
        //         impl<F: FnOnce( &mut CircuitEvaluator)
        // 			+ Serialize
        // 			+ DeserializeOwned
        // 			+ PartialEq
        // 			+ Eq
        // 			+ Clone
        // 			+ Debug+Hash> InstanceOf for Prover<F>{
        //                  fn instance_of(&self, name: &str) -> bool {
        //                     self.name() == name
        //                     }
        //             }
        //  impl<F: FnOnce( &mut CircuitEvaluator)
        // 			+ Serialize
        // 			+ DeserializeOwned
        // 			+ PartialEq
        // 			+ Eq
        // 			+ Clone
        // 			+ Debug+Hash> StructNameConfig for Prover<F>{
        //                 fn name(&self) -> String {
        //                     String::new()
        //                 }
        //             }
        self.addToEvaluationQueue(e);
    }
    // fn addToEvaluationQueue(
    //     &mut self,
    //     e: Box<dyn Instruction + Send + Sync>,
    // ) -> Option<Vec<Option<WireType>>> {
    //     // self.addToEvaluationQueues(Box<dyn Instruction + Send + Sync>::Trait( e))
    //     None
    // }
    fn addToEvaluationQueue(
        &mut self,
        e: Box<dyn Instruction + Send + Sync>,
    ) -> Option<Vec<Option<WireType>>> {
        let evaluation_queue = self.evaluation_queue();
        if let Some(existingInstruction) = evaluation_queue.get(&e) {
            return existingInstruction.basic_op().map(|op| op.getOutputs());
        }

        evaluation_queue.entry(e.clone()).or_insert(e.clone());
        if e.instance_of("BasicOp") {
            *self.num_of_constraints() += e.basic_op().as_ref().unwrap().getNumMulGates();
        }
        None // returning null means we have not seen this instruction before
        // have seen this instruction before, but can't de-duplicate

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
            self.get_num_of_constraints()
        );
    }

    /**
     * Asserts an r1cs constraint. w1*w2 = w3
     *
     */
    fn addAssertion(&mut self, w1: WireType, w2: WireType, w3: WireType, desc: &Option<String>) {
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

    fn addZeroAssertion(&mut self, w: WireType, desc: &Option<String>) {
        self.addAssertion(
            w,
            self.get_one_wire().clone().unwrap(),
            self.get_zero_wire().clone().unwrap(),
            desc,
        );
    }

    fn addOneAssertion(&mut self, w: WireType, desc: &Option<String>) {
        self.addAssertion(
            w,
            self.get_one_wire().clone().unwrap(),
            self.get_one_wire().clone().unwrap(),
            desc,
        );
    }

    fn addBinaryAssertion(&mut self, w: WireType, desc: &Option<String>) {
        let inv = w.invAsBit(desc).unwrap();
        self.addAssertion(w, inv, self.get_zero_wire().clone().unwrap(), desc);
    }

    fn addEqualityAssertion(&mut self, w1: WireType, w2: WireType, desc: &Option<String>) {
        if w1 != w2 {
            self.addAssertion(w1, self.get_one_wire().clone().unwrap(), w2, desc);
        }
    }

    fn addEqualityAssertionb(&mut self, w1: WireType, b: BigInteger, desc: &Option<String>) {
        self.addAssertion(
            w1,
            self.get_one_wire().clone().unwrap(),
            self.createConstantWire(b, desc),
            desc,
        );
    }

    fn evalCircuit(&mut self) -> CircuitEvaluator {
        let mut circuitEvaluator = CircuitEvaluator::new(&self.name());
        self.generateSampleInput(&mut circuitEvaluator);
        circuitEvaluator.evaluate();
        // *self.circuit_evaluator() = Some(circuitEvaluator);
        circuitEvaluator
    }

    fn prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.writeCircuitFile();
        assert!(
            circuit_evaluator.is_some(),
            "evalCircuit() must be called before prepFiles()"
        );
        circuit_evaluator.as_ref().unwrap().writeInputFile();
    }

    fn runLibsnark(&self) {
        let p = run_command(
            vec![
                &Configs.libsnark_exec.clone(),
                &(self.get_name().clone() + ".arith"),
                &(self.get_name().clone() + ".in"),
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

    // fn getCircuitEvaluator(&self) -> CircuitEvaluator {
    //     assert!(
    //         self.circuit_evaluator().is_some(),
    //         "evalCircuit() must be called before getCircuitEvaluator()"
    //     );

    //     return self.circuit_evaluator().clone().unwrap();
    // }
}

impl<T: Debug> crate::circuit::structure::circuit_generator::CGConfigFields
    for CircuitGenerator<T>
{
    fn current_wire_id(&mut self) -> &mut i32 {
        &mut self.current_wire_id
    }
    fn evaluation_queue(
        &mut self,
    ) -> &mut HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>> {
        &mut self.evaluation_queue
    }

    fn zero_wire(&mut self) -> &mut Option<WireType> {
        &mut self.zero_wire
    }
    fn one_wire(&mut self) -> &mut Option<WireType> {
        &mut self.one_wire
    }
    fn in_wires(&mut self) -> &mut Vec<Option<WireType>> {
        &mut self.in_wires
    }
    fn out_wires(&mut self) -> &mut Vec<Option<WireType>> {
        &mut self.out_wires
    }
    fn prover_witness_wires(&mut self) -> &mut Vec<Option<WireType>> {
        &mut self.prover_witness_wires
    }
    fn circuit_name(&mut self) -> &mut String {
        &mut self.circuit_name
    }
    fn known_constant_wires(&mut self) -> &mut HashMap<BigInteger, WireType> {
        &mut self.known_constant_wires
    }
    fn num_of_constraints(&mut self) -> &mut i32 {
        &mut self.num_of_constraints
    }
    // fn circuit_evaluator(&self) -> Option<CircuitEvaluator> {
    //     self.circuitEvaluator.clone()
    // }
    fn get_name(&self) -> String {
        self.circuit_name.clone()
    }
    fn get_zero_wire(&self) -> Option<WireType> {
        self.zero_wire.clone()
    }
    fn get_one_wire(&self) -> Option<WireType> {
        println!("=====get_one_wire============={:?}", self.get_name());
        self.one_wire.clone()
    }

    fn get_evaluation_queue(
        &self,
    ) -> &HashMap<Box<dyn Instruction + Send + Sync>, Box<dyn Instruction + Send + Sync>> {
        &self.evaluation_queue
    }

    fn get_num_wires(&self) -> i32 {
        self.get_current_wire_id()
    }
    fn get_current_wire_id(&self) -> i32 {
        self.current_wire_id
    }

    fn get_num_of_constraints(&self) -> i32 {
        self.num_of_constraints
    }

    fn get_in_wires(&self) -> &Vec<Option<WireType>> {
        &self.in_wires
    }

    fn get_out_wires(&self) -> &Vec<Option<WireType>> {
        &self.out_wires
    }

    fn get_prover_witness_wires(&self) -> &Vec<Option<WireType>> {
        &self.prover_witness_wires
    }
}

pub trait CreateConstantWire<T = WireType> {
    fn create_constant_wire(&self, x: T, desc: &Option<String>) -> WireType;
}

impl<T: Debug> CreateConstantWire<BigInteger> for CircuitGenerator<T>
where
    CircuitGenerator<T>: CGConfig,
{
    fn create_constant_wire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().mulb(x, desc)
    }
}
impl<T: Debug> CreateConstantWire<i64> for CircuitGenerator<T>
where
    CircuitGenerator<T>: CGConfig,
{
    fn create_constant_wire(&self, x: i64, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().muli(x, desc)
    }
}
pub trait CreateConstantWireArray<T = WireType> {
    fn create_constant_wire_array(&self, a: T, desc: &Option<String>) -> Vec<Option<WireType>>;
}
impl<T: Debug> CreateConstantWireArray<Vec<BigInteger>> for CircuitGenerator<T>
where
    CircuitGenerator<T>: CGConfig,
{
    fn create_constant_wire_array(
        &self,
        a: Vec<BigInteger>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.create_constant_wire(a[i].clone(), desc));
        }
        w
    }
}
impl<T: Debug> CreateConstantWireArray<Vec<i64>> for CircuitGenerator<T>
where
    CircuitGenerator<T>: CGConfig,
{
    fn create_constant_wire_array(
        &self,
        a: Vec<i64>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut w = vec![None; a.len()];
        for i in 0..a.len() {
            w[i] = Some(self.create_constant_wire(a[i], desc));
        }
        w
    }
}

pub trait CreateNegConstantWire<T = WireType> {
    fn create_neg_constant_wire(&self, x: T, desc: &Option<String>) -> WireType;
}
impl<T: Debug> CreateNegConstantWire<BigInteger> for CircuitGenerator<T>
where
    CircuitGenerator<T>: CGConfig,
{
    fn create_neg_constant_wire(&self, x: BigInteger, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().mulb(x.neg(), desc)
    }
}
impl<T: Debug> CreateNegConstantWire<i64> for CircuitGenerator<T>
where
    CircuitGenerator<T>: CGConfig,
{
    fn create_neg_constant_wire(&self, x: i64, desc: &Option<String>) -> WireType {
        self.get_one_wire().clone().unwrap().muli(-x, desc)
    }
}

// #[macro_export]
// macro_rules! impl_specify_prover_witness_computation_for {
//     ($impl_type:ty) => {
//         impl  $impl_type {

//         }
//     };
// }

// impl_specify_prover_witness_computation_for!(CircuitGenerator<CGBase>);

#[macro_export]
macro_rules! to_closure_str {
    ($expr:expr) => {
        serde_json::to_string(&$expr).unwrap()
    };
}

// ($vis:vis fn $name:ident(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
// $vis fn $name(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {
//      match self{
//         Self::Web3TesterBlockchain(tester)=>tester.$name($($arg_name),*),
//         Self::Web3HttpGanacheBlockchain(ganache)=>ganache.$name($($arg_name),*),
//     }
// }
#[macro_export]
macro_rules! impl_instruction_for_prover {
    ( $body:block ) => {
                    impl  Instruction for Prover {
                        fn evaluate(&self, evaluator: &mut CircuitEvaluator)
                            $body

                    }


    };
}
#[macro_export]
macro_rules! impl_prover {
    (  eval($($arg_name:ident : $arg_ty:ty),*  )$body:block ) => {
                    {#[derive(Hash, Clone, Debug, ImplStructNameConfig)]
                    struct Prover {
                        $( pub $arg_name : $arg_ty),*
                    }
                    // impl  Instruction for Prover {
                        // fn evaluate(&self, evaluator: &mut CircuitEvaluator)
                        $body

                    // }
                    Box::new(Prover {
                        $( $arg_name : $arg_name.clone()),*
                    })}

    };
}

// struct Test<T>{
// me: Option<WeakCell<Self>>,
// _t:PhantomData<T>
// }

// trait TestConfig:Sized{
//     fn config(&self)->RcCell<impl TestConfig>;
// }
// struct A;
// impl TestConfig for Test<A>{
//     fn config(&self)->RcCell<impl TestConfig>{
//         self.me.clone().unwrap().upgrade().unwrap()
//     }
// }
// impl<T> Test<T>{
// pub fn new()->RcCell<Test<T>>{
// let mut selfs = RcCell(Rc::new_cyclic(|_me| {
//             RefCell::new(Self {
//                 me: None,
//                 _t:PhantomData,
//             })
//         }));
//         let weakselfs = selfs.downgrade();
//         selfs.borrow_mut().me = Some(weakselfs.clone());
//     selfs
// }}
