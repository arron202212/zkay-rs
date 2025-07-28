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
                assert_basic_op::{AssertBasicOp, new_assert},
                basic_op::BasicOp,
                mul_basic_op::{MulBasicOp, new_mul},
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::{ConstantWire, new_constant},
            variable_bit_wire::VariableBitWire,
            variable_wire::{VariableWire, new_variable},
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
use std::ops::{Add, Div, Mul, Sub};
use zkay_derive::ImplStructNameConfig;
/**
 * This gadget computes the modular multiplicative inverse a^(-1) mod m,
 * where a and m are LongElements.
 * If restrictRange is set to true, the output will be the sole inverse a^(-1)
 * for which a < m holds. If restrictRange is false, the inverse may be any
 * value x for which ax = 1 mod m holds.
 * It is the responsibility of the caller to ensure that a and m are
 * relatively co-prime, i.e. the modular inverse actually exists.
 */
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerModInverseGadget {
    a: LongElement,      // the value to be inverted
    m: LongElement,      // the modulus
    restrictRange: bool, // whether to enforce that a^(-1) < m
    inverse: LongElement,
}
impl LongIntegerModInverseGadget {
    pub fn new(
        a: LongElement,
        m: LongElement,
        restrictRange: bool,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                inverse: a.clone(),
                a,
                m,
                restrictRange,
            },
        };

        _self.buildCircuit();
        _self
    }
}
impl Gadget<LongIntegerModInverseGadget> {
    fn buildCircuit(&mut self) {
        let inverseWires = self
            .generator
            .createProverWitnessWireArray(self.t.m.getSize(), &None);
        let inverse = LongElement::new(
            inverseWires.clone(),
            self.t.m.getCurrentBitwidth(),
            self.generator.clone().downgrade(),
        );
        let quotientWires = self
            .generator
            .createProverWitnessWireArray(self.t.m.getSize(), &None);
        let quotient = LongElement::new(
            quotientWires.clone(),
            self.t.m.getCurrentBitwidth(),
            self.generator.clone().downgrade(),
        );
        let a = &self.t.a;
        let m = &self.t.m;
        // generator.specifyProverWitnessComputation(&|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a, LongElement::CHUNK_BITWIDTH);
        //             let mValue = evaluator.getWireValue(m, LongElement::CHUNK_BITWIDTH);
        //             let inverseValue = aValue.modInverse(mValue);
        //             let quotientValue = aValue.mul(inverseValue).divide(mValue);

        //             evaluator.setWireValue(
        //                 inverseWires,
        //                 &Util::split(inverseValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //             evaluator.setWireValue(
        //                 quotientWires,
        //                 &Util::split(quotientValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //         });

        let prover = crate::impl_prover!(
                        eval( a: LongElement, m: LongElement,  quotientWires:  Vec<Option<WireType>>,
                    inverseWires:  Vec<Option<WireType>>
                )  {
        impl Instruction for Prover{
         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                   let aValue = evaluator.getWireValuei(&self.a, LongElement::CHUNK_BITWIDTH);
            let mValue = evaluator.getWireValuei(&self.m, LongElement::CHUNK_BITWIDTH);
            let inverseValue = aValue.modinv(&mValue);
            let quotientValue = aValue.mul(inverseValue.as_ref().unwrap()).div(&mValue);

            evaluator.setWireValuea(
                &self.inverseWires,
                &Util::split(inverseValue.as_ref().unwrap(), LongElement::CHUNK_BITWIDTH),
            );
            evaluator.setWireValuea(
                &self.quotientWires,
                &Util::split(&quotientValue, LongElement::CHUNK_BITWIDTH),
            );


        }
        }
                    }
                );
        self.generator.specifyProverWitnessComputation(prover);
        // {
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a, LongElement::CHUNK_BITWIDTH);
        //             let mValue = evaluator.getWireValue(m, LongElement::CHUNK_BITWIDTH);
        //             let inverseValue = aValue.modInverse(mValue);
        //             let quotientValue = aValue.mul(inverseValue).divide(mValue);

        //             evaluator.setWireValue(
        //                 inverseWires,
        //                 Util::split(inverseValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //             evaluator.setWireValue(
        //                 quotientWires,
        //                 Util::split(quotientValue, LongElement::CHUNK_BITWIDTH),
        //             );
        //         }
        //     }
        //     Prover
        // });

        inverse.restrictBitwidth();
        quotient.restrictBitwidth();

        // a * a^(-1) = 1   (mod m)
        // <=> Exist q:  a * a^(-1) = q * m + 1
        let product = a.clone().mul(&inverse);
        let oneModM = quotient.mul(m).add(1);
        product.assertEquality(&oneModM);

        if self.t.restrictRange {
            inverse.assertLessThan(m);
        }
        self.t.inverse = inverse;
    }

    pub fn getResult(&self) -> &LongElement {
        &self.t.inverse
    }
}
impl GadgetConfig for Gadget<LongIntegerModInverseGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.t.inverse.getArray()
    }
}
