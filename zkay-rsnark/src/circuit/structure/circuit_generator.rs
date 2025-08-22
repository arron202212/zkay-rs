#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fs::File,
    hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher},
    io::{BufReader, Write},
    marker::PhantomData,
    ops::{Add, Mul, Neg, Rem, Sub},
    rc::Rc,
    sync::{Arc, LazyLock, Mutex},
    time::Instant,
    {fmt::Debug, mem::size_of},
};

use ahash::RandomState;
use dyn_clone::DynClone;
use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use nohash_hasher::{BuildNoHashHasher, NoHashHasher};
use rccell::{RcCell, WeakCell};
use rustc_hash::FxBuildHasher;
use serde::{Serialize, de::DeserializeOwned};
use serde_closure::{Fn, FnMut, FnOnce};
use zkay_derive::ImplStructNameConfig;
lazy_static! {
    static ref active_circuit_generators: ARcCell<HashMap<String, ARcCell<dyn CGConfig + Send + Sync>>> =
        arc_cell_new!(HashMap::<String, ARcCell<dyn CGConfig + Send + Sync>>::new());
    static ref CG_NAME: ARcCell<String> = arc_cell_new!("CGBase".to_owned());
}
//  ConcurrentHashMap<Long, CircuitGenerator> activeCircuitGenerators = new ConcurrentHashMap<>();
// 	  CircuitGenerator instance;

// use std::{collections::HashMap, time::Instant};

// pub type LinkedHashMap<K, V> = HashMap<K, V, BuildNoHashHasher<K>>;

#[derive(Debug, Clone)]
pub struct CGBase;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CircuitGenerator {
    pub current_wire_id: i32,
    pub evaluation_queue: LinkedHashMap<u64, Box<dyn Instruction>>,

    pub zero_wire: Option<WireType>,
    pub one_wire: Option<WireType>,

    pub num_of_constraints: i32,
    pub known_constant_wires: BTreeMap<BigInteger, WireType>,
    pub in_wires: Vec<Option<WireType>>,
    pub out_wires: Vec<Option<WireType>>,
    pub prover_witness_wires: Vec<Option<WireType>>,

    pub circuit_name: String,

    // pub num_of_constraints: i32,
    pub me: Option<WeakCell<Self>>,
}

impl CGInstance for CircuitGenerator {
    fn cg(&self) -> RcCell<CircuitGenerator> {
        self.me.clone().unwrap().upgrade().unwrap()
    }
    fn cg_weak(&self) -> WeakCell<CircuitGenerator> {
        self.me.clone().unwrap()
    }
    // fn generator(&self) -> &CircuitGenerator{
    // self
    // }
}

#[derive(Debug, Clone)]
pub struct CircuitGeneratorExtend<T: Debug> {
    // pub circuitEvaluator: Option<CircuitEvaluator>,
    pub cg: RcCell<CircuitGenerator>,
    // pub generator: CircuitGenerator,
    pub t: T,
    // pub me: Option<WeakCell<Self>>,
}

impl CircuitGenerator {
    pub fn new(circuit_name: &str) -> RcCell<Self> {
        if Configs.running_multi_generators {
            // activeCircuitGenerators.put(Thread.currentThread().getId(), this);
        }
        // CircuitGenerator::<T> {
        //     circuit_name: circuit_name.to_owned(),
        //     in_wires: vec![],
        //     out_wires: vec![],
        //     zero_wire: None,
        //     one_wire: None,
        //     prover_witness_wires: vec![],
        //     evaluation_queue: HashMap::new(),
        //     known_constant_wires: HashMap::new(),
        //     current_wire_id: 0,
        //     num_of_constraints: 0,
        //     // circuitEvaluator: None,
        //     t,
        // }
        let mut selfs = RcCell(Rc::new_cyclic(|_me| {
            RefCell::new(Self {
                zero_wire: None,
                one_wire: None,
                // prover_witness_wires: vec![],
                evaluation_queue: LinkedHashMap::default(),
                known_constant_wires: BTreeMap::new(),
                current_wire_id: 0,
                num_of_constraints: 0,
                circuit_name: circuit_name.to_owned(),
                in_wires: vec![],
                out_wires: vec![],
                prover_witness_wires: vec![],
                // evaluation_queue: HashMap::new(),
                // known_constant_wires: HashMap::new(),
                // num_of_constraints: 0,
                me: None,
            })
        }));
        let weakselfs = selfs.downgrade();
        selfs.borrow_mut().me = Some(weakselfs.clone());
        selfs
    }
}

impl<T: Debug> CircuitGeneratorExtend<T> {
    pub fn newc(cg: RcCell<CircuitGenerator>, t: T) -> Self {
        // let generator = cg.borrow().clone();
        Self { cg, t }
    }
    pub fn new(circuit_name: &str, t: T) -> Self {
        if Configs.running_multi_generators {
            // activeCircuitGenerators.put(Thread.currentThread().getId(), this);
        }
        let cg = CircuitGenerator::new(circuit_name);
        // let generator = cg.borrow().clone();
        CircuitGeneratorExtend::<T> { cg, t }
        // let mut selfs = RcCell(Rc::new_cyclic(|_me| {
        //     RefCell::new(Self {
        //         circuit_name: circuit_name.to_owned(),
        //         in_wires: vec![],
        //         out_wires: vec![],
        //         prover_witness_wires: vec![],
        //         // evaluation_queue: HashMap::new(),
        //         // known_constant_wires: HashMap::new(),
        //         // num_of_constraints: 0,
        //         me: None,
        //         cg:CircuitGenerator::new()
        //     })
        // }));
        // let weakselfs = selfs.downgrade();
        // selfs.borrow_mut().me = Some(weakselfs.clone());
        // selfs
    }
}

impl<T: Debug> CGInstance for CircuitGeneratorExtend<T> {
    fn cg(&self) -> RcCell<CircuitGenerator> {
        self.cg.clone()
    }
    fn cg_weak(&self) -> WeakCell<CircuitGenerator> {
        self.cg.clone().downgrade()
    }
    // fn generator(&self) -> &CircuitGenerator {
    //     &self.generator
    // }
}

