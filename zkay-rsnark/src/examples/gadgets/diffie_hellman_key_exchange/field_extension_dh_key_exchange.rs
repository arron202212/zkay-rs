#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::GadgetConfig;
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
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
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{BigInteger, Util};
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
use zkay_derive::ImplStructNameConfig;

//  * Performs Key Exchange using a field extension F_p[x]/(x^\mu - \omega), where
//  * the polynomial (x^\mu - \omega) is irreducible. The inputs to this gadget:
//  * the base g, the other party's input h = g^a, the bits of the secret exponent
//  * secExpBits and omega. The outputs of this gadget: the derived key h^s to be
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
    pub secretExponentBits: Vec<Option<WireType>>, // the bits of the secret exponent of the
    // party
    // executing this gadget
    pub omega: i64,
    pub mu: i32,

    // gadget outputs
    pub outputPublicValue: Vec<Option<WireType>>, // g^s (to be sent to the other party)
    pub sharedSecret: Vec<Option<WireType>>,      // the derived secret key h^s
    pub gPowersTable: Vec<Vec<Option<WireType>>>,
    pub hPowersTable: Vec<Vec<Option<WireType>>>,
    pub output: Vec<Option<WireType>>,
}
impl FieldExtensionDHKeyExchange {
    //Note: In the default mode, the gadget only validates the secret input
    //provided by the prover, but it does not validate that the base and pub
    //input of the other's party are proper elements. Since these values are
    //pub , they could be checked outside the circuit.
    //
    //If the validation is needed inside, the method "validateInputs()" should
    //be called explicitly. Example is provided in
    //FieldExtensionDHKeyExchange_Test
    //

    pub fn new(
        g: Vec<Option<WireType>>,
        h: Vec<Option<WireType>>,
        secretExponentBits: Vec<Option<WireType>>,
        omega: i64,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(h.len() == g.len(), "g and h must have the same dimension");

        // since this is typically a  input by the prover,
        // the check is also done here for safety. No need to remove this if
        // done also outside the gadget. The back end takes care of caching
        let generators = generator.clone();
        for w in &secretExponentBits {
            CircuitGenerator::add_binary_assertion(generator.clone(), w.as_ref().unwrap(), &None);
        }
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                mu: g.len() as i32,
                g,
                h,
                secretExponentBits,
                omega,
                outputPublicValue: vec![],
                sharedSecret: vec![],
                gPowersTable: vec![],
                hPowersTable: vec![],
                output: vec![],
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<FieldExtensionDHKeyExchange> {
    fn build_circuit(&mut self) {
        self.t.gPowersTable = self.preparePowersTable(&self.t.g);
        self.t.hPowersTable = self.preparePowersTable(&self.t.h);
        self.t.outputPublicValue =
            self.exp(&self.t.g, &self.t.secretExponentBits, &self.t.gPowersTable);
        self.t.sharedSecret = self.exp(&self.t.h, &self.t.secretExponentBits, &self.t.hPowersTable);
        self.t.output = Util::concat(&self.t.outputPublicValue, &self.t.sharedSecret);
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
        let mut powersTable = vec![vec![None; mu]; self.t.secretExponentBits.len() + 1];
        powersTable[0] = base[..mu].to_vec();
        for j in 1..self.t.secretExponentBits.len() + 1 {
            powersTable[j] = self.mul(&powersTable[j - 1], &powersTable[j - 1]);
        }
        powersTable
    }

    fn exp(
        &self,
        base: &Vec<Option<WireType>>,
        expBits: &Vec<Option<WireType>>,
        powersTable: &Vec<Vec<Option<WireType>>>,
    ) -> Vec<Option<WireType>> {
        let mut c = vec![self.generator.get_zero_wire(); self.t.mu as usize];
        c[0] = self.generator.get_one_wire();
        for j in 0..expBits.len() {
            let tmp = self.mul(&c, &powersTable[j]);
            for i in 0..self.t.mu as usize {
                c[i] = Some(
                    c[i].clone().unwrap().add(
                        expBits[j]
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
    pub fn validateInputs(&self, subGroupOrder: BigInteger) {
        // g and h are not zero and not one

        // checking the first chunk
        let zeroOrOne1 = self.t.g[0]
            .clone()
            .unwrap()
            .mul(self.t.g[0].clone().unwrap().sub(1));
        let zeroOrOne2 = self.t.h[0]
            .clone()
            .unwrap()
            .mul(self.t.h[0].clone().unwrap().sub(1));

        // checking the rest
        let mut allZero1 = self.generator.get_one_wire().unwrap();
        let mut allZero2 = self.generator.get_one_wire().unwrap();

        for i in 1..self.t.mu as usize {
            allZero1 = allZero1.mul(
                self.t.g[i]
                    .as_ref()
                    .unwrap()
                    .check_non_zero(&None)
                    .inv_as_bit(&None)
                    .as_ref()
                    .unwrap(),
            );
            allZero2 = allZero2.mul(
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
            &zeroOrOne1.mul(allZero1),
            &None,
        );

        CircuitGenerator::add_zero_assertion(
            self.generator.clone(),
            &zeroOrOne2.mul(allZero2),
            &None,
        );

        // verify order of points

        let bitLength = subGroupOrder.bits();
        let bits: Vec<_> = (0..bitLength)
            .map(|i| {
                if subGroupOrder.bit(i) {
                    self.generator.get_one_wire()
                } else {
                    self.generator.get_zero_wire()
                }
            })
            .collect();

        let result1 = self.exp(&self.t.g, &bits, &self.t.gPowersTable);
        let result2 = self.exp(&self.t.h, &bits, &self.t.hPowersTable);

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
    pub fn getOutputPublicValue(&self) -> &Vec<Option<WireType>> {
        &self.t.outputPublicValue
    }

    pub fn getSharedSecret(&self) -> &Vec<Option<WireType>> {
        &self.t.sharedSecret
    }
}
impl GadgetConfig for Gadget<FieldExtensionDHKeyExchange> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
