#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element,
        auxiliary::long_element::LongElement,
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            gadget::GadgetConfig,
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    ops::{Add, Div, Mul, Rem, Sub},
};

use rccell::RcCell;

//  * This gadget computes q and r such that a = q * b + r, when both operands are represented
//  * as long elements. You can check the RSA gadgets/circuit generators for an example.
//  * Most of the optimizations that reduce the cost of this step are more visible
//  * in the LongElement class methods called by this gadget.

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerDivision<T: Debug + Clone> {
    pub a: LongElement,
    pub b: LongElement,
    pub r: LongElement,
    pub q: LongElement,
    pub b_min_bitwidth: i32,
    pub restrict_range: bool,
    pub t: PhantomData<T>,
}
impl<T: Debug + Clone> LongIntegerDivision<T> {
    //@param a
    //@param b
    //@param restrict_range
    //		if true, the output will be forced to be less than b,
    //		otherwise the output remainder will only be guaranteed
    //		to have the same bitwidth as b, but not necessarily less
    //		than b. The second case is helpful when the purpose is
    //		just to reduce the range, while having consistent
    //		output. As an example (in a short integer case for
    //		simplicity): assume we are interested in this operation
    //		3001 % 10. The output should be 1 in normal cases, but
    //		to save some operations, we might skip checking that the
    //		result is less than the modulus and just check that it
    //		has the same bitwidth as the modulus, which we must do
    //		anyway since the result is provided as a witness. In
    //		that case, the output of this gadget could be 1 or 11,
    //		which in some contexts would be ok, e.g. in intermediate
    //		operations. See the RSA encryption gadget for an
    //		illustration.
    //@param desc

    //@param a
    //@param b
    //@param b_min_bitwidth
    //		The minimum bitwidth of the second operand
    //@param restrict_range
    //		if true, the output will be forced to be less than b,
    //		otherwise the output remainder will only be guaranteed to have
    //		the same bitwidth as b, but not necessarily less than b. The
    //		second case is helpful when the purpose is just to reduce the
    //		range, while having consistent output. As an example (in a
    //		short integer case for simplicity): assume we are interested
    //		in this operation 3001 % 10. The output should be 1 in normal
    //		cases, but to save some operations, we might skip checking
    //		that the result is less than the modulus and just check that
    //		it has the same bitwidth as the modulus, which we must do
    //		anyway since the result is provided as a witness. In that
    //		case, the output of this gadget could be 1 or 11, which in
    //		some contexts would be ok, e.g. in intermediate operations.
    //		See the RSA encryption gadget for an illustration.
    //@param desc

    pub fn new(
        a: LongElement,
        b: LongElement,
        b_min_bitwidth: i32,
        restrict_range: bool,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                r: a.clone(),
                q: b.clone(),
                a,
                b,
                b_min_bitwidth,
                restrict_range,
                t: PhantomData,
            },
        );
        _self.build_circuit();
        _self
    }
}

impl<T: Debug + Clone> Gadget<LongIntegerDivision<T>> {
    fn build_circuit(&mut self) {
        let a_bitwidth = self
            .t
            .a
            .get_max_val(LongElement::CHUNK_BITWIDTH)
            .bits()
            .max(1);
        let b_bitwidth = self
            .t
            .b
            .get_max_val(LongElement::CHUNK_BITWIDTH)
            .bits()
            .max(1);

        let mut r_bitwidth = std::cmp::min(a_bitwidth, b_bitwidth);
        let mut q_bitwidth = a_bitwidth;

        if self.t.b_min_bitwidth > 0 {
            q_bitwidth = 1i32.max(q_bitwidth as i32 - self.t.b_min_bitwidth + 1) as u64;
        }

        // length in what follows means the number of chunks
        let r_length = (r_bitwidth as f64 / LongElement::CHUNK_BITWIDTH as f64).ceil() as i32;
        let q_length = (q_bitwidth as f64 / LongElement::CHUNK_BITWIDTH as f64).ceil() as i32;
        let start = std::time::Instant::now();

        let r_wires = CircuitGenerator::create_prover_witness_wire_array(
            self.generator.clone(),
            r_length as usize,
        );
        let q_wires = CircuitGenerator::create_prover_witness_wire_array(
            self.generator.clone(),
            q_length as usize,
        );

        let mut r_chunk_bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; r_length as usize];
        let mut q_chunk_bitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; q_length as usize];

        if r_bitwidth % LongElement::CHUNK_BITWIDTH as u64 != 0 {
            r_chunk_bitwidths[r_length as usize - 1] =
                r_bitwidth % LongElement::CHUNK_BITWIDTH as u64;
        }
        if q_bitwidth % LongElement::CHUNK_BITWIDTH as u64 != 0 {
            q_chunk_bitwidths[q_length as usize - 1] =
                q_bitwidth % LongElement::CHUNK_BITWIDTH as u64;
        }
        let a = &self.t.a;
        let b = &self.t.b;
        let mut r = LongElement::new(
            r_wires,
            r_chunk_bitwidths,
            self.generator.clone().downgrade(),
        );
        let mut q = LongElement::new(
            q_wires,
            q_chunk_bitwidths,
            self.generator.clone().downgrade(),
        );

        let prover = crate::impl_prover!(
                                eval(  a: LongElement,
                            b: LongElement,r: LongElement,
                            q: LongElement
                        )  {
                impl Instruction for Prover{
                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                           let a_value = evaluator.get_wire_valuei(&self.a, LongElement::CHUNK_BITWIDTH);
                    let b_value = evaluator.get_wire_valuei(&self.b, LongElement::CHUNK_BITWIDTH);
                    let r_value = a_value.clone().rem(&b_value);
                    let q_value = a_value.clone().div(&b_value);

                    evaluator.set_wire_valuea(
                        self.r.get_array(),
                        &Util::split(&r_value, LongElement::CHUNK_BITWIDTH),
                    );
                    evaluator.set_wire_valuea(
                        self.q.get_array(),
                        &Util::split(&q_value, LongElement::CHUNK_BITWIDTH),
                    );
        Ok(())

                }
                }
                            }
                        );
        CircuitGenerator::specify_prover_witness_computation(self.generator.clone(), prover);

        r.restrict_bitwidth();
        q.restrict_bitwidth(); //bits  16

        let res = q.clone().mul(b).add(&r);

        // implements the improved long integer equality assertion from xjsnark
        res.assert_equality(a);

        if self.t.restrict_range {
            r.assert_less_than(b);
        }
        self.t.r = r;
        self.t.q = q;
    }
}
pub trait LongIntegerDivisionConfig: GadgetConfig {
    fn get_quotient(&self) -> &LongElement;
    fn get_remainder(&self) -> &LongElement;
}

#[macro_export]
macro_rules! impl_long_integer_division_config_for {
    ($impl_type:ident) => {
        impl LongIntegerDivisionConfig for Gadget<LongIntegerDivision<$impl_type>> {
            fn get_quotient(&self) -> &LongElement {
                &self.t.q
            }
            fn get_remainder(&self) -> &LongElement {
                &self.t.r
            }
        }
    };
}
