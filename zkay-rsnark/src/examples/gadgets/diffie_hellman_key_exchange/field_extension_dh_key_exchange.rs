#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::{Gadget, GadgetConfig},
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

use rccell::RcCell;
use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Neg, Rem, Sub},
};
use zkay_derive::ImplStructNameConfig;

//  * Performs Key Exchange using a field extension F_p[x]/(x^\mu - \omega), where
//  * the polynomial (x^\mu - \omega) is irreducible. The inputs to this gadget:
//  * the base g, the other party's input h = g^a, the bits of the secret exponent
//  * sec_exp_bits and omega. The outputs of this gadget: the derived key h^s to be
//  * used for symmetric key derivation, and g^s which is sent to the other party.
//  *
//  * A sample parameterization that gives low security (~80 bits of security) can
//  * be found in the Junit tests. A sample usage is in:
//  * examples/generators/EncryptionCircuitGenerator.java

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct FieldExtensionDHKeyExchange {
    pub g: Vec<Option<WireType>>, // base
    pub h: Vec<Option<WireType>>, // other party's pub  input (supposedly, h = g^(the
    // other party's secret))
    pub secret_exponent_bits: Vec<Option<WireType>>, // the bits of the secret exponent of the
    // party
    // executing this gadget
    pub omega: i64,
    pub mu: i32,

    // gadget outputs
    pub output_public_value: Vec<Option<WireType>>, // g^s (to be sent to the other party)
    pub shared_secret: Vec<Option<WireType>>,       // the derived secret key h^s
    pub g_powers_table: Vec<Vec<Option<WireType>>>,
    pub h_powers_table: Vec<Vec<Option<WireType>>>,
    pub output: Vec<Option<WireType>>,
}
impl FieldExtensionDHKeyExchange {
    //Note: In the default mode, the gadget only validates the secret input
    //provided by the prover, but it does not validate that the base and pub
    //input of the other's party are proper elements. Since these values are
    //pub , they could be checked outside the circuit.
    //
    //If the validation is needed inside, the method "validate_inputs()" should
    //be called explicitly. Example is provided in
    //FieldExtensionDHKeyExchange_Test
    //

