#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
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
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::eval::instruction::Instruction;
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{Util,BigInteger};

use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Rem, Sub};
/**
 * This gadget computes q and r such that a = q * b + r, when both operands are represented
 * as long elements. You can check the RSA gadgets/circuit generators for an example.
 * Most of the optimizations that reduce the cost of this step are more visible
 * in the LongElement class methods called by this gadget.
 */
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerDivision<T: Debug + Clone> {
    pub a: LongElement,
    pub b: LongElement,
    pub r: LongElement,
    pub q: LongElement,
    pub bMinBitwidth: i32,
    pub restrictRange: bool,
    pub t: PhantomData<T>,
}
impl<T: Debug + Clone> LongIntegerDivision<T> {
    /**
     * @param a
     * @param b
     * @param restrictRange
     * 		if true, the output will be forced to be less than b,
     * 		otherwise the output remainder will only be guaranteed
     * 		to have the same bitwidth as b, but not necessarily less
     * 		than b. The second case is helpful when the purpose is
     * 		just to reduce the range, while having consistent
     * 		output. As an example (in a short integer case for
     * 		simplicity): assume we are interested in this operation
     * 		3001 % 10. The output should be 1 in normal cases, but
     * 		to save some operations, we might skip checking that the
     * 		result is less than the modulus and just check that it
     * 		has the same bitwidth as the modulus, which we must do
     * 		anyway since the result is provided as a witness. In
     * 		that case, the output of this gadget could be 1 or 11,
     * 		which in some contexts would be ok, e.g. in intermediate
     * 		operations. See the RSA encryption gadget for an
     * 		illustration.
     * @param desc
     */

    /**
     * @param a
     * @param b
     * @param bMinBitwidth
     * 		The minimum bitwidth of the second operand
     * @param restrictRange
     * 		if true, the output will be forced to be less than b,
     * 		otherwise the output remainder will only be guaranteed to have
     * 		the same bitwidth as b, but not necessarily less than b. The
     * 		second case is helpful when the purpose is just to reduce the
     * 		range, while having consistent output. As an example (in a
     * 		short integer case for simplicity): assume we are interested
     * 		in this operation 3001 % 10. The output should be 1 in normal
     * 		cases, but to save some operations, we might skip checking
     * 		that the result is less than the modulus and just check that
     * 		it has the same bitwidth as the modulus, which we must do
     * 		anyway since the result is provided as a witness. In that
     * 		case, the output of this gadget could be 1 or 11, which in
     * 		some contexts would be ok, e.g. in intermediate operations.
     * 		See the RSA encryption gadget for an illustration.
     * @param desc
     */
    pub fn new(
        a: LongElement,
        b: LongElement,
        bMinBitwidth: i32,
        restrictRange: bool,
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
                bMinBitwidth,
                restrictRange,
                t: PhantomData,
            },
        );
        _self.buildCircuit();
        _self
    }
}

impl<T: Debug + Clone> Gadget<LongIntegerDivision<T>> {
    fn buildCircuit(&mut self) {
        let aBitwidth = self
            .t
            .a
            .getMaxVal(LongElement::CHUNK_BITWIDTH)
            .bits()
            .max(1);
        // println!("=====aBitwidth================{aBitwidth}");
        let bBitwidth = self
            .t
            .b
            .getMaxVal(LongElement::CHUNK_BITWIDTH)
            .bits()
            .max(1);

        let mut rBitwidth = std::cmp::min(aBitwidth, bBitwidth);
        let mut qBitwidth = aBitwidth;

        if self.t.bMinBitwidth > 0 {
            qBitwidth = 1i32.max(qBitwidth as i32 - self.t.bMinBitwidth + 1) as u64;
        }

        // length in what follows means the number of chunks
        let rLength = (rBitwidth as f64 / LongElement::CHUNK_BITWIDTH as f64).ceil() as i32;
        let qLength = (qBitwidth as f64 / LongElement::CHUNK_BITWIDTH as f64).ceil() as i32;
        let start = std::time::Instant::now();

        let rWires = CircuitGenerator::createProverWitnessWireArray(
            self.generator.clone(),
            rLength as usize,
            &None,
        );
        let qWires = CircuitGenerator::createProverWitnessWireArray(
            self.generator.clone(),
            qLength as usize,
            &None,
        );

        let mut rChunkBitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; rLength as usize];
        let mut qChunkBitwidths = vec![LongElement::CHUNK_BITWIDTH as u64; qLength as usize];

