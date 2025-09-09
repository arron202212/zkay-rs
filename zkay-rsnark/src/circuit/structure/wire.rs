#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        InstanceOf, StructNameConfig,
        config::config::CONFIGS,
        eval::instruction::Instruction,
        operations::primitive::{
            const_mul_basic_op::ConstMulBasicOp, mul_basic_op::MulBasicOp,
            non_zero_check_basic_op::NonZeroCheckBasicOp, or_basic_op::OrBasicOp,
            pack_basic_op::PackBasicOp, split_basic_op::SplitBasicOp, xor_basic_op::XorBasicOp,
        },
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                CreateConstantWire, add_to_evaluation_queue, get_active_circuit_generator,
            },
            linear_combination_wire::LinearCombination_wire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{ARcCell, BigInteger, Util},
};
use enum_dispatch::enum_dispatch;
use rccell::{RcCell, WeakCell};
use std::{
    fmt::{self, Debug},
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
use zkay_derive::ImplStructNameConfig;
#[enum_dispatch]
pub trait SetBitsConfig {
    fn set_bits(&self, bits: Option<WireArray>) {
        // method overriden in subclasses
        // default action:
        // println!(
        //     "Warning --  you are trying to set bits for either a constant or a bit wire. -- Action Ignored"
        // );
    }
}
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct Base;
impl SetBitsConfig for Base {}
impl SetBitsConfig for Wire<Base> {}
impl WireConfig for Wire<Base> {}
crate::impl_name_instance_of_wire_g_for!(Wire<Base>);

impl<T: SetBitsConfig + Clone + Debug + PartialEq> Hash for Wire<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let id = *self.wire_id.borrow();
        id.hash(state);
    }
}
impl<T: SetBitsConfig + Clone + Debug + PartialEq> std::fmt::Display for Wire<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.wire_id.borrow())
    }
}
impl<T: SetBitsConfig + Clone + Debug + PartialEq> Eq for Wire<T> {}
impl<T: SetBitsConfig + Clone + Debug + PartialEq> PartialEq for Wire<T> {
    fn eq(&self, other: &Self) -> bool {
        other.wire_id == self.wire_id && other.generator == self.generator
    }
}
impl<T: SetBitsConfig + Clone + Debug + PartialEq> Clone for Wire<T> {
    fn clone(&self) -> Self {
        Self {
            wire_id: self.wire_id.clone(), //RcCell::new(*self.wire_id.borrow()),//
            generator: self.generator.clone(),
            t: self.t.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Wire<T: SetBitsConfig + Clone + Debug> {
    pub wire_id: RcCell<i32>,
    pub generator: WeakCell<CircuitGenerator>,
    pub t: T,
}

impl<T: SetBitsConfig + Clone + Debug + PartialEq> Wire<T> {
    pub fn new(t: T, wire_id: i32, generator: WeakCell<CircuitGenerator>) -> eyre::Result<Self> {
        Ok(Self {
            wire_id: RcCell::new(wire_id),
            generator,
            t,
        })
    }

    pub fn new_array(bits: WireArray, t: T, generator: WeakCell<CircuitGenerator>) -> Self {
        let mut _self = Self {
            wire_id: RcCell::new(-1),
            generator,
            t,
        };
        _self.t.set_bits(Some(bits));
        _self
    }
}
#[enum_dispatch]
pub trait GeneratorConfig {
    fn generator(&self) -> RcCell<CircuitGenerator>;
    fn generator_weak(&self) -> WeakCell<CircuitGenerator>;
}
impl<T: SetBitsConfig + Clone + Debug + PartialEq> GeneratorConfig for Wire<T> {
    fn generator(&self) -> RcCell<CircuitGenerator> {
        self.generator.clone().upgrade().unwrap()
    }
    fn generator_weak(&self) -> WeakCell<CircuitGenerator> {
        self.generator.clone()
    }
}

#[enum_dispatch]
pub trait WireConfig: PartialEq + SetBitsConfig + InstanceOf + GetWireId + GeneratorConfig {
    fn get_bit_wires(&self) -> Option<WireArray> {
        None
    }
    fn self_clone(&self) -> Option<WireType> {
        None
    }

    #[inline]
    fn mulb(&self, b: &BigInteger) -> WireType {
        self.mulb_with_option(b, &None)
    }

    #[inline]
    fn mulb_with_str(&self, b: &BigInteger, desc: &str) -> WireType {
        self.mulb_with_option(b, &Some(desc.to_owned()))
    }
    fn mulb_with_option(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed_with_option(desc);
        if b == &Util::one() {
            return self.self_clone().unwrap();
        }
        if b == &BigInteger::ZERO {
            return generator.get_zero_wire().unwrap();
        }

        let out = WireType::LinearCombination(LinearCombination_wire::new(
            generator.get_current_wire_id(),
            None,
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = ConstMulBasicOp::new(
            self.self_clone().as_ref().unwrap(),
            &out,
            b,
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;

            cached_outputs[0].clone().unwrap()
        } else {
            out
        }
    }

    #[inline]
    fn muli(&self, l: i64) -> WireType {
        self.muli_with_option(l, &None)
    }
    fn muli_with_option(&self, l: i64, desc: &Option<String>) -> WireType {
        self.mulb_with_option(&BigInteger::from(l), desc)
    }

    fn mulii(&self, base: i64, exp: i32, desc: &Option<String>) -> WireType {
        let mut b = BigInteger::from(base);
        b = b.pow(exp as u32);
        self.mulb_with_option(&b, desc)
    }

    #[inline]
    fn mulw(&self, w: &WireType) -> WireType {
        self.mulw_with_option(w, &None)
    }

    #[inline]
    fn mulw_with_str(&self, w: &WireType, desc: &str) -> WireType {
        self.mulw_with_option(w, &Some(desc.to_owned()))
    }
    fn mulw_with_option(&self, w: &WireType, desc: &Option<String>) -> WireType {
        use std::time::Instant;
        let start = Instant::now();
        let mut generator = self.generator();
        if w.instance_of("ConstantWire") {
            let v = self.mulb_with_option(&w.try_as_constant_ref().unwrap().get_constant(), desc);
            return v;
        }

        self.pack_if_needed_with_option(desc);

        w.pack_if_needed_with_option(desc);

        let output = WireType::Variable(VariableWire::new(
            generator.get_current_wire_id(),
            self.generator_weak(),
        ));

        generator.borrow_mut().current_wire_id += 1;
        let op = MulBasicOp::new(
            &self.self_clone().unwrap(),
            w,
            &output,
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));

        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            cached_outputs[0].clone().unwrap()
        } else {
            output
        }
    }
    #[inline]
    fn addw(&self, w: &WireType) -> WireType {
        self.addw_with_option(w, &None)
    }
    #[inline]
    fn addw_with_str(&self, w: &WireType, desc: &str) -> WireType {
        self.addw_with_option(w, &Some(desc.to_owned()))
    }
    fn addw_with_option(&self, w: &WireType, desc: &Option<String>) -> WireType {
        self.pack_if_needed_with_option(desc);
        w.pack_if_needed_with_option(desc);
        WireArray::new(
            vec![Some(self.self_clone().unwrap()), Some(w.clone())],
            self.generator_weak(),
        )
        .sum_all_elements(desc)
    }

    fn addi(&self, v: i64, desc: &Option<String>) -> WireType {
        self.addw_with_option(
            &self.generator().create_constant_wire_with_option(v, desc),
            desc,
        )
    }

    fn addb(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        self.addw_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            desc,
        )
    }

    fn subw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        self.pack_if_needed_with_option(desc);
        w.pack_if_needed_with_option(desc);
        let neg = w.muli_with_option(-1, desc);
        self.addw_with_option(&neg, desc)
    }

    fn subi(&self, v: i64, desc: &Option<String>) -> WireType {
        self.subw(
            &self.generator().create_constant_wire_with_option(v, desc),
            desc,
        )
    }

    fn subb(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        self.subw(
            &self.generator().create_constant_wire_with_option(b, desc),
            desc,
        )
    }
    #[inline]
    fn negate(&self) -> WireType {
        self.negate_with_option(&None)
    }
    fn negate_with_option(&self, desc: &Option<String>) -> WireType {
        self.generator()
            .get_zero_wire()
            .unwrap()
            .subw(&self.self_clone().unwrap(), desc)
    }

    fn mux(&self, true_value: &WireType, false_value: &WireType) -> WireType {
        false_value.addw(
            &self
                .self_clone()
                .unwrap()
                .mulw(&true_value.subw(false_value, &None)),
        )
    }
    #[inline]
    fn check_non_zero(&self) -> WireType {
        self.check_non_zero_with_option(&None)
    }
    fn check_non_zero_with_option(&self, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed_with_option(desc);

        //  * self wire is not currently used for anything - It's for compatibility
        //  * with earlier experimental versions when the target was Pinocchio

        let out1 = WireType::Wire(
            Wire::<Base>::new(
                Base,
                generator.get_current_wire_id(),
                generator.clone().downgrade(),
            )
            .unwrap(),
        );
        generator.borrow_mut().current_wire_id += 1;
        let out2 = WireType::VariableBit(VariableBitWire::new(
            generator.get_current_wire_id(),
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = NonZeroCheckBasicOp::new(
            &self.self_clone().unwrap(),
            &out1,
            &out2,
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));

        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 2;
            cached_outputs[1].clone().unwrap()
        } else {
            out2
        }
    }
    #[inline]
    fn inv_as_bit(&self) -> Option<WireType> {
        self.inv_as_bit_with_option(&None)
    }
    fn inv_as_bit_with_option(&self, desc: &Option<String>) -> Option<WireType> {
        let mut generator = self.generator();

        self.pack_if_needed_with_option(desc); // just a precaution .. should not be really needed
        let w1 = self.muli_with_option(-1, desc);
        let s = generator.get_one_wire();
        let out = generator
            .get_one_wire()
            .unwrap()
            .addw_with_option(&w1, desc);
        Some(out)
    }

    #[inline]
    fn orw(&self, w: &WireType) -> WireType {
        self.orw_with_option(w, &None)
    }

    fn orw_with_option(&self, w: &WireType, desc: &Option<String>) -> WireType {
        use std::time::Instant;
        let start = Instant::now();

        let mut generator = self.generator();

        if w.instance_of("ConstantWire") {
            return w.orw_with_option(self.self_clone().as_ref().unwrap(), desc);
        }
        self.pack_if_needed_with_option(desc); // just a precaution .. should not be really

        // needed
        let out = WireType::Variable(VariableWire::new(
            generator.get_current_wire_id(),
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let sc = self.self_clone().unwrap();
        let desc = desc.clone().unwrap_or(String::new());

        let op = OrBasicOp::new(&sc, w, &out, desc);

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));

        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            cached_outputs[0].clone().unwrap()
        } else {
            out
        }
    }

