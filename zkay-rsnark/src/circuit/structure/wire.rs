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
    fn setBits(&self, bits: Option<WireArray>) {
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
        _self.t.setBits(Some(bits));
        _self
    }
}
impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> setBitsConfig for Wire<T> {}
impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> WireConfig for Wire<T> {}
pub trait WireConfig: PartialEq + setBitsConfig{
 fn instance_of(&self, name: &str) -> bool {
        self.name() == name
    }
    fn name(&self) -> &str {
        ""
    }
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
    fn self_clone(&self) -> Option<WireType> {
        None
    }
    fn mulb(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        if b == Util::one() {
            return self.self_clone().unwrap();
        }
        if b == BigInteger::ZERO {
            return self.generator().zeroWire.borrow().clone().unwrap();
        }
        let out =
            WireType::LinearCombination(LinearCombinationWire::new(*self.generator().currentWireId.borrow_mut()));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_const_mul(self.self_clone().unwrap(), out.clone(), b, desc.clone());
        //		self.generator().addToEvaluationQueue(Box::new(op));
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn muli(&self, l: i64, desc: Vec<String>) -> WireType {
        return self.mulb(BigInteger::from(l), desc.clone());
    }

    fn mulii(&self, base: i64, exp: i32, desc: Vec<String>) -> WireType {
        let mut b = BigInteger::from(base);
        b = b.pow(exp as u32);
        return self.mulb(b, desc.clone());
    }

    fn mulw(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return self.mulb(w.getConstant(), desc.clone());
        }
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let output = WireType::Variable(VariableWire::new(*self.generator().currentWireId.borrow_mut()));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_mul(self.self_clone().unwrap(), w, output.clone(), desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        output
    }

    fn addw(&self, w: WireType, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        return WireArray::new(vec![Some(self.self_clone().unwrap()), Some(w)])
            .sumAllElements(desc.clone());
    }

    fn addi(&self, v: i64, desc: Vec<String>) -> WireType {
        return self.addw(
            self.generator().createConstantWirei(v, desc.clone()),
            desc.clone(),
        );
    }

    fn addb(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        return self.addw(
            self.generator().createConstantWire(b, desc.clone()),
            desc.clone(),
        );
    }

    fn subw(&self, w: WireType, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let neg = w.muli(-1, desc.clone());
        return self.addw(neg, desc.clone());
    }

    fn subi(&self, v: i64, desc: Vec<String>) -> WireType {
        return self.subw(
            self.generator().createConstantWirei(v, desc.clone()),
            desc.clone(),
        );
    }

    fn subb(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        return self.subw(
            self.generator().createConstantWire(b, desc.clone()),
            desc.clone(),
        );
    }

    fn negate(&self, desc: Vec<String>) -> WireType {
        return self
            .generator()
            .getZeroWire()
            .unwrap()
            .subw(self.self_clone().unwrap(), desc.clone());
    }

    fn mux(&self, trueValue: WireType, falseValue: WireType) -> WireType {
        return falseValue.clone().addw(
            self.self_clone().unwrap().mulw(trueValue.subw(falseValue, vec![]), vec![]),
            vec![],
        );
    }

    fn checkNonZero(&self, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());

        //  * self wire is not currently used for anything - It's for compatibility
        //  * with earlier experimental versions when the target was Pinocchio

        let out1 = WireType::Wire(Wire::<Base>::new(*self.generator().currentWireId.borrow_mut(), Base).unwrap());
        *self.generator().currentWireId.borrow_mut() += 1;
        let out2 = WireType::VariableBit(VariableBitWire::new(*self.generator().currentWireId.borrow_mut()));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_non_zero_check(self.self_clone().unwrap(), out1, out2.clone(), desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));

        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 2;
            return cachedOutputs[1].clone().unwrap();
        }
        out2
    }

    fn invAsBit(&self, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone()); // just a precaution .. should not be really needed
        let w1 = self.muli(-1, desc.clone());
        let out = self.generator().oneWire.unwrap().addw(w1, desc.clone());
        return out;
    }

    fn orw(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.orw(self.self_clone().unwrap(), desc.clone());
        }
        self.packIfNeeded(desc.clone()); // just a precaution .. should not be really
        // needed
        let out = WireType::Variable(VariableWire::new(*self.generator().currentWireId.borrow_mut()));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_or(self.self_clone().unwrap(), w, out.clone(), desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn xorw(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.xorw(self.self_clone().unwrap(), desc.clone());
        }
        self.packIfNeeded(desc.clone()); // just a precaution .. should not be really
        // needed
        let out = WireType::Variable(VariableWire::new(*self.generator().currentWireId.borrow_mut()));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_xor(self.self_clone().unwrap(), w, out.clone(), desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn and(&self, w: WireType, desc: Vec<String>) -> WireType {
        return self.mulw(w, desc.clone());
    }

    fn getBitWiresi(&self, bitwidth: u64, desc: Vec<String>) -> WireArray {
        let mut bitWires = self.getBitWires();
        if let Some(bitWires) = bitWires {
            if bitwidth < bitWires.size() as u64 && !self.instance_of("ConstantWire"){
                println!(
                    "Warning: getBitWires() was called with different arguments on the same wire more than once"
                );
                println!(
                    "\t It was noted that the argument in the second call was less than the first."
                );
                println!(
                    "\t If self was called for enforcing a bitwidth constraint, you must use restrictBitLengh(), otherwise you can ignore self."
                );
                if Configs.get().unwrap().print_stack_trace_at_warnings {
                    println!("Thread.dumpStack();");
                } else {
                    println!(
                        "\t You can view the stack trace by setting Config.printStackTraceAtWarnings to true in the code."
                    );
                }
            }
            return bitWires.adjustLengthi(bitwidth as usize);
        }

        bitWires = Some(self.forceSplit(bitwidth as i32, desc.clone()));
        self.setBits(bitWires.clone());
        return bitWires.unwrap();
    }

    fn getBitWiresIfExistAlready(&self) -> WireArray {
        return self.getBitWires().unwrap();
    }

    fn forceSplit(&self, bitwidth: i32, desc: Vec<String>) -> WireArray {
        let mut ws = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            ws[i] = Some(WireType::VariableBit(VariableBitWire::new(
                *self.generator().currentWireId.borrow_mut(),
            )));
            *self.generator().currentWireId.borrow_mut() += 1;
        }
        let op = new_split(self.self_clone().unwrap(), ws.clone(), desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= bitwidth;
            return WireArray::new(cachedOutputs).adjustLengthi(bitwidth as usize);
        }
        WireArray::new(ws)
    }

    fn restrictBitLength(&self, bitWidth: u64, desc: Vec<String>) {
        let mut bitWires = self.getBitWires();
        if let Some(_bitWires) = bitWires {
            if _bitWires.size() > bitWidth as usize {
                bitWires = Some(self.forceSplit(bitWidth as i32, desc.clone()));
                self.setBits(bitWires);
            } else {
                // nothing to be done.
            }
            return;
        }
        self.getBitWiresi(bitWidth, desc.clone());
    }

    fn xorBitwise(&self, w: WireType, numBits: i32, desc: Vec<String>) -> WireType {
        let bits1 = self.getBitWiresi(numBits  as u64, desc.clone());
        let bits2 = w.getBitWiresi(numBits as u64, desc.clone());
        let result = bits1
            .xorWireArray(bits2, numBits as usize, desc.clone());
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn xorBitwisei(&self, v: i64, numBits: i32, desc: Vec<String>) -> WireType {
        return self.xorBitwise(
            self.generator().createConstantWirei(v, desc.clone()),
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
        let bits1 = self.getBitWiresi(numBits as u64, desc.clone());
        let bits2 = w.getBitWiresi(numBits  as u64, desc.clone());
        let result = bits1.andWireArray(bits2, numBits as usize, desc.clone());
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn andBitwisei(&self, v: i64, numBits: i32, desc: Vec<String>) -> WireType {
        return self.andBitwise(
            self.generator().createConstantWirei(v , desc.clone()),
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
        let bits1 = self.getBitWiresi(numBits  as u64, desc.clone());
        let bits2 = w.getBitWiresi(numBits  as u64, desc.clone());
        let result = bits1
            .orWireArray(bits2, numBits as usize, desc.clone());
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn orBitwisei(&self, v: i64, numBits: i32, desc: Vec<String>) -> WireType {
        return self.orBitwise(
            self.generator().createConstantWirei(v, desc.clone()),
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
        let s = self.subw(w, desc.clone());
        return s.checkNonZero(desc.clone()).invAsBit(desc.clone()).unwrap();
    }

    fn isEqualTob(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        return self.isEqualTo(self.generator().createConstantWire(b, desc.clone()), vec![]);
    }

    fn isEqualToi(&self, v: i64, desc: Vec<String>) -> WireType {
        return self.isEqualTo(self.generator().createConstantWirei(v, desc.clone()), vec![]);
    }

    fn isLessThanOrEqual(&self, w: WireType, bitwidth: i32, desc: Vec<String>) -> WireType {
        self.packIfNeeded(desc.clone());
        w.packIfNeeded(desc.clone());
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc.clone());
        let sum = pWire
            .addw(w, desc.clone())
            .subw(self.self_clone().unwrap(), desc.clone());
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc.clone());
        return bitWires.get(bitwidth as usize);
    }

    fn isLessThanOrEquali(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> WireType {
        return self.isLessThanOrEqual(
            self.generator().createConstantWirei(v, desc.clone()),
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
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc.clone());
        let sum = pWire
            .addw(self.self_clone().unwrap(), desc.clone())
            .subw(w, desc.clone());
        let bitWires = sum.getBitWiresi(bitwidth  as u64 + 1, desc.clone());
        return bitWires.get(bitwidth as usize).invAsBit(desc.clone()).unwrap();
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
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc.clone());
        let sum = pWire
            .addw(self.self_clone().unwrap(), desc.clone())
            .subw(w, desc.clone());
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc.clone());
        return bitWires.get(bitwidth as usize);
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
            .addw(w, desc.clone())
            .subw(self.self_clone().unwrap(), desc.clone());
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc.clone());
        return bitWires.get(bitwidth as usize).invAsBit(desc.clone()).unwrap();
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

    fn rotateLeft(&self, numBits: usize, s: usize, desc: Vec<String>) -> WireType {
        let bits = self.getBitWiresi(numBits as u64, desc.clone());
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i < s {
                rotatedBits[i] = Some(bits.get(i + (numBits - s)));
            } else {
                rotatedBits[i] = Some(bits.get(i - s));
            }
        }
        let result = WireArray::new(rotatedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn rotateRight(&self, numBits: usize, s: usize, desc: Vec<String>) -> WireType {
        let bits = self.getBitWiresi(numBits as u64, desc.clone());
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                rotatedBits[i] =Some (bits.get(i - (numBits - s)));
            } else {
                rotatedBits[i] = Some(bits.get(i + s));
            }
        }
        let result = WireArray::new(rotatedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn shl(&self, numBits: usize, s: usize, desc: Vec<String>) -> WireType {
        if s >= numBits {
            // Will always be zero in that case
            return self.generator().zeroWire.borrow().clone().unwrap();
        }

        let bits = self.getBitWiresi(numBits as u64, desc.clone());
        let mut shiftedBits = vec![None; numBits];
        for i in 0..numBits {
            if i < s {
                shiftedBits[i] = self.generator().zeroWire.borrow().clone();
            } else {
                shiftedBits[i] = Some(bits.get(i - s));
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn shiftRight(&self, numBits: usize, s: usize, desc: Vec<String>) -> WireType {
        if s >= numBits {
            // Will always be zero in that case
            return self.generator().zeroWire.borrow().clone().unwrap();
        }

        let bits = self.getBitWiresi(numBits as u64, desc.clone());
        let mut shiftedBits = vec![None; numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = self.generator().zeroWire.borrow().clone();
            } else {
                shiftedBits[i] = Some(bits.get(i + s));
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn shiftArithRight(&self, numBits: usize, s: usize, desc: Vec<String>) -> WireType {
        let bits = self.getBitWiresi(numBits as u64, desc.clone());
        let mut shiftedBits = vec![None; numBits];
        let sign = bits.get(numBits - 1);
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = Some(sign.clone());
            } else {
                shiftedBits[i] = Some(bits.get(i + s));
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn invBits(&self, bitwidth: u64, desc: Vec<String>) -> WireType {
        let bits = self.getBitWiresi(bitwidth, desc.clone()).asArray();
        let mut resultBits = vec![None; bits.len()];
        for i in 0..resultBits.len() {
            resultBits[i] = bits[i].as_ref().and_then(|x|x.clone().invAsBit(desc.clone()));
        }
        return WireType::LinearCombination(LinearCombinationWire::newa(WireArray::new(resultBits)));
    }

    fn trimBits(
        &self,
        currentNumOfBits: i32,
        desiredNumofBits: i32,
        desc: Vec<String>,
    ) -> WireType {
        let bitWires = self.getBitWiresi(currentNumOfBits as u64, desc.clone());
        let result = bitWires.adjustLengthi(desiredNumofBits as usize);
        let v = result.checkIfConstantBits(desc.clone());
        if let Some(v) = v {
            return self.generator().createConstantWire(v, vec![]);
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }

    fn packIfNeeded(&self, desc: Vec<String>) {
        if self.getWireId() == -1 {
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
        let mut wireId = *self.generator().currentWireId.borrow_mut();
        *self.generator().currentWireId.borrow_mut() += 1;
        //			Instruction op = PackBasicOp::new(bits.array, self, desc.clone());
        //			self.generator().addToEvaluationQueue(Box::new(op));

        let op = new_pack(bits.unwrap().array.clone(), self.self_clone().unwrap(), desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));

        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            wireId = cachedOutputs[0].as_ref().unwrap().getWireId();
        }
    }

    fn hashCode(&self) -> u64 {
        self.getWireId() as u64
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let w = rhs;
        w.getWireId() == self.getWireId() && w.generator() == self.generator()
    }
}
