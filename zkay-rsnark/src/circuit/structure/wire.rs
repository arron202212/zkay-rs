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
use crate::circuit::config::config::Configs;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::non_zero_check_basic_op::{
    NonZeroCheckBasicOp, new_non_zero_check,
};
use crate::circuit::operations::primitive::or_basic_op::{OrBasicOp, new_or};
use crate::circuit::operations::primitive::pack_basic_op::{PackBasicOp, new_pack};
use crate::circuit::operations::primitive::split_basic_op::{SplitBasicOp, new_split};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};

use crate::circuit::structure::circuit_generator::CreateConstantWire;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend, getActiveCircuitGenerator,
};
use crate::circuit::structure::linear_combination_wire::{
    LinearCombinationWire, new_linear_combination,
};
use crate::circuit::structure::variable_bit_wire::{VariableBitWire, new_variable_bit};
use crate::circuit::structure::variable_wire::{VariableWire, new_variable};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::ARcCell;
use crate::util::util::{BigInteger, Util};
use enum_dispatch::enum_dispatch;
use rccell::{RcCell, WeakCell};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use zkay_derive::ImplStructNameConfig;
#[enum_dispatch]
pub trait setBitsConfig {
    fn setBits(&self, bits: Option<WireArray>) {
        // method overriden in subclasses
        // default action:
        // println!(
        //     "Warning --  you are trying to set bits for either a constant or a bit wire. -- Action Ignored"
        // );
    }
}
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct Base;
impl setBitsConfig for Base {}
impl setBitsConfig for Wire<Base> {}
impl WireConfig for Wire<Base> {}
crate::impl_name_instance_of_wire_g_for!(Wire<Base>);

impl Hash for Wire<Base> {
    fn hash<H: Hasher>(&self, state: &mut H) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wire<T: setBitsConfig + Hash + Clone + Debug + PartialEq> {
    pub wireId: i32,
    pub generator: WeakCell<CircuitGenerator>,
    pub t: T,
}

impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> Wire<T> {
    pub fn new(wireId: i32, t: T, generator: WeakCell<CircuitGenerator>) -> eyre::Result<Self> {
        if wireId < 0 {
            eyre::bail!("wire id cannot be negative");
        }

        Ok(Self {
            wireId,
            generator,
            t,
        })
    }

    pub fn new_array(bits: WireArray, t: T, generator: WeakCell<CircuitGenerator>) -> Self {
        let mut _self = Self {
            wireId: -1,
            generator,
            t,
        };
        _self.t.setBits(Some(bits));
        _self
    }
}
#[enum_dispatch]
pub trait GeneratorConfig {
    fn generator(&self) -> RcCell<CircuitGenerator>;
}
impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> GeneratorConfig for Wire<T> {
    fn generator(&self) -> RcCell<CircuitGenerator> {
        self.generator.clone().upgrade().unwrap()
    }
}

// impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> setBitsConfig for Wire<T> {}
// impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> WireConfig for Wire<T> {}
#[enum_dispatch]
pub trait WireConfig: PartialEq + setBitsConfig + InstanceOf + GetWireId + GeneratorConfig {
    // fn instance_of(&self, name: &str) -> bool {
    //     self.name() == name
    // }
    // fn name(&self) -> &str {
    //     ""
    // }

    // fn toString(&self) -> String {
    //     self.getWireId().to_string()
    // }

    fn getBitWires(&self) -> Option<WireArray> {
        None
    }
    fn self_clone(&self) -> Option<WireType> {
        None
    }