    fn xorw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        if w.instance_of("ConstantWire") {
            return w.xorw(&self.self_clone().unwrap(), desc);
        }
        self.pack_if_needed_with_option(desc); // just a precaution .. should not be really
        // needed
        let out = WireType::Variable(VariableWire::new(
            generator.get_current_wire_id(),
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = XorBasicOp::new(
            &self.self_clone().unwrap(),
            w,
            &out,
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            cached_outputs[0].clone().unwrap()
        } else {
            out
        }
    }
    #[inline]
    fn and(&self, w: &WireType) -> WireType {
        self.and_with_option(w, &None)
    }
    fn and_with_option(&self, w: &WireType, desc: &Option<String>) -> WireType {
        self.mulw_with_option(w, desc)
    }
    #[inline]
    fn get_bit_wiresi(&self, bitwidth: u64) -> WireArray {
        self.get_bit_wiresi_with_option(bitwidth, &None)
    }
    fn get_bit_wiresi_with_option(&self, bitwidth: u64, desc: &Option<String>) -> WireArray {
        let mut bit_wires = self.get_bit_wires();
        if let Some(bit_wires) = bit_wires {
            if bitwidth < bit_wires.size() as u64 && !self.instance_of("ConstantWire") {
                println!(
                    "Warning: get_bit_wires() was called with different arguments on the same wire more than once"
                );
                println!(
                    "\t It was noted that the argument in the second call was less than the first."
                );
                println!(
                    "\t If self was called for enforcing a bitwidth constraint, you must use restrictBitLengh(), otherwise you can ignore self."
                );
                if CONFIGS.print_stack_trace_at_warnings {
                    //println!("Thread.dumpStack();");
                } else {
                    println!(
                        "\t You can view the stack trace by setting CONFIGS.printStackTraceAtWarnings to true in the code."
                    );
                }
            }
            return bit_wires.adjust_length(None, bitwidth as usize);
        }
        bit_wires = Some(self.force_split(bitwidth as i32, desc));
        self.set_bits(bit_wires.clone());
        bit_wires.unwrap()
    }

    fn get_bit_wires_if_exist_already(&self) -> Option<WireArray> {
        self.get_bit_wires()
    }

    fn force_split(&self, bitwidth: i32, desc: &Option<String>) -> WireArray {
        let mut generator = self.generator();

        let mut ws = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            ws[i] = Some(WireType::VariableBit(VariableBitWire::new(
                generator.get_current_wire_id(),
                self.generator_weak(),
            )));

            generator.borrow_mut().current_wire_id += 1;
        }
        let op = SplitBasicOp::new(
            &self.self_clone().unwrap(),
            ws.clone(),
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= bitwidth;

            WireArray::new(cached_outputs, generator.clone().downgrade())
                .adjust_length(None, bitwidth as usize)
        } else {
            WireArray::new(ws, generator.clone().downgrade())
        }
    }