pub fn addToEvaluationQueue(
    cg: RcCell<CircuitGenerator>,
    e: Box<dyn Instruction>,
) -> Option<Vec<Option<WireType>>> {
    use std::time::Instant;
    let start = Instant::now();
    // let mut m=std::collections::HashMap::new();
    // let evaluation_queue = self.get_evaluation_queue();
    // println!(
    //     "End +++++++++++++addToEvaluationQueue 0 Time: == {:?} ",
    //     start.elapsed()
    // );
    // let mut ss = DefaultHasher::new();
    // 1i32.hash(&mut ss);
    // 2i32.hash(&mut ss);
    // let hash_codes = ss.finish();
    // println!("===hash_codes========{hash_codes}======");
    // let mut sss = DefaultHasher::new();
    // 2i32.hash(&mut sss);
    // 1i32.hash(&mut sss);
    // let hash_codess = sss.finish();
    // println!("===hash_codess========{hash_codess}======");

    let mut s = DefaultHasher::new();
    e.hash(&mut s);
    let hash_code = s.finish();
    // assert!(3899388557723912248 != hash_code, "===e===e========{e:?}");
    ///TEST
    // let mut s:BuildHasherDefault<NoHashHasher<Box<dyn Instruction>>> = BuildNoHashHasher::new();//:
    // // let hash_code = <BuildHasherDefault<NoHashHasher<_>> as BuildHasher>::hash_one::<Box<dyn Instruction>>(&s, e);
    // let hash_code=FxBuildHasher.hash_one(&e);
    // println!(
    //     "End +++++++++++++addToEvaluationQueue 4200 Time: == {:?} ",
    //     start.elapsed()
    // );
    // let hash_builder = RandomState::with_seed(42);
    // let hash_code = hash_builder.hash_one(&e);
    //    println!(
    //         "End +++++++++++++addToEvaluationQueue 42 Time: == {:?} ",
    //         start.elapsed()
    //     );
    if let Some(existingInstruction) = cg.borrow().evaluation_queue.get(&hash_code) {
        // println!(
        //     "End ++++++++++addToEvaluationQueue 01 Time: ===hash_code====={hash_code}======== {:?} ",
        //     start.elapsed()
        // );
        return existingInstruction.basic_op().map(|op| op.getOutputs());
    }
    // println!(
    //     "End +++++++++++++addToEvaluationQueue 33 Time: == {} s",
    //     start.elapsed().as_micros()
    // );
    //    let mut s = DefaultHasher::new();
    //     e.hash(&mut s);
    //    let h= s.finish();
    // println!(
    //     "End +++++++++++++addToEvaluationQueue 333 Time: == {} s",
    //     start.elapsed().as_micros()
    // );

    // m.insert(hash_code,e.clone());
    // println!(
    //     "End +++++++++++++addToEvaluationQueue 3333 Time: == {} s",
    //     start.elapsed().as_micros()
    // );

    // println!(
    //     "End +++++++++++++addToEvaluationQueue 1 Time: == {:?} s",
    //     start.elapsed()
    // );
    if e.name().ends_with("Op") {
        //BasicOp
        // print!("====e===={}===",e.name());
        cg.borrow_mut().num_of_constraints += e.basic_op().as_ref().unwrap().getNumMulGates();
    }
    // println!(
    //     "End +++++++++++++addToEvaluationQueue 2 Time: == {:?} ",
    //     start.elapsed()
    // );
    // println!("==hash_code===={hash_code}====e======{e:?}=========");
    cg.borrow_mut().evaluation_queue.insert(hash_code, e);
    // .entry(e.clone())
    // .or_insert(e.clone());
    // println!(
    //     "End +++++++++++++addToEvaluationQueue 3 Time: == {:?} ",
    //     start.elapsed()
    // );

    // println!(
    //     "End +++++++++++++addToEvaluationQueue 0 Time: == {:?} ",
    //     start.elapsed()
    // );
    None // returning None means we have not seen this instruction before
    // have seen this instruction before, but can't de-duplicate

    // if existingInstruction.unwrap().instance_of("BasicOp") {
    //     return Some(existingInstruction.unwrap().basic_op().unwrap().getOutputs());
    // } else {
    //     return None;
    // }
}
pub trait CGInstance {
    fn cg(&self) -> RcCell<CircuitGenerator>;
    fn cg_weak(&self) -> WeakCell<CircuitGenerator>;
    // fn generator(&self) -> &CircuitGenerator;
}

impl<T: CGInstance> CGInstance for RcCell<T> {
    fn cg(&self) -> RcCell<CircuitGenerator> {
        self.borrow().cg()
    }
    fn cg_weak(&self) -> WeakCell<CircuitGenerator> {
        self.borrow().cg_weak()
    }
    // fn generator(&self) -> &CircuitGenerator{
    //     self.borrow().generator()
    // }
}

impl<T: Debug> CGConfigFields for CircuitGeneratorExtend<T> {
    fn get_zero_wire(&self) -> Option<WireType> {
        self.cg.borrow().zero_wire.clone()
    }
    fn get_one_wire(&self) -> Option<WireType> {
        // println!("=====get_one_wire============={:?}", self.get_name());
        self.cg.borrow().one_wire.clone()
    }

    fn get_evaluation_queue(&self) -> LinkedHashMap<u64, Box<dyn Instruction>> {
        self.cg.borrow().evaluation_queue.clone()
    }
    fn get(&self, hash_code: u64) -> Option<Box<dyn Instruction>> {
        self.cg.borrow().evaluation_queue.get(&hash_code).cloned()
    }
    fn get_current_wire_id(&self) -> i32 {
        // assert! (175548!=self.cg.borrow().current_wire_id);///TEST
        self.cg.borrow().current_wire_id
    }
    fn get_num_of_constraints(&self) -> i32 {
        self.cg.borrow().num_of_constraints
    }
    fn get_known_constant_wires(&self) -> BTreeMap<BigInteger, WireType> {
        self.cg.borrow().known_constant_wires.clone()
    }
    fn get_name(&self) -> String {
        self.cg.borrow().circuit_name.clone()
    }

    fn get_num_wires(&self) -> i32 {
        self.cg.borrow().get_current_wire_id()
    }

    fn get_in_wires(&self) -> Vec<Option<WireType>> {
        self.cg.borrow().in_wires.clone()
    }

    fn get_out_wires(&self) -> Vec<Option<WireType>> {
        self.cg.borrow().out_wires.clone()
    }

    fn get_prover_witness_wires(&self) -> Vec<Option<WireType>> {
        self.cg.borrow().prover_witness_wires.clone()
    }
}

