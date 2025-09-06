#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        InstanceOf, StructNameConfig,
        config::config::Configs,
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
            linear_combination_wire::LinearCombinationWire,
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
        // assert!(wire_id!=20056,"===wire_id====={wire_id}==");
        // if wire_id < 0 {
        //     eyre::bail!("wire id cannot be negative");
        // }
        // if t.name()=="ConstantWire"{
        // println!("==new wire={t:?}========{wire_id}===");
        // }
        Ok(Self {
            wire_id: RcCell::new(wire_id),
            generator,
            t,
        })
    }

    pub fn new_array(bits: WireArray, t: T, generator: WeakCell<CircuitGenerator>) -> Self {
        // println!("==newarray============={t:?}========{bits:?}===");
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

// impl<T: SetBitsConfig + Clone + Debug + PartialEq> SetBitsConfig for Wire<T> {}
// impl<T: SetBitsConfig + Clone + Debug + PartialEq> WireConfig for Wire<T> {}
#[enum_dispatch]
pub trait WireConfig: PartialEq + SetBitsConfig + InstanceOf + GetWireId + GeneratorConfig {
    // fn instance_of(&self, name: &str) -> bool {
    //     self.name() == name
    // }
    // fn name(&self) -> &str {
    //     ""
    // }

    // fn toString(&self) -> String {
    //     self.get_wire_id().to_string()
    // }

    fn get_bit_wires(&self) -> Option<WireArray> {
        None
    }
    fn self_clone(&self) -> Option<WireType> {
        None
    }

    fn mulb(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        // assert!( b!=&BigInteger::from(-1));
        // println!(
        //     "=======================mulb============{}=========== {} ",
        //     file!(),
        //     line!()
        // );
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        // println!("End Name Time: 444 {} s", line!());
        if b == &Util::one() {
            return self.self_clone().unwrap();
        }
        if b == &BigInteger::ZERO {
            return generator.get_zero_wire().unwrap();
        }

        let out = WireType::LinearCombination(LinearCombinationWire::new(
            generator.get_current_wire_id(),
            None,
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        // println!("===self======{} ==={}=={}==={}", line!(),self.get_wire_id(),self.name(),b);
        let op = ConstMulBasicOp::new(
            self.self_clone().as_ref().unwrap(),
            &out,
            b,
            desc.clone().unwrap_or(String::new()),
        );
        //		generator.add_to_evaluation_queue(Box::new(op));

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        // println!("===out====={}======={}====== {} ", line!(),out,out.name());
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            // println!(
            //     "====generator.borrow_mut().current_wire_id==={}==={}=={}==={}{}",
            //     generator.borrow_mut().current_wire_id,
            //     file!(),
            //     line!(),out,out.name()
            // );

            cached_outputs[0].clone().unwrap()
        } else {
            out
        }
    }

    fn muli(&self, l: i64, desc: &Option<String>) -> WireType {
        // println!("End Name Time: 444 {} s", line!());
        self.mulb(&BigInteger::from(l), desc)
    }

    fn mulii(&self, base: i64, exp: i32, desc: &Option<String>) -> WireType {
        let mut b = BigInteger::from(base);
        b = b.pow(exp as u32);
        self.mulb(&b, desc)
    }

    fn mulw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        use std::time::Instant;
        let start = Instant::now();
        let mut generator = self.generator();
        //  println!("End adjust_length  Time: == {} s", start.elapsed().as_secs());
        if w.instance_of("ConstantWire") {
            // println!(
            //     "===w.instance_of(ConstantWire)================={}===={}=======",
            //     line!(),
            //     file!()
            // );
            let v = self.mulb(&w.try_as_constant_ref().unwrap().get_constant(), desc);
            //  println!("End mulb  Time: == {} s", start.elapsed().as_micros());
            return v;
        }

        self.pack_if_needed(desc);
        // println!(
        //     "End pack_if_needed  Time: == {} s",
        //     start.elapsed().as_micros()
        // );
        w.pack_if_needed(desc);
        // println!(
        //     "End pack_if_needed 2 Time: == {} s",
        //     start.elapsed().as_micros()
        // );
        let output = WireType::Variable(VariableWire::new(
            generator.get_current_wire_id(),
            self.generator_weak(),
        ));
        // println!(
        //     "End VariableWire::new  Time: == {} s",
        //     start.elapsed().as_micros()
        // );
        generator.borrow_mut().current_wire_id += 1;
        let op = MulBasicOp::new(
            &self.self_clone().unwrap(),
            w,
            &output,
            desc.clone().unwrap_or(String::new()),
        );
        // println!(" ============MulBasicOp::new==self.get_wire_id==={}========  w.get_wire_id: == {} ",self.self_clone().unwrap().get_wire_id(), w.get_wire_id());

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        // println!(
        //     "End add_to_evaluation_queue  Time: == {} s",
        //     start.elapsed().as_micros()
        // );
        // assert!( output.get_wire_id()!=175548);
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            // println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());
            cached_outputs[0].clone().unwrap()
        } else {
            output
        }
    }

    fn addw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        WireArray::new(
            vec![Some(self.self_clone().unwrap()), Some(w.clone())],
            self.generator_weak(),
        )
        .sum_all_elements(desc)
    }

    fn addi(&self, v: i64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.addw(&generator.create_constant_wire(v, desc), desc)
    }

    fn addb(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.addw(&generator.create_constant_wire(b, desc), desc)
    }

    fn subw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let neg = w.muli(-1, desc);
        self.addw(&neg, desc)
    }

    fn subi(&self, v: i64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.subw(&generator.create_constant_wire(v, desc), desc)
    }

    fn subb(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.subw(&generator.create_constant_wire(b, desc), desc)
    }

    fn negate(&self, desc: &Option<String>) -> WireType {
        return self
            .generator()
            .get_zero_wire()
            .unwrap()
            .subw(&self.self_clone().unwrap(), desc);
    }

    fn mux(&self, trueValue: &WireType, falseValue: &WireType) -> WireType {
        return falseValue.addw(
            &self
                .self_clone()
                .unwrap()
                .mulw(&trueValue.subw(falseValue, &None), &None),
            &None,
        );
    }

    fn check_non_zero(&self, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed(desc);

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

    fn inv_as_bit(&self, desc: &Option<String>) -> Option<WireType> {
        let mut generator = self.generator();

        self.pack_if_needed(desc); // just a precaution .. should not be really needed
        let w1 = self.muli(-1, desc);
        let s = generator.get_one_wire();
        // println!("====inv_as_bit==========={}===={s:?}", generator.get_name());
        let out = generator.get_one_wire().unwrap().addw(&w1, desc);
        Some(out)
    }

    fn orw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        use std::time::Instant;
        let start = Instant::now();

        let mut generator = self.generator();

        if w.instance_of("ConstantWire") {
            // println!(
            //     "===w.instance_of(ConstantWire)================={}===={}=======",
            //     line!(),
            //     file!()
            // );
            return w.orw(self.self_clone().as_ref().unwrap(), desc);
        }
        self.pack_if_needed(desc); // just a precaution .. should not be really
        //  println!(
        //             "End orw add_to_evaluation_queue 013 Time: == {:?} ",
        //             start.elapsed()
        //         );
        // needed
        let out = WireType::Variable(VariableWire::new(
            generator.get_current_wire_id(),
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let sc = self.self_clone().unwrap();
        let desc = desc.clone().unwrap_or(String::new());
        //  println!(
        //             "End orw add_to_evaluation_queue 014 Time: == {:?} ",
        //             start.elapsed()
        //         );
        let op = OrBasicOp::new(&sc, w, &out, desc);

        //  println!(
        //             "End orw add_to_evaluation_queue 0153333 Time: == {:?} ",
        //             start.elapsed()
        //         );

        //  println!(
        //             "End orw add_to_evaluation_queue 015 Time: == {:?} ",
        //             start.elapsed()
        //         );
        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        //  println!(
        //             "End orw add_to_evaluation_queue 02 Time: == {:?} ",
        //             start.elapsed()
        //         );
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());
            cached_outputs[0].clone().unwrap()
        } else {
            out
        }
    }

    fn xorw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        if w.instance_of("ConstantWire") {
            // println!(
            //     "===w.instance_of(ConstantWire)================={}===={}=======",
            //     line!(),
            //     file!()
            // );
            return w.xorw(&self.self_clone().unwrap(), desc);
        }
        self.pack_if_needed(desc); // just a precaution .. should not be really
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
            //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());
            cached_outputs[0].clone().unwrap()
        } else {
            out
        }
    }

    fn and(&self, w: &WireType, desc: &Option<String>) -> WireType {
        self.mulw(w, desc)
    }

    fn get_bit_wiresi(&self, bitwidth: u64, desc: &Option<String>) -> WireArray {
        //println!("======================{},{}",file!(),line!());
        let mut bit_wires = self.get_bit_wires();
        //println!("======================{},{}",file!(),line!());
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
                if Configs.print_stack_trace_at_warnings {
                    //println!("Thread.dumpStack();");
                } else {
                    println!(
                        "\t You can view the stack trace by setting Configs.printStackTraceAtWarnings to true in the code."
                    );
                }
            }
            //println!("======================{},{}",file!(),line!());
            return bit_wires.adjust_length(None, bitwidth as usize);
        }
        //println!("======================{},{}",file!(),line!());
        bit_wires = Some(self.force_split(bitwidth as i32, desc));
        //println!("======================{},{}",file!(),line!());
        self.set_bits(bit_wires.clone());
        //println!("======================{},{}",file!(),line!());
        bit_wires.unwrap()
    }

    fn get_bit_wires_if_exist_already(&self) -> Option<WireArray> {
        self.get_bit_wires()
    }

    fn force_split(&self, bitwidth: i32, desc: &Option<String>) -> WireArray {
        let mut generator = self.generator();

        //println!("====force_split==========={bitwidth}======={},{}",file!(),line!());
        let mut ws = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            //println!("======================{},{}",file!(),line!());
            ws[i] = Some(WireType::VariableBit(VariableBitWire::new(
                generator.get_current_wire_id(),
                self.generator_weak(),
            )));
            //println!("======================{},{}",file!(),line!());

            generator.borrow_mut().current_wire_id += 1;
        }
        //println!("======================{},{}",file!(),line!());
        let op = SplitBasicOp::new(
            &self.self_clone().unwrap(),
            ws.clone(),
            desc.clone().unwrap_or(String::new()),
        );
        //println!("======================{},{}",file!(),line!());

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= bitwidth;
            //println!("======================{},{}",file!(),line!());
            WireArray::new(cached_outputs, generator.clone().downgrade())
                .adjust_length(None, bitwidth as usize)
        } else {
            WireArray::new(ws, generator.clone().downgrade())
        }
    }

    fn restrict_bit_length(&self, bit_width: u64, desc: &Option<String>) {
        let Some(mut bit_wires) = self.get_bit_wires() else {
            self.get_bit_wiresi(bit_width, desc);
            return;
        };
        if bit_wires.size() > bit_width as usize {
            let bit_wires = Some(self.force_split(bit_width as i32, desc));
            self.set_bits(bit_wires);
        }
    }

    fn xor_bitwise(&self, w: &WireType, num_bits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi(num_bits as u64, desc);
        let bits2 = w.get_bit_wiresi(num_bits as u64, desc);
        let result = bits1.xor_wire_array(&bits2, num_bits as usize, desc);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v, &None);
        }
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn xor_bitwisei(&self, v: i64, num_bits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.xor_bitwise(&generator.create_constant_wire(v, desc), num_bits, desc);
    }

    fn xor_bitwiseb(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.xor_bitwise(&generator.create_constant_wire(b, desc), num_bits, desc);
    }

    fn and_bitwise(&self, w: &WireType, num_bits: u64, desc: &Option<String>) -> WireType {
        use std::time::Instant;
        let start = Instant::now();
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi(num_bits as u64, desc);
        // println!("End get_bit_wiresi  Time: == {} s", start.elapsed().as_secs());
        let bits2 = w.get_bit_wiresi(num_bits as u64, desc);
        // println!(
        //     "End getBitWiresi2  Time: == {} s",
        //     start.elapsed().as_secs()
        // );
        let result = bits1.and_wire_array(&bits2, num_bits as usize, desc);
        // println!("End and_wire_array  Time: == {} s", start.elapsed().as_secs());
        let v = result.check_if_constant_bits(desc);
        // println!(
        //     "End check_if_constant_bits  Time: == {} s",
        //     start.elapsed().as_secs()
        // );
        if let Some(v) = v {
            return generator.create_constant_wire(&v, &None);
        }
        // println!(
        //     "End create_constant_wire  Time: == {} s",
        //     start.elapsed().as_secs()
        // );
        let v = WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ));
        // println!(
        //     "End LinearCombinationWire::new  Time: == {} s",
        //     start.elapsed().as_secs()
        // );
        v
    }

    fn and_bitwisei(&self, v: i64, num_bits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.and_bitwise(&generator.create_constant_wire(v, desc), num_bits, desc);
    }

    fn and_bitwiseb(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.and_bitwise(&generator.create_constant_wire(b, desc), num_bits, desc);
    }

    fn or_bitwises(&self, w: &WireType, num_bits: u64, desc: &Option<String>) -> WireType {
        use std::time::Instant;
        let start = Instant::now();
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi(num_bits as u64, desc);
        //   println!(
        //             "End or_bitwises getBitWiresi0 Time: == {:?} ",
        //             start.elapsed()
        //         );
        let bits2 = w.get_bit_wiresi(num_bits as u64, desc);
        //   println!(
        //             "End or_bitwises getBitWiresi01 Time: == {:?} ",
        //             start.elapsed()
        //         );
        let result = bits1.or_wire_array(bits2, num_bits as usize, desc);
        //   println!(
        //             "End or_bitwises orWireArray0 Time: == {:?} ",
        //             start.elapsed()
        //         );
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            //   println!(
            //             "End or_bitwises checkIfConstantBits0 Time: == {:?} ",
            //             start.elapsed()
            //         );
            return generator.create_constant_wire(&v, &None);
        }
        //   println!(
        //             "End or_bitwises 0 Time: == {:?} ",
        //             start.elapsed()
        //         );
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn or_bitwisei(&self, v: i64, num_bits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.or_bitwises(&generator.create_constant_wire(v, desc), num_bits, desc);
    }

    fn or_bitwiseb(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.or_bitwises(&generator.create_constant_wire(b, desc), num_bits, desc);
    }

    fn is_equal_tos(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let s = self.subw(w, desc);
        s.check_non_zero(desc).inv_as_bit(desc).unwrap()
    }

    fn is_equal_tob(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.is_equal_tos(&generator.create_constant_wire(b, desc), &None)
    }

    fn is_equal_toi(&self, v: i64, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.is_equal_tos(&generator.create_constant_wire(v, desc), &None)
    }

    fn is_less_than_or_equals(
        &self,
        w: &WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(w, desc).subw(&self.self_clone().unwrap(), desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize].clone().unwrap()
    }

    fn is_less_than_or_equali(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.is_less_than_or_equals(
            &generator.create_constant_wire(v, desc),
            bitwidth,
            desc,
        );
    }

    fn is_less_than_or_equalb(
        &self,
        b: &BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        return self.is_less_than_or_equals(
            &generator.create_constant_wire(b, desc),
            bitwidth,
            desc,
        );
    }

    fn is_less_thans(&self, w: &WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(&self.self_clone().unwrap(), desc).subw(w, desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        return bit_wires[bitwidth as usize]
            .as_ref()
            .unwrap()
            .inv_as_bit(desc)
            .unwrap();
    }

    fn is_less_thani(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.is_less_thans(&generator.create_constant_wire(v, desc), bitwidth, desc);
    }

    fn is_less_thanb(&self, b: &BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.is_less_thans(&generator.create_constant_wire(b, desc), bitwidth, desc);
    }

    fn is_greater_than_or_equals(
        &self,
        w: &WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(&self.self_clone().unwrap(), desc).subw(w, desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize].clone().unwrap()
    }

    fn is_greater_than_or_equali(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.is_greater_than_or_equals(
            &generator.create_constant_wire(v, desc),
            bitwidth,
            desc,
        );
    }

    fn is_greater_than_or_equalb(
        &self,
        b: &BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        return self.is_greater_than_or_equals(
            &generator.create_constant_wire(b, desc),
            bitwidth,
            desc,
        );
    }

    fn is_greater_thans(&self, w: &WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(w, desc).subw(&self.self_clone().unwrap(), desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        return bit_wires[bitwidth as usize]
            .clone()
            .unwrap()
            .inv_as_bit(desc)
            .unwrap();
    }

    fn is_greater_thani(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.is_greater_thans(&generator.create_constant_wire(v, desc), bitwidth, desc);
    }

    fn is_greater_thanb(&self, b: &BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        return self.is_greater_thans(&generator.create_constant_wire(b, desc), bitwidth, desc);
    }

    fn rotate_left(&self, num_bits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        let bits = self.get_bit_wiresi(num_bits as u64, desc);
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
            return generator.create_constant_wire(&v, &None);
        }
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn rotate_right(&self, num_bits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        let bits = self.get_bit_wiresi(num_bits as u64, desc);
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
            return generator.create_constant_wire(&v, &None);
        }
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn shift_left(&self, num_bits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        if s >= num_bits {
            // Will always be zero in that case
            return generator.get_zero_wire().unwrap();
        }

        let bits = self.get_bit_wiresi(num_bits as u64, desc);
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
            return generator.create_constant_wire(&v, &None);
        }
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn shift_right(&self, num_bits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        //println!("======================{},{}",file!(),line!());
        if s >= num_bits {
            //println!("======================{},{}",file!(),line!());
            // Will always be zero in that case
            return generator.get_zero_wire().unwrap();
        }
        //println!("======================{},{}",file!(),line!());
        let bits = self.get_bit_wiresi(num_bits as u64, desc);
        let mut shifted_bits = vec![None; num_bits];
        for i in 0..num_bits {
            if i >= num_bits - s {
                shifted_bits[i] = generator.get_zero_wire();
            } else {
                shifted_bits[i] = bits[i + s].clone();
            }
        }
        //println!("======================{},{}",file!(),line!());
        let result = WireArray::new(shifted_bits, generator.clone().downgrade());
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            //println!("======================{},{}",file!(),line!());
            return generator.create_constant_wire(&v, &None);
        }
        //println!("======================{},{}",file!(),line!());
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn shift_arith_right(&self, num_bits: usize, s: usize, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        let bits = self.get_bit_wiresi(num_bits as u64, desc);
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
            return generator.create_constant_wire(&v, &None);
        }
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn inv_bits(&self, bitwidth: u64, desc: &Option<String>) -> WireType {
        let bits = self.get_bit_wiresi(bitwidth, desc);
        let bits = bits.as_array();
        let mut resultBits = vec![None; bits.len()];
        for i in 0..resultBits.len() {
            resultBits[i] = bits[i].as_ref().and_then(|x| x.clone().inv_as_bit(desc));
        }
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(WireArray::new(resultBits, self.generator_weak())),
            self.generator_weak(),
        ))
    }

    fn trim_bits(
        &self,
        currentNumOfBits: i32,
        desiredNumofBits: i32,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        let bit_wires = self.get_bit_wiresi(currentNumOfBits as u64, desc);
        let result = bit_wires.adjust_length(None, desiredNumofBits as usize);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            return generator.create_constant_wire(&v, &None);
        }
        WireType::LinearCombination(LinearCombinationWire::new(
            -1,
            Some(result),
            generator.clone().downgrade(),
        ))
    }

    fn pack_if_needed(&self, desc: &Option<String>) {
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
        //self.get_wire_id_mut().borrow_mut() = generator.borrow().current_wire_id;
        generator.borrow_mut().current_wire_id += 1;
        //			Instruction op = PackBasicOp::new(bits.array, self, desc);
        //			generator.add_to_evaluation_queue(Box::new(op));

        let op = PackBasicOp::new(
            bits.unwrap().array.clone(),
            &self.self_clone().unwrap(),
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));

        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            if cached_outputs[0].as_ref().unwrap().get_wire_id() == 349252 {
                println!(
                    "=***********=pack==current_wire_id===={}=={}====={}{}",
                    cached_outputs[0].as_ref().unwrap().get_wire_id(),
                    generator.borrow_mut().current_wire_id,
                    file!(),
                    line!()
                );
            }
            //self.get_wire_id_mut().borrow_mut() = cached_outputs[0].as_ref().unwrap().get_wire_id();
        }
        //  println!("=***********=pack=====get_wire_id=========={}",self.get_wire_id());
    }

    // fn hashCode(&self) -> u64 {
    //     self.get_wire_id() as u64
    // }

    // fn equals(&self, rhs: &Self) -> bool {
    //     if self == rhs {
    //         return true;
    //     }

    //     let w = rhs;
    //     w.get_wire_id() == self.get_wire_id() && w.generator() == self.generator().lock()
    // }
}