    #[inline]
    fn restrict_bit_length(&self, bit_width: u64) {
        self.restrict_bit_length_with_option(bit_width, &None)
    }
    fn restrict_bit_length_with_option(&self, bit_width: u64, desc: &Option<String>) {
        let Some(mut bit_wires) = self.get_bit_wires() else {
            self.get_bit_wiresi_with_option(bit_width, desc);
            return;
        };
        if bit_wires.size() > bit_width as usize {
            let bit_wires = Some(self.force_split(bit_width as i32, desc));
            self.set_bits(bit_wires);
        }
    }
    #[inline]
    fn xor_bitwises(&self, w: &WireType, num_bits: u64) -> WireType {
        self.xor_bitwises_with_option(w, num_bits, &None)
    }
    fn xor_bitwises_with_option(
        &self,
        w: &WireType,
        num_bits: u64,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let bits2 = w.get_bit_wiresi_with_option(num_bits as u64, desc);
        let result = bits1.xor_wire_array(&bits2, num_bits as usize, desc);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }
        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn xor_bitwisei(&self, v: i64, num_bits: u64, desc: &Option<String>) -> WireType {
        self.xor_bitwises_with_option(
            &self.generator().create_constant_wire_with_option(v, desc),
            num_bits,
            desc,
        )
    }

