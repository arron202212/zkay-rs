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
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::crypto_backend::Backend;

use crate::util::util::BigInteger;
use crate::zkay::crypto::crypto_backend::CryptoBackend;
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
        let mut _self = Self {
            realCircuitName: name,
            cryptoBackends: HashMap::new(),
            currentPubInIdx: 0,
            currentPubOutIdx: pubInSize,
            allPubIOWires: vec![None, (pubInSize + pubOutSize) as usize],
            currentPrivInIdx: 0,
            allPrivInWires: vec![None, privSize as usize],
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
        };
        _self.clearPrefix(_self.namePrefix, _self.namePrefixIndices);
        _self.pushGuardPrefix(_self.guardPrefixes, _self.guardPrefixIndices);
        if let Some(crypto_backend) = cryptoBackend {
            // Legacy handling: add default "main" crypto backend
            let id = cryptoBackendId.unwrap_or(LEGACY_CRYPTO_BACKEND);
            assert!(
                _self
                    .cryptoBackends
                    .insert(id, CryptoBackend::create(crypto_backend, keyBits),)
                    .is_none()
            );
        }

        CircuitGeneratorExtend::<Self>::new("circuit", _self)
    }
}

impl<T: crate::circuit::StructNameConfig + std::fmt::Debug> crate::circuit::StructNameConfig
    for CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    fn name(&self) -> String {
        self.t.name()
    }
}

impl<
    T: ZkayCircuitBaseConfig + crate::circuit::StructNameConfig + std::fmt::Debug + std::clone::Clone,
> CGConfig for CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    fn buildCircuit(&mut self) {
        let (pubInCount, pubOutCount) = (self.t.pubInCount, self.t.pubOutCount);
        // Create IO wires
        let pubOutCount = self.t.t.allPubIOWires.len() - pubInCount;
        let (inArray, outArray) = if self.t.useInputHashing {
            (
                self.createProverWitnessWireArray(pubInCount, "in_"),
                self.createProverWitnessWireArray(pubOutCount, "out_"),
            );
        } else {
            (
                self.createInputWireArray(pubInCount, "in_"),
                self.createInputWireArray(pubOutCount, "out_"),
            )
        };
        let privInArray = self.createProverWitnessWireArray(self.t.allPrivInWires.len(), "priv_");

        // Legacy handling
        let legacyCryptoBackend = self.t.cryptoBackends.get(LEGACY_CRYPTO_BACKEND);
        if legacyCryptoBackend.is_some_and(|v| v.isSymmetric()) {
            let myPk = inArray[0];
            let mySk = privInArray[0];
            self.setKeyPair(LEGACY_CRYPTO_BACKEND, myPk, mySk);
        }

        self.t.allPubIOWires[0..pubInCount].clone_from_slice(&inArray[0..]);
        self.t.allPubIOWires[pubInCount..pubOutCount].clone_from_slice(&outArray[0..]);
        self.t.allPrivInWires[0..self.t.allPrivInWires.len()].clone_from_slice(&privInArray[0..]);
        (self.t.pubOutCount) = (pubOutCount);
        self.t.t.buildCircuit();
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        assert!(
            self.serializedArguments.is_some(),
            "No inputs specified, this should not have been called"
        );

        assert!(
            self.serializedArguments.len() == self.allPubIOWires.len() + self.allPrivInWires.len(),
            "Invalid serialized argument count, expected {} was {}",
            self.allPubIOWires.len(),
            self.serializedArguments.len()
        );

        let mut idx = 0;
        for ioNameList in [&self.pubInNames, &self.pubOutNames, &self.privInNames] {
            for name in ioNameList {
                let wires = self.vars.get(name).unwrap();
                let mut sb = format!("Setting '{name}' to [");
                for w in wires {
                    let val = self.serializedArguments[idx];
                    idx += 1;
                    evaluator.setWireValue(w.wire, val);
                    sb.push_str(&format!("wid {}={}, ", w.wire.getWireId(), val));
                }
                sb.pop();
                sb.pop();
                sb.push(']');
                println!("{sb}");
            }
        }

        assert!(
            idx == self.allPubIOWires.len() + self.allPrivInWires.len(),
            "Not all inputs consumed"
        );
    }
    fn prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        if self.serializedArguments != None {
            CGConfig::prepFiles(self, circuit_evaluator);
        } else {
            self.writeCircuitFile();
            self.writeDummyInputFile();
        }
    }
}

impl<T: ZkayCircuitBaseConfig + std::fmt::Debug> CircuitGeneratorExtend<ZkayCircuitBase<T>> {
    fn writeDummyInputFile(&self) {
        let printWriter = File::create(self.get_name() + ".in");
        write!(printWriter, "0 1");
        let allIOWires = vec![
            0;
            self.get_in_wires().len()
                + self.get_out_wires().len()
                + self.get_prover_witness_wires().len()
        ];
        allIOWires.addAll(&self.get_in_wires()[1..]);
        allIOWires.addAll(self.get_out_wires());
        allIOWires.addAll(self.get_prover_witness_wires());
        for w in allIOWires {
            write!(printWriter, "{} 0", w.getWireId());
        }
    }
}
// impl<T: ZkayCircuitBaseConfig + std::fmt::Debug> ZkayCircuitBaseConfig for CircuitGeneratorExtend<ZkayCircuitBase<T>> {
// }