// impl CGConfigFields for CircuitGenerator{

//     fn get_name(&self) -> String {
//         self.circuit_name.clone()
//     }

//     fn get_num_wires(&self) -> i32 {
//         self.cg.get_current_wire_id()
//     }

//     fn get_num_of_constraints(&self) -> i32 {
//         self.num_of_constraints
//     }

//     fn get_in_wires(&self) -> Vec<Option<WireType>> {
//        self.in_wires.clone()
//     }

//     fn get_out_wires(&self) -> Vec<Option<WireType>> {
//        self.out_wires.clone()
//     }

//     fn get_prover_witness_wires(&self) -> Vec<Option<WireType>> {
//        self.prover_witness_wires.clone()
//     }
// }

pub trait CGConfigFields: CGInstance + Debug {
    fn get_known_constant_wires(&self) -> BTreeMap<BigInteger, WireType>;

    fn get_zero_wire(&self) -> Option<WireType>;
    fn get_one_wire(&self) -> Option<WireType>;

    fn get_evaluation_queue(&self) -> LinkedHashMap<u64, Box<dyn Instruction>>;
    fn get(&self, hash_code: u64) -> Option<Box<dyn Instruction>>;
    fn get_current_wire_id(&self) -> i32;
    fn get_num_of_constraints(&self) -> i32;
    // fn get_prover_witness_wires(&self) -> Vec<Option<WireType>>;

    // fn in_wires(&mut self) -> &mut Vec<Option<WireType>>;
    // fn out_wires(&mut self) -> &mut Vec<Option<WireType>>;
    // fn prover_witness_wires(&mut self) -> &mut Vec<Option<WireType>>;
    // fn circuit_name(&mut self) -> &mut String;

    // fn circuit_evaluator(&self) -> Option<CircuitEvaluator>;
    fn get_name(&self) -> String;

    fn get_num_wires(&self) -> i32;

    fn get_in_wires(&self) -> Vec<Option<WireType>>;

    fn get_out_wires(&self) -> Vec<Option<WireType>>;
    //  fn get_num_of_constraints(&self) -> i32;
    fn get_prover_witness_wires(&self) -> Vec<Option<WireType>>;
}

// dyn_clone::clone_trait_object!(CGConfig);
// impl  Clone for Box<dyn CGConfig> {
//     fn clone(&self) -> Self {
//         dyn_clone::clone_box(&**self)
//     }
// }

pub fn getActiveCircuitGenerator() { //-> eyre::Result<ARcCell<dyn CGConfig + Send + Sync>>
    // if !Configs.runningMultiGenerators {
    //     return Ok(instance);
    // }

    // let threadId = Thread.currentThread().getId();
    // let currentGenerator = activeCircuitGenerators.get(threadId);

    // currentGenerator.ok_or(eyre::eyre!(
    //     "The current thread does not have any active circuit generators"
    // ))
    // eyre::bail!("The current thread does not have any active circuit generators")

    // let cg_name: String = CG_NAME.lock().to_owned();
    // //println!("====cg_name=========={cg_name}");
    // active_circuit_generators
    //     .lock()
    //     .get(&cg_name)
    //     .cloned()
    //     .ok_or(eyre::eyre!(
    //         "The current thread does not have any active circuit generators"
    //     ))
}
pub fn put_active_circuit_generator(name: &str, cg: &str) { //ARcCell<dyn CGConfig + Send + Sync>
    // *CG_NAME.lock() = name.to_owned();
    // active_circuit_generators.lock().insert(name.to_owned(), cg);
}
impl CircuitGenerator {
    pub fn createInputWire(cg: RcCell<CircuitGenerator>, desc: &Option<String>) -> WireType {
        let newInputWire = WireType::Variable(VariableWire::new(
            cg.get_current_wire_id(),
            cg.clone().downgrade(),
        ));
        // println!("==get_current_wire_id===={}==={}===",self.get_current_wire_id(),cg.borrow().current_wire_id );
        cg.borrow_mut().current_wire_id += 1;
        addToEvaluationQueue(
            cg.clone(),
            Box::new(WireLabelInstruction::new(
                LabelType::input,
                newInputWire.clone(),
                desc.clone().unwrap_or(String::new()),
            )),
        );
        cg.borrow_mut().in_wires.push(Some(newInputWire.clone()));
        newInputWire
    }
    pub fn createProverWitnessWire(
        cg: RcCell<CircuitGenerator>,
        desc: &Option<String>,
    ) -> WireType {
        // println!(
        //     "===self.get_current_wire_id()======createProverWitnessWire=={}=========={}",
        //     cg.borrow_mut().current_wire_id,
        //     self.get_current_wire_id()
        // );
        let wire = WireType::Variable(VariableWire::new(
            cg.get_current_wire_id(),
            cg.clone().downgrade(),
        ));
        cg.borrow_mut().current_wire_id += 1;
        addToEvaluationQueue(
            cg.clone(),
            Box::new(WireLabelInstruction::new(
                LabelType::nizkinput,
                wire.clone(),
                desc.clone().unwrap_or(String::new()),
            )),
        );
        cg.borrow_mut()
            .prover_witness_wires
            .push(Some(wire.clone()));
        wire
    }
    pub fn makeOutput(
        cg: RcCell<CircuitGenerator>,
        wire: &WireType,
        desc: &Option<String>,
    ) -> WireType {
        // println!("===========makeOutput=============");
        let mut outputWire = wire.clone();
        let some_wire = Some(wire.clone());

        let outputWire = if cg.borrow().prover_witness_wires.contains(&some_wire) {
            // The first case is allowed for usability. In some cases, gadgets provide their witness wires as intermediate outputs, e.g., division gadgets,
            // and the programmer could choose any of these intermediate outputs to be circuit outputs later.
            // The drawback of this method is that this will add one constraint for every witness wire that is transformed to be a circuit output.
            // As the statement size is usually small, this will not lead to issues in practice.
            // The constraint is just added for separation. Note: prover witness wires are actually variable wires. The following method is used
            // in order to introduce a different variable.
            Self::makeVariable(cg.clone(), &wire, desc)
            // If this causes overhead, the programmer can create the wires that are causing the bottleneck
            // as input wires instead of prover witness wires and avoid calling makeOutput().
        } else if cg.borrow().in_wires.contains(&some_wire) {
            eprintln!(
                "Warning: An input wire is redeclared as an output. This leads to an additional unnecessary constraint."
            );
            eprintln!(
                "\t->This situation could happen by calling makeOutput() on input wires or in some cases involving multiplication of an input wire by 1 then declaring the result as an output wire."
            );
            Self::makeVariable(cg.clone(), &wire, desc)
        } else if !(wire.instance_of("VariableWire") || wire.instance_of("VariableBitWire")) {
            wire.packIfNeeded(&None);
            Self::makeVariable(cg.clone(), &wire, desc)
        } else {
            wire.packIfNeeded(&None);

            wire.clone()
        };
        // println!("----------------------------------------");
        cg.borrow_mut().out_wires.push(Some(outputWire.clone()));
        addToEvaluationQueue(
            cg.clone(),
            Box::new(WireLabelInstruction::new(
                LabelType::output,
                outputWire.clone(),
                desc.clone().unwrap_or(String::new()),
            )),
        );
        outputWire
    }

