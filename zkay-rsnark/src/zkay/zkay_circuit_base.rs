#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CGConfigFields;
use crate::circuit::structure::circuit_generator::CGInstance;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};

use crate::circuit::structure::wire::GetWireId;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::BigInteger;
use crate::zkay::crypto::crypto_backend::Backend;
use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfig;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfigs;
use crate::zkay::crypto::homomorphic_backend::HomomorphicBackend;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_sha256_gadget::ZkaySHA256Gadget;
use crate::zkay::zkay_type::ZkayType;
use crate::zkay::zkay_type::zkbool;
use crate::zkay::zkay_util::ZkayUtil;
use num_bigint::Sign;
use rccell::RcCell;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Mul, Neg, Sub};
const ADD_OP_LABELS: bool = true;
const LEGACY_CRYPTO_BACKEND: &str = "LEGACY_CRYPTO_BACKEND";

#[derive(Debug, Clone)]
pub struct ZkayCircuitBase<T> {
    /**
     * Whether to include comments for the more complex operations in the circuit.arith file
     */
    pub realCircuitName: String,

    pub cryptoBackends: HashMap<String, Backend>,

    pub currentPubInIdx: i32,
    pub currentPubOutIdx: i32,
    pub allPubIOWires: Vec<Option<WireType>>,

    pub currentPrivInIdx: i32,
    pub allPrivInWires: Vec<Option<WireType>>,

    pub pubInNames: Vec<String>,
    pub pubOutNames: Vec<String>,
    pub privInNames: Vec<String>,

    pub pubInCount: i32,
    pub useInputHashing: bool,

    pub vars: HashMap<String, Vec<TypedWire>>,

    pub currentGuardCondition: VecDeque<TypedWire>,
    pub serializedArguments: Vec<BigInteger>,

    pub namePrefixIndices: HashMap<String, i32>,
    pub namePrefix: VecDeque<String>,

    pub guardPrefixes: VecDeque<VecDeque<String>>,
    pub guardPrefixIndices: VecDeque<HashMap<String, i32>>,
    pub t: T,
}
impl<T: std::fmt::Debug + std::clone::Clone> ZkayCircuitBase<T> {
    pub fn new(
        name: String,
        cryptoBackendId: Option<String>,
        cryptoBackend: Option<String>,
        keyBits: i32,
        pubInSize: i32,
        pubOutSize: i32,
        privSize: i32,
        useInputHashing: bool,
        t: T,
    ) -> CircuitGeneratorExtend<ZkayCircuitBase<T>> {
        let mut _self = CircuitGeneratorExtend::new(
            "circuit",
            Self {
                realCircuitName: name,
                cryptoBackends: HashMap::new(),
                currentPubInIdx: 0,
                currentPubOutIdx: pubInSize,
                allPubIOWires: vec![None; (pubInSize + pubOutSize) as usize],
                currentPrivInIdx: 0,
                allPrivInWires: vec![None; privSize as usize],
                pubInNames: vec![],
                pubOutNames: vec![],
                privInNames: vec![],
                pubInCount: pubInSize,
                useInputHashing,
                vars: HashMap::new(),
                currentGuardCondition: VecDeque::new(),
                serializedArguments: vec![],
                namePrefixIndices: HashMap::new(),
                namePrefix: VecDeque::new(),
                guardPrefixes: VecDeque::new(),
                guardPrefixIndices: VecDeque::new(),
                t,
            },
        );
        Self::clearPrefix(&mut _self.t.namePrefix, &mut _self.t.namePrefixIndices);
        Self::pushGuardPrefix(&mut _self.t.guardPrefixes, &mut _self.t.guardPrefixIndices);

        if let Some(crypto_backend) = cryptoBackend {
            // Legacy handling: add default "main" crypto backend
            let id = cryptoBackendId.unwrap_or(LEGACY_CRYPTO_BACKEND.to_owned());
            assert!(
                _self
                    .t
                    .cryptoBackends
                    .insert(
                        id,
                        Backend::create(&crypto_backend, keyBits, _self.cg.clone()),
                    )
                    .is_none()
            );
        }

        _self
    }

    fn clearPrefix(prefix: &mut VecDeque<String>, indices: &mut HashMap<String, i32>) {
        prefix.clear();
        prefix.push_front("".to_owned());
        indices.clear();
    }

    fn pushPrefix(
        prefix: &mut VecDeque<String>,
        prefixIndices: &mut HashMap<String, i32>,
        newStr: &str,
    ) {
        let mut newPrefix = format!("{}{}.", prefix.front().unwrap(), newStr);
        let count = *prefixIndices.get(&newPrefix).unwrap_or(&0);
        prefixIndices.insert(newPrefix.clone(), count + 1);
        prefix.push_front(format!("{}{}.", newPrefix, count));
    }

    fn pushGuardPrefix(
        guardPrefixes: &mut VecDeque<VecDeque<String>>,
        guardPrefixIndices: &mut VecDeque<HashMap<String, i32>>,
    ) {
        let mut newPrefix = VecDeque::new();
        let mut newPrefixIndices = HashMap::new();
        Self::clearPrefix(&mut newPrefix, &mut newPrefixIndices);
        guardPrefixes.push_front(newPrefix);
        guardPrefixIndices.push_front(newPrefixIndices);
    }
}