    fn xor_bitwiseb(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> WireType {
        self.xor_bitwises_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            num_bits,
            desc,
        )
    }

    fn and_bitwises(&self, w: &WireType, num_bits: u64, desc: &Option<String>) -> WireType {
        use std::time::Instant;
        let start = Instant::now();
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let bits2 = w.get_bit_wiresi_with_option(num_bits as u64, desc);

        let result = bits1.and_wire_array(&bits2, num_bits as usize, desc);
        let v = result.check_if_constant_bits(desc);

        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }

        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn and_bitwisei(&self, v: i64, num_bits: u64, desc: &Option<String>) -> WireType {
        self.and_bitwises(
            &self.generator().create_constant_wire_with_option(v, desc),
            num_bits,
            desc,
        )
    }

    fn and_bitwiseb(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> WireType {
        self.and_bitwises(
            &self.generator().create_constant_wire_with_option(b, desc),
            num_bits,
            desc,
        )
    }
    #[inline]
    fn or_bitwises(&self, w: &WireType, num_bits: u64) -> WireType {
        self.or_bitwises_with_option(w, num_bits, &None)
    }
    fn or_bitwises_with_option(
        &self,
        w: &WireType,
        num_bits: u64,
        desc: &Option<String>,
    ) -> WireType {
        use std::time::Instant;
        let start = Instant::now();
        let mut generator = self.generator();
        let bits1 = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let bits2 = w.get_bit_wiresi_with_option(num_bits as u64, desc);
        let result = bits1.or_wire_array(bits2, num_bits as usize, desc);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }

        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn or_bitwisei(&self, v: i64, num_bits: u64, desc: &Option<String>) -> WireType {
        self.or_bitwises_with_option(
            &self.generator().create_constant_wire_with_option(v, desc),
            num_bits,
            desc,
        )
    }

    fn or_bitwiseb(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> WireType {
        self.or_bitwises_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            num_bits,
            desc,
        )
    }
    #[inline]
    fn is_equal_tos(&self, w: &WireType) -> WireType {
        self.is_equal_tos_with_option(w, &None)
    }
    fn is_equal_tos_with_option(&self, w: &WireType, desc: &Option<String>) -> WireType {
        self.pack_if_needed_with_option(desc);
        w.pack_if_needed_with_option(desc);
        let s = self.subw(w, desc);
        s.check_non_zero_with_option(desc)
            .inv_as_bit_with_option(desc)
            .unwrap()
    }

    fn is_equal_tob(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        self.is_equal_tos_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            desc,
        )
    }
    #[inline]
    fn is_equal_toi(&self, v: i64) -> WireType {
        self.is_equal_toi_with_option(v, &None)
    }
    fn is_equal_toi_with_option(&self, v: i64, desc: &Option<String>) -> WireType {
        self.is_equal_tos(&self.generator().create_constant_wire_with_option(v, desc))
    }
    #[inline]
    fn is_less_than_or_equals(&self, w: &WireType, bitwidth: i32) -> WireType {
        self.is_less_than_or_equals_with_option(w, bitwidth, &None)
    }
    fn is_less_than_or_equals_with_option(
        &self,
        w: &WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.pack_if_needed_with_option(desc);
        w.pack_if_needed_with_option(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().create_constant_wire_with_option(&p, desc);
        let sum = pWire
            .addw_with_option(w, desc)
            .subw(&self.self_clone().unwrap(), desc);
        let bit_wires = sum.get_bit_wiresi_with_option(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize].clone().unwrap()
    }

    fn is_less_than_or_equali(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        self.is_less_than_or_equals_with_option(
            &self.generator().create_constant_wire_with_option(v, desc),
            bitwidth,
            desc,
        )
    }

    fn is_less_than_or_equalb(
        &self,
        b: &BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.is_less_than_or_equals_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            bitwidth,
            desc,
        )
    }
    #[inline]
    fn is_less_thans(&self, w: &WireType, bitwidth: i32) -> WireType {
        self.is_less_thans_with_option(w, bitwidth, &None)
    }
    fn is_less_thans_with_option(
        &self,
        w: &WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.pack_if_needed_with_option(desc);
        w.pack_if_needed_with_option(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().create_constant_wire_with_option(&p, desc);
        let sum = pWire
            .addw_with_option(&self.self_clone().unwrap(), desc)
            .subw(w, desc);
        let bit_wires = sum.get_bit_wiresi_with_option(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize]
            .as_ref()
            .unwrap()
            .inv_as_bit_with_option(desc)
            .unwrap()
    }

    fn is_less_thani(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        self.is_less_thans_with_option(
            &self.generator().create_constant_wire_with_option(v, desc),
            bitwidth,
            desc,
        )
    }
    #[inline]
    fn is_less_thanb(&self, b: &BigInteger, bitwidth: i32) -> WireType {
        self.is_less_thanb_with_option(b, bitwidth, &None)
    }
    fn is_less_thanb_with_option(
        &self,
        b: &BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.is_less_thans_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            bitwidth,
            desc,
        )
    }
    #[inline]
    fn is_greater_than_or_equals(
        &self,
        w: &WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.is_greater_than_or_equals_with_option(w, bitwidth, &None)
    }