    pub fn makeVariable(
        cg: RcCell<CircuitGenerator>,
        wire: &WireType,
        desc: &Option<String>,
    ) -> WireType {
        let mut outputWire = WireType::Variable(VariableWire::new(
            cg.get_current_wire_id(),
            cg.clone().downgrade(),
        ));

        cg.borrow_mut().current_wire_id += 1;
        let op = MulBasicOp::new(
            wire,
            cg.get_one_wire().as_ref().unwrap(),
            &outputWire,
            desc.clone().unwrap_or(String::new()),
        );
        let cachedOutputs = addToEvaluationQueue(cg.clone(), Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            cg.borrow_mut().current_wire_id -= 1;
            cachedOutputs[0].clone().unwrap()
        } else {
            outputWire
        }
    }

    pub fn createInputWireArray(
        cg: RcCell<CircuitGenerator>,
        n: usize,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        (0..n)
            .map(|i| {
                Some(CircuitGenerator::createInputWire(
                    cg.clone(),
                    &desc.as_ref().map(|d| format!("{} {i}", d)),
                ))
            })
            .collect()
    }

    pub fn createLongElementInput(
        cg: RcCell<CircuitGenerator>,
        totalBitwidth: i32,
        desc: &Option<String>,
    ) -> LongElement {
        let numWires =
            (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as usize;
        let w = Self::createInputWireArray(cg.clone(), numWires, desc);
        let mut bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; numWires];
        if numWires as i32 * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = (totalBitwidth % LongElement::CHUNK_BITWIDTH) as u64;
        }
        LongElement::new(w, bitwidths, cg.downgrade())
    }

    pub fn createLongElementProverWitness(
        cg: RcCell<CircuitGenerator>,
        totalBitwidth: i32,
        desc: &Option<String>,
    ) -> LongElement {
        let numWires =
            (totalBitwidth as f64 * 1.0 / LongElement::CHUNK_BITWIDTH as f64).ceil() as usize;
        let w = Self::createProverWitnessWireArray(cg.clone(), numWires, desc);
        let mut bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; numWires];
        if numWires as i32 * LongElement::CHUNK_BITWIDTH != totalBitwidth {
            bitwidths[numWires - 1] = (totalBitwidth % LongElement::CHUNK_BITWIDTH) as u64;
        }
        LongElement::new(w, bitwidths, cg.downgrade())
    }

    pub fn createProverWitnessWireArray(
        cg: RcCell<CircuitGenerator>,
        n: usize,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        (0..n)
            .map(|k| {
                Some(CircuitGenerator::createProverWitnessWire(
                    cg.clone(),
                    &desc.as_ref().map(|d| format!("{} {k}", d)),
                ))
            })
            .collect()
    }
    pub fn makeOutputArray(
        cg: RcCell<CircuitGenerator>,
        wires: &Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        // println!("================makeOutputArray========");
        wires
            .iter()
            .enumerate()
            .map(|(i, w)| {
                // println!("{i}=i==w={}",w.as_ref().unwrap().getWireId());
                let out = CircuitGenerator::makeOutput(
                    cg.clone(),
                    w.as_ref().unwrap(),
                    &desc.as_ref().map(|d| format!("{}[{i}]", d)),
                );
                // println!("{i}=i==out={}",out.getWireId());
                Some(out)
            })
            .collect()
    }

    /**
     * Asserts an r1cs constraint. w1*w2 = w3
     *
     */
    pub fn addAssertion(
        cg: RcCell<CircuitGenerator>,
        w1: &WireType,
        w2: &WireType,
        w3: &WireType,
        desc: &Option<String>,
    ) {
        if w1.instance_of("ConstantWire")
            && w2.instance_of("ConstantWire")
            && w3.instance_of("ConstantWire")
        {
            let const1 = w1.try_as_constant_ref().unwrap().getConstant();
            let const2 = w2.try_as_constant_ref().unwrap().getConstant();
            let const3 = w3.try_as_constant_ref().unwrap().getConstant();
            assert!(
                const3 == const1.mul(const2).rem(&Configs.field_prime),
                "Assertion failed on the provided constant wires .. "
            );
        } else {
            w1.packIfNeeded(&None);
            w2.packIfNeeded(&None);
            w3.packIfNeeded(&None);
            let op = AssertBasicOp::new(w1, w2, w3, desc.clone().unwrap_or(String::new()));
            addToEvaluationQueue(cg, Box::new(op));
        }
    }

    pub fn addZeroAssertion(cg: RcCell<CircuitGenerator>, w: &WireType, desc: &Option<String>) {
        Self::addAssertion(
            cg.clone(),
            w,
            cg.get_one_wire().as_ref().unwrap(),
            cg.get_zero_wire().as_ref().unwrap(),
            desc,
        );
    }

    pub fn addOneAssertion(cg: RcCell<CircuitGenerator>, w: &WireType, desc: &Option<String>) {
        Self::addAssertion(
            cg.clone(),
            w,
            cg.get_one_wire().as_ref().unwrap(),
            cg.get_one_wire().as_ref().unwrap(),
            desc,
        );
    }

    pub fn addBinaryAssertion(cg: RcCell<CircuitGenerator>, w: &WireType, desc: &Option<String>) {
        let inv = w.invAsBit(desc).unwrap();
        Self::addAssertion(
            cg.clone(),
            w,
            &inv,
            cg.get_zero_wire().as_ref().unwrap(),
            desc,
        );
    }

    pub fn addEqualityAssertion(
        cg: RcCell<CircuitGenerator>,
        w1: &WireType,
        w2: &WireType,
        desc: &Option<String>,
    ) {
        if w1 != w2 {
            Self::addAssertion(
                cg.clone(),
                w1,
                cg.get_one_wire().as_ref().unwrap(),
                w2,
                desc,
            );
        }
    }

    pub fn addEqualityAssertionb(
        cg: RcCell<CircuitGenerator>,
        w1: &WireType,
        b: &BigInteger,
        desc: &Option<String>,
    ) {
        Self::addAssertion(
            cg.clone(),
            w1,
            cg.get_one_wire().as_ref().unwrap(),
            &Self::createConstantWire(cg.clone(), b, desc),
            desc,
        );
    }
    pub fn addDebugInstruction(cg: RcCell<CircuitGenerator>, w: &WireType, desc: &Option<String>) {
        w.packIfNeeded(&None);
        addToEvaluationQueue(
            cg,
            Box::new(WireLabelInstruction::new(
                LabelType::debug,
                w.clone(),
                desc.clone().unwrap_or(String::new()),
            )),
        );
    }

    pub fn addDebugInstructiona(
        cg: RcCell<CircuitGenerator>,
        wires: &Vec<Option<WireType>>,
        desc: &Option<String>,
    ) {
        for i in 0..wires.len() {
            wires[i].as_ref().unwrap().packIfNeeded(&None);
            addToEvaluationQueue(
                cg.clone(),
                Box::new(WireLabelInstruction::new(
                    LabelType::debug,
                    wires[i].clone().unwrap(),
                    desc.clone().unwrap_or(String::new()),
                )),
            );
        }
    }

    pub fn createConstantWire(
        cg: RcCell<CircuitGenerator>,
        x: &BigInteger,
        desc: &Option<String>,
    ) -> WireType {
        // println!(
        //     "========before===============createConstantWire====={}======={}=========== {} ",
        //     file!(),
        //     line!(),
        //     cg.get_current_wire_id()
        // );
        let v = cg.get_one_wire().unwrap().mulb(x, desc);
        // println!(
        //     "=========after==============createConstantWire====={}======={}=========== {} ",
        //     file!(),
        //     line!(),
        //     cg.get_current_wire_id()
        // );
        v
    }

    pub fn createConstantWireArray(
        cg: RcCell<CircuitGenerator>,
        a: &Vec<BigInteger>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        a.iter()
            .map(|v| Some(Self::createConstantWire(cg.clone(), v, desc)))
            .collect()
    }

    pub fn createConstantWirei(
        cg: RcCell<CircuitGenerator>,
        x: i64,
        desc: &Option<String>,
    ) -> WireType {
        cg.get_one_wire().unwrap().muli(x, desc)
    }

    pub fn createConstantWireArrayi(
        cg: RcCell<CircuitGenerator>,
        a: &Vec<i64>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        a.iter()
            .map(|&i| Some(Self::createConstantWirei(cg.clone(), i, desc)))
            .collect()
    }

    pub fn createNegConstantWire(
        cg: RcCell<CircuitGenerator>,
        x: &BigInteger,
        desc: &Option<String>,
    ) -> WireType {
        cg.get_one_wire().unwrap().mulb(&x.neg(), desc)
    }

    pub fn createNegConstantWirei(
        cg: RcCell<CircuitGenerator>,
        x: i64,
        desc: &Option<String>,
    ) -> WireType {
        cg.get_one_wire().unwrap().muli(-x, desc)
    }

    /**
     * Use to support computation for prover witness values outside of the
     * circuit. See Mod_Gadget and Field_Division gadgets for examples.
     *
     * @param instruction
     */
    pub fn specifyProverWitnessComputation(cg: RcCell<CircuitGenerator>, e: Box<dyn Instruction>) {
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
        //         // Self::addToEvaluationQueues();
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
        //                 fn evaluate(cg: RcCell<CircuitGenerator>, evaluator: &mut CircuitEvaluator) {
        //                     (Self::f).clone()(evaluator);
        //                 }
        //             }
        //         impl<F: FnOnce( &mut CircuitEvaluator)
        // 			+ Serialize
        // 			+ DeserializeOwned
        // 			+ PartialEq
        // 			+ Eq
        // 			+ Clone
        // 			+ Debug+Hash> InstanceOf for Prover<F>{
        //                  fn instance_of(cg: RcCell<CircuitGenerator>, name: &str) -> bool {
        //                     Self::name() == name
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
        addToEvaluationQueue(cg, e);
    }
}
//+ CreateConstantWire + CreateConstantWireArray + CreateNegConstantWire
pub trait CGConfig: DynClone + CGConfigFields + StructNameConfig {
    fn buildCircuit(&mut self) {}
    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {}
    fn generateCircuit(&mut self) {
        println!("Running Circuit Generator for <  {}  >", self.get_name());

        self.initCircuitConstruction();
        // println!("before buildCircuit  {},{}", file!(), line!());
        self.buildCircuit();

        println!(
            "Circuit Generation Done for < {} > \n \t Total Number of Constraints : {} \n \t Total Number of Wires : {}",
            self.get_name(),
            self.get_num_of_constraints(),
            self.get_num_wires()
        );
    }