impl<T: crate::circuit::StructNameConfig + std::fmt::Debug> crate::circuit::StructNameConfig
    for CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    fn name(&self) -> String {
        self.t.t.name()
    }
}
pub trait ZkayCircuitBaseFields {
    fn realCircuitName(&self) -> &String;
    fn cryptoBackends(&self) -> &HashMap<String, Backend>;
    fn currentPubInIdx(&self) -> i32;
    fn currentPubOutIdx(&self) -> i32;
    fn allPubIOWires(&self) -> &Vec<Option<WireType>>;
    fn currentPrivInIdx(&self) -> i32;
    fn allPrivInWires(&self) -> &Vec<Option<WireType>>;
    fn pubInNames(&self) -> &Vec<String>;
    fn pubOutNames(&self) -> &Vec<String>;
    fn privInNames(&self) -> &Vec<String>;
    fn pubInCount(&self) -> i32;
    fn useInputHashing(&self) -> bool;
    fn vars(&self) -> &HashMap<String, Vec<TypedWire>>;
    fn currentGuardCondition(&self) -> &VecDeque<TypedWire>;
    fn serializedArguments(&self) -> &Vec<BigInteger>;
    fn namePrefixIndices(&self) -> &HashMap<String, i32>;
    fn namePrefix(&self) -> &VecDeque<String>;
    fn guardPrefixes(&self) -> &VecDeque<VecDeque<String>>;
    fn guardPrefixIndices(&self) -> &VecDeque<HashMap<String, i32>>;
    fn serializedArguments_mut(&mut self) -> &mut Vec<BigInteger>;
}
impl<T: crate::circuit::StructNameConfig + std::fmt::Debug> ZkayCircuitBaseFields
    for CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    fn realCircuitName(&self) -> &String {
        &self.t.realCircuitName
    }
    fn cryptoBackends(&self) -> &HashMap<String, Backend> {
        &self.t.cryptoBackends
    }
    fn currentPubInIdx(&self) -> i32 {
        self.t.currentPubInIdx
    }
    fn currentPubOutIdx(&self) -> i32 {
        self.t.currentPubOutIdx
    }
    fn allPubIOWires(&self) -> &Vec<Option<WireType>> {
        &self.t.allPubIOWires
    }
    fn currentPrivInIdx(&self) -> i32 {
        self.t.currentPrivInIdx
    }
    fn allPrivInWires(&self) -> &Vec<Option<WireType>> {
        &self.t.allPrivInWires
    }
    fn pubInNames(&self) -> &Vec<String> {
        &self.t.pubInNames
    }
    fn pubOutNames(&self) -> &Vec<String> {
        &self.t.pubOutNames
    }
    fn privInNames(&self) -> &Vec<String> {
        &self.t.privInNames
    }
    fn pubInCount(&self) -> i32 {
        self.t.pubInCount
    }
    fn useInputHashing(&self) -> bool {
        self.t.useInputHashing
    }
    fn vars(&self) -> &HashMap<String, Vec<TypedWire>> {
        &self.t.vars
    }
    fn currentGuardCondition(&self) -> &VecDeque<TypedWire> {
        &self.t.currentGuardCondition
    }
    fn serializedArguments(&self) -> &Vec<BigInteger> {
        &self.t.serializedArguments
    }
    fn namePrefixIndices(&self) -> &HashMap<String, i32> {
        &self.t.namePrefixIndices
    }
    fn namePrefix(&self) -> &VecDeque<String> {
        &self.t.namePrefix
    }
    fn guardPrefixes(&self) -> &VecDeque<VecDeque<String>> {
        &self.t.guardPrefixes
    }
    fn guardPrefixIndices(&self) -> &VecDeque<HashMap<String, i32>> {
        &self.t.guardPrefixIndices
    }

    fn serializedArguments_mut(&mut self) -> &mut Vec<BigInteger> {
        &mut self.t.serializedArguments
    }
}
pub trait ZkayCircuitBaseConfig: ZkayCircuitBaseFields + CGConfig {
    fn run(&mut self, args: &Vec<String>) {
        match args[0].as_str() {
            "compile" => self.compileCircuit(),
            "prove" => {
                self.compileCircuit();
                self.parseInputs(&args[1..].to_vec());
                //println!("Evaluating circuit '" + realCircuitName + "'");
                self.evalCircuit();
            }

            _ => panic!("invalid command"),
        }
        self.prepFiles(None);
    }

    fn parseInputs(&mut self, args: &Vec<String>) {
        let totCount = self.allPubIOWires().len() + self.allPrivInWires().len();
        assert!(
            args.len() == totCount,
            "Input count mismatch, expected {}, was {}",
            totCount,
            args.len()
        );
        let mut serializedArguments = vec![BigInteger::default(); totCount];
        for i in 0..totCount {
            let v = BigInteger::parse_bytes(args[i].as_bytes(), 16).unwrap();
            assert!(
                v.sign() != Sign::Minus,
                "No signed inputs (signed must be converted to unsigned beforehand)"
            );
            serializedArguments[i] = v;
        }
        *self.serializedArguments_mut() = serializedArguments;
    }
    fn compileCircuit(&mut self) {
        println!("Compiling circuit '{}'", self.realCircuitName());
        // let mut generator=self.cg.borrow().clone();
        self.generateCircuit();
        assert!(
            self.currentPubInIdx() == self.pubInCount()
                && self.currentPubOutIdx() == self.allPubIOWires().len() as i32,
            "Not all pub inputs assigned {},{},{},{}",
            self.currentPubInIdx(),
            self.pubInCount(),
            self.currentPubOutIdx(),
            self.allPubIOWires().len()
        );
        assert!(
            self.currentPrivInIdx() == self.allPrivInWires().len() as i32,
            "Not all  inputs assigned {},{}",
            self.currentPrivInIdx(),
            self.allPrivInWires().len()
        );
        if self.useInputHashing() {
            self.makeOutputArray(
                ZkaySHA256Gadget::new(self.allPubIOWires().clone(), 253, &None, self.cg())
                    .getOutputWires(),
                &Some("digest".to_owned()),
            );
        }
        //println!("Done with generateCircuit, preparing dummy files...");
    }
}

impl<T: crate::circuit::StructNameConfig + std::fmt::Debug + std::clone::Clone>
    CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    pub fn super_buildCircuit(&mut self) {
        let generator = &self.generator;
        let pubInCount = self.t.pubInCount as usize;
        // Create IO wires
        let pubOutCount = self.t.allPubIOWires.len() - pubInCount;
        let (inArray, outArray) = if self.t.useInputHashing {
            (
                generator.createProverWitnessWireArray(pubInCount, &Some("in_".to_owned())),
                generator.createProverWitnessWireArray(pubOutCount, &Some("out_".to_owned())),
            )
        } else {
            (
                generator.createInputWireArray(pubInCount, &Some("in_".to_owned())),
                generator.createInputWireArray(pubOutCount, &Some("out_".to_owned())),
            )
        };
        let privInArray = generator
            .createProverWitnessWireArray(self.t.allPrivInWires.len(), &Some("priv_".to_owned()));

        // Legacy handling
        let legacyCryptoBackend = self.t.cryptoBackends.get(LEGACY_CRYPTO_BACKEND);
        if legacyCryptoBackend.is_some_and(|v| v.isSymmetric()) {
            let myPk = inArray[0].as_ref().unwrap();
            let mySk = privInArray[0].as_ref().unwrap();
            self.setKeyPair(LEGACY_CRYPTO_BACKEND, myPk, mySk);
        }

        self.t.allPubIOWires[..pubInCount].clone_from_slice(&inArray[..pubInCount]);
        self.t.allPubIOWires[pubInCount..pubInCount + pubOutCount]
            .clone_from_slice(&outArray[..pubOutCount]);
        let len = self.t.allPrivInWires.len();
        self.t.allPrivInWires[..len].clone_from_slice(&privInArray[..len]);
    }

    pub fn super_generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        assert!(
            !self.t.serializedArguments.is_empty(),
            "No inputs specified, this should not have been called"
        );

        assert!(
            self.t.serializedArguments.len()
                == self.t.allPubIOWires.len() + self.t.allPrivInWires.len(),
            "Invalid serialized argument count, expected {} was {}",
            self.t.allPubIOWires.len(),
            self.t.serializedArguments.len()
        );

        let mut idx = 0;
        for ioNameList in [&self.t.pubInNames, &self.t.pubOutNames, &self.t.privInNames] {
            for name in ioNameList {
                let wires = self.t.vars.get(name).unwrap();
                let mut sb = format!("Setting '{name}' to [");
                for w in wires {
                    let val = &self.t.serializedArguments[idx];
                    idx += 1;
                    evaluator.setWireValue(&w.wire, val);
                    sb.push_str(&format!("wid {}={}, ", w.wire.getWireId(), val));
                }
                sb.pop();
                sb.pop();
                sb.push(']');
                println!("{sb}");
            }
        }

        assert!(
            idx == self.t.allPubIOWires.len() + self.t.allPrivInWires.len(),
            "Not all inputs consumed"
        );
    }
    pub fn super_prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        if !self.t.serializedArguments.is_empty() {
            // CGConfig::prepFiles(self, circuit_evaluator);
        } else {
            self.cg.writeCircuitFile();
            self.writeDummyInputFile();
        }
    }
}