pub trait ZkayCircuitBaseConfig {
    fn run(&self, args: &Vec<String>) {
        match &args[0] {
            "compile" => self.compileCircuit(),

            "prove" => {
                self.compileCircuit();
                self.parseInputs(args[1..].to_vec());
                //println!("Evaluating circuit '" + realCircuitName + "'");
                self.evalCircuit();
            }

            _ => panic!("invalid command"),
        }
        self.prepFiles();
    }

    fn parseInputs(&self, args: &Vec<String>) {
        let totCount = self.allPubIOWires.len() + self.allPrivInWires.len();
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
        self.serializedArguments = serializedArguments;
    }
    fn compileCircuit(&self) {
        println!("Compiling circuit '{}'", self.realCircuitName);
        self.generateCircuit();
        assert!(
            self.currentPubInIdx == self.pubInCount
                && self.currentPubOutIdx == self.allPubIOWires.len(),
            "Not all pub  inputs assigned"
        );
        assert!(
            self.currentPrivInIdx == self.allPrivInWires.len(),
            "Not all  inputs assigned"
        );
        if self.useInputHashing {
            self.makeOutputArray(
                ZkaySHA256Gadget::new(self.allPubIOWires.clone(), 253).getOutputWires(),
                "digest",
            );
        }
        //println!("Done with generateCircuit, preparing dummy files...");
    }

    fn addIO(
        &self,
        typeName: &str,
        mut name: &str,
        nameList: &mut Vec<String>,
        src: &Vec<Option<WireType>>,
        startIdx: i32,
        size: i32,
        t: ZkayType,
        restrict: bool,
    ) -> Vec<Option<WireType>> {
        name = self.getQualifiedName(&name);
        println!(
            "Adding '{name}' = {typeName}[{startIdx}:{}]",
            startIdx + size
        );
        let input = src[startIdx..startIdx + size].to_vec();
        let mut tInput = vec![TypedWire::default(); input.len()];
        for i in 0..input.len() {
            // Enforce size and associate wire with type (technically restrict is only required for  inputs)
            tInput[i] = TypedWire::new(input[i], t, name, restrict);
        }
        self.vars.insert(name, tInput);
        nameList.push(name);
        input
    }

    // CRYPTO BACKENDS
    fn addCryptoBackend(&self, cryptoBackendId: &str, cryptoBackendName: &str, keyBits: i32) {
        assert!(
            !self.cryptoBackends.contains_key(cryptoBackendId),
            "Crypto backend {cryptoBackendId} already registered"
        );

        self.cryptoBackends.insert(
            cryptoBackendId,
            CryptoBackend::create(cryptoBackendName, keyBits),
        );
    }

    fn setKeyPairn(&self, cryptoBackendId: &str, pkName: &str, skName: &str) {
        self.setKeyPair(
            cryptoBackendId,
            self.get(pkName).wire,
            self.get(skName).wire,
        );
    }

    fn setKeyPair(&self, cryptoBackendId: &str, myPk: &WireType, mySk: &WireType) {
        let cryptoBackend = self.getCryptoBackend(cryptoBackendId);
        assert!(
            cryptoBackend.isSymmetric(),
            "Crypto backend is not symmetric"
        );

        let symmetricCrypto = cryptoBackend;
        symmetricCrypto.setKeyPair(myPk, mySk);
    }

    fn getCryptoBackend(&self, cryptoBackendId: &str) -> Backend {
        let backend = self.cryptoBackends.get(cryptoBackendId);
        assert!(
            backend.is_some(),
            "Unknown crypto backend: {cryptoBackendId}"
        );
        backend
    }

    fn getHomomorphicCryptoBackend(&self, cryptoBackendId: &str) -> Box<dyn HomomorphicBackend> {
        let cryptoBackend = self.getCryptoBackend(cryptoBackendId);
        assert!(
            cryptoBackend.instance_of("HomomorphicBackend"),
            "Crypto backend {cryptoBackendId} is not homomorphic"
        );

        cryptoBackend
    }

    // CIRCUIT IO

    fn addIn(&self, name: &str, size: i32, t: ZkayType) {
        self.addIO(
            "in",
            name,
            &self.pubInNames,
            &self.allPubIOWires,
            self.currentPubInIdx,
            size,
            t,
            false,
        );
        self.t.currentPubInIdx += size;
    }