    fn generateZeroWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        let zero_wire = self.cg().get_zero_wire();
        vec![zero_wire; n]
    }

    fn generateOneWireArray(&self, n: usize) -> Vec<Option<WireType>> {
        let one_wire = self.cg().get_one_wire();
        vec![one_wire; n]
    }

    fn writeCircuitFile(&self) {
        let mut printWriter = File::create(self.get_name() + ".arith").unwrap();
        write!(printWriter, "total {}", self.cg().get_current_wire_id());
        let evaluation_queue = self.cg().get_evaluation_queue();
        for e in evaluation_queue.values() {
            if e.doneWithinCircuit() {
                let _ = write!(printWriter, "{e:?} \n");
            }
        }
    }

    fn printCircuit(&self) {
        let evaluation_queue = self.cg().get_evaluation_queue();
        for e in evaluation_queue.values() {
            if e.doneWithinCircuit() {
                //println!("{e:?}");
            }
        }
    }

    fn initCircuitConstruction(&self) {
        let s = crate::util::build_circuit_timer::time_measure(&format!("{}", line!()));
        let one_wire = WireType::Constant(ConstantWire::new(
            self.cg().get_current_wire_id(),
            Util::one(),
            self.cg_weak(),
        ));
        //println!("{},{}",file!(),line!());
        self.cg().borrow_mut().one_wire = Some(one_wire.clone());
        let start = Instant::now();
        // println!(
        //     "==**********initCircuitConstruction************=={:?}====*self.one_wire() ========{:?}",
        //     self.name(),
        //     self.cg().get_one_wire()
        // );
        self.cg().borrow_mut().current_wire_id += 1;
        self.cg()
            .borrow_mut()
            .known_constant_wires
            .insert(Util::one(), one_wire.clone());
        // println!("{},{}", file!(), line!());
        // println!("End Name  Time: 2222 {} s", start.elapsed().as_secs());
        addToEvaluationQueue(
            self.cg(),
            Box::new(WireLabelInstruction::new(
                LabelType::input,
                one_wire.clone(),
                "The one-input wire.".to_owned(),
            )),
        );
        // println!("End Name Time: 333 {} s", start.elapsed().as_secs());
        //println!("{},{}",file!(),line!());
        self.cg().borrow_mut().in_wires.push(Some(one_wire.clone()));
        // println!("End Name Time: 23343 {} s", start.elapsed().as_secs());
        let v = one_wire.muli(0, &None);
        self.cg().borrow_mut().zero_wire = Some(v);
        // println!("End Name Time: 444 {} s", start.elapsed().as_secs());
    }

    // fn addToEvaluationQueue(
    //     &self,
    //     e: Box<dyn Instruction>,
    // ) -> Option<Vec<Option<WireType>>> {
    //     // self.addToEvaluationQueues(Box<dyn Instruction>::Trait( e))
    //     None
    // }
    // fn addToEvaluationQueue(
    //     &self,
    //     e: Box<dyn Instruction>,
    // ) -> Option<Vec<Option<WireType>>> {
    //     let evaluation_queue = self.cg().evaluation_queue();
    //     if let Some(existingInstruction) = evaluation_queue.get(&e) {
    //         return existingInstruction.basic_op().map(|op| op.getOutputs());
    //     }

    //     evaluation_queue.entry(e.clone()).or_insert(e.clone());
    //     if e.instance_of("BasicOp") {
    //         *self.cg().borrow_mut().num_of_constraints += e.basic_op().as_ref().unwrap().getNumMulGates();
    //     }
    //     None // returning None means we have not seen this instruction before
    //     // have seen this instruction before, but can't de-duplicate

    //     // if existingInstruction.unwrap().instance_of("BasicOp") {
    //     //     return Some(existingInstruction.unwrap().basic_op().unwrap().getOutputs());
    //     // } else {
    //     //     return None;
    //     // }
    // }

    fn printState(&self, message: String) {
        println!("\nGenerator State @ {message}");
        println!(
            "\tCurrent Number of Multiplication Gates  .  {}\n",
            self.cg().get_num_of_constraints()
        );
    }

    fn evalCircuit(&mut self) -> eyre::Result<CircuitEvaluator> {
        let mut circuitEvaluator = CircuitEvaluator::new(&self.name(), &self.cg());
        self.generateSampleInput(&mut circuitEvaluator);
        circuitEvaluator.evaluate(&self.cg());
        // *self.circuit_evaluator() = Some(circuitEvaluator);
        Ok(circuitEvaluator)
    }

    fn prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.writeCircuitFile();
        assert!(
            circuit_evaluator.is_some(),
            "evalCircuit() must be called before prepFiles()"
        );
        circuit_evaluator
            .as_ref()
            .unwrap()
            .writeInputFile(self.cg().clone());
    }

    fn runLibsnark(&self) {
        let p = run_command(
            vec![
                &Configs.libsnark_exec.clone(),
                &(self.get_name() + ".arith"),
                &(self.get_name() + ".in"),
            ],
            None,
            false,
        );
        println!(
            "\n-----------------------------------RUNNING LIBSNARK -----------------------------------------"
        );
        let inp = p.0.clone().unwrap();
        let buf = inp.replace(" ", "\n");
        println!("===={buf}");
    }

    // fn getCircuitEvaluator(&self) -> CircuitEvaluator {
    //     assert!(
    //         self.circuit_evaluator().is_some(),
    //         "evalCircuit() must be called before getCircuitEvaluator()"
    //     );

    //     return self.circuit_evaluator().clone().unwrap();
    // }
}