impl<T: std::fmt::Debug> CircuitGeneratorExtend<ZkayCircuitBase<T>> {
    fn writeDummyInputFile(&self) {
        let mut printWriter = File::create(self.get_name() + ".in").unwrap();
        write!(printWriter, "0 1");
        let mut allIOWires = Vec::with_capacity(
            self.get_in_wires().len()
                + self.get_out_wires().len()
                + self.get_prover_witness_wires().len(),
        );
        allIOWires.append(&mut self.get_in_wires()[1..].to_vec());
        allIOWires.append(&mut self.get_out_wires());
        allIOWires.append(&mut self.get_prover_witness_wires());
        for w in allIOWires {
            write!(printWriter, "{} 0", w.as_ref().unwrap().getWireId());
        }
    }
}

impl<T: crate::circuit::StructNameConfig + std::fmt::Debug + std::clone::Clone>
    CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    pub fn addIO(
        typeName: &str,
        name: String,
        nameList: &mut Vec<String>,
        src: &Vec<Option<WireType>>,
        startIdx: i32,
        size: i32,
        t: ZkayType,
        restrict: bool,
        vars: &mut HashMap<String, Vec<TypedWire>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        // let name = self.getQualifiedName(&name);
        println!(
            "Adding '{name}' = {typeName}[{startIdx}:{}]",
            startIdx + size
        );
        let input = src[startIdx as usize..(startIdx + size) as usize].to_vec();
        // Enforce size and associate wire with type (technically restrict is only required for  inputs)

        let mut tInput: Vec<_> = input
            .iter()
            .map(|i| {
                TypedWire::new(
                    i.clone().unwrap(),
                    t.clone(),
                    name.clone(),
                    &vec![restrict],
                    generator.clone(),
                )
            })
            .collect();

        vars.insert(name.to_owned(), tInput);
        nameList.push(name.to_owned());
        input
    }

    // CRYPTO BACKENDS
    pub fn addCryptoBackend(
        &mut self,
        cryptoBackendId: &str,
        cryptoBackendName: &str,
        keyBits: i32,
    ) {
        assert!(
            !self.t.cryptoBackends.contains_key(cryptoBackendId),
            "Crypto backend {cryptoBackendId} already registered"
        );

        self.t.cryptoBackends.insert(
            cryptoBackendId.to_owned(),
            Backend::create(cryptoBackendName, keyBits, self.cg()),
        );
    }

    pub fn setKeyPairn(&mut self, cryptoBackendId: &str, pkName: &str, skName: &str) {
        self.setKeyPair(
            cryptoBackendId,
            &self.get(pkName).wire,
            &self.get(skName).wire,
        );
    }

    pub fn setKeyPair(&mut self, cryptoBackendId: &str, myPk: &WireType, mySk: &WireType) {
        let generator = self.cg();
        let cryptoBackend = self.get_crypto_backend_mut(cryptoBackendId);
        assert!(
            cryptoBackend.isSymmetric(),
            "Crypto backend is not symmetric"
        );

        let symmetricCrypto = cryptoBackend;
        symmetricCrypto.setKeyPair(myPk, mySk, generator); //TODO
    }
    #[inline]
    pub fn getCryptoBackend(&self, cryptoBackendId: &str) -> &Backend {
        self.t
            .cryptoBackends
            .get(cryptoBackendId)
            .expect(&format!("Unknown crypto backend: {cryptoBackendId}"))
    }
    #[inline]
    pub fn get_crypto_backend_mut(&mut self, cryptoBackendId: &str) -> &mut Backend {
        self.t
            .cryptoBackends
            .get_mut(cryptoBackendId)
            .expect(&format!("Unknown crypto backend: {cryptoBackendId}"))
    }
    pub fn getHomomorphicCryptoBackend(
        &mut self,
        cryptoBackendId: &str,
    ) -> Box<dyn HomomorphicBackend + '_> {
        let mut cryptoBackend = self.getCryptoBackend(cryptoBackendId);
        cryptoBackend.homomorphic_backend().expect(&format!(
            "Crypto backend {cryptoBackendId} is not homomorphic"
        ))
    }

    // CIRCUIT IO

    pub fn addIn(&mut self, name: &str, size: i32, t: ZkayType) {
        let generator = self.cg();
        Self::addIO(
            "in",
            self.getQualifiedName(name),
            &mut self.t.pubInNames,
            &self.t.allPubIOWires,
            self.t.currentPubInIdx,
            size,
            t,
            false,
            &mut self.t.vars,
            generator,
        );
        self.t.currentPubInIdx += size;
    }

    pub fn addKi(&mut self, cryptoBackendId: &str, name: &str, size: i32) {
        let generator = self.cg();

        let (name, allPubIOWires, currentPubInIdx) = (
            self.getQualifiedName(name),
            &self.t.allPubIOWires,
            self.t.currentPubInIdx,
        );
        self.t.currentPubInIdx += size;
        let mut cryptoBackend = self.getCryptoBackend(cryptoBackendId);
        let chunkSize = cryptoBackend.getKeyChunkSize();
        let input = Self::addIO(
            "in",
            name.clone(),
            &mut self.t.pubInNames,
            allPubIOWires,
            currentPubInIdx,
            size,
            ZkayType::ZkUint(chunkSize),
            false,
            &mut self.t.vars,
            generator.clone(),
        );

        let mut cryptoBackend = self.get_crypto_backend_mut(cryptoBackendId);
        cryptoBackend.addKey(&name, &input, generator); //TODO
    }

    pub fn addK(&mut self, name: &str, size: i32) {
        self.addKi(LEGACY_CRYPTO_BACKEND, name, size);
    }

    pub fn addOut(&mut self, name: &str, size: i32, t: ZkayType) {
        let generator = self.cg();
        Self::addIO(
            "out",
            self.getQualifiedName(name),
            &mut self.t.pubOutNames,
            &self.t.allPubIOWires,
            self.t.currentPubOutIdx,
            size,
            t,
            false,
            &mut self.t.vars,
            generator,
        );
        self.t.currentPubOutIdx += size;
    }

    pub fn addS(&mut self, name: &str, size: i32, t: ZkayType) {
        let generator = self.cg();
        Self::addIO(
            "priv",
            self.getQualifiedName(name),
            &mut self.t.privInNames,
            &self.t.allPrivInWires,
            self.t.currentPrivInIdx,
            size,
            t,
            true,
            &mut self.t.vars,
            generator,
        );
        self.t.currentPrivInIdx += size;
    }

    // CONTROL FLOW

    pub fn stepIn(&mut self, fct: &str) {
        Self::pushPrefix(
            &mut self.t.namePrefix,
            &mut self.t.namePrefixIndices,
            &(self
                .t
                .guardPrefixes
                .front()
                .unwrap()
                .front()
                .unwrap()
                .to_owned()
                + fct),
        );
        Self::pushGuardPrefix(&mut self.t.guardPrefixes, &mut self.t.guardPrefixIndices);
    }

    pub fn stepOut(&mut self) {
        self.t.namePrefix.pop_front();
        self.t.guardPrefixes.pop_front();
        self.t.guardPrefixIndices.pop_front();
    }

    pub fn addGuard(&mut self, name: &str, isTrue: bool) {
        let mut newWire = self.get(name).wire.clone();

        Self::pushPrefix(
            self.t.guardPrefixes.front_mut().unwrap(),
            self.t.guardPrefixIndices.front_mut().unwrap(),
            &format!("{name}_{isTrue}"),
        );

        if !isTrue {
            newWire = newWire.invAsBit(&None).unwrap();
        }

        if let Some(v) = self.t.currentGuardCondition.front() {
            newWire = v.wire.and(&newWire, &None);
        }
        self.t.currentGuardCondition.push_front(TypedWire::new(
            newWire,
            zkbool().clone(),
            format!("guard_cond_top_{name}_{isTrue}"),
            &vec![],
            self.cg(),
        ));
    }

    pub fn popGuard(&mut self) {
        self.t.currentGuardCondition.pop_front();
        self.t.guardPrefixes.pop_front();
    }

    pub fn ite(
        &self,
        condition: &TypedWire,
        trueVal: &TypedWire,
        falseVal: &TypedWire,
    ) -> TypedWire {
        ZkayType::checkType(zkbool(), &condition.zkay_type);
        ZkayType::checkType(&trueVal.zkay_type, &falseVal.zkay_type);
        TypedWire::new(
            self.condExpr(&condition.wire, &trueVal.wire, &falseVal.wire),
            trueVal.zkay_type.clone(),
            format!(
                "if {}  {{{}}}  {{{}}}",
                condition.name, trueVal.name, falseVal.name
            ),
            &vec![],
            self.cg(),
        )
    }

    // UNARY OPS

    pub fn negate(&self, val: &TypedWire) -> TypedWire {
        let bits = val.zkay_type.bitwidth;
        if bits < 256 {
            // Take two's complement
            let invBits = TypedWire::new(
                val.wire.invBits(val.zkay_type.bitwidth as u64, &None),
                val.zkay_type.clone(),
                format!("~{}", val.name),
                &vec![],
                self.cg(),
            );
            invBits.plus(&self.val_iz(1, val.zkay_type.clone()))
        } else {
            TypedWire::new(
                val.wire.clone().muli(-1, &Some(format!("-{}", val.name))),
                val.zkay_type.clone(),
                format!("-{}", val.name),
                &vec![],
                self.cg(),
            )
        }
    }

    pub fn bitInv(&self, val: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&val.zkay_type, &val.zkay_type, false);
        let res = val
            .wire
            .invBits(resultType.bitwidth as u64, &Some(format!("~{}", val.name)));
        TypedWire::new(
            res,
            resultType,
            format!("~{}", val.name),
            &vec![],
            self.cg(),
        )
    }

    pub fn not(&self, val: &TypedWire) -> TypedWire {
        ZkayType::checkType(zkbool(), &val.zkay_type);
        TypedWire::new(
            val.wire.invAsBit(&Some(format!("!{}", val.name))).unwrap(),
            zkbool().clone(),
            format!("!{}", val.name),
            &vec![],
            self.cg(),
        )
    }

    // String op interface

    pub fn o_(&self, op: char, wire: &TypedWire) -> TypedWire {
        match op {
            '-' => self.negate(wire),
            '~' => self.bitInv(wire),
            '!' => self.not(wire),
            _ => panic!(),
        }
    }

    pub fn o_tct(&self, lhs: &TypedWire, op: char, rhs: &TypedWire) -> TypedWire {
        match op {
            '+' => lhs.plus(rhs),
            '-' => lhs.minus(rhs),
            '*' => lhs.times(rhs),
            '/' => lhs.divideBy(rhs,self.cg()),
            '%' => lhs.modulo(rhs,self.cg()),
            '|' => lhs.bitOr(rhs),
            '&' => lhs.bitAnd(rhs),
            '^' => lhs.bitXor(rhs),
            '<' => lhs.isLessThan(rhs),
            '>'/*'*/=> lhs.isGreaterThan(rhs),
            _ => panic!(),
        }
    }

    pub fn o_tctct(
        &self,
        cond: &TypedWire,
        condChar: char,
        trueVal: &TypedWire,
        altChar: char,
        falseVal: &TypedWire,
    ) -> TypedWire {
        assert!(condChar == '?' && altChar == ':');
        self.ite(cond, trueVal, falseVal)
    }

    pub fn o_tsi(&self, lhs: &TypedWire, op: &str, rhs: i32) -> TypedWire {
        match op {
            "<<" => lhs.shiftLeftBy(rhs),
            ">>" => lhs.shiftRightBy(rhs),
            _ => panic!(),
        }
    }

    pub fn o_tst(&self, lhs: &TypedWire, op: &str, rhs: &TypedWire) -> TypedWire {
        match op {
            "==" => lhs.isEqualTo(rhs),
            "!=" => lhs.isNotEqualTo(rhs),
            "<=" => lhs.isLessThanOrEqual(rhs),
            ">=" => lhs.isGreaterThanOrEqual(rhs),
            "&&" => lhs.and(rhs),
            "||" => lhs.or(rhs),
            _ => panic!(),
        }
    }

    // Homomorphic operations

    pub fn o_hom(
        &mut self,
        cryptoBackendId: &str,
        key: &str,
        op: char,
        arg: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let generator = self.cg();
        let key = self.getQualifiedName(key);
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicOpu(op, arg, &key, generator)
    }

    pub fn o_hom_sshch(
        &mut self,
        cryptoBackendId: &str,
        key: &str,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let generator = self.cg();
        let key = self.getQualifiedName(key);
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicOp(lhs, op, rhs, &key, generator)
    }

    pub fn o_hom_sshchch(
        &mut self,
        cryptoBackendId: &str,
        key: &str,
        cond: &HomomorphicInput,
        condChar: char,
        trueVal: &HomomorphicInput,
        altChar: char,
        falseVal: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        assert!(condChar == '?' && altChar == ':');
        let key = self.getQualifiedName(key);
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicCond(cond, trueVal, falseVal, &key)
    }

    pub fn o_hom_sshsh(
        &mut self,
        cryptoBackendId: &str,
        key: &str,
        lhs: &HomomorphicInput,
        op: &str,
        rhs: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let key = self.getQualifiedName(key);
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicOps(lhs, op, rhs, &key)
    }

    pub fn o_rerand(
        &mut self,
        arg: &Vec<TypedWire>,
        cryptoBackendId: &str,
        key: &str,
        randomness: &TypedWire,
    ) -> Vec<TypedWire> {
        let generator = self.cg();
        let key = self.getQualifiedName(key);
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicRerand(arg, &key, randomness, generator)
    }

    // TYPE CASTING
    pub fn cast(&self, w: &TypedWire, targetType: ZkayType) -> TypedWire {
        self.convertTo(w, targetType)
    }

    // SOURCE

    pub fn get(&self, name: &str) -> TypedWire {
        let w = self.getTypedArr(name);
        assert!(w.len() == 1, "Tried to treat array as a single wire");
        w[0].clone()
    }

    pub fn getCipher(&self, name: &str) -> &Vec<TypedWire> {
        self.getTypedArr(name)
    }

    pub fn val(&self, val: bool) -> TypedWire {
        TypedWire::new(
            if val {
                self.cg.get_one_wire().unwrap()
            } else {
                self.cg.get_zero_wire().unwrap()
            },
            zkbool().clone(),
            format!("const_{val}"),
            &vec![],
            self.cg(),
        )
    }

    pub fn val_iz(&self, val: i32, t: ZkayType) -> TypedWire {
        if val == 0 {
            TypedWire::new(
                self.get_zero_wire().unwrap(),
                t,
                format!("const_{val}"),
                &vec![],
                self.cg(),
            )
        } else if val == 1 {
            TypedWire::new(
                self.get_one_wire().unwrap(),
                t,
                format!("const_{val}"),
                &vec![],
                self.cg(),
            )
        } else {
            self.val_sz(&val.to_string(), t)
        }
    }

    pub fn val_sz(&self, val: &str, t: ZkayType) -> TypedWire {
        let generator = &self.generator;
        let v = BigInteger::parse_bytes(val.as_bytes(), 10).unwrap();
        let w = if v.sign() == Sign::Minus {
            assert!(!t.signed, "Cannot store negative constant in unsigned wire");
            let vNeg = ZkayType::GetNegativeConstant(&v.clone().neg(), t.bitwidth);
            assert!(vNeg.sign() != Sign::Minus, "Constant is still negative");
            generator.createConstantWire(&vNeg, &Some(format!("const_{v}")))
        } else {
            generator.createConstantWire(&v, &Some(format!("const_{v}")))
        };
        TypedWire::new(w, t, format!("const_{v}"), &vec![], self.cg())
    }

    // SINK

    pub fn decl(&mut self, lhs: &str, val: TypedWire) {
        // assert!(val.zkay_type.is_some(), "Tried to use untyped wires");

        // Get old value and check type
        let mut oldVal;
        if self.t.vars.contains_key(lhs) {
            oldVal = self.get(lhs);
            ZkayType::checkType(&oldVal.zkay_type, &val.zkay_type);
        } else {
            oldVal = self.val_iz(0, val.zkay_type.clone());
        }

        // Only assign value if guard condition is met
        if self.t.currentGuardCondition.is_empty() {
            self.set(
                lhs,
                &TypedWire::new(val.wire, val.zkay_type, lhs.to_owned(), &vec![], self.cg()),
            );
        } else {
            self.set(
                lhs,
                &TypedWire::new(
                    self.condExpr(
                        &self.t.currentGuardCondition.front().unwrap().wire,
                        &val.wire,
                        &oldVal.wire,
                    ),
                    val.zkay_type.clone(),
                    lhs.to_owned(),
                    &vec![],
                    self.cg(),
                ),
            );
        }
    }

    pub fn decl_svt(&mut self, lhs: &str, val: &Vec<TypedWire>) {
        assert!(!val.is_empty(), "val");
        // assert!(val[0].zkay_type.is_some(), "Tried to use untyped wires");
        // Check that all types match; else this gets really strange
        for i in 0..val.len() - 1 {
            ZkayType::checkType(&val[i].zkay_type, &val[i + 1].zkay_type);
        }

        // Get old value and check type and length
        let mut oldVal;
        if self.t.vars.contains_key(lhs) {
            oldVal = self.getTypedArr(lhs).clone();
            ZkayType::checkType(&oldVal[0].zkay_type, &val[0].zkay_type);
            assert!(
                val.len() == oldVal.len(),
                "WireType amounts differ - old ={}, new = {}",
                oldVal.len(),
                val.len()
            );
        } else {
            oldVal = vec![self.val_iz(0, val[0].zkay_type.clone()); val.len()];
        }

        // Only assign value if guard condition is met

        let guard = self.t.currentGuardCondition.front(); // Null if empty
        let resVal: Vec<_> = val
            .iter()
            .zip(&oldVal)
            .map(|(v, ov)| {
                if let Some(g) = guard {
                    TypedWire::new(
                        self.condExpr(&g.wire, &v.wire, &ov.wire),
                        v.zkay_type.clone(),
                        lhs.to_owned(),
                        &vec![],
                        self.cg(),
                    )
                } else {
                    TypedWire::new(
                        v.wire.clone(),
                        v.zkay_type.clone(),
                        lhs.to_owned(),
                        &vec![],
                        self.cg(),
                    ) // No guard, just rename
                }
            })
            .collect();
        self.set_svt(lhs, &resVal);
    }

    pub fn condExpr(&self, cond: &WireType, trueVal: &WireType, falseVal: &WireType) -> WireType {
        if ZkayUtil::ZKAY_RESTRICT_EVERYTHING {
            self.generator.addBinaryAssertion(cond, &None);
        }
        cond.mulw(trueVal, &Some("ite_true".to_owned())).addw(
            &cond
                .invAsBit(&None)
                .unwrap()
                .mulw(falseVal, &Some("ite_false".to_owned())),
            &Some("ite_res".to_owned()),
        )
    }

    pub fn convertTo(&self, w: &TypedWire, targetType: ZkayType) -> TypedWire {
        let fromType = &w.zkay_type;

        let fromBitWidth = fromType.bitwidth;
        let wasSigned = fromType.signed;
        let toBitWidth = targetType.bitwidth;

        let newWire = if fromBitWidth < toBitWidth {
            // Upcast -> sign/zero extend
            if !wasSigned && w.wire.getBitWiresIfExistAlready().is_none() {
                // If this wire was not yet split we can return it without splitting as an optimization
                // -> upcasts from an unsigned type (most common ) are for free this way
                w.wire.clone()
            } else {
                let bitWires = w.wire.getBitWiresi(fromBitWidth as u64, &None);
                if wasSigned && toBitWidth == 256 {
                    // Special  -> sign extension not possible since not enough bits,
                    // want -1 to be field_prime - 1
                    let signBit = bitWires[fromBitWidth as usize - 1].clone().unwrap();
                    signBit.mux(&self.negate(w).wire.muli(-1, &None), &w.wire)
                } else {
                    let extendBit = if wasSigned {
                        bitWires[fromBitWidth as usize - 1].clone().unwrap()
                    } else {
                        self.get_zero_wire().unwrap()
                    };
                    let mut newWs = vec![None; toBitWidth as usize];
                    newWs[..fromBitWidth as usize].clone_from_slice(&bitWires.asArray());
                    newWs[fromBitWidth as usize..toBitWidth as usize].fill(Some(extendBit));
                    WireArray::new(newWs, self.cg().downgrade()).packAsBits(
                        None,
                        Some(toBitWidth as usize),
                        &None,
                    )
                }
            }
        } else if fromBitWidth > toBitWidth {
            // Downcast -> only keep lower bits
            w.wire
                .getBitWiresi(fromBitWidth as u64, &Some(format!("downcast1 {} ", w.name)))
                .packAsBits(
                    None,
                    Some(toBitWidth as usize),
                    &Some(format!("downcast2 {}", w.name)),
                )
        } else {
            // Type stays the same -> no expensive bitwise operations necessary
            w.wire.clone()
        };
        TypedWire::new(
            newWire,
            targetType.clone(),
            format!("({}) {}", targetType, w.name),
            &vec![],
            self.cg(),
        )
    }

    pub fn cryptoEnc(
        &mut self,
        cryptoBackendId: &str,
        plain: &str,
        key: &str,
        rnd: &str,
        isDec: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        let desc = ADD_OP_LABELS.then(|| {
            format!(
                "enc{}({}, {}, {})",
                if isDec { "[dec]" } else { "" },
                self.getQualifiedName(plain),
                self.getQualifiedName(key),
                self.getQualifiedName(rnd)
            )
        });
        let (plain, key, rnd) = (
            &self.get(plain),
            &self.getQualifiedName(key),
            &self.getArr(rnd),
        );
        let mut cryptoBackend = self.get_crypto_backend_mut(cryptoBackendId);
        assert!(
            !cryptoBackend.isSymmetric(),
            "Crypto backend is not asymmetric"
        );

        let enc = cryptoBackend.createEncryptionGadget(plain, key, rnd, &desc, generator);
        enc.getOutputWires().clone()
    }

    pub fn cryptoDec(
        &mut self,
        cryptoBackendId: &str,
        cipher: &str,
        pkey: &str,
        skey: &str,
        expPlain: &str,
    ) -> WireType {
        let generator = self.cg();
        let desc = ADD_OP_LABELS.then(|| {
            format!(
                "dec({}, {}, {})",
                self.getQualifiedName(cipher),
                self.getQualifiedName(pkey),
                self.getQualifiedName(skey)
            )
        });
        let (expPlain, cipher, pkey, skey) = (
            &self.get(expPlain),
            &self.getArr(cipher),
            &self.getQualifiedName(pkey),
            &self.getArr(skey),
        );
        let mut cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        let dec =
            cryptoBackend.createDecryptionGadget(expPlain, cipher, pkey, skey, &desc, generator);
        dec.getOutputWires()[0].clone().unwrap()
    }

    pub fn cryptoSymmEnc(
        &mut self,
        cryptoBackendId: &str,
        plain: &str,
        otherPk: &str,
        ivCipher: &str,
        isDec: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        let desc = ADD_OP_LABELS
            .then(|| format!("enc{}({}, k, iv)", if isDec { "[dec]" } else { "" }, plain));
        let (plain, otherPk, ivCipher) = (
            &self.get(plain),
            &self.getQualifiedName(otherPk),
            &self.getArr(ivCipher),
        );
        let mut cryptoBackend = self.get_crypto_backend_mut(cryptoBackendId);
        assert!(
            cryptoBackend.isSymmetric(),
            "Crypto backend is not symmetric"
        );

        let enc = cryptoBackend.createEncryptionGadget(plain, otherPk, ivCipher, &desc, generator);
        enc.getOutputWires().clone()
    }

    pub fn addGuardedEncryptionAssertion(
        &self,
        expectedCipher: &str,
        computedCipher: &Vec<Option<WireType>>,
    ) {
        let expCipher = self.getArr(expectedCipher);
        let compStr = if ADD_OP_LABELS {
            format!("{} == cipher", self.getQualifiedName(expectedCipher))
        } else {
            "".to_owned()
        };
        self.addGuardedOneAssertion(
            &self.isEqual(&expCipher, &expectedCipher, &computedCipher, "cipher"),
            &Some(compStr),
        );
    }

    pub fn addGuardedNonZeroAssertion(&self, value: &Vec<Option<WireType>>, name: &str) {
        self.addGuardedOneAssertion(
            &self.isNonZero(value, name),
            &Some(format!("assert {} != 0", self.getQualifiedName(name))),
        );
    }

    /**
     * Asymmetric Encryption
     */
    pub fn checkEnc(
        &mut self,
        cryptoBackendId: &str,
        plain: &str,
        key: &str,
        rnd: &str,
        expectedCipher: &str,
    ) {
        // let mut cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        self.addGuardedNonZeroAssertion(&self.getArr(expectedCipher), expectedCipher);

        // 2. Encrypt
        let computedCipher = self.cryptoEnc(cryptoBackendId, plain, key, rnd, false, self.cg());

        // 3. Check encryption == expected cipher
        self.addGuardedEncryptionAssertion(expectedCipher, &computedCipher);
    }

    /**
     * Symmetric Encryption
     */
    pub fn checkSymmEnc(
        &mut self,
        cryptoBackendId: &str,
        plain: &str,
        otherPk: &str,
        ivCipher: &str,
        generator: RcCell<CircuitGenerator>,
    ) {
        // let mut cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        self.addGuardedNonZeroAssertion(&self.getArr(ivCipher), ivCipher);

        // 2. Encrypt
        let computedCipher =
            self.cryptoSymmEnc(cryptoBackendId, plain, otherPk, ivCipher, false, generator);

        // 3. Check encryption == expected cipher
        self.addGuardedEncryptionAssertion(ivCipher, &computedCipher);
    }

    // /**
    //  * Asymmetric Decryption
    //  */
    pub fn checkDec(
        &mut self,
        cryptoBackendId: &str,
        plain: &str,
        key: &str,
        rnd: &str,
        cipher: &str,
    ) {
        let mut cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        if cryptoBackend.usesDecryptionGadget() {
            // TODO we are misusing the randomness wire for the secret key, which is extremely ugly...
            let msgOk = self.cryptoDec(cryptoBackendId, cipher, key, rnd, plain);

            let expCipher = self.getArr(cipher);
            let expCipherIsNonZero = self.isNonZero(&expCipher, cipher); // "!= 0"
            let expCipherIsZero = expCipherIsNonZero
                .invAsBit(&Some(cipher.to_owned() + " == 0"))
                .unwrap();
            let plainZero = self.isZero(&self.getArr(plain), plain);

            // handle uninitialized ciphertext: cipher == 0 => plain == 0
            self.addGuardedOneAssertion(&expCipherIsNonZero.orw(&plainZero, &None), &None);

            // else: cipher != 0 ==> msgOk == 1
            self.addGuardedOneAssertion(&expCipherIsZero.orw(&msgOk, &None), &None);
        } else {
            // 1. Decrypt [dec(cipher, rnd, sk) -> enc(plain, rnd, pk)] (compute inverse op)
            let computedCipher = self.cryptoEnc(cryptoBackendId, plain, key, rnd, true, self.cg());

            let expCipher = self.getArr(cipher);
            let expCipherIsNonZero = self.isNonZero(&expCipher, cipher); // "!= 0"
            let expCipherIsZero = expCipherIsNonZero
                .invAsBit(&Some(cipher.to_owned() + " == 0"))
                .unwrap();
            let plainZero = self.isZero(&self.getArr(plain), plain);
            let rndZero = self.isZero(&self.getArr(rnd), rnd);

            // 2. Check that: expectedCipher == 0 => plain == 0 && rnd == 0
            self.addGuardedOneAssertion(
                &expCipherIsNonZero.orw(&plainZero.and(&rndZero, &None), &None),
                &None,
            );

            // 3. Check that expectedCipher != 0 => expectedCipher == computedCipher
            self.addGuardedOneAssertion(
                &expCipherIsZero.orw(
                    &self.isEqual(&expCipher, cipher, &computedCipher, "cipher"),
                    &None,
                ),
                &None,
            );
        }
    }

    /**
     * Symmetric Decryption
     */
    pub fn checkSymmDec(
        &mut self,
        cryptoBackendId: &str,
        plain: &str,
        otherPk: &str,
        ivCipher: &str,
        generator: RcCell<CircuitGenerator>,
    ) {
        // let mut cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        // 1. Decrypt [dec(cipher, rnd, sk) -> encSymm(plain, ecdh(mySk, otherPk), iv)] (compute inverse op)
        let computedCipher =
            self.cryptoSymmEnc(cryptoBackendId, plain, otherPk, ivCipher, true, generator);

        let expIvCipher = self.getArr(ivCipher);
        let expCipherNonZero = self.isNonZero(&expIvCipher, ivCipher);
        let expCipherZero = expCipherNonZero
            .invAsBit(&Some(ivCipher.to_owned() + " == 0"))
            .unwrap();
        let otherPkNonZero = self
            .get(otherPk)
            .wire
            .checkNonZero(&Some(otherPk.to_owned() + "!= 0"));
        let otherPkZero = otherPkNonZero
            .invAsBit(&Some(otherPk.to_owned() + " == 0"))
            .unwrap();
        let plainZero = self.isZero(&self.getArr(plain), plain);

        // Some of these checks are probably not necessary, as zkay should already enforce that
        // otherPk == 0 <=> expCipher == 0

        // 2. Check that: ivCipher == 0 => plain == 0 && otherPk == 0
        self.addGuardedOneAssertion(
            &expCipherNonZero.orw(&plainZero.and(&otherPkZero, &None), &None),
            &ADD_OP_LABELS
                .then(|| format!("{} == 0 => {} == 0 && {} == 0", ivCipher, plain, otherPk)),
        );

        // 3. Check that: otherPk == 0 => plain == 0 && ivCipher == 0
        self.addGuardedOneAssertion(
            &otherPkNonZero.orw(&plainZero.and(&expCipherZero, &None), &None),
            &ADD_OP_LABELS
                .then(|| format!("{} == 0 => {} == 0 && {} == 0", otherPk, plain, ivCipher)),
        );

        // 4. Check that: (ivCipher != 0 && otherPk != 0) => ivCipher == computedCipher
        let cipherZeroOrPkZero = expCipherZero.orw(&otherPkZero, &None);
        self.addGuardedOneAssertion(
            &cipherZeroOrPkZero.orw(
                &self.isEqual(&expIvCipher, ivCipher, &computedCipher, "cipher"),
                &None,
            ),
            &ADD_OP_LABELS.then(|| {
                format!(
                    "({} != 0 && {} != 0) => {} == {}",
                    ivCipher, otherPk, ivCipher, "cipher"
                )
            }),
        );
    }

    // Legacy handling

    pub fn checkEncs(&mut self, plain: &str, key: &str, rnd: &str, expectedCipher: &str) {
        self.checkEnc(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expectedCipher);
    }

    pub fn checkEncss(&mut self, plain: &str, otherPk: &str, ivCipher: &str) {
        self.checkSymmEnc(LEGACY_CRYPTO_BACKEND, plain, otherPk, ivCipher, self.cg());
    }

    pub fn checkDecs(&mut self, plain: &str, key: &str, rnd: &str, expectedCipher: &str) {
        self.checkDec(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expectedCipher);
    }

    pub fn checkDecsss(&mut self, plain: &str, otherPk: &str, ivCipher: &str) {
        self.checkSymmDec(LEGACY_CRYPTO_BACKEND, plain, otherPk, ivCipher, self.cg());
    }

    pub fn checkEq(&self, lhs: &str, rhs: &str) {
        let (l, r) = (self.getArr(lhs), self.getArr(rhs));
        let len = l.len();
        assert!(r.len() == len, "Size mismatch for equality check");

        for i in 0..len {
            let compStr = if ADD_OP_LABELS {
                &Some(format!(
                    "{}[{}] == {}[{}]",
                    self.getQualifiedName(lhs),
                    i,
                    self.getQualifiedName(rhs),
                    i
                ))
            } else {
                &None
            };
            self.addGuardedEqualityAssertion(
                l[i].as_ref().unwrap(),
                r[i].as_ref().unwrap(),
                compStr,
            );
        }
    }

    pub fn isNonZero(&self, value: &Vec<Option<WireType>>, name: &str) -> WireType {
        let mut res = value[0]
            .as_ref()
            .unwrap()
            .checkNonZero(&Some(name.to_owned() + "[0] != 0"));
        for i in 1..value.len() {
            res = res.addw(
                &value[i]
                    .as_ref()
                    .unwrap()
                    .checkNonZero(&Some(format!("{}[{}] != 0", name, i))),
                &Some(format!("or {}[{}] != 0", name, i)),
            );
        }
        res.checkNonZero(&Some(name.to_owned() + " != 0"))
    }

    pub fn isZero(&self, value: &Vec<Option<WireType>>, name: &str) -> WireType {
        self.isNonZero(value, name)
            .invAsBit(&Some(name.to_owned() + " == 0"))
            .unwrap()
    }

    pub fn isEqual(
        &self,
        wires1: &Vec<Option<WireType>>,
        name1: &str,
        wires2: &Vec<Option<WireType>>,
        name2: &str,
    ) -> WireType {
        assert!(wires1.len() == wires2.len(), "WireType array size mismatch");
        let mut res = self.get_one_wire().unwrap();
        for i in 0..wires1.len() {
            res = res.and(
                &wires1[i].as_ref().unwrap().isEqualTo(
                    wires2[i].as_ref().unwrap(),
                    &Some(format!("{}[{}] == {}[{}]", name1, i, name2, i)),
                ),
                &None,
            );
        }
        res
    }

    pub fn clearPrefix(prefix: &mut VecDeque<String>, indices: &mut HashMap<String, i32>) {
        prefix.clear();
        prefix.push_front("".to_owned());
        indices.clear();
    }

    pub fn pushPrefix(
        prefix: &mut VecDeque<String>,
        prefixIndices: &mut HashMap<String, i32>,
        newStr: &str,
    ) {
        let newPrefix = format!("{}{}.", prefix.front().unwrap(), newStr);
        let count = *prefixIndices.get(&newPrefix).unwrap_or(&0);
        prefixIndices.insert(newPrefix.clone(), count + 1);
        prefix.push_front(format!("{}{}.", newPrefix, count));
    }

    pub fn pushGuardPrefix(
        guardPrefixes: &mut VecDeque<VecDeque<String>>,
        guardPrefixIndices: &mut VecDeque<HashMap<String, i32>>,
    ) {
        let mut newPrefix = VecDeque::new();
        let mut newPrefixIndices = HashMap::new();
        Self::clearPrefix(&mut newPrefix, &mut newPrefixIndices);
        guardPrefixes.push_front(newPrefix);
        guardPrefixIndices.push_front(newPrefixIndices);
    }

    pub fn getQualifiedName(&self, name: &str) -> String {
        if name.starts_with("glob_") {
            name.to_owned()
        } else {
            self.t.namePrefix.front().unwrap().clone() + &name
        }
    }

    pub fn addGuardedEqualityAssertion(
        &self,
        lhs: &WireType,
        rhs: &WireType,
        desc: &Option<String>,
    ) {
        if self.t.currentGuardCondition.is_empty() {
            self.generator.addEqualityAssertion(lhs, rhs, desc);
        } else {
            let eq = lhs.isEqualTo(rhs, &None);
            self.generator.addOneAssertion(
                &self
                    .t
                    .currentGuardCondition
                    .front()
                    .unwrap()
                    .wire
                    .invAsBit(&None)
                    .unwrap()
                    .orw(&eq, &None),
                desc,
            ); // guard => lhs == rhs
        }
    }

    pub fn addGuardedOneAssertion(&self, val: &WireType, desc: &Option<String>) {
        if self.t.currentGuardCondition.is_empty() {
            self.generator.addOneAssertion(val, desc);
        } else {
            self.generator.addOneAssertion(
                &self
                    .t
                    .currentGuardCondition
                    .front()
                    .unwrap()
                    .wire
                    .invAsBit(&None)
                    .unwrap()
                    .orw(val, &None),
                desc,
            ); // guard => val
        }
    }

    pub fn getTypedArr(&self, name: &str) -> &Vec<TypedWire> {
        let name = self.getQualifiedName(name);
        self.t
            .vars
            .get(&name)
            .expect(&format!("Variable {name} is not associated with a wire"))
    }

    pub fn getArr(&self, name: &str) -> Vec<Option<WireType>> {
        self.getTypedArr(name)
            .iter()
            .map(|v| Some(v.wire.clone()))
            .collect()
    }

    pub fn set(&mut self, name: &str, val: &TypedWire) {
        self.set_svt(name, &vec![val.clone()]);
    }

    pub fn set_svt(&mut self, name: &str, val: &Vec<TypedWire>) {
        let name = self.getQualifiedName(name);
        assert!(!val.is_empty(), "Tried to set value {name} to None");
        assert!(
            self.t.vars.insert(name.clone(), val.clone()).is_none(),
            "SSA violation when trying to write to {name}"
        );
    }
}