    fn addKi(&self, cryptoBackendId: &str, name: &str, size: i32) {
        let cryptoBackend = self.getCryptoBackend(cryptoBackendId);
        let chunkSize = cryptoBackend.getKeyChunkSize();
        let input = self.addIO(
            "in",
            name,
            self.pubInNames,
            self.allPubIOWires,
            self.currentPubInIdx,
            size,
            ZkayType::ZkUint(chunkSize),
            false,
        );
        self.currentPubInIdx += size;
        self.cryptoBackend
            .insert(self.getQualifiedName(name), input);
    }

    fn addK(&self, name: &str, size: i32) {
        self.addK(LEGACY_CRYPTO_BACKEND, name, size);
    }

    fn addOut(&self, name: &str, size: i32, t: ZkayType) {
        self.addIO(
            "out",
            name,
            self.pubOutNames,
            self.allPubIOWires,
            self.currentPubOutIdx,
            size,
            t,
            false,
        );
        self.currentPubOutIdx += size;
    }

    fn addS(&self, name: &str, size: i32, t: ZkayType) {
        self.addIO(
            "priv",
            name,
            self.privInNames,
            self.allPrivInWires,
            self.currentPrivInIdx,
            size,
            t,
            true,
        );
        self.currentPrivInIdx += size;
    }

    // CONTROL FLOW

    fn stepIn(&self, fct: &str) {
        self.pushPrefix(
            self.namePrefix,
            self.namePrefixIndices,
            self.guardPrefixes.front().unwrap().front().unwrap() + fct,
        );
        self.pushGuardPrefix(self.guardPrefixes, self.guardPrefixIndices);
    }

    fn stepOut(&self) {
        self.popPrefix(self.namePrefix);
        self.guardPrefixes.pop_front();
        self.guardPrefixIndices.pop_front();
    }

    fn addGuard(&self, name: &str, isTrue: bool) {
        let mut newWire = self.get(name).wire.clone();

        self.pushPrefix(
            self.guardPrefixes.front().unwrap(),
            self.guardPrefixIndices.front().unwrap(),
            format!("{name}_{isTrue}"),
        );

        if !isTrue {
            newWire = newWire.invAsBit();
        }

        if !self.currentGuardCondition.isEmpty() {
            newWire = self
                .currentGuardCondition
                .front()
                .unwrap()
                .wire
                .and(newWire);
        }
        self.currentGuardCondition.push(TypedWire::new(
            newWire,
            zkbool(),
            format!("guard_cond_top_{name}_{isTrue}"),
        ));
    }

    fn popGuard(&self) {
        self.currentGuardCondition.pop_front();
        self.popPrefix(self.guardPrefixes.front().unwrap());
    }

    fn ite(&self, condition: &TypedWire, trueVal: &TypedWire, falseVal: &TypedWire) -> TypedWire {
        ZkayType::checkType(zkbool(), condition.zkay_type);
        ZkayType::checkType(trueVal.zkay_type, falseVal.zkay_type);
        TypedWire::new(
            self.condExpr(condition.wire, trueVal.wire, falseVal.wire),
            trueVal.zkay_type,
            format!(
                "if {}  {{{}}}  {{{}}}",
                condition.name, trueVal.name, falseVal.name
            ),
        )
    }

    // UNARY OPS

    fn negate(&self, val: &TypedWire) -> TypedWire {
        let bits = val.zkay_type.bitwidth;
        if bits < 256 {
            // Take two's complement
            let invBits = TypedWire::new(
                val.wire.invBits(val.zkay_type.bitwidth),
                val.zkay_type,
                format!("~{}", val.name),
            );
            invBits.plus(self.val_iz(1, val.zkay_type))
        } else {
            TypedWire::new(
                val.wire.mul(-1, format!("-{}", val.name)),
                val.zkay_type,
                format!("-{}", val.name),
            )
        }
    }

    fn bitInv(&self, val: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkType(val.zkay_type, val.zkay_type, false);
        let res = val
            .wire
            .invBits(resultType.bitwidth, format!("~{}", val.name));
        TypedWire::new(res, resultType, format!("~{}", val.name))
    }

    fn not(&self, val: &TypedWire) -> TypedWire {
        ZkayType::checkType(zkbool(), val.zkay_type);
        TypedWire::new(
            val.wire.invAsBit(format!("!{}", val.name)),
            zkbool(),
            format!("!{}", val.name),
        )
    }

    // String op interface

    fn o_(&self, op: char, wire: &TypedWire) -> TypedWire {
        match op {
            '-' => self.negate(wire),
            '~' => self.bitInv(wire),
            '!' => self.not(wire),
            _ => panic!(),
        }
    }

    fn o_tct(&self, lhs: &TypedWire, op: char, rhs: &TypedWire) -> TypedWire {
        match op {
            '+' => lhs.plus(rhs),
            '-' => lhs.minus(rhs),
            '*' => lhs.times(rhs),
            '/' => lhs.divideBy(rhs),
            '%' => lhs.modulo(rhs),
            '|' => lhs.bitOr(rhs),
            '&' => lhs.bitAnd(rhs),
            '^' => lhs.bitXor(rhs),
            '<' => lhs.isLessThan(rhs),
            '>'/*'*/=> lhs.isGreaterThan(rhs),
            _ => panic!(),
        }
    }

