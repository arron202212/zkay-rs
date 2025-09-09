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
    ops::{Add, Div, Mul, Sub},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

//  * This gadget computes the modular multiplicative inverse a^(-1) mod m,
//  * where a and m are LongElements.
//  * If restrict_range is set to true, the output will be the sole inverse a^(-1)
//  * for which a < m holds. If restrict_range is false, the inverse may be any
//  * value x for which ax = 1 mod m holds.
//  * It is the responsibility of the caller to ensure that a and m are
//  * relatively co-prime, i.e. the modular inverse actually exists.

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerModInverseGadget {
    pub a: LongElement,       // the value to be inverted
    pub m: LongElement,       // the modulus
    pub restrict_range: bool, // whether to enforce that a^(-1) < m
    pub inverse: Option<LongElement>,
}
impl LongIntegerModInverseGadget {
    #[inline]
    pub fn new(
        a: LongElement,
        m: LongElement,
        restrict_range: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        Self::new_with_option(a, m, restrict_range, &None, generator)
    }
    pub fn new_with_option(
        a: LongElement,
        m: LongElement,
        restrict_range: bool,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                inverse: None,
                a,
                m,
                restrict_range,
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<LongIntegerModInverseGadget> {
    fn build_circuit(&mut self) {
        let inverse_wires = CircuitGenerator::create_prover_witness_wire_array(
            self.generator.clone(),
            self.t.m.get_size(),
        );
        let inverse = LongElement::new(
            inverse_wires.clone(),
            self.t.m.get_current_bitwidth(),
            self.generator.clone().downgrade(),
        );
        let quotient_wires = CircuitGenerator::create_prover_witness_wire_array(
            self.generator.clone(),
            self.t.m.get_size(),
        );
        let quotient = LongElement::new(
            quotient_wires.clone(),
            self.t.m.get_current_bitwidth(),
            self.generator.clone().downgrade(),
        );
        let a = &self.t.a;
        let m = &self.t.m;

        let prover = crate::impl_prover!(
                                eval( a: LongElement, m: LongElement,  quotient_wires:  Vec<Option<WireType>>,
                            inverse_wires:  Vec<Option<WireType>>
                        )  {
                impl Instruction for Prover{
                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                           let a_value = evaluator.get_wire_valuei(&self.a, LongElement::CHUNK_BITWIDTH);
                    let m_value = evaluator.get_wire_valuei(&self.m, LongElement::CHUNK_BITWIDTH);
                    let inverse_value = a_value.modinv(&m_value);
                    let quotient_value = a_value.mul(inverse_value.as_ref().unwrap()).div(&m_value);

                    evaluator.set_wire_valuea(
                        &self.inverse_wires,
                        &Util::split(inverse_value.as_ref().unwrap(), LongElement::CHUNK_BITWIDTH),
                    );
                    evaluator.set_wire_valuea(
                        &self.quotient_wires,
                        &Util::split(&quotient_value, LongElement::CHUNK_BITWIDTH),
                    );
        Ok(())

                }
                }
                            }
                        );
        CircuitGenerator::specify_prover_witness_computation(self.generator.clone(), prover);

        inverse.restrict_bitwidth();
        quotient.restrict_bitwidth();
        // a * a^(-1) = 1   (mod m)
        // <=> Exist q:  a * a^(-1) = q * m + 1
        let product = a.clone().mul(&inverse);
        let one_mod_m = quotient.mul(m).add(1);
        product.assert_equality(&one_mod_m);

        if self.t.restrict_range {
            inverse.assert_less_than(m);
        }
        self.t.inverse = Some(inverse);
    }

    pub fn get_result(&self) -> &LongElement {
        self.t.inverse.as_ref().unwrap()
    }
}
impl GadgetConfig for Gadget<LongIntegerModInverseGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.t.inverse.as_ref().unwrap().get_array()
    }
}