pub fn s_negate(val: &TypedWire, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    let bits = val.zkay_type.bitwidth;
    if bits < 256 {
        // Take two's complement
        let invBits = TypedWire::new(
            val.wire.invBits(val.zkay_type.bitwidth as u64, &None),
            val.zkay_type.clone(),
            format!("~{}", val.name),
            &vec![],
            generator.clone(),
        );
        invBits.plus(&s_val_iz(1, val.zkay_type.clone(), generator))
    } else {
        TypedWire::new(
            val.wire.muli(-1, &Some(format!("-{}", val.name))),
            val.zkay_type.clone(),
            format!("-{}", val.name),
            &vec![],
            generator.clone(),
        )
    }
}

pub fn s_val_iz(val: i32, t: ZkayType, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    if val == 0 {
        TypedWire::new(
            generator.get_zero_wire().unwrap(),
            t,
            format!("const_{val}"),
            &vec![],
            generator.clone(),
        )
    } else if val == 1 {
        TypedWire::new(
            generator.get_one_wire().unwrap(),
            t,
            format!("const_{val}"),
            &vec![],
            generator.clone(),
        )
    } else {
        s_val_sz(&val.to_string(), t, generator)
    }
}

pub fn s_val_sz(val: &str, t: ZkayType, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    let v = BigInteger::parse_bytes(val.as_bytes(), 10).unwrap();
    let w = if v.sign() == Sign::Minus {
        assert!(!t.signed, "Cannot store negative constant in unsigned wire");
        let vNeg = ZkayType::GetNegativeConstant(&v.clone().neg(), t.bitwidth);
        assert!(vNeg.sign() != Sign::Minus, "Constant is still negative");
        generator.createConstantWire(&vNeg, &Some(format!("const_{v}")))
    } else {
        generator.createConstantWire(&v, &Some(format!("const_{v}")))
    };
    TypedWire::new(w, t, format!("const_{v}"), &vec![], generator.clone())
}
