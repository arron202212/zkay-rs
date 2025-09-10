#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    bouncycastle_rs::pqc::math::linearalgebra::integer_functions::IntegerFunctions,
    circuit::{
        config::config::CONFIGS,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{circuit_generator::CircuitGenerator, wire::WireConfig, wire_type::WireType},
    },
    examples::gadgets::math::field_division_gadget::FieldDivisionGadget,
    util::util::{BigInteger, Util},
};

use std::ops::{Add, Mul, Rem, Sub};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, ImplStructNameConfig)]
pub struct AffinePoint {
    pub x: Option<WireType>,
    pub y: Option<WireType>,
}
impl AffinePoint {
    pub fn new(x: Option<WireType>, y: Option<WireType>) -> Self {
        Self { x, y }
    }
}

//  Constants and common functionality defined in jsnark's ECDHKeyExchangeGadget
#[derive(Debug, Clone)]
pub struct ZkayEcGadget<T> {
    pub t: T,
}

impl<T> ZkayEcGadget<T> {
    #[inline]
    pub fn new(t: T, generator: RcCell<CircuitGenerator>) -> Gadget<Self> {
        Self::new_with_option(&None, t, generator)
    }
    pub fn new_with_option(
        desc: &Option<String>,
        t: T,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        Gadget::<Self>::new(generator, desc, ZkayEcGadget::<T> { t })
    }
}

impl<T> Gadget<ZkayEcGadget<T>> {
    // Note: this parameterization assumes that the underlying field has
    // CONFIGS.field_prime =
    // 21888242871839275222246405745257275088548364400416034343698204186575808495617

    pub const SECRET_BITWIDTH: usize = 253; // number of bits in the
    // exponent. Note that the
    // most significant bit
    // should
    // be set to 1, and the
    // three least significant
    // bits should be be zero.
    // See
    // the constructor

    pub const COEFF_A: i32 = 126932; //BigInteger::parse_bytes(b"", 10).unwrap(); // parameterization
    // in
    // https://eprint.iacr.org/2015/1093.pdf

    pub const CURVE_ORDER: &str =
        "21888242871839275222246405745257275088597270486034011716802747351550446453784";

    // As in curve25519, CURVE_ORDER = SUBGROUP_ORDER * 2^3
    pub const SUBGROUP_ORDER: &str =
        "2736030358979909402780800718157159386074658810754251464600343418943805806723";
    pub fn check_secret_bits(
        generator: &RcCell<CircuitGenerator>,
        secret_bits: &Vec<Option<WireType>>,
    ) {
        //The secret key bits must be of length SECRET_BITWIDTH and are
        //expected to follow a little endian order. The most significant bit
        //should be 1, and the three least significant bits should be zero.

        let desc = "Asserting secret bit conditions";
        CircuitGenerator::add_zero_assertion_with_str(
            generator.clone(),
            secret_bits[0].as_ref().unwrap(),
            desc,
        );
        CircuitGenerator::add_zero_assertion_with_str(
            generator.clone(),
            secret_bits[1].as_ref().unwrap(),
            desc,
        );
        CircuitGenerator::add_zero_assertion_with_str(
            generator.clone(),
            secret_bits[2].as_ref().unwrap(),
            desc,
        );
        CircuitGenerator::add_one_assertion_with_str(
            generator.clone(),
            secret_bits[Self::SECRET_BITWIDTH as usize - 1]
                .as_ref()
                .unwrap(),
            desc,
        );

        for i in 3..Self::SECRET_BITWIDTH - 1 {
            // verifying all other bit wires are binary (as this is typically a
            // secret
            // witness by the prover)
            CircuitGenerator::add_binary_assertion(
                generator.clone(),
                secret_bits[i].as_ref().unwrap(),
            );
        }
    }

    // this is only called, when WireType y is provided as witness by the prover
    // (not as input to the gadget)
    pub fn assert_valid_point_on_ec(&self, x: &WireType, y: &WireType) {
        let y_sqr = y.clone().mul(y);
        let x_sqr = x.clone().mul(x);
        let x_cube = x_sqr.clone().mul(x);
        let f = x_cube
            .add(x_sqr.mul(&BigInteger::from(Self::COEFF_A)))
            .add(x);
        CircuitGenerator::add_equality_assertion(self.generator.clone(), &y_sqr, &f);
    }

    pub fn preprocess(p: &AffinePoint, generator: RcCell<CircuitGenerator>) -> Vec<AffinePoint> {
        let start = std::time::Instant::now();
        let mut precomputed_table: Vec<AffinePoint> = (1..Self::SECRET_BITWIDTH)
            .scan(p.clone(), |pre, _| {
                *pre = Self::double_affine_point(&pre, generator.clone());
                Some(pre.clone())
            })
            .collect();
        precomputed_table.insert(0, p.clone());
        precomputed_table
    }

    //Performs scalar multiplication (secret_bits must comply with the
    //conditions above)

