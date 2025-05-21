#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::non_zero_check_basic_op::{
    NonZeroCheckBasicOp, new_non_zero_check,
};
use crate::circuit::operations::primitive::or_basic_op::{ORBasicOp, new_or};
use crate::circuit::operations::primitive::pack_basic_op::{PackBasicOp, new_pack};
use crate::circuit::operations::primitive::split_basic_op::{SplitBasicOp, new_split};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::linear_combination_wire::LinearCombinationWire;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
pub trait setBitsConfig {
    fn setBits(&self, bits: WireArray) {
        // method overriden in subclasses
        // default action:
        println!(
            "Warning --  you are trying to set bits for either a constant or a bit wire. -- Action Ignored"
        );
    }
}
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Base;
impl setBitsConfig for Base {}
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Wire<T: setBitsConfig + Hash + Clone + Debug + PartialEq> {
    pub wireId: i32,
    pub t: T,
}

impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> Wire<T> {
    pub fn new(wireId: i32, t: T) -> eyre::Result<Self> {
        if wireId < 0 {
            eyre::bail!("wire id cannot be negative");
        }

        Ok(Self { wireId, t })
    }

    pub fn new_array(bits: WireArray, t: T) -> Self {
        let mut _self = Self { wireId: -1, t };
        _self.t.setBits(bits);
        _self
    }
}
impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> WireConfig for Wire<T> {}
pub trait WireConfig: PartialEq {
    fn generator(&self) -> CircuitGenerator {
        CircuitGenerator::getActiveCircuitGenerator()
            .unwrap()
            .clone()
    }
    fn toString(&self) -> String {
        self.getWireId().to_string()
    }

    fn getWireId(&self) -> i32 {
        -1
    }

    fn getBitWires(&self) -> Option<WireArray> {
        None
    }