    fn is_greater_than_or_equals_with_option(
        &self,
        w: &WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.pack_if_needed_with_option(desc);
        w.pack_if_needed_with_option(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().create_constant_wire_with_option(&p, desc);
        let sum = pWire
            .addw_with_option(&self.self_clone().unwrap(), desc)
            .subw(w, desc);
        let bit_wires = sum.get_bit_wiresi_with_option(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize].clone().unwrap()
    }

    fn is_greater_than_or_equali(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        self.is_greater_than_or_equals_with_option(
            &self.generator().create_constant_wire_with_option(v, desc),
            bitwidth,
            desc,
        )
    }

    fn is_greater_than_or_equalb(
        &self,
        b: &BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.is_greater_than_or_equals_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            bitwidth,
            desc,
        )
    }
    #[inline]
    fn is_greater_thans(&self, w: &WireType, bitwidth: i32) -> WireType {
        self.is_greater_thans_with_option(w, bitwidth, &None)
    }
    fn is_greater_thans_with_option(
        &self,
        w: &WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        self.pack_if_needed_with_option(desc);
        w.pack_if_needed_with_option(desc);
        let p = BigInteger::from(2).pow(bitwidth as u32);
        let pWire = self.generator().create_constant_wire_with_option(&p, desc);
        let sum = pWire
            .addw_with_option(w, desc)
            .subw(&self.self_clone().unwrap(), desc);
        let bit_wires = sum.get_bit_wiresi_with_option(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize]
            .clone()
            .unwrap()
            .inv_as_bit_with_option(desc)
            .unwrap()
    }