    pub fn mul(
        p: &AffinePoint,
        secret_bits: &Vec<Option<WireType>>,
        precomputed_table: &Vec<AffinePoint>,
        generator: RcCell<CircuitGenerator>,
    ) -> AffinePoint {
        let mut result = precomputed_table[secret_bits.len() - 1].clone();
        for j in (0..=secret_bits.len() - 2).rev() {
            let tmp = Self::add_affine_points(&result, &precomputed_table[j], generator.clone());
            let is_one = secret_bits[j].clone().unwrap();
            result.x = result.x.clone().map(|x| {
                x.add(
                    is_one
                        .clone()
                        .mul(tmp.x.clone().unwrap().sub(result.x.as_ref().unwrap())),
                )
            });
            result.y = result.y.clone().map(|y| {
                y.add(
                    is_one
                        .clone()
                        .mul(tmp.y.clone().unwrap().sub(result.y.as_ref().unwrap())),
                )
            });
        }
        result
    }

    pub fn double_affine_point(
        p: &AffinePoint,
        generator: RcCell<CircuitGenerator>,
    ) -> AffinePoint {
        let start = std::time::Instant::now();
        let x_2 = p.x.clone().unwrap().mul(p.x.as_ref().unwrap());

        let l1 = FieldDivisionGadget::new(
            x_2.mul(3)
                .add(
                    p.x.clone()
                        .unwrap()
                        .mul(&BigInteger::from(Self::COEFF_A))
                        .mul(2),
                )
                .add(1),
            p.y.clone().unwrap().mul(2),
            generator,
        )
        .get_output_wires()[0]
            .clone()
            .unwrap();

        let l2 = l1.clone().mul(&l1);
        let new_x = l2
            .clone()
            .sub(&BigInteger::from(Self::COEFF_A))
            .sub(p.x.as_ref().unwrap())
            .sub(p.x.as_ref().unwrap());

        let new_y =
            p.x.clone()
                .unwrap()
                .mul(3)
                .add(&BigInteger::from(Self::COEFF_A))
                .sub(l2)
                .mul(l1)
                .sub(p.y.as_ref().unwrap());

        AffinePoint::new(Some(new_x), Some(new_y))
    }

    pub fn add_affine_points(
        p1: &AffinePoint,
        p2: &AffinePoint,
        generator: RcCell<CircuitGenerator>,
    ) -> AffinePoint {
        let diff_y = p1.y.clone().unwrap().sub(p2.y.as_ref().unwrap());
        let diff_x = p1.x.clone().unwrap().sub(p2.x.as_ref().unwrap());
        let q = FieldDivisionGadget::new(diff_y, diff_x, generator).get_output_wires()[0]
            .clone()
            .unwrap();
        let q2 = q.clone().mul(&q);
        let q3 = q2.clone().mul(&q);
        let new_x = q2
            .clone()
            .sub(&BigInteger::from(Self::COEFF_A))
            .sub(p1.x.as_ref().unwrap())
            .sub(p2.x.as_ref().unwrap());
        let new_y =
            p1.x.clone()
                .unwrap()
                .mul(2)
                .add(p2.x.as_ref().unwrap())
                .add(&BigInteger::from(Self::COEFF_A))
                .mul(&q)
                .sub(&q3)
                .sub(p1.y.as_ref().unwrap());
        AffinePoint::new(Some(new_x), Some(new_y))
    }

    pub fn compute_y_coordinate(x: BigInteger) -> BigInteger {
        let x_sqred = x.clone().mul(&x).rem(&CONFIGS.field_prime);
        let x_cubed = x_sqred.clone().mul(&x).rem(&CONFIGS.field_prime);
        let y_sqred = x_cubed
            .add(BigInteger::from(Self::COEFF_A).mul(&x_sqred))
            .add(&x)
            .rem(&CONFIGS.field_prime);
        let y = IntegerFunctions::ressol(y_sqred, &CONFIGS.field_prime); //MYTODO
        y
    }

    pub fn assert_point_order(&self, p: &AffinePoint, table: &Vec<AffinePoint>) {
        // let generator = &self.generators;
        let o = &CircuitGenerator::create_constant_wire(
            self.generator.clone(),
            &Util::parse_big_int(Self::SUBGROUP_ORDER),
        );
        let bits = o
            .get_bit_wiresi(Util::parse_big_int(Self::SUBGROUP_ORDER).bits())
            .as_array()
            .clone();

        let mut result = table[bits.len() - 1].clone();
        for j in (1..=bits.len() - 2).rev() {
            let tmp = Self::add_affine_points(&result, &table[j], self.generator.clone());
            let is_one = bits[j].clone().unwrap();
            result.x = result.x.clone().map(|x| {
                x.add(
                    is_one
                        .clone()
                        .mul(tmp.x.clone().unwrap().sub(result.x.as_ref().unwrap())),
                )
            });
            result.y = result.y.clone().map(|y| {
                y.add(
                    is_one
                        .clone()
                        .mul(tmp.y.clone().unwrap().sub(result.y.as_ref().unwrap())),
                )
            });
        }

        // verify that: result = -p
        CircuitGenerator::add_equality_assertion(
            self.generator.clone(),
            result.x.as_ref().unwrap(),
            p.x.as_ref().unwrap(),
        );
        CircuitGenerator::add_equality_assertion(
            self.generator.clone(),
            result.y.as_ref().unwrap(),
            &p.y.clone().unwrap().mul(-1),
        );

        // the reason the last iteration is handled separately is that the
        // addition of
        // affine points will throw an error due to not finding inverse for zero
        // at the last iteration of the scalar multiplication. So, the check in
        // the last iteration is done manually

        // TODO: add more tests to check this method
    }
}
// impl GadgetConfig for Gadget<ZkayEcGadget> {
// }
