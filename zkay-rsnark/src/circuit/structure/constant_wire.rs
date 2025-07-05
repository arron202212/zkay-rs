#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        InstanceOf,
        config::config::Configs,
        eval::instruction::Instruction,
        operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul},
        structure::{
            circuit_generator::CreateConstantWire,
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                addToEvaluationQueue, getActiveCircuitGenerator,
            },
            wire::GeneratorConfig,
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};

use num_bigint::Sign;
use rccell::{RcCell, WeakCell};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Neg, Rem, Sub},
    time::Instant,
};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct ConstantWire {
    pub constant: BigInteger,
}
crate::impl_hash_code_of_wire_g_for!(Wire<ConstantWire>);
crate::impl_name_instance_of_wire_g_for!(Wire<ConstantWire>);
pub fn new_constant(
    wireId: i32,
    value: BigInteger,
    generator: WeakCell<CircuitGenerator>,
) -> Wire<ConstantWire> {
    // super(wireId);
    // Wire::<ConstantWire> {
    //     wireId,
    //     generator,
    //     t: ConstantWire {
    //         constant: value.rem(&Configs.field_prime),
    //     },
    // }
    Wire::<ConstantWire>::new(
        ConstantWire {
            constant: value.rem(&Configs.field_prime),
        },
        wireId,
        generator,
    )
    .unwrap()
}

impl setBitsConfig for ConstantWire {}
impl setBitsConfig for Wire<ConstantWire> {}
impl Wire<ConstantWire> {
    // pub fn new(wireId: i32, value: BigInteger) -> Self {
    //     // super(wireId);
    //     Self {
    //         constant: value.rem(Configs.field_prime.clone()),
    //     }
    // }
    pub fn getConstant(&self) -> BigInteger {
        self.t.constant.clone()
    }

    pub fn isBinary(&self) -> bool {
        self.t.constant == Util::one() || self.t.constant == BigInteger::ZERO
    }
}
impl WireConfig for Wire<ConstantWire> {
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::Constant(self.clone()))
    }
    fn mulw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let start = Instant::now();
        let generator = self.generator();
        //  println!("End const mulw Time: == {} s", start.elapsed().as_secs());
        if w.instance_of("ConstantWire") {
            generator.create_constant_wire(
                &self
                    .t
                    .constant
                    .clone()
                    .mul(&w.try_as_constant_ref().unwrap().getConstant()),
                desc,
            )
        } else {
            w.mulb(&self.t.constant, desc)
        }
    }

    fn mulb(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        // println!("End Name Time: ccccccc {} s", line!());
        let sign = b.sign() == Sign::Minus;
        let newConstant = self.t.constant.clone().mul(b).rem(&Configs.field_prime);
        //println!"End Name Time: ccccccc {} s", line!());
        let mut generator = self.generator();
        //println!"End Name Time: ccccccc {} s", line!());

        //println!"End Name Time: ccccccc {} s", line!());
        let mut out: Option<WireType> = generator
            .get_known_constant_wires()
            .get(&newConstant)
            .cloned();
        if let Some(out) = out {
            return out.clone();
        }
        //println!"End Name Time: ccccccc {} s", line!());
        out = Some(WireType::Constant(new_constant(
            generator.get_current_wire_id(),
            if !sign {
                newConstant.clone()
            } else {
                newConstant.clone().sub(Configs.field_prime.clone())
            },
            self.generator.clone(),
        )));
        //println!"End Name Time: ccccccc {} s", line!());
        generator.borrow_mut().current_wire_id += 1;
        let op = new_const_mul(
            &WireType::Constant(self.clone()),
            out.as_ref().unwrap(),
            b,
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        //println!"End Name Time: ccccccc {} s", line!());
        let g = generator.borrow().clone();
        let cachedOutputs = addToEvaluationQueue(generator.clone(), Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            // self branch might not be needed
            generator.borrow_mut().current_wire_id -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        //println!"End Name Time: ccccccc {} s", line!());
        generator
            .borrow_mut()
            .known_constant_wires
            .insert(newConstant, out.clone().unwrap());
        out.clone().unwrap()
    }

    fn checkNonZero(&self, desc: &Option<String>) -> WireType {
        let generator = self.generator();

        if self.t.constant == BigInteger::ZERO {
            generator.get_zero_wire().unwrap()
        } else {
            generator.get_one_wire().unwrap()
        }
    }

    fn invAsBit(&self, desc: &Option<String>) -> Option<WireType> {
        assert!(self.isBinary(), "Trying to invert a non-binary constant!");

        let generator = self.generator();

        if self.t.constant == BigInteger::ZERO {
            generator.get_one_wire()
        } else {
            generator.get_zero_wire()
        }
    }

    fn orw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let generator = self.generator();

        if w.instance_of("ConstantWire") {
            let cw = w;
            assert!(
                self.isBinary() && cw.try_as_constant_ref().unwrap().isBinary(),
                "Trying to OR two non-binary constants"
            );
            return if self.t.constant == BigInteger::ZERO
                && cw.try_as_constant_ref().unwrap().getConstant() == BigInteger::ZERO
            {
                generator.get_zero_wire().unwrap()
            } else {
                generator.get_one_wire().unwrap()
            };
        }
        if self.t.constant == Util::one() {
            generator.get_one_wire().unwrap()
        } else {
            w.clone()
        }
    }

    fn xorw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let generator = self.generator();

        if w.instance_of("ConstantWire") {
            let cw = w;
            assert!(
                self.isBinary() && cw.try_as_constant_ref().unwrap().isBinary(),
                "Trying to XOR two non-binary constants"
            );
            return if self.t.constant == cw.try_as_constant_ref().unwrap().getConstant() {
                generator.get_zero_wire().unwrap()
            } else {
                generator.get_one_wire().unwrap()
            };
        }
        if self.t.constant == Util::one() {
            w.invAsBit(desc).unwrap()
        } else {
            w.clone()
        }
    }

    fn getBitWiresi(&self, bitwidth: u64, desc: &Option<String>) -> WireArray {
        assert!(
            self.t.constant.bits() <= bitwidth,
            "Trying to split a constant of {} bits into  {bitwidth} bits",
            self.t.constant.bits()
        );
        let generator = self.generator();

        let mut bits = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            bits[i] = if self.t.constant.bit(i as u64) {
                generator.get_one_wire()
            } else {
                generator.get_zero_wire()
            };
        }
        WireArray::new(bits, self.generator.clone())
    }

    fn restrictBitLength(&self, bitwidth: u64, desc: &Option<String>) {
        self.getBitWiresi(bitwidth, desc);
    }

    fn pack(&self, desc: &Option<String>) {}
}