// #[macro_export]
// macro_rules! impl_hash_code_of_wire_for {
//     ($impl_type:ty) => {
//         impl std::hash::Hash for $impl_type {
//             fn hash<H: Hasher>(&self, state: &mut H) {
//                 self.get_wire_id().hash(state);
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! impl_display_of_wire_for {
//     ($impl_type:ty) => {
//         impl std::fmt::Display for $impl_type {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 write!(f, "{}", self.get_wire_id())
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! impl_hash_code_of_wire_g_for {
//     ($impl_type:ty) => {
//         impl std::hash::Hash for $impl_type {
//             fn hash<H: Hasher>(&self, state: &mut H) {
//                 self.get_wire_id().hash(state);
//             }
//         }
//     };
// }

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

// #[macro_export]
// macro_rules! impl_eq_of_wire_for {
//     ($impl_type:ty) => {
//         impl Eq for $impl_type {}
//         impl PartialEq for $impl_type {
//             fn eq(&self, other: &Self) -> bool {
//                 other.get_wire_id() == self.get_wire_id()
//                     && other.generator() == self.generator().lock()
//             }
//         }
//     };
// }

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

// #[macro_export]
// macro_rules! impl_get_wire_id_of_wire_for {
//     ($impl_type:ty) => {
// impl GetWireId for $impl_type {
//      fn get_wire_id(&self) -> i32 {
//         self.wire_id
//     }
// }
//     };
// }

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