    fn is_greater_thani(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        self.is_greater_thans_with_option(
            &self.generator().create_constant_wire_with_option(v, desc),
            bitwidth,
            desc,
        )
    }

    fn is_greater_thanb(&self, b: &BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        self.is_greater_thans_with_option(
            &self.generator().create_constant_wire_with_option(b, desc),
            bitwidth,
            desc,
        )
    }
    #[inline]
    fn rotate_left(&self, num_bits: usize, s: usize) -> WireType {
        self.rotate_left_with_option(num_bits, s, &None)
    }
    fn rotate_left_with_option(
        &self,
        num_bits: usize,
        s: usize,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        let bits = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let mut rotated_bits = vec![None; num_bits];
        for i in 0..num_bits {
            if i < s {
                rotated_bits[i] = bits[i + (num_bits - s)].clone();
            } else {
                rotated_bits[i] = bits[i - s].clone();
            }
        }
        let result = WireArray::new(rotated_bits, generator.clone().downgrade());
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire_with_option(&v, &None);
        }
        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }
    #[inline]
    fn rotate_right(&self, num_bits: usize, s: usize) -> WireType {
        self.rotate_right_with_option(num_bits, s, &None)
    }
    fn rotate_right_with_option(
        &self,
        num_bits: usize,
        s: usize,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        let bits = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let mut rotated_bits = vec![None; num_bits];
        for i in 0..num_bits {
            if i >= num_bits - s {
                rotated_bits[i] = bits[i - (num_bits - s)].clone();
            } else {
                rotated_bits[i] = bits[i + s].clone();
            }
        }
        let result = WireArray::new(rotated_bits, generator.clone().downgrade());
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }
        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }
    #[inline]
    fn shift_left(&self, num_bits: usize, s: usize) -> WireType {
        self.shift_left_with_option(num_bits, s, &None)
    }
    fn shift_left_with_option(&self, num_bits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        if s >= num_bits {
            // Will always be zero in that case
            return generator.get_zero_wire().unwrap();
        }

        let bits = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let mut shifted_bits = vec![None; num_bits];
        for i in 0..num_bits {
            if i < s {
                shifted_bits[i] = generator.get_zero_wire();
            } else {
                shifted_bits[i] = bits[i - s].clone();
            }
        }
        let result = WireArray::new(shifted_bits, generator.clone().downgrade());
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }
        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    #[inline]
    fn shift_right(&self, num_bits: usize, s: usize) -> WireType {
        self.shift_right_with_option(num_bits, s, &None)
    }
    fn shift_right_with_option(
        &self,
        num_bits: usize,
        s: usize,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        if s >= num_bits {
            // Will always be zero in that case
            return generator.get_zero_wire().unwrap();
        }

        let bits = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let mut shifted_bits = vec![None; num_bits];
        for i in 0..num_bits {
            if i >= num_bits - s {
                shifted_bits[i] = generator.get_zero_wire();
            } else {
                shifted_bits[i] = bits[i + s].clone();
            }
        }

        let result = WireArray::new(shifted_bits, generator.clone().downgrade());
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }

        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn shift_arith_right(&self, num_bits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        let bits = self.get_bit_wiresi_with_option(num_bits as u64, desc);
        let mut shifted_bits = vec![None; num_bits];
        let sign = &bits[num_bits - 1];
        for i in 0..num_bits {
            if i >= num_bits - s {
                shifted_bits[i] = sign.clone();
            } else {
                shifted_bits[i] = bits[i + s].clone();
            }
        }
        let result = WireArray::new(shifted_bits, generator.clone().downgrade());
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }
        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }
    #[inline]
    fn inv_bits(&self, bitwidth: u64) -> WireType {
        self.inv_bits_with_option(bitwidth, &None)
    }
    fn inv_bits_with_option(&self, bitwidth: u64, desc: &Option<String>) -> WireType {
        let bits = self.get_bit_wiresi_with_option(bitwidth, desc);
        let bits = bits.as_array();
        let mut resultBits = vec![None; bits.len()];
        for i in 0..resultBits.len() {
            resultBits[i] = bits[i]
                .as_ref()
                .and_then(|x| x.clone().inv_as_bit_with_option(desc));
        }
        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(WireArray::new(resultBits, self.generator_weak())),
            self.generator_weak(),
        ))
    }
    #[inline]
    fn trim_bits(&self, current_num_of_bits: i32, desired_num_of_bits: i32) -> WireType {
        self.trim_bits_with_option(current_num_of_bits, desired_num_of_bits, &None)
    }
    fn trim_bits_with_option(
        &self,
        current_num_of_bits: i32,
        desired_num_of_bits: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        let bit_wires = self.get_bit_wiresi_with_option(current_num_of_bits as u64, desc);
        let result = bit_wires.adjust_length(None, desired_num_of_bits as usize);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v);
        }
        WireType::LinearCombination(LinearCombination_wire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }
    #[inline]
    fn pack_if_needed(&self) {
        self.pack_if_needed_with_option(&None)
    }
    fn pack_if_needed_with_option(&self, desc: &Option<String>) {
        if self.get_wire_id() == -1 {
            self.pack(&None);
        }
    }

    fn pack(&self, desc: &Option<String>) {
        if self.get_wire_id() != -1 {
            return;
        }
        let mut generator = self.generator();

        let bits = self.get_bit_wires();
        assert!(
            bits.is_some(),
            "A Pack operation is tried on a wire that has no bits."
        );
        generator.borrow_mut().current_wire_id += 1;

        let op = PackBasicOp::new(
            bits.unwrap().array.clone(),
            &self.self_clone().unwrap(),
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));

        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            *self.get_wire_id_mut().borrow_mut() =
                cached_outputs[0].as_ref().unwrap().get_wire_id();
        }
    }
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

#[enum_dispatch]
pub trait GetWireId {
    fn get_wire_id(&self) -> i32;
    fn get_wire_id_mut(&self) -> RcCell<i32>;
}

impl<T: SetBitsConfig + Clone + Debug + PartialEq> GetWireId for Wire<T> {
    fn get_wire_id(&self) -> i32 {
        let id = *self.wire_id.borrow();
        id
    }
    fn get_wire_id_mut(&self) -> RcCell<i32> {
        self.wire_id.clone()
    }
}

#[macro_export]
macro_rules! new_wire {
    ($impl_type:ident,$wire_id:expr,$generator:ident) => {
        Wire::<$impl_type> {
            wire_id: $wire_id,
            $generator,
            t: $impl_type,
        }
    };
}