impl CGConfigFields for CircuitGenerator {
    // fn current_wire_id(&mut self) -> &mut i32 {
    //     &mut self.current_wire_id
    // }
    // fn evaluation_queue(
    //     &self,
    // ) -> &mut HashMap<Box<dyn Instruction>, Box<dyn Instruction>> {
    //     &mut self.evaluation_queue
    // }

    // fn zero_wire(&mut self) -> &mut Option<WireType> {
    //     &mut self.zero_wire
    // }
    // fn one_wire(&mut self) -> &mut Option<WireType> {
    //     &mut self.one_wire
    // }
    // fn in_wires(&mut self) -> &mut Vec<Option<WireType>> {
    //     &mut self.in_wires
    // }
    // fn out_wires(&mut self) -> &mut Vec<Option<WireType>> {
    //     &mut self.out_wires
    // }
    // fn prover_witness_wires(&mut self) -> &mut Vec<Option<WireType>> {
    //     &mut self.prover_witness_wires
    // }
    // fn circuit_name(&mut self) -> &mut String {
    //     &mut self.circuit_name
    // }
    fn get_known_constant_wires(&self) -> BTreeMap<BigInteger, WireType> {
        self.known_constant_wires.clone()
    }
    // fn get_num_of_constraints(&self) -> i32 {
    //     self.num_of_constraints
    // }
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
        // println!("=====get_one_wire============={:?}", self.get_name());
        self.one_wire.clone()
    }

    fn get_evaluation_queue(&self) -> LinkedHashMap<u64, Box<dyn Instruction>> {
        self.evaluation_queue.clone()
    }
    fn get(&self, hash_code: u64) -> Option<Box<dyn Instruction>> {
        self.evaluation_queue.get(&hash_code).cloned()
    }
    fn get_num_wires(&self) -> i32 {
        self.get_current_wire_id()
    }
    fn get_current_wire_id(&self) -> i32 {
        // assert! (175548!=self.current_wire_id);///TEST
        self.current_wire_id
    }

    fn get_num_of_constraints(&self) -> i32 {
        self.num_of_constraints
    }

    fn get_in_wires(&self) -> Vec<Option<WireType>> {
        self.in_wires.clone()
    }

    fn get_out_wires(&self) -> Vec<Option<WireType>> {
        self.out_wires.clone()
    }

    fn get_prover_witness_wires(&self) -> Vec<Option<WireType>> {
        self.prover_witness_wires.clone()
    }
}