    fn o_tctct(
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

    fn o_tsi(&self, lhs: &TypedWire, op: &str, rhs: i32) -> TypedWire {
        match op {
            "<<" => lhs.shiftLeftBy(rhs),
            ">>" => lhs.shiftRightBy(rhs),
            _ => panic!(),
        }
    }

    fn o_tst(&self, lhs: &TypedWire, op: &str, rhs: &TypedWire) -> TypedWire {
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

    fn o_hom(
        &self,
        cryptoBackendId: &str,
        key: &str,
        op: char,
        arg: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicOp(op, arg, self.getQualifiedName(key))
    }

    fn o_hom_sshch(
        &self,
        cryptoBackendId: &str,
        key: &str,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicOp(lhs, op, rhs, self.getQualifiedName(key))
    }

    fn o_hom_sshchch(
        &self,
        cryptoBackendId: &str,
        key: &str,
        cond: &HomomorphicInput,
        condChar: char,
        trueVal: &HomomorphicInput,
        altChar: char,
        falseVal: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        assert!(condChar == '?' && altChar == ':');
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicCond(cond, trueVal, falseVal, self.getQualifiedName(key))
    }

    fn o_hom_sshsh(
        &self,
        cryptoBackendId: &str,
        key: &str,
        lhs: &HomomorphicInput,
        op: &str,
        rhs: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicOp(lhs, op, rhs, self.getQualifiedName(key))
    }

    fn o_rerand(
        &self,
        arg: &Vec<TypedWire>,
        cryptoBackendId: &str,
        key: &str,
        randomness: &TypedWire,
    ) -> Vec<TypedWire> {
        let backend = self.getHomomorphicCryptoBackend(cryptoBackendId);
        backend.doHomomorphicRerand(arg, self.getQualifiedName(key), randomness)
    }

    // TYPE CASTING
    fn cast(&self, w: &TypedWire, targetType: ZkayType) -> TypedWire {
        self.convertTo(w, targetType)
    }

    // SOURCE

    fn get(&self, name: &str) -> TypedWire {
        let w = self.getTypedArr(name);
        assert!(w.len() == 1, "Tried to treat array as a single wire");
        w[0]
    }

    fn getCipher(&self, name: &str) -> Vec<TypedWire> {
        self.getTypedArr(name)
    }

    fn val(&self, val: bool) -> TypedWire {
        TypedWire::new(
            if val {
                self.cg.get_one_wire()
            } else {
                self.cg.get_zero_wire()
            },
            zkbool(),
            format!("const_{val}"),
        )
    }

    fn val_iz(&self, val: i32, t: ZkayType) -> TypedWire {
        if val == 0 {
            TypedWire::new(self.get_zero_wire(), t, format!("const_{val}"))
        } else if val == 1 {
            TypedWire::new(self.get_one_wire(), t, format!("const_{val}"))
        } else {
            self.val_sz(val.to_string(), t)
        }
    }

    fn val_sz(&self, val: &str, t: ZkayType) -> TypedWire {
        let v = BigInteger::parse_bytes(val.as_bytes(), 10).unwrap();
        let w = if v.sign() == Sign::Minus {
            assert!(!t.signed, "Cannot store negative constant in unsigned wire");
            let vNeg = ZkayType::GetNegativeConstant(v.neg(), t.bitwidth);
            assert!(vNeg.sign() != Sign::Minus, "Constant is still negative");
            self.createConstantWire(vNeg, format!("const_{v}"))
        } else {
            self.createConstantWire(v, format!("const_{v}"))
        };
        TypedWire::new(w, t, format!("const_{v}"))
    }

    // SINK

    fn decl(&self, lhs: &str, val: TypedWire) {
        assert!(val.zkay_type.is_some(), "Tried to use untyped wires");

        // Get old value and check type
        let mut oldVal;
        if self.vars.contains_key(lhs) {
            oldVal = self.get(lhs);
            ZkayType::checkType(oldVal.zkay_type, val.zkay_type);
        } else {
            oldVal = val(0, val.zkay_type);
        }

        // Only assign value if guard condition is met
        if self.currentGuardCondition.isEmpty() {
            self.set(lhs, TypedWire::new(val.wire, val.zkay_type, lhs));
        } else {
            self.set(
                lhs,
                TypedWire::new(
                    self.condExpr(
                        self.currentGuardCondition.front().unwrap().wire,
                        val.wire,
                        oldVal.wire,
                    ),
                    val.zkay_type,
                    lhs,
                ),
            );
        }
    }

    fn decl_svt(&self, lhs: &str, val: &Vec<TypedWire>) {
        assert!(val.is_some() && !val.empty(), "val");
        assert!(val[0].zkay_type.is_some(), "Tried to use untyped wires");
        // Check that all types match; else this gets really strange
        for i in 0..val.len() - 1 {
            ZkayType::checkType(val[i].zkay_type, val[i + 1].zkay_type);
        }

        // Get old value and check type and length
        let mut oldVal;
        if self.vars.contains_key(lhs) {
            oldVal = self.getTypedArr(lhs);
            ZkayType::checkType(oldVal[0].zkay_type, val[0].zkay_type);
            assert!(
                val.len() == oldVal.len(),
                "WireType amounts differ - old ={}, new = {}",
                oldVal.len(),
                val.len()
            );
        } else {
            oldVal = vec![self.val(0, val[0].zkay_type); val.len()];
        }

        // Only assign value if guard condition is met
        let resVal = vec![TypedWire::default(); val.len()];
        let guard = self.currentGuardCondition.front(); // Null if empty
        for i in 0..val.len() {
            if guard == None {
                resVal[i] = TypedWire::new(val[i].wire, val[i].zkay_type, lhs); // No guard, just rename
            } else {
                resVal[i] = TypedWire::new(
                    self.condExpr(guard.wire, val[i].wire, oldVal[i].wire),
                    val[i].zkay_type,
                    lhs,
                );
            }
        }
        self.set(lhs, resVal);
    }

    fn condExpr(&self, cond: &WireType, trueVal: &WireType, falseVal: &WireType) -> WireType {
        if ZkayUtil::ZKAY_RESTRICT_EVERYTHING {
            self.addBinaryAssertion(cond);
        }
        return cond
            .mul(trueVal, "ite_true")
            .add(cond.invAsBit().mul(falseVal, "ite_false"), "ite_res");
    }

    fn convertTo(&self, w: &TypedWire, targetType: ZkayType) -> TypedWire {
        let fromType = w.zkay_type;

        let fromBitWidth = fromType.bitwidth;
        let wasSigned = fromType.signed;
        let toBitWidth = targetType.bitwidth;

        let mut newWire;
        if fromBitWidth < toBitWidth {
            // Upcast -> sign/zero extend
            if !wasSigned && w.wire.getBitWiresIfExistAlready() == None {
                // If this wire was not yet split we can return it without splitting as an optimization
                // -> upcasts from an unsigned type (most common ) are for free this way
                newWire = w.wire.clone();
            } else {
                let bitWires = w.wire.getBitWires(fromBitWidth);
                if wasSigned && toBitWidth == 256 {
                    // Special  -> sign extension not possible since not enough bits,
                    // want -1 to be field_prime - 1
                    let signBit = bitWires.get(fromBitWidth - 1);
                    newWire = signBit.mux(self.negate(w).wire.clone().mul(-1), w.wire);
                } else {
                    let extendBit = if wasSigned {
                        bitWires.get(fromBitWidth - 1)
                    } else {
                        self.get_zero_wire()
                    };
                    let mut newWs = vec![None; toBitWidth];
                    newWs[..fromBitWidth].clone_from_slice(&bitWires.asArray());
                    newWs[fromBitWidth..toBitWidth].fill(extendBit);
                    newWire = WireArray::new(newWs).packAsBits(None, toBitWidth);
                }
            }
        } else if fromBitWidth > toBitWidth {
            // Downcast -> only keep lower bits
            newWire = w
                .wire
                .getBitWires(fromBitWidth, format!("downcast1 {} ", w.name))
                .packAsBits(None, None, toBitWidth, format!("downcast2 {}", w.name));
        } else {
            // Type stays the same -> no expensive bitwise operations necessary
            newWire = w.wire.clone();
        }
        TypedWire::new(newWire, targetType, format!("({}) {}", targetType, w.name))
    }

    fn cryptoEnc(
        &self,
        cryptoBackend: &Backend,
        plain: &str,
        key: &str,
        rnd: &str,
        isDec: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        assert!(
            !cryptoBackend.isSymmetric(),
            "Crypto backend is not asymmetric"
        );

        let desc = if ADD_OP_LABELS {
            format!(
                "enc{}({}, {}, {})",
                if isDec { "[dec]" } else { "" },
                self.getQualifiedName(plain),
                self.getQualifiedName(key),
                self.getQualifiedName(rnd)
            )
        } else {
            String::new()
        };
        let enc = cryptoBackend.createEncryptionGadget(
            self.get(plain),
            self.getQualifiedName(key),
            self.getArr(rnd),
            desc,
            generator,
        );
        enc.getOutputWires().clone()
    }

    fn cryptoDec(
        &self,
        cryptoBackend: &Backend,
        cipher: &str,
        pkey: &str,
        skey: &str,
        expPlain: &str,
    ) -> WireType {
        let desc = if ADD_OP_LABELS {
            format!(
                "dec({}, {}, {})",
                self.getQualifiedName(cipher),
                self.getQualifiedName(pkey),
                self.getQualifiedName(skey)
            )
        } else {
            "".to_owned()
        };
        let dec = cryptoBackend.createDecryptionGadget(
            self.get(expPlain),
            self.getArr(cipher),
            self.getQualifiedName(pkey),
            self.getArr(skey),
            desc,
        );
        dec.getOutputWires()[0].clone().unwrap()
    }

    fn cryptoSymmEnc(
        &self,
        cryptoBackend: &Backend,
        plain: &str,
        otherPk: &str,
        ivCipher: &str,
        isDec: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        assert!(
            cryptoBackend.isSymmetric(),
            "Crypto backend is not symmetric"
        );

        let desc = if ADD_OP_LABELS {
            format!("enc{}({}, k, iv)", if isDec { "[dec]" } else { "" }, plain)
        } else {
            "".to_owned()
        };
        let enc = cryptoBackend.createEncryptionGadget(
            self.get(plain),
            self.getQualifiedName(otherPk),
            self.getArr(ivCipher),
            desc,
            generator,
        );
        enc.getOutputWires().clone()
    }

    fn addGuardedEncryptionAssertion(
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
            self.isEqual(expCipher, expectedCipher, computedCipher, "cipher"),
            compStr,
        );
    }

    fn addGuardedNonZeroAssertion(&self, value: &Vec<Option<WireType>>, name: &str) {
        self.addGuardedOneAssertion(
            self.isNonZero(value, name),
            format!("assert {} != 0", self.getQualifiedName(name)),
        );
    }

    /**
     * Asymmetric Encryption
     */
    fn checkEnc(
        &self,
        cryptoBackendId: &str,
        plain: &str,
        key: &str,
        rnd: &str,
        expectedCipher: &str,
    ) {
        let cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        self.addGuardedNonZeroAssertion(self.getArr(expectedCipher), expectedCipher);

        // 2. Encrypt
        let computedCipher = self.cryptoEnc(cryptoBackend, plain, key, rnd, false);

        // 3. Check encryption == expected cipher
        self.addGuardedEncryptionAssertion(expectedCipher, computedCipher);
    }

    /**
     * Symmetric Encryption
     */
    fn checkSymmEnc(
        &self,
        cryptoBackendId: &str,
        plain: &str,
        otherPk: &str,
        ivCipher: &str,
        generator: RcCell<CircuitGenerator>,
    ) {
        let cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        self.addGuardedNonZeroAssertion(self.getArr(ivCipher), ivCipher);

        // 2. Encrypt
        let computedCipher =
            self.cryptoSymmEnc(cryptoBackend, plain, otherPk, ivCipher, false, generator);

        // 3. Check encryption == expected cipher
        self.addGuardedEncryptionAssertion(ivCipher, computedCipher);
    }

    // /**
    //  * Asymmetric Decryption
    //  */
    fn checkDec(&self, cryptoBackendId: &str, plain: &str, key: &str, rnd: &str, cipher: &str) {
        let cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        if cryptoBackend.usesDecryptionGadget() {
            // TODO we are misusing the randomness wire for the secret key, which is extremely ugly...
            let msgOk = self.cryptoDec(cryptoBackend, cipher, key, rnd, plain);

            let expCipher = self.getArr(cipher);
            let expCipherIsNonZero = self.isNonZero(expCipher, cipher); // "!= 0"
            let expCipherIsZero = expCipherIsNonZero.invAsBit(cipher + " == 0");
            let plainZero = self.isZero(self.getArr(plain), plain);

            // handle uninitialized ciphertext: cipher == 0 => plain == 0
            self.addGuardedOneAssertion(expCipherIsNonZero.or(plainZero));

            // else: cipher != 0 ==> msgOk == 1
            self.addGuardedOneAssertion(expCipherIsZero.or(msgOk));
        } else {
            // 1. Decrypt [dec(cipher, rnd, sk) -> enc(plain, rnd, pk)] (compute inverse op)
            let computedCipher = self.cryptoEnc(cryptoBackend, plain, key, rnd, true);

            let expCipher = self.getArr(cipher);
            let expCipherIsNonZero = self.isNonZero(expCipher, cipher); // "!= 0"
            let expCipherIsZero = expCipherIsNonZero.invAsBit(cipher + " == 0");
            let plainZero = self.isZero(self.getArr(plain), plain);
            let rndZero = self.isZero(self.getArr(rnd), rnd);

            // 2. Check that: expectedCipher == 0 => plain == 0 && rnd == 0
            self.addGuardedOneAssertion(expCipherIsNonZero.or(plainZero.and(rndZero)));

            // 3. Check that expectedCipher != 0 => expectedCipher == computedCipher
            self.addGuardedOneAssertion(expCipherIsZero.or(self.isEqual(
                expCipher,
                cipher,
                computedCipher,
                "cipher",
            )));
        }
    }

    /**
     * Symmetric Decryption
     */
    fn checkSymmDec(
        &self,
        cryptoBackendId: &str,
        plain: &str,
        otherPk: &str,
        ivCipher: &str,
        generator: RcCell<CircuitGenerator>,
    ) {
        let cryptoBackend = self.getCryptoBackend(cryptoBackendId);

        // 1. Decrypt [dec(cipher, rnd, sk) -> encSymm(plain, ecdh(mySk, otherPk), iv)] (compute inverse op)
        let computedCipher =
            self.cryptoSymmEnc(cryptoBackend, plain, otherPk, ivCipher, true, generator);

        let expIvCipher = self.getArr(ivCipher);
        let expCipherNonZero = self.isNonZero(expIvCipher, ivCipher);
        let expCipherZero = expCipherNonZero.invAsBit(ivCipher + " == 0");
        let otherPkNonZero = self.get(otherPk).wire.checkNonZero(otherPk + "!= 0");
        let otherPkZero = otherPkNonZero.invAsBit(otherPk + " == 0");
        let plainZero = self.isZero(self.getArr(plain), plain);

        // Some of these checks are probably not necessary, as zkay should already enforce that
        // otherPk == 0 <=> expCipher == 0

        // 2. Check that: ivCipher == 0 => plain == 0 && otherPk == 0
        self.addGuardedOneAssertion(
            expCipherNonZero.or(plainZero.and(otherPkZero)),
            if ADD_OP_LABELS {
                format!("{} == 0 => {} == 0 && {} == 0", ivCipher, plain, otherPk)
            } else {
                "".to_owned()
            },
        );

        // 3. Check that: otherPk == 0 => plain == 0 && ivCipher == 0
        self.addGuardedOneAssertion(
            otherPkNonZero.or(plainZero.and(expCipherZero)),
            if ADD_OP_LABELS {
                format!("{} == 0 => {} == 0 && {} == 0", otherPk, plain, ivCipher)
            } else {
                "".to_owned()
            },
        );

        // 4. Check that: (ivCipher != 0 && otherPk != 0) => ivCipher == computedCipher
        let cipherZeroOrPkZero = expCipherZero.or(otherPkZero);
        self.addGuardedOneAssertion(
            cipherZeroOrPkZero.or(self.isEqual(expIvCipher, ivCipher, computedCipher, "cipher")),
            if ADD_OP_LABELS {
                format!(
                    "({} != 0 && {} != 0) => {} == {}",
                    ivCipher, otherPk, ivCipher, "cipher"
                )
            } else {
                "".to_owned()
            },
        );
    }

    // Legacy handling

    fn checkEncs(&self, plain: &str, key: &str, rnd: &str, expectedCipher: &str) {
        self.checkEnc(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expectedCipher);
    }

    fn checkEncss(&self, plain: &str, otherPk: &str, ivCipher: &str) {
        self.checkSymmEnc(LEGACY_CRYPTO_BACKEND, plain, otherPk, ivCipher);
    }

    fn checkDecs(&self, plain: &str, key: &str, rnd: &str, expectedCipher: &str) {
        self.checkDec(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expectedCipher);
    }

    fn checkDecsss(&self, plain: &str, otherPk: &str, ivCipher: &str) {
        self.checkSymmDec(LEGACY_CRYPTO_BACKEND, plain, otherPk, ivCipher);
    }

    fn checkEq(&self, lhs: &str, rhs: &str) {
        let (l, r) = (self.getArr(lhs), self.getArr(rhs));
        let len = l.len();
        assert!(r.len() == len, "Size mismatch for equality check");

        for i in 0..len {
            let compStr = if ADD_OP_LABELS {
                format!(
                    "{}[{}] == {}[{}]",
                    self.getQualifiedName(lhs),
                    i,
                    self.getQualifiedName(rhs),
                    i
                )
            } else {
                ""
            };
            self.addGuardedEqualityAssertion(l[i], r[i], compStr);
        }
    }

    fn isNonZero(&self, value: &Vec<Option<WireType>>, name: &str) -> WireType {
        let res = value[0].checkNonZero(name + "[0] != 0");
        for i in 1..value.len() {
            res = res.add(
                value[i].checkNonZero(format!("{}[{}] != 0", name, i)),
                format!("or {}[{}] != 0", name, i),
            );
        }
        res.checkNonZero(name + " != 0")
    }

    fn isZero(&self, value: &Vec<Option<WireType>>, name: &str) -> WireType {
        self.isNonZero(value, name).invAsBit(name + " == 0")
    }

    fn isEqual(
        &self,
        wires1: &Vec<Option<WireType>>,
        name1: &str,
        wires2: &Vec<Option<WireType>>,
        name2: &str,
    ) -> WireType {
        assert!(
            wires1.length == wires2.length,
            "WireType array size mismatch"
        );
        let res = self.get_one_wire();
        for i in 0..wires1.length {
            res = res.and(
                wires1[i].isEqualTo(wires2[i], format!("{}[{}] == {}[{}]", name1, i, name2, i)),
            );
        }
        res
    }

    fn clearPrefix(prefix: &mut VecDeque<String>, &mut indices: HashMap<String, i32>) {
        prefix.clear();
        prefix.push("".to_owned());
        indices.clear();
    }

    fn pushPrefix(
        prefix: &mut VecDeque<String>,
        prefixIndices: &mut HashMap<String, i32>,
        newStr: &str,
    ) {
        let newPrefix = format!("{}{}.", prefix.front().unwrap(), newStr);
        let count = *prefixIndices.get(newPrefix).unwrap_or(&0);
        prefixIndices.insert(newPrefix, count + 1);
        prefix.push_front(format!("{}{}.", newPrefix, count));
    }

    fn pushGuardPrefix(
        guardPrefixes: &mut VecDeque<VecDeque<String>>,
        guardPrefixIndices: &mut VecDeque<HashMap<String, i32>>,
    ) {
        let newPrefix = VecDeque::new();
        let newPrefixIndices = HashMap::new();
        Self::clearPrefix(&mut newPrefix, &mut newPrefixIndices);
        guardPrefixes.push(newPrefix);
        guardPrefixIndices.push(newPrefixIndices);
    }

    fn popPrefix(prefix: &mut VecDeque<String>) {
        prefix.pop_front();
    }

    fn getQualifiedName(&self, name: &str) -> String {
        if name.starts_with("glob_") {
            name.to_owned()
        } else {
            self.namePrefix.front().unwrap().clone() + &name
        }
    }

    fn addGuardedEqualityAssertion(&self, lhs: &WireType, rhs: &WireType, desc: &Option<String>) {
        if self.currentGuardCondition.isEmpty() {
            self.addEqualityAssertion(lhs, rhs, desc);
        } else {
            let eq = lhs.isEqualTo(rhs);
            self.addOneAssertion(
                self.currentGuardCondition
                    .front()
                    .unwrap()
                    .wire
                    .invAsBit()
                    .or(eq),
                desc,
            ); // guard => lhs == rhs
        }
    }

    fn addGuardedOneAssertion(&self, val: &WireType, desc: &Option<String>) {
        if self.currentGuardCondition.isEmpty() {
            self.addOneAssertion(val, desc);
        } else {
            self.addOneAssertion(
                self.currentGuardCondition
                    .front()
                    .unwrap()
                    .wire
                    .invAsBit()
                    .or(val),
                desc,
            ); // guard => val
        }
    }

    fn getTypedArr(&self, name: &str) -> &Vec<TypedWire> {
        let name = self.getQualifiedName(name);
        let w = self.vars.get(name).unwrap();
        assert!(w.is_some(), "Variable {name} is not associated with a wire");
        w
    }

    fn getArr(&self, name: &str) -> Vec<Option<WireType>> {
        self.getTypedArr(name)
            .iter()
            .map(|v| v.wire.clone())
            .collect()
    }

    fn set(&self, name: &str, val: &TypedWire) {
        self.set(name, vec![val.clone()]);
    }

    fn set_svt(&self, name: &str, val: &Vec<TypedWire>) {
        let name = self.getQualifiedName(name);
        assert!(!val.is_empty(), "Tried to set value {name} to None");
        assert!(
            !self.vars.contains_key(name),
            "SSA violation when trying to write to {name}"
        );
        self.vars.insert(name, val);
    }
}

pub fn s_negate(val: &TypedWire, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    let bits = val.zkay_type.bitwidth;
    if bits < 256 {
        // Take two's complement
        let invBits = TypedWire::new(
            val.wire.invBits(val.zkay_type.bitwidth),
            val.zkay_type,
            format!("~{}", val.name),
        );
        invBits.plus(s_val_iz(1, val.zkay_type, generator))
    } else {
        TypedWire::new(
            val.wire.mul(-1, format!("-{}", val.name)),
            val.zkay_type,
            format!("-{}", val.name),
        )
    }
}

pub fn s_val_iz(val: i32, t: ZkayType, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    if val == 0 {
        TypedWire::new(generator.get_zero_wire(), t, format!("const_{val}"))
    } else if val == 1 {
        TypedWire::new(generator.get_one_wire(), t, format!("const_{val}"))
    } else {
        s_val_sz(val.to_string(), t, generator)
    }
}

pub fn s_val_sz(val: &str, t: ZkayType, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    let v = BigInteger::parse_bytes(val.as_bytes(), 10).unwrap();
    let w = if v.sign() == Sign::Minus {
        assert!(!t.signed, "Cannot store negative constant in unsigned wire");
        let vNeg = ZkayType::GetNegativeConstant(v.neg(), t.bitwidth);
        assert!(vNeg.sign() != Sign::Minus, "Constant is still negative");
        generator.createConstantWire(vNeg, format!("const_{v}"))
    } else {
        generator.createConstantWire(v, format!("const_{v}"))
    };
    TypedWire::new(w, t, format!("const_{v}"))
}