    fn mulb(&self, b: BigInteger, desc: &Option<String>) -> WireType {
        println!("End Name Time: 33333333333 {} s", line!());
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc);
        println!("End Name Time: 444 {} s", line!());
        if b == Util::one() {
            return self.self_clone().unwrap();
        }
        if b == BigInteger::ZERO {
            return generator.get_zero_wire().unwrap();
        }
        println!("End Name Time: 444 {} s", line!());
        let out = WireType::LinearCombination(new_linear_combination(
            generator.get_current_wire_id(),
            None,
            self.generator().clone().downgrade(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = new_const_mul(
            self.self_clone().unwrap(),
            out.clone(),
            b,
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        //		generator.addToEvaluationQueue(Box::new(op));
        let g = generator.borrow().clone();
        let cachedOutputs = g.addToEvaluationQueue(Box::new(op));
        println!("End Name Time: 444 {} s", line!());
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            cachedOutputs[0].clone().unwrap()
        } else {
            out
        }
    }

    fn muli(&self, l: i64, desc: &Option<String>) -> WireType {
        println!("End Name Time: 444 {} s", line!());
        self.mulb(BigInteger::from(l), desc)
    }

    fn mulii(&self, base: i64, exp: i32, desc: &Option<String>) -> WireType {
        let mut b = BigInteger::from(base);
        b = b.pow(exp as u32);
        self.mulb(b, desc)
    }

    fn mulw(&self, w: WireType, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        if w.instance_of("ConstantWire") {
            return self.mulb(w.try_as_constant_ref().unwrap().getConstant(), desc);
        }
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let output = WireType::Variable(new_variable(
            generator.get_current_wire_id(),
            self.generator().clone().downgrade(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = new_mul(
            self.self_clone().unwrap(),
            w,
            output.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let g = generator.borrow().clone();
        let cachedOutputs = g.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        output
    }

    fn addw(&self, w: WireType, desc: &Option<String>) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        return WireArray::new(
            vec![Some(self.self_clone().unwrap()), Some(w)],
            self.generator().clone().downgrade(),
        )
        .sumAllElements(desc);
    }

    fn addi(&self, v: i64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.addw(generator.create_constant_wire(v, desc), desc)
    }

    fn addb(&self, b: BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.addw(generator.create_constant_wire(b, desc), desc)
    }

    fn subw(&self, w: WireType, desc: &Option<String>) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let neg = w.muli(-1, desc);
        self.addw(neg, desc)
    }

    fn subi(&self, v: i64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.subw(generator.create_constant_wire(v, desc), desc)
    }

    fn subb(&self, b: BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.subw(generator.create_constant_wire(b, desc), desc)
    }

    fn negate(&self, desc: &Option<String>) -> WireType {
        return self
            .generator()
            .get_zero_wire()
            .unwrap()
            .subw(self.self_clone().unwrap(), desc);
    }

    fn mux(&self, trueValue: WireType, falseValue: WireType) -> WireType {
        return falseValue.clone().addw(
            self.self_clone()
                .unwrap()
                .mulw(trueValue.subw(falseValue, &None), &None),
            &None,
        );
    }

    fn checkNonZero(&self, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc);

        //  * self wire is not currently used for anything - It's for compatibility
        //  * with earlier experimental versions when the target was Pinocchio

        let out1 = WireType::Wire(
            Wire::<Base>::new(
                generator.get_current_wire_id(),
                Base,
                generator.clone().downgrade(),
            )
            .unwrap(),
        );
        generator.borrow_mut().current_wire_id += 1;
        let out2 = WireType::VariableBit(new_variable_bit(
            generator.get_current_wire_id(),
            self.generator().clone().downgrade(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = new_non_zero_check(
            self.self_clone().unwrap(),
            out1,
            out2.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let g = generator.borrow().clone();
        let cachedOutputs = g.addToEvaluationQueue(Box::new(op));

        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 2;
            return cachedOutputs[1].clone().unwrap();
        }
        out2
    }

    fn invAsBit(&self, desc: &Option<String>) -> Option<WireType> {
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc); // just a precaution .. should not be really needed
        let w1 = self.muli(-1, desc);
        let s = generator.get_one_wire();
        // println!("====invAsBit==========={}===={s:?}", generator.get_name());
        let out = generator.get_one_wire().unwrap().addw(w1, desc);
        Some(out)
    }

    fn orw(&self, w: WireType, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        if w.instance_of("ConstantWire") {
            return w.orw(self.self_clone().unwrap(), desc);
        }
        self.packIfNeeded(desc); // just a precaution .. should not be really
        // needed
        let out = WireType::Variable(new_variable(
            generator.get_current_wire_id(),
            self.generator().clone().downgrade(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = new_or(
            self.self_clone().unwrap(),
            w,
            out.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let g = generator.borrow().clone();
        let cachedOutputs = g.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn xorw(&self, w: WireType, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        if w.instance_of("ConstantWire") {
            return w.xorw(self.self_clone().unwrap(), desc);
        }
        self.packIfNeeded(desc); // just a precaution .. should not be really
        // needed
        let out = WireType::Variable(new_variable(
            generator.get_current_wire_id(),
            self.generator().clone().downgrade(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = new_xor(
            self.self_clone().unwrap(),
            w,
            out.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let g = generator.borrow().clone();
        let cachedOutputs = g.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn and(&self, w: WireType, desc: &Option<String>) -> WireType {
        self.mulw(w, desc)
    }

    fn getBitWiresi(&self, bitwidth: u64, desc: &Option<String>) -> WireArray {
        //println!("======================{},{}",file!(),line!());
        let mut bitWires = self.getBitWires();
        //println!("======================{},{}",file!(),line!());
        if let Some(bitWires) = bitWires {
            if bitwidth < bitWires.size() as u64 && !self.instance_of("ConstantWire") {
                println!(
                    "Warning: getBitWires() was called with different arguments on the same wire more than once"
                );
                println!(
                    "\t It was noted that the argument in the second call was less than the first."
                );
                println!(
                    "\t If self was called for enforcing a bitwidth constraint, you must use restrictBitLengh(), otherwise you can ignore self."
                );
                if Configs.print_stack_trace_at_warnings {
                    //println!("Thread.dumpStack();");
                } else {
                    println!(
                        "\t You can view the stack trace by setting Configs.printStackTraceAtWarnings to true in the code."
                    );
                }
            }
            //println!("======================{},{}",file!(),line!());
            return bitWires.adjustLength(None, bitwidth as usize);
        }
        //println!("======================{},{}",file!(),line!());
        bitWires = Some(self.forceSplit(bitwidth as i32, desc));
        //println!("======================{},{}",file!(),line!());
        self.setBits(bitWires.clone());
        //println!("======================{},{}",file!(),line!());
        bitWires.unwrap()
    }

    fn getBitWiresIfExistAlready(&self) -> Option<WireArray> {
        self.getBitWires()
    }

    fn forceSplit(&self, bitwidth: i32, desc: &Option<String>) -> WireArray {
        let mut generator = self.generator().clone();

        //println!("====forceSplit==========={bitwidth}======={},{}",file!(),line!());
        let mut ws = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            //println!("======================{},{}",file!(),line!());

            ws[i] = Some(WireType::VariableBit(new_variable_bit(
                generator.get_current_wire_id(),
                self.generator().clone().downgrade(),
            )));
            //println!("======================{},{}",file!(),line!());

            generator.borrow_mut().current_wire_id += 1;
        }
        //println!("======================{},{}",file!(),line!());
        let op = new_split(
            self.self_clone().unwrap(),
            ws.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        //println!("======================{},{}",file!(),line!());
        let g = generator.borrow().clone();
        let cachedOutputs = g.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= bitwidth;
            //println!("======================{},{}",file!(),line!());
            return WireArray::new(cachedOutputs, generator.clone().downgrade())
                .adjustLength(None, bitwidth as usize);
        }
        WireArray::new(ws, generator.clone().downgrade())
    }

    fn restrictBitLength(&self, bitWidth: u64, desc: &Option<String>) {
        let mut bitWires = self.getBitWires();
        if let Some(_bitWires) = bitWires {
            if _bitWires.size() > bitWidth as usize {
                bitWires = Some(self.forceSplit(bitWidth as i32, desc));
                self.setBits(bitWires);
            } else {
                // nothing to be done.
            }
            return;
        }
        self.getBitWiresi(bitWidth, desc);
    }

    fn xorBitwise(&self, w: WireType, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.xorWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn xorBitwisei(&self, v: i64, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.xorBitwise(generator.create_constant_wire(v, desc), numBits, desc);
    }

    fn xorBitwiseb(&self, b: BigInteger, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.xorBitwise(generator.create_constant_wire(b, desc), numBits, desc);
    }

    fn andBitwise(&self, w: WireType, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.andWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn andBitwisei(&self, v: i64, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.andBitwise(generator.create_constant_wire(v, desc), numBits, desc);
    }

    fn andBitwiseb(&self, b: BigInteger, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.andBitwise(generator.create_constant_wire(b, desc), numBits, desc);
    }

    fn orBitwise(&self, w: WireType, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.orWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn orBitwisei(&self, v: i64, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.orBitwise(generator.create_constant_wire(v, desc), numBits, desc);
    }

    fn orBitwiseb(&self, b: BigInteger, numBits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.orBitwise(generator.create_constant_wire(b, desc), numBits, desc);
    }

    fn isEqualTo(&self, w: WireType, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let s = self.subw(w, desc);
        s.checkNonZero(desc).invAsBit(desc).unwrap()
    }

    fn isEqualTob(&self, b: BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.isEqualTo(generator.create_constant_wire(b, desc), &None)
    }

    fn isEqualToi(&self, v: i64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.isEqualTo(generator.create_constant_wire(v, desc), &None)
    }

    fn isLessThanOrEqual(&self, w: WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(p, desc);
        let sum = pWire.addw(w, desc).subw(self.self_clone().unwrap(), desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        bitWires[bitwidth as usize].clone().unwrap()
    }

    fn isLessThanOrEquali(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.isLessThanOrEqual(generator.create_constant_wire(v, desc), bitwidth, desc);
    }

    fn isLessThanOrEqualb(&self, b: BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.isLessThanOrEqual(generator.create_constant_wire(b, desc), bitwidth, desc);
    }

    fn isLessThan(&self, w: WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(p, desc);
        let sum = pWire.addw(self.self_clone().unwrap(), desc).subw(w, desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires[bitwidth as usize]
            .clone()
            .unwrap()
            .invAsBit(desc)
            .unwrap();
    }

    fn isLessThani(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.isLessThan(generator.create_constant_wire(v, desc), bitwidth, desc);
    }

    fn isLessThanb(&self, b: BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.isLessThan(generator.create_constant_wire(b, desc), bitwidth, desc);
    }

    fn isGreaterThanOrEqual(&self, w: WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(p, desc);
        let sum = pWire.addw(self.self_clone().unwrap(), desc).subw(w, desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        bitWires[bitwidth as usize].clone().unwrap()
    }

    fn isGreaterThanOrEquali(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.isGreaterThanOrEqual(generator.create_constant_wire(v, desc), bitwidth, desc);
    }

    fn isGreaterThanOrEqualb(
        &self,
        b: BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator().clone();

        return self.isGreaterThanOrEqual(generator.create_constant_wire(b, desc), bitwidth, desc);
    }

    fn isGreaterThan(&self, w: WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(p, desc);
        let sum = pWire.addw(w, desc).subw(self.self_clone().unwrap(), desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires[bitwidth as usize]
            .clone()
            .unwrap()
            .invAsBit(desc)
            .unwrap();
    }

    fn isGreaterThani(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.isGreaterThan(generator.create_constant_wire(v, desc), bitwidth, desc);
    }

    fn isGreaterThanb(&self, b: BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        return self.isGreaterThan(generator.create_constant_wire(b, desc), bitwidth, desc);
    }

    fn rotateLeft(&self, numBits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        let bits = self.getBitWiresi(numBits as u64, desc);
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i < s {
                rotatedBits[i] = bits[i + (numBits - s)].clone();
            } else {
                rotatedBits[i] = bits[i - s].clone();
            }
        }
        let result = WireArray::new(rotatedBits, generator.clone().downgrade());
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn rotateRight(&self, numBits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        let bits = self.getBitWiresi(numBits as u64, desc);
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                rotatedBits[i] = bits[i - (numBits - s)].clone();
            } else {
                rotatedBits[i] = bits[i + s].clone();
            }
        }
        let result = WireArray::new(rotatedBits, generator.clone().downgrade());
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn shiftLeft(&self, numBits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        if s >= numBits {
            // Will always be zero in that case
            return generator.get_zero_wire().unwrap();
        }

        let bits = self.getBitWiresi(numBits as u64, desc);
        let mut shiftedBits = vec![None; numBits];
        for i in 0..numBits {
            if i < s {
                shiftedBits[i] = generator.get_zero_wire();
            } else {
                shiftedBits[i] = bits[i - s].clone();
            }
        }
        let result = WireArray::new(shiftedBits, generator.clone().downgrade());
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn shiftRight(&self, numBits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        //println!("======================{},{}",file!(),line!());
        if s >= numBits {
            //println!("======================{},{}",file!(),line!());
            // Will always be zero in that case
            return generator.get_zero_wire().unwrap();
        }
        //println!("======================{},{}",file!(),line!());
        let bits = self.getBitWiresi(numBits as u64, desc);
        let mut shiftedBits = vec![None; numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = generator.get_zero_wire();
            } else {
                shiftedBits[i] = bits[i + s].clone();
            }
        }
        //println!("======================{},{}",file!(),line!());
        let result = WireArray::new(shiftedBits, generator.clone().downgrade());
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            //println!("======================{},{}",file!(),line!());
            return generator.create_constant_wire(v, &None);
        }
        //println!("======================{},{}",file!(),line!());
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn shiftArithRight(&self, numBits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        let bits = self.getBitWiresi(numBits as u64, desc);
        let mut shiftedBits = vec![None; numBits];
        let sign = &bits[numBits - 1];
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = sign.clone();
            } else {
                shiftedBits[i] = bits[i + s].clone();
            }
        }
        let result = WireArray::new(shiftedBits, generator.clone().downgrade());
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn invBits(&self, bitwidth: u64, desc: &Option<String>) -> WireType {
        let bits = self.getBitWiresi(bitwidth, desc).asArray();
        let mut resultBits = vec![None; bits.len()];
        for i in 0..resultBits.len() {
            resultBits[i] = bits[i].as_ref().and_then(|x| x.clone().invAsBit(desc));
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(WireArray::new(
                resultBits,
                self.generator().clone().downgrade(),
            )),
            self.generator().clone().downgrade(),
        ))
    }

    fn trimBits(
        &self,
        currentNumOfBits: i32,
        desiredNumofBits: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator().clone();

        let bitWires = self.getBitWiresi(currentNumOfBits as u64, desc);
        let result = bitWires.adjustLength(None, desiredNumofBits as usize);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(v, &None);
        }
        WireType::LinearCombination(new_linear_combination(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn packIfNeeded(&self, desc: &Option<String>) {
        if self.getWireId() == -1 {
            self.pack(&None);
        }
    }

    fn pack(&self, desc: &Option<String>) {
        if self.getWireId() != -1 {
            return;
        }
        let mut generator = self.generator().clone();

        let bits = self.getBitWires();
        assert!(
            bits.is_some(),
            "A Pack operation is tried on a wire that has no bits."
        );
        let mut wireId = generator.borrow_mut().current_wire_id;
        generator.borrow_mut().current_wire_id += 1;
        //			Instruction op = PackBasicOp::new(bits.array, self, desc);
        //			generator.addToEvaluationQueue(Box::new(op));

        let op = new_pack(
            bits.unwrap().array.clone(),
            self.self_clone().unwrap(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let g = generator.borrow().clone();
        let cachedOutputs = g.addToEvaluationQueue(Box::new(op));

        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            wireId = cachedOutputs[0].as_ref().unwrap().getWireId();
        }
    }

    // fn hashCode(&self) -> u64 {
    //     self.getWireId() as u64
    // }

    // fn equals(&self, rhs: &Self) -> bool {
    //     if self == rhs {
    //         return true;
    //     }

    //     let w = rhs;
    //     w.getWireId() == self.getWireId() && w.generator() == self.generator().lock()
    // }
}

#[macro_export]
macro_rules! impl_hash_code_of_wire_for {
    ($impl_type:ty) => {
        impl std::hash::Hash for $impl_type {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.getWireId().hash(state);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display_of_wire_for {
    ($impl_type:ty) => {
        impl std::fmt::Display for $impl_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.getWireId())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_hash_code_of_wire_g_for {
    ($impl_type:ty) => {
        impl std::hash::Hash for $impl_type {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.getWireId().hash(state);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_name_instance_of_wire_g_for {
    ($impl_type:ty) => {
        impl crate::circuit::StructNameConfig for $impl_type {
            fn name(&self) -> String {
                self.t.name()
            }
        }
        impl crate::circuit::InstanceOf for $impl_type {
            fn instance_of(&self, name: &str) -> bool {
                self.t.instance_of(name)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_name_instance_of_wire_for {
    ($impl_type:ty) => {
        impl crate::circuit::StructNameConfig for $impl_type {
            fn name(&self) -> String {
                self.t.name()
            }
        }
        impl crate::circuit::InstanceOf for $impl_type {
            fn instance_of(&self, name: &str) -> bool {
                self.t.instance_of(name)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_eq_of_wire_for {
    ($impl_type:ty) => {
        impl Eq for $impl_type {}
        impl PartialEq for $impl_type {
            fn eq(&self, other: &Self) -> bool {
                other.getWireId() == self.getWireId()
                    && other.generator() == self.generator().lock()
            }
        }
    };
}

#[enum_dispatch]
pub trait GetWireId {
    fn getWireId(&self) -> i32;
}

impl<T: setBitsConfig + Hash + Clone + Debug + PartialEq> GetWireId for Wire<T> {
    fn getWireId(&self) -> i32 {
        self.wireId
    }
}

// #[macro_export]
// macro_rules! impl_get_wire_id_of_wire_for {
//     ($impl_type:ty) => {
// impl GetWireId for $impl_type {
//      fn getWireId(&self) -> i32 {
//         self.wireId
//     }
// }
//     };
// }

#[macro_export]
macro_rules! new_wire {
    ($impl_type:ty,$wire_id:expr,$generator:ident) => {
        Wire::<$impl_type> {
            wireId: $wire_id,
            generator,
            t: $impl_type,
        }
    };
}