    fn mulb(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        if b == Util::one() {
            return self.self_instance().unwrap();
        }
        if b == BigInteger::ZERO {
            return self.generator().zeroWire.clone();
        }
        let out =
            WireType::LinearCombination(LinearCombinationWire::new(self.generator().currentWireId));
        self.generator().currentWireId += 1;
        let op = new_const_mul(WireType::Wire(self.clone()), out, b, desc.clone());
        //		self.generator().addToEvaluationQueue(Box::new(op));
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    fn muli(&self, l: i64, desc: Vec<String>) -> WireType {
        return self.mulb(BigInteger::from(l), desc.clone());
    }

    fn mulii(&self, base: i64, exp: i32, desc: Vec<String>) -> WireType {
        let b = BigInteger::from(base);
        b = b.pow(exp as u32);
        return self.mulb(b, desc.clone());
    }

    fn mul(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return self.mulb(w.getConstant(), desc.clone());
        }
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let output = WireType::Variable(VariableWire::new(self.generator().currentWireId));
        self.generator().currentWireId += 1;
        let op = new_mul(WireType::Wire(self.clone()), w, output, desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        output
    }

    fn add(&self, w: WireType, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        return WireArray::new(vec![Some(WireType::Wire(self.clone())), Some(w)])
            .sumAllElements(desc.clone());
    }

    fn addi(&self, v: i64, desc: Vec<String>) -> WireType {
        return self.add(
            self.generator().createConstantWirei(v, desc.clone()),
            desc.clone(),
        );
    }

    fn addb(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        return self.add(
            self.generator().createConstantWire(b, desc.clone()),
            desc.clone(),
        );
    }

    fn sub(&self, w: WireType, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let neg = w.muli(-1, desc.clone());
        return self.add(neg, desc.clone());
    }

    fn subi(&self, v: i64, desc: Vec<String>) -> WireType {
        return self.sub(
            self.generator().createConstantWirei(v, desc.clone()),
            desc.clone(),
        );
    }

    fn subb(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        return self.sub(
            self.generator().createConstantWire(b, desc.clone()),
            desc.clone(),
        );
    }

    fn negate(&self, desc: Vec<String>) -> WireType {
        return self
            .generator()
            .getZeroWire()
            .unwrap()
            .sub(WireType::Wire(self.clone()), desc.clone());
    }

    fn mux(&self, trueValue: WireType, falseValue: WireType) -> WireType {
        return falseValue.clone().add(
            WireType::Wire(self.clone()).mul(trueValue.sub(falseValue, vec![]), vec![]),
            vec![],
        );
    }

    fn checkNonZero(&self, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());

        //  * self wire is not currently used for anything - It's for compatibility
        //  * with earlier experimental versions when the target was Pinocchio

        let out1 = WireType::Wire(Wire::<Base>::new(self.generator().currentWireId, Base));
        self.generator().currentWireId += 1;
        let out2 = WireType::VariableBit(VariableBitWire::new(self.generator().currentWireId));
        self.generator().currentWireId += 1;
        let op = new_non_zero_check(WireType::Wire(self.clone()), out1, out2, desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));

        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 2;
            return cachedOutputs[1].clone().unwrap();
        }
        out2
    }

    fn invAsBit(&self, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone()); // just a precaution .. should not be really needed
        let w1 = self.muli(-1, desc.clone());
        let out = self.generator().oneWire.unwrap().add(w1, desc.clone());
        return out;
    }

    fn or(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.or(WireType::Wire(self.clone()), desc.clone());
        }
        self.packIfNeeded(desc.clone()); // just a precaution .. should not be really
        // needed
        let out = WireType::Variable(VariableWire::new(self.generator().currentWireId));
        self.generator().currentWireId += 1;
        let op = new_or(WireType::Wire(self.clone()), w, out, desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn xor(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.xor(WireType::Wire(self.clone()), desc.clone());
        }
        self.packIfNeeded(desc.clone()); // just a precaution .. should not be really
        // needed
        let out = WireType::Variable(VariableWire::new(self.generator().currentWireId));
        self.generator().currentWireId += 1;
        let op = new_xor(WireType::Wire(self.clone()), w, out, desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn and(&self, w: WireType, desc: Vec<String>) -> WireType {
        return self.mul(w, desc.clone());
    }

    fn getBitWiresi(&self, bitwidth: u64, desc: Vec<String>) -> WireArray {
        let mut bitWires = self.getBitWires();
        if let Some(bitWires) = bitWires {
            if bitwidth < bitWires.len() && self.ConstantWire().is_none() {
                println!(
                    "Warning: getBitWires() was called with different arguments on the same wire more than once"
                );
                println!(
                    "\t It was noted that the argument in the second call was less than the first."
                );
                println!(
                    "\t If self was called for enforcing a bitwidth constraint, you must use restrictBitLengh(), otherwise you can ignore self."
                );
                if Configs.get().unwrap().printStackTraceAtWarnings {
                    println!("Thread.dumpStack();");
                } else {
                    println!(
                        "\t You can view the stack trace by setting Config.printStackTraceAtWarnings to true in the code."
                    );
                }
            }
            return bitWires.adjustLengthi(bitwidth);
        }

        bitWires = self.forceSplit(bitwidth, desc.clone());
        self.setBits(bitWires);
        return bitWires;
    }

    fn getBitWiresIfExistAlready(&self) -> WireArray {
        return self.getBitWires();
    }

    fn forceSplit(&self, bitwidth: i32, desc: Vec<String>) -> WireArray {
        let ws = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            ws[i] = Some(WireType::VariableBit(VariableBitWire::new(
                self.generator().currentWireId,
            )));
            self.generator().currentWireId += 1;
        }
        let op = new_split(WireType::Wire(self.clone()), ws, desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= bitwidth;
            return WireArray::new(cachedOutputs).adjustLengthi(bitwidth);
        }
        WireArray::new(ws)
    }

    fn restrictBitLength(&self, bitWidth: u64, desc: Vec<String>) {
        let mut bitWires = self.getBitWires();
        if let Some(bitWires) = bitWires {
            if bitWires.size() > bitWidth as usize {
                bitWires = self.forceSplit(bitWidth, desc.clone());
                self.setBits(bitWires);
            } else {
                // nothing to be done.
            }
            return;
        }
        self.getBitWires(bitWidth, desc.clone())
    }

    fn xorBitwise(&self, w: WireType, numBits: i32, desc: Vec<String>) -> WireType {
        let bits1 = self.getBitWiresi(numBits, desc.clone());
        let bits2 = w.getBitWiresi(numBits, desc.clone());
        let result = bits1
            .unwrap()
            .xorWireArray(bits2.unwrap(), numBits, desc.clone());
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn xorBitwisei(&self, v: i64, numBits: i32, desc: Vec<String>) -> WireType {
        return self.xorBitwise(
            self.generator().createConstantWire(v, desc.clone()),
            numBits,
            desc.clone(),
        );
    }

    fn xorBitwiseb(&self, b: BigInteger, numBits: i32, desc: Vec<String>) -> WireType {
        return self.xorBitwise(
            self.generator().createConstantWire(b, desc.clone()),
            numBits,
            desc.clone(),
        );
    }

    fn andBitwise(&self, w: WireType, numBits: i32, desc: Vec<String>) -> WireType {
        let bits1 = self.getBitWires(numBits, desc.clone());
        let bits2 = w.getBitWires(numBits, desc.clone());
        let result = bits1.andWireArray(bits2, numBits, desc.clone());
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn andBitwisei(&self, v: i64, numBits: i32, desc: Vec<String>) -> WireType {
        return self.andBitwise(
            self.generator().createConstantWire(v, desc.clone()),
            numBits,
            desc.clone(),
        );
    }

    fn andBitwiseb(&self, b: BigInteger, numBits: i32, desc: Vec<String>) -> WireType {
        return self.andBitwise(
            self.generator().createConstantWire(b, desc.clone()),
            numBits,
            desc.clone(),
        );
    }

    fn orBitwise(&self, w: WireType, numBits: i32, desc: Vec<String>) -> WireType {
        let bits1 = self.getBitWires(numBits, desc.clone());
        let bits2 = w.getBitWires(numBits, desc.clone());
        let result = bits1
            .unwrap()
            .orWireArray(bits2.unwrap(), numBits, desc.clone());
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn orBitwisei(&self, v: i64, numBits: i32, desc: Vec<String>) -> WireType {
        return self.orBitwise(
            self.generator().createConstantWire(v, desc.clone()),
            numBits,
            desc.clone(),
        );
    }

    fn orBitwiseb(&self, b: BigInteger, numBits: i32, desc: Vec<String>) -> WireType {
        return self.orBitwise(
            self.generator().createConstantWire(b, desc.clone()),
            numBits,
            desc.clone(),
        );
    }

    fn isEqualTo(&self, w: WireType, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let s = self.sub(w, desc.clone());
        return s.checkNonZero(desc.clone()).invAsBit(desc.clone());
    }

    fn isEqualTob(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        return self.isEqualTo(self.generator().createConstantWire(b, desc.clone()), vec![]);
    }

    fn isEqualToi(&self, v: i64, desc: Vec<String>) -> WireType {
        return self.isEqualTo(self.generator().createConstantWire(v, desc.clone()), vec![]);
    }

    fn isLessThanOrEqual(&self, w: WireType, bitwidth: i32, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc.clone());
        let sum = pWire
            .add(w, desc.clone())
            .sub(WireType::Wire(self.clone()), desc.clone());
        let bitWires = sum.getBitWiresi(bitwidth + 1, desc.clone());
        return bitWires.get(bitwidth);
    }

    fn isLessThanOrEquali(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isLessThanOrEqual(
            self.generator().createConstantWire(v, desc.clone()),
            bitwidth,
            desc.clone(),
        );
    }

    fn isLessThanOrEqualb(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isLessThanOrEqual(
            self.generator().createConstantWire(b, desc.clone()),
            bitwidth,
            desc.clone(),
        );
    }

    fn isLessThan(&self, w: WireType, bitwidth: i32, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let p = BigInteger::from(2u8).pow(bitwidth);
        let pWire = self.generator().createConstantWire(p, desc.clone());
        let sum = pWire
            .add(WireType::Wire(self.clone()), desc.clone())
            .sub(w, desc.clone());
        let bitWires = sum.getBitWires(bitwidth + 1, desc.clone());
        return bitWires.get(bitwidth).invAsBit(desc.clone());
    }

    fn isLessThani(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isLessThan(
            self.generator().createConstantWirei(v, desc.clone()),
            bitwidth,
            desc.clone(),
        );
    }

    fn isLessThanb(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isLessThan(
            self.generator().createConstantWire(b, desc.clone()),
            bitwidth,
            desc.clone(),
        );
    }

    fn isGreaterThanOrEqual(&self, w: WireType, bitwidth: i32, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let p = BigInteger::from(2u8).pow(bitwidth);
        let pWire = self.generator().createConstantWire(p, desc.clone());
        let sum = pWire
            .add(WireType::Wire(self.clone()), desc.clone())
            .sub(w, desc.clone());
        let bitWires = sum.getBitWires(bitwidth + 1, desc.clone()).unwrap();
        return bitWires.get(bitwidth);
    }

    fn isGreaterThanOrEquali(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isGreaterThanOrEqual(
            self.generator().createConstantWirei(v, desc.clone()),
            bitwidth,
            desc,
        );
    }

    fn isGreaterThanOrEqualb(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isGreaterThanOrEqual(
            self.generator().createConstantWire(b, desc.clone()),
            bitwidth,
            desc,
        );
    }

    fn isGreaterThan(&self, w: WireType, bitwidth: i32, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let p = BigInteger::from(2).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc.clone());
        let sum = pWire
            .add(w, desc.clone())
            .sub(WireType::Wire(self.clone()), desc.clone());
        let bitWires = sum.getBitWiresi(bitwidth + 1, desc.clone()).unwrap();
        return bitWires.get(bitwidth).invAsBit(desc.clone());
    }

    fn isGreaterThani(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isGreaterThan(
            self.generator().createConstantWirei(v, desc.clone()),
            bitwidth,
            desc.clone(),
        );
    }

    fn isGreaterThanb(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isGreaterThan(
            self.generator().createConstantWire(b, desc.clone()),
            bitwidth,
            desc.clone(),
        );
    }

    fn rotateLeft(&self, numBits: usize, s: i32, desc: Vec<String>) -> WireType {
        let bits = self.getBitWiresi(numBits, desc.clone());
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i < s {
                rotatedBits[i] = bits.get(i + (numBits - s));
            } else {
                rotatedBits[i] = bits.get(i - s);
            }
        }
        let result = WireArray::new(rotatedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn rotateRight(&self, numBits: usize, s: i32, desc: Vec<String>) -> WireType {
        let bits = getBitWires(numBits, desc.clone());
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                rotatedBits[i] = bits.get(i - (numBits - s));
            } else {
                rotatedBits[i] = bits.get(i + s);
            }
        }
        let result = WireArray::new(rotatedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn shiftLeft(&self, numBits: usize, s: i32, desc: Vec<String>) -> WireType {
        if s >= numBits {
            // Will always be zero in that case
            return self.generator().zeroWire;
        }

        let bits = self.getBitWires(numBits, desc.clone());
        let mut shiftedBits = vec![None; numBits];
        for i in 0..numBits {
            if i < s {
                shiftedBits[i] = self.generator().zeroWire;
            } else {
                shiftedBits[i] = bits.get(i - s);
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn shiftRight(&self, numBits: i32, s: i32, desc: Vec<String>) -> WireType {
        if s >= numBits {
            // Will always be zero in that case
            return self.generator().zeroWire;
        }

        let bits = self.getBitWires(numBits, desc.clone());
        let mut shiftedBits = vec![None; numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = self.generator().zeroWire;
            } else {
                shiftedBits[i] = bits.get(i + s);
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn shiftArithRight(&self, numBits: i32, s: i32, desc: Vec<String>) -> WireType {
        let bits = self.getBitWires(numBits, desc.clone());
        let mut shiftedBits = vec![None; numBits];
        let sign = bits.get(numBits - 1);
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = sign;
            } else {
                shiftedBits[i] = bits.get(i + s);
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn invBits(&self, bitwidth: i32, desc: Vec<String>) -> WireType {
        let bits = self.getBitWires(bitwidth, desc.clone()).asArray();
        let mut resultBits = vec![None; bits.len()];
        for i in 0..resultBits.len() {
            resultBits[i] = bits[i].invAsBit(desc.clone());
        }
        return WireType::LinearCombination(LinearCombinationWire::new(WireArray::new(resultBits)));
    }

    fn trimBits(
        &self,
        currentNumOfBits: i32,
        desiredNumofBits: i32,
        desc: Vec<String>,
    ) -> WireType {
        let bitWires = self.getBitWires(currentNumOfBits, desc.clone());
        let result = bitWires.adjustLengthi(desiredNumofBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::new(result))
    }

    fn packIfNeeded(&self, desc: Vec<String>) {
        if self.wireId == -1 {
            self.pack(vec![]);
        }
    }

    fn pack(&self, desc: Vec<String>) {
        if self.getWireId() != -1 {
            return;
        }
        let bits = self.getBitWires();
        assert!(
            bits.is_some(),
            "A Pack operation is tried on a wire that has no bits."
        );
        let mut wireId = self.generator().currentWireId;
        self.generator().currentWireId += 1;
        //			Instruction op = PackBasicOp::new(bits.array, self, desc.clone());
        //			self.generator().addToEvaluationQueue(Box::new(op));

        let op = new_pack(bits.array, self, desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));

        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            wireId = cachedOutputs[0].getWireId();
        }
    }

    fn hashCode(&self) -> u64 {
        self.getWireId()
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let w = rhs;
        w.getWireId() == self.getWireId() && w.generator() == self.generator()
    }
}