        if rBitwidth % LongElement::CHUNK_BITWIDTH as u64 != 0 {
            rChunkBitwidths[rLength as usize - 1] = rBitwidth % LongElement::CHUNK_BITWIDTH as u64;
        }
        if qBitwidth % LongElement::CHUNK_BITWIDTH as u64 != 0 {
            println!(
                "===LongElement::CHUNK_BITWIDTH====={}===={}====={qBitwidth} % {} ",
                LongElement::CHUNK_BITWIDTH,
                file!(),
                line!()
            );
            qChunkBitwidths[qLength as usize - 1] = qBitwidth % LongElement::CHUNK_BITWIDTH as u64;
        }
        let a = &self.t.a;
        let b = &self.t.b;
        let mut r = LongElement::new(rWires, rChunkBitwidths, self.generator.clone().downgrade());
        let mut q = LongElement::new(qWires, qChunkBitwidths, self.generator.clone().downgrade());

        // CircuitGenerator::specifyProverWitnessComputation(generator.clone(),&|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a, LongElement::CHUNK_BITWIDTH);
        //             let bValue = evaluator.getWireValue(b, LongElement::CHUNK_BITWIDTH);
        //             let rValue = aValue.rem(bValue);
        //             let qValue = aValue.div(bValue);

        //             evaluator.setWireValue(
        //                 r.getArray(),
        //                 &Util::split(rValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //             evaluator.setWireValue(
        //                 q.getArray(),
        //                 &Util::split(qValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //         });
        let prover = crate::impl_prover!(
                        eval(  a: LongElement,
                    b: LongElement,r: LongElement,
                    q: LongElement
                )  {
        impl Instruction for Prover{
         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                   let aValue = evaluator.getWireValuei(&self.a, LongElement::CHUNK_BITWIDTH);
            let bValue = evaluator.getWireValuei(&self.b, LongElement::CHUNK_BITWIDTH);
            let rValue = aValue.clone().rem(&bValue);
            let qValue = aValue.clone().div(&bValue);

            evaluator.setWireValuea(
                self.r.getArray(),
                &Util::split(&rValue, LongElement::CHUNK_BITWIDTH),
            );
            evaluator.setWireValuea(
                self.q.getArray(),
                &Util::split(&qValue, LongElement::CHUNK_BITWIDTH),
            );


        }
        }
                    }
                );
        CircuitGenerator::specifyProverWitnessComputation(self.generator.clone(), prover);
        // {
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a, LongElement::CHUNK_BITWIDTH);
        //             let bValue = evaluator.getWireValue(b, LongElement::CHUNK_BITWIDTH);
        //             let rValue = aValue.rem(bValue);
        //             let qValue = aValue.div(bValue);

        //             evaluator.setWireValue(
        //                 r.getArray(),
        //                 Util::split(rValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //             evaluator.setWireValue(
        //                 q.getArray(),
        //                 Util::split(qValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //         }
        //     }
        //     Prover
        // });

        r.restrictBitwidth();
        q.restrictBitwidth(); //bits  16

        let res = q.clone().mul(b).add(&r);

        // implements the improved long integer equality assertion from xjsnark
        res.assertEquality(a);

        if self.t.restrictRange {
            r.assertLessThan(b);
        }
        self.t.r = r;
        self.t.q = q;
    }
}
pub trait LongIntegerDivisionConfig: GadgetConfig {
    fn getQuotient(&self) -> &LongElement;
    fn getRemainder(&self) -> &LongElement;
}

#[macro_export]
macro_rules! impl_long_integer_division_config_for {
    ($impl_type:ident) => {
        impl LongIntegerDivisionConfig for Gadget<LongIntegerDivision<$impl_type>> {
            fn getQuotient(&self) -> &LongElement {
                &self.t.q
            }
            fn getRemainder(&self) -> &LongElement {
                &self.t.r
            }
        }
    };
}
