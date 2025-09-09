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
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            wire::WireConfig,
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};

use rccell::RcCell;
use std::ops::{Add, Mul, Sub};
use zkay_derive::ImplStructNameConfig;

#[derive(Debug, Clone)]
pub struct JubJubPoint {
    pub x: WireType,
    pub y: WireType,
}
impl JubJubPoint {
    pub fn new(x: WireType, y: WireType) -> Self {
        Self { x, y }
    }
}

//  * Gadget for operations on the BabyJubJub elliptic curve (Twisted Edwards curve over BN254).
//  * Parameters are from:
//  * https://iden3-docs.readthedocs.io/en/latest/iden3_repos/research/publications/zkproof-standards-workshop-2/baby-jubjub/baby-jubjub.html

#[derive(Debug, Clone)]
pub struct ZkayBabyJubJubGadget<T> {
    pub t: T,
}

impl<T> ZkayBabyJubJubGadget<T> {
    pub fn new(t: T, generator: RcCell<CircuitGenerator>) -> Gadget<Self> {
        Self::new_with_option(&None, t, generator)
    }
    pub fn new_with_option(
        desc: &Option<String>,
        t: T,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        // We assume the underlying field matches the base field of BabyJubJub (so that we can avoid alignment/modulus)
        assert_eq!(
            CONFIGS.field_prime.to_str_radix(10),
            "21888242871839275222246405745257275088548364400416034343698204186575808495617"
        );
        Gadget::<Self>::new(generator, desc, ZkayBabyJubJubGadget::<T> { t })
    }
}

pub trait ZkayBabyJubJubGadgetConfig {
    const BASE_ORDER: &str =
        "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    const CURVE_ORDER: &str =
        "2736030358979909402780800718157159386076813972158567259200215660948447373041";
    const COFACTOR: u8 = 8;
    const COEFF_A: u8 = 1;
    const COEFF_D: &str =
        "9706598848417545097372247223557719406784115219466060233080913168975159366771";
    // arbitrary generator
    const GENERATOR_X: &str =
        "11904062828411472290643689191857696496057424932476499415469791423656658550213";
    const GENERATOR_Y: &str =
        "9356450144216313082194365820021861619676443907964402770398322487858544118183";
    fn generators(&self) -> RcCell<CircuitGenerator>;
    // {
    //     &self.generators
    // }
    fn get_infinity(&self) -> JubJubPoint {
        JubJubPoint::new(
            self.generators().get_zero_wire().unwrap(),
            self.generators().get_one_wire().unwrap(),
        )
    }

    fn get_generator(&self) -> JubJubPoint {
        let g_x = CircuitGenerator::create_constant_wire(
            self.generators(),
            &Util::parse_big_int(Self::GENERATOR_X),
        );
        let g_y = CircuitGenerator::create_constant_wire(
            self.generators(),
            &Util::parse_big_int(Self::GENERATOR_Y),
        );
        JubJubPoint::new(g_x, g_y)
    }

    fn assert_on_curve(&self, x: &WireType, y: &WireType) {
        // assert COEFF_A*x*x + y*y == 1 + COEFF_D*x*x*y*y
        let x_sqr = x.clone().mul(x);
        let y_sqr = y.clone().mul(y);
        let prod = x_sqr.clone().mul(&y_sqr);
        let lhs = x_sqr.mul(&BigInteger::from(Self::COEFF_A)).add(y_sqr);
        let rhs = prod.mul(&Util::parse_big_int(Self::COEFF_D)).add(1);
        CircuitGenerator::add_equality_assertion(self.generators(), &lhs, &rhs);
    }

    fn add_points(&self, p1: &JubJubPoint, p2: &JubJubPoint) -> JubJubPoint {
        // Twisted Edwards addition according to https://en.wikipedia.org/wiki/Twisted_Edwards_curve#Addition_on_twisted_Edwards_curves

        let a1 = p1.x.clone().mul(&p2.y).add(p1.y.clone().mul(&p2.x));
        let a2 =
            p1.x.clone()
                .mul(&p2.x)
                .mul(p1.y.clone().mul(&p2.y))
                .mul(&Util::parse_big_int(Self::COEFF_D))
                .add(1);
        let b1 = p1.y.clone().mul(&p2.y).sub(
            p1.x.clone()
                .mul(&p2.x)
                .mul(&BigInteger::from(Self::COEFF_A)),
        );
        let b2 =
            p1.x.clone()
                .mul(&p2.x)
                .mul(p1.y.clone().mul(&p2.y))
                .mul(&Util::parse_big_int(Self::COEFF_D))
                .negate()
                .add(1);

        let x = a1.clone().mul(self.native_inverse(&a2));
        let y = b1.clone().mul(self.native_inverse(&b2));
        JubJubPoint::new(x, y)
    }

    fn negate_point(p: &JubJubPoint) -> JubJubPoint {
        let new_x = p.x.negate();
        JubJubPoint::new(new_x, p.y.clone())
    }

    //@param scalar_bits the scalar bit representation in little-endian order

    fn mul_scalar(&self, p: &JubJubPoint, scalar_bits: &Vec<Option<WireType>>) -> JubJubPoint {
        // Scalar point multiplication using double-and-add algorithm
        let mut result = self.get_infinity();
        let mut doubling = p.clone();

        for i in 0..scalar_bits.len() {
            let q = self.add_points(&doubling, &result);
            let new_x = scalar_bits[i].as_ref().unwrap().mux(&q.x, &result.x);
            let new_y = scalar_bits[i].as_ref().unwrap().mux(&q.y, &result.y);
            result = JubJubPoint::new(new_x, new_y);
            doubling = self.add_points(&doubling, &doubling);
        }

        result
    }

    //Returns a wire holding the inverse of a in the native base field.

    fn native_inverse(&self, a: &WireType) -> WireType {
        let ainv = CircuitGenerator::create_prover_witness_wire(self.generators());

        let base_order = Self::BASE_ORDER.to_owned();
        let prover = crate::impl_prover!(
                                eval(  a: WireType,
                                    ainv:WireType,
                                    base_order:String
                        )  {
                impl Instruction for Prover{
                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                    let a_value = evaluator.get_wire_value(&self.a);
                    let inverse_value = a_value.modinv(&Util::parse_big_int(&self.base_order));
                    evaluator.set_wire_value(&self.ainv, inverse_value.as_ref().unwrap());
        Ok(())
                }
                }
                            }
                        );
        CircuitGenerator::specify_prover_witness_computation(self.generators(), prover);

        // check if a * ainv = 1 (natively)
        let test = a.clone().mul(&ainv);
        CircuitGenerator::add_equality_assertion(
            self.generators(),
            &test,
            self.generators().get_one_wire().as_ref().unwrap(),
        );

        ainv
    }
}

impl<T> ZkayBabyJubJubGadgetConfig for Gadget<ZkayBabyJubJubGadget<T>> {
    fn generators(&self) -> RcCell<CircuitGenerator> {
        self.generator.clone()
    }
}

// impl<T> GadgetConfig for Gadget<ZkayBabyJubJubGadget<T>> {
// }