pub trait CreateConstantWire<T = WireType> {
    fn create_constant_wire(&self, x: T, desc: &Option<String>) -> WireType;
}

impl CreateConstantWire<&BigInteger> for CircuitGenerator {
    fn create_constant_wire(&self, x: &BigInteger, desc: &Option<String>) -> WireType {
        self.get_one_wire().unwrap().mulb(x, desc)
    }
}
impl CreateConstantWire<i64> for CircuitGenerator {
    fn create_constant_wire(&self, x: i64, desc: &Option<String>) -> WireType {
        self.get_one_wire().unwrap().muli(x, desc)
    }
}

impl CreateConstantWire<&BigInteger> for RcCell<CircuitGenerator> {
    fn create_constant_wire(&self, x: &BigInteger, desc: &Option<String>) -> WireType {
        self.get_one_wire().unwrap().mulb(x, desc)
    }
}
impl CreateConstantWire<i64> for RcCell<CircuitGenerator> {
    fn create_constant_wire(&self, x: i64, desc: &Option<String>) -> WireType {
        self.get_one_wire().unwrap().muli(x, desc)
    }
}
pub trait CreateConstantWireArray<T = WireType> {
    fn create_constant_wire_array(&self, a: T, desc: &Option<String>) -> Vec<Option<WireType>>;
}
impl CreateConstantWireArray<&Vec<BigInteger>> for RcCell<CircuitGenerator> {
    fn create_constant_wire_array(
        &self,
        a: &Vec<BigInteger>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        a.iter()
            .map(|v| Some(self.create_constant_wire(v, desc)))
            .collect()
    }
}
impl CreateConstantWireArray<&Vec<i64>> for RcCell<CircuitGenerator> {
    fn create_constant_wire_array(
        &self,
        a: &Vec<i64>,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        a.iter()
            .map(|&v| Some(self.create_constant_wire(v, desc)))
            .collect()
    }
}

pub trait CreateNegConstantWire<T = WireType> {
    fn create_neg_constant_wire(&self, x: T, desc: &Option<String>) -> WireType;
}
impl CreateNegConstantWire<&BigInteger> for RcCell<CircuitGenerator> {
    fn create_neg_constant_wire(&self, x: &BigInteger, desc: &Option<String>) -> WireType {
        self.get_one_wire().unwrap().mulb(&x.neg(), desc)
    }
}
impl CreateNegConstantWire<i64> for RcCell<CircuitGenerator> {
    fn create_neg_constant_wire(&self, x: i64, desc: &Option<String>) -> WireType {
        self.get_one_wire().unwrap().muli(-x, desc)
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
                    impl Instruction for Prover{
                        fn evaluate(&self, evaluator: &mut CircuitEvaluator)
                            $body

                    }


    };
}