    pub fn new(
        g: Vec<Option<WireType>>,
        h: Vec<Option<WireType>>,
        secret_exponent_bits: Vec<Option<WireType>>,
        omega: i64,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(h.len() == g.len(), "g and h must have the same dimension");

        // since this is typically a  input by the prover,
        // the check is also done here for safety. No need to remove this if
        // done also outside the gadget. The back end takes care of caching
        let generators = generator.clone();
        for w in &secret_exponent_bits {
            CircuitGenerator::add_binary_assertion(generator.clone(), w.as_ref().unwrap(), &None);
        }
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                mu: g.len() as i32,
                g,
                h,
                secret_exponent_bits,
                omega,
                output_public_value: vec![],
                shared_secret: vec![],
                g_powers_table: vec![],
                h_powers_table: vec![],
                output: vec![],
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<FieldExtensionDHKeyExchange> {
    fn build_circuit(&mut self) {
        self.t.g_powers_table = self.preparePowersTable(&self.t.g);
        self.t.h_powers_table = self.preparePowersTable(&self.t.h);
        self.t.output_public_value = self.exp(
            &self.t.g,
            &self.t.secret_exponent_bits,
            &self.t.g_powers_table,
        );
        self.t.shared_secret = self.exp(
            &self.t.h,
            &self.t.secret_exponent_bits,
            &self.t.h_powers_table,
        );
        self.t.output = Util::concat(&self.t.output_public_value, &self.t.shared_secret);
    }

    fn mul(&self, a: &Vec<Option<WireType>>, b: &Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let mu = self.t.mu as usize;
        let zero_wire = self.generator.get_zero_wire().unwrap();
        let mut c = vec![zero_wire; mu];

        for i in 0..mu {
            for j in 0..mu {
                let mut k = i + j;
                if k < mu {
                    c[k] = c[k]
                        .clone()
                        .add(&a[i].clone().unwrap().mul(b[j].as_ref().unwrap()));
                }

                if i + j >= mu {
                    k = i + j - mu;
                    c[k] = c[k].clone().add(
                        &a[i]
                            .clone()
                            .unwrap()
                            .mul(b[j].as_ref().unwrap())
                            .muli(self.t.omega, &None),
                    );
                }
            }
        }
        c.into_iter().map(|x| Some(x)).collect()
    }

    fn preparePowersTable(&self, base: &Vec<Option<WireType>>) -> Vec<Vec<Option<WireType>>> {
        let mu = self.t.mu as usize;
        let mut powers_table = vec![vec![None; mu]; self.t.secret_exponent_bits.len() + 1];
        powers_table[0] = base[..mu].to_vec();
        for j in 1..self.t.secret_exponent_bits.len() + 1 {
            powers_table[j] = self.mul(&powers_table[j - 1], &powers_table[j - 1]);
        }
        powers_table
    }

    fn exp(
        &self,
        base: &Vec<Option<WireType>>,
        exp_bits: &Vec<Option<WireType>>,
        powers_table: &Vec<Vec<Option<WireType>>>,
    ) -> Vec<Option<WireType>> {
        let mut c = vec![self.generator.get_zero_wire(); self.t.mu as usize];
        c[0] = self.generator.get_one_wire();
        for j in 0..exp_bits.len() {
            let tmp = self.mul(&c, &powers_table[j]);
            for i in 0..self.t.mu as usize {
                c[i] = Some(
                    c[i].clone().unwrap().add(
                        exp_bits[j]
                            .clone()
                            .unwrap()
                            .mul(tmp[i].clone().unwrap().sub(c[i].as_ref().unwrap())),
                    ),
                );
            }
        }
        c
    }

    // TODO: Test more scenarios
    pub fn validate_inputs(&self, sub_group_order: BigInteger) {
        // g and h are not zero and not one

        // checking the first chunk
        let zero_or_one1 = self.t.g[0]
            .clone()
            .unwrap()
            .mul(self.t.g[0].clone().unwrap().sub(1));
        let zero_or_one2 = self.t.h[0]
            .clone()
            .unwrap()
            .mul(self.t.h[0].clone().unwrap().sub(1));

        // checking the rest
        let mut all_zero1 = self.generator.get_one_wire().unwrap();
        let mut all_zero2 = self.generator.get_one_wire().unwrap();

        for i in 1..self.t.mu as usize {
            all_zero1 = all_zero1.mul(
                self.t.g[i]
                    .as_ref()
                    .unwrap()
                    .check_non_zero(&None)
                    .inv_as_bit(&None)
                    .as_ref()
                    .unwrap(),
            );
            all_zero2 = all_zero2.mul(
                self.t.h[i]
                    .as_ref()
                    .unwrap()
                    .check_non_zero(&None)
                    .inv_as_bit(&None)
                    .as_ref()
                    .unwrap(),
            );
        }

        // assertion

        CircuitGenerator::add_zero_assertion(
            self.generator.clone(),
            &zero_or_one1.mul(all_zero1),
            &None,
        );

        CircuitGenerator::add_zero_assertion(
            self.generator.clone(),
            &zero_or_one2.mul(all_zero2),
            &None,
        );

        // verify order of points

        let bit_length = sub_group_order.bits();
        let bits: Vec<_> = (0..bit_length)
            .map(|i| {
                if sub_group_order.bit(i) {
                    self.generator.get_one_wire()
                } else {
                    self.generator.get_zero_wire()
                }
            })
            .collect();

        let result1 = self.exp(&self.t.g, &bits, &self.t.g_powers_table);
        let result2 = self.exp(&self.t.h, &bits, &self.t.h_powers_table);

        // both should be one

        CircuitGenerator::add_one_assertion(
            self.generator.clone(),
            result1[0].as_ref().unwrap(),
            &None,
        );

        CircuitGenerator::add_one_assertion(
            self.generator.clone(),
            result2[0].as_ref().unwrap(),
            &None,
        );
        for i in 1..self.t.mu as usize {
            CircuitGenerator::add_zero_assertion(
                self.generator.clone(),
                result1[i].as_ref().unwrap(),
                &None,
            );

            CircuitGenerator::add_zero_assertion(
                self.generator.clone(),
                result1[i].as_ref().unwrap(),
                &None,
            );
        }
    }
    pub fn get_output_public_value(&self) -> &Vec<Option<WireType>> {
        &self.t.output_public_value
    }

    pub fn get_shared_secret(&self) -> &Vec<Option<WireType>> {
        &self.t.shared_secret
    }
}
impl GadgetConfig for Gadget<FieldExtensionDHKeyExchange> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