#[macro_export]
macro_rules! impl_prover {
    (  eval($($arg_name:ident : $arg_ty:ty),*  )$body:block ) => {
                    {#[derive(Hash, Clone, Debug, ImplStructNameConfig)]
                    struct Prover{
                        $( pub $arg_name : $arg_ty),*
                    }
                    // impl Instruction for Prover{
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

// impl CGConfigFieldsIQ for RcCell<CircuitGenerator>{
//     crate::impl_fn_of_trait!(fn get_zero_wire(&self) -> Option<WireType> );
//    crate::impl_fn_of_trait!( fn get_one_wire(&self) -> Option<WireType> );

//     crate::impl_fn_of_trait!(fn get_evaluation_queue(
//         &self,
//     ) -> HashMap<Box<dyn Instruction>, Box<dyn Instruction>> );

//     crate::impl_fn_of_trait!(fn get_current_wire_id(&self) -> i32 );

// }

impl<T: CGConfigFields> CGConfigFields for RcCell<T> {
    crate::impl_fn_of_trait!(fn get_zero_wire(&self) -> Option<WireType> );
    crate::impl_fn_of_trait!( fn get_one_wire(&self) -> Option<WireType> );

    crate::impl_fn_of_trait!(fn get_evaluation_queue(
        &self,
    ) -> LinkedHashMap<u64, Box<dyn Instruction>> );

    crate::impl_fn_of_trait!(fn get(&self,hash_code:u64) -> Option<Box<dyn Instruction>> );
    crate::impl_fn_of_trait!(fn get_current_wire_id(&self) -> i32 );
    crate::impl_fn_of_trait!( fn get_name(&self) -> String );

    crate::impl_fn_of_trait!( fn get_num_wires(&self) -> i32 );

    crate::impl_fn_of_trait!( fn get_num_of_constraints(&self) -> i32 );

    crate::impl_fn_of_trait!(fn get_in_wires(&self) -> Vec<Option<WireType>> );

    crate::impl_fn_of_trait!( fn get_out_wires(&self) -> Vec<Option<WireType>> );

    crate::impl_fn_of_trait!(fn get_prover_witness_wires(&self) -> Vec<Option<WireType>> );
    crate::impl_fn_of_trait!(fn get_known_constant_wires(&self) -> BTreeMap<BigInteger, WireType> );
    // crate::impl_fn_of_trait!(fn addToEvaluationQueue(&self, e: Box<dyn Instruction>) -> Option<Vec<Option<WireType>>> );
}
impl StructNameConfig for CircuitGenerator {
    fn name(&self) -> String {
        "self.t.name()".to_owned()
    }
}

impl CGConfig for CircuitGenerator {}

impl CGConfig for RcCell<CircuitGenerator> {
    fn buildCircuit(&mut self) {}
    crate::impl_fn_of_trait!( fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator));
    crate::impl_fn_of_trait!(fn generateCircuit(&mut self));

    // crate::impl_fn_of_trait!(fn createInputWire(&self, desc: &Option<String>) -> WireType );

    // crate::impl_fn_of_trait!(fn createInputWireArray(&self, n: usize, desc: &Option<String>) -> Vec<Option<WireType>>);

    // crate::impl_fn_of_trait!(fn createLongElementInput(&self, totalBitwidth: i32, desc: &Option<String>) -> LongElement);

    // crate::impl_fn_of_trait! (fn createLongElementProverWitness(
    //     &self,
    //     totalBitwidth: i32,
    //     desc: &Option<String>
    // ) -> LongElement );

    // crate::impl_fn_of_trait!(fn createProverWitnessWire(&self, desc: &Option<String>) -> WireType );

    // crate::impl_fn_of_trait!(fn createProverWitnessWireArray(
    //     &self,
    //     n: usize,
    //     desc: &Option<String>
    // ) -> Vec<Option<WireType>> );

    crate::impl_fn_of_trait!(fn generateZeroWireArray(&self, n: usize) -> Vec<Option<WireType>> );

    crate::impl_fn_of_trait!(fn generateOneWireArray(&self, n: usize) -> Vec<Option<WireType>>);

    // crate::impl_fn_of_trait!(fn makeOutput(&self, wire: &WireType, desc: &Option<String>) -> WireType );

    // crate::impl_fn_of_trait!(fn makeVariable(&self, wire: &WireType, desc: &Option<String>) -> WireType );

    // crate::impl_fn_of_trait!(fn makeOutputArray(
    //     &self,
    //     wires: &Vec<Option<WireType>>,
    //     desc: &Option<String>
    // ) -> Vec<Option<WireType>> );

    // crate::impl_fn_of_trait!(fn addDebugInstruction(&self, w: &WireType, desc: &Option<String>));

    // crate::impl_fn_of_trait!(fn addDebugInstructiona(&self, wires: &Vec<Option<WireType>>, desc: &Option<String>));

    crate::impl_fn_of_trait!(fn writeCircuitFile(&self));

    crate::impl_fn_of_trait!(fn printCircuit(&self) );

    crate::impl_fn_of_trait!(fn initCircuitConstruction(&self));

    // crate::impl_fn_of_trait!(fn createConstantWire(&self, x: &BigInteger, desc: &Option<String>) -> WireType );

    // crate::impl_fn_of_trait!(fn createConstantWireArray(
    //     &self,
    //     a: &Vec<BigInteger>,
    //     desc: &Option<String>
    // ) -> Vec<Option<WireType>>);

    // crate::impl_fn_of_trait!(fn createConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType );

    // crate::impl_fn_of_trait!(fn createConstantWireArrayi(
    //     &self,
    //     a: &Vec<i64>,
    //     desc: &Option<String>
    // ) -> Vec<Option<WireType>>);

    // crate::impl_fn_of_trait!(fn createNegConstantWire(&self, x: &BigInteger, desc: &Option<String>) -> WireType );

    // crate::impl_fn_of_trait!(fn createNegConstantWirei(&self, x: i64, desc: &Option<String>) -> WireType );

    /**
     * Use to support computation for prover witness values outside of the
     * circuit. See Mod_Gadget and Field_Division gadgets for examples.
     *
     * @param instruction
     */
    // crate::impl_fn_of_trait!(fn specifyProverWitnessComputation(&self, e: Box<dyn Instruction>));

    // crate::impl_fn_of_trait!(fn addToEvaluationQueue(
    //     &self,
    //     e: Box<dyn Instruction>
    // ) -> Option<Vec<Option<WireType>>>);

    crate::impl_fn_of_trait!(fn printState(&self, message: String));

    // crate::impl_fn_of_trait!(fn addAssertion(&self, w1: &WireType, w2: &WireType, w3: &WireType, desc: &Option<String>) );

    // crate::impl_fn_of_trait!(fn addZeroAssertion(&self, w: &WireType, desc: &Option<String>));

    // crate::impl_fn_of_trait!(fn addOneAssertion(&self, w: &WireType, desc: &Option<String>) );

    // crate::impl_fn_of_trait!(fn addBinaryAssertion(&self, w: &WireType, desc: &Option<String>) );

    // crate::impl_fn_of_trait!(fn addEqualityAssertion(&self, w1: &WireType, w2: &WireType, desc: &Option<String>));

    // crate::impl_fn_of_trait!(fn addEqualityAssertionb(&self, w1: &WireType, b: &BigInteger, desc: &Option<String>) );

    crate::impl_fn_of_trait!(fn evalCircuit(&mut self) -> eyre::Result<CircuitEvaluator> );

    crate::impl_fn_of_trait!(fn prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) );

    crate::impl_fn_of_trait!(fn runLibsnark(&self) );
}

impl<T: StructNameConfig> StructNameConfig for RcCell<T> {
    fn name(&self) -> String {
        self.borrow().name()
    }
}

// impl<T:OpCodeConfig> OpCodeConfig for RcCell<T>{
//     fn op_code(&self) -> String {
//         self.borrow().name()
//     }
// }

impl<T: InstanceOf> InstanceOf for RcCell<T> {
    fn instance_of(&self, name: &str) -> bool {
        self.borrow().instance_of(name)
    }
}

#[macro_export]
macro_rules! impl_fn_of_trait {
    ($vis:vis fn $name:ident(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis fn $name(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {
            //  match self{
            //     Self::Web3TesterBlockchain(tester)=>tester.$name($($arg_name),*),
            //     Self::Web3HttpGanacheBlockchain(ganache)=>ganache.$name($($arg_name),*),
            // }
            self.borrow().$name($($arg_name),*)
        }
    };
    // ($vis:vis fn $name:ident< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+  >(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
    //     $vis fn $name<$( $lt $(: $clt$(+$dlt)*)? ),+>(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {

    //     }
    // };
    ($vis:vis  fn $name:ident(&mut self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis  fn $name(&mut self, $($arg_name : $arg_ty),*) $(-> $ret)? {
             self.borrow_mut().$name($($arg_name),*)
        }
    };
    // ($vis:vis async fn $name:ident<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>(&self $(,)? $($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
    //     $vis async fn $name<$( $lt $(: $clt$(+$dlt)*)? ),+>(&self, $($arg_name : $arg_ty),*) $(-> $ret)? {

    //     }
    // };
}
