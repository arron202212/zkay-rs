#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::config::config::Configs;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::field_division_gadget::FieldDivisionGadget;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;

// use crate::zkay::zkay_ec_gadget::AffinePoint;
use crate::util::util::BigInteger;

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

/** Constants and common functionality defined in jsnark's ECDHKeyExchangeGadget */
#[derive(Debug, Clone)]
pub struct ZkayEcGadget<T> {
    pub generators: CircuitGenerator,
    pub t: T,
}

impl<T> ZkayEcGadget<T> {
    pub fn new(desc: &Option<String>, t: T, generator: RcCell<CircuitGenerator>) -> Gadget<Self> {
        let generators = generator.borrow().clone();
        Gadget::<Self> {
            generator,
            description: desc.clone().unwrap_or(String::new()),
            t: ZkayEcGadget::<T> { generators, t },
        }
    }
}

impl<T> Gadget<ZkayEcGadget<T>> {
    // Note: this parameterization assumes that the underlying field has
    // Configs.field_prime =
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
    pub fn checkSecretBits(
        generator: &RcCell<CircuitGenerator>,
        secretBits: &Vec<Option<WireType>>,
    ) {
        /**
         * The secret key bits must be of length SECRET_BITWIDTH and are
         * expected to follow a little endian order. The most significant bit
         * should be 1, and the three least significant bits should be zero.
         */
        let desc = Some("Asserting secret bit conditions".to_owned());
        generator.addZeroAssertion(secretBits[0].as_ref().unwrap(), &desc);
        generator.addZeroAssertion(secretBits[1].as_ref().unwrap(), &desc);
        generator.addZeroAssertion(secretBits[2].as_ref().unwrap(), &desc);
        generator.addOneAssertion(
            secretBits[Self::SECRET_BITWIDTH as usize - 1]
                .as_ref()
                .unwrap(),
            &desc,
        );

        for i in 3..Self::SECRET_BITWIDTH - 1 {
            // verifying all other bit wires are binary (as this is typically a
            // secret
            // witness by the prover)
            generator.addBinaryAssertion(secretBits[i].as_ref().unwrap(), &None);
        }
    }

    // this is only called, when WireType y is provided as witness by the prover
    // (not as input to the gadget)
    pub fn assertValidPointOnEC(&self, x: &WireType, y: &WireType) {
        let ySqr = y.clone().mul(y);
        let xSqr = x.clone().mul(x);
        let xCube = xSqr.clone().mul(x);
        self.generator.addEqualityAssertion(
            &ySqr,
            &xCube.add(xSqr.mul(&BigInteger::from(Self::COEFF_A))).add(x),
            &None,
        );
    }

    pub fn preprocess(p: &AffinePoint, generator: RcCell<CircuitGenerator>) -> Vec<AffinePoint> {
        let mut precomputedTable: Vec<AffinePoint> = (1..Self::SECRET_BITWIDTH)
            .scan(p.clone(), |pre, _| {
                *pre = Self::doubleAffinePoint(&pre, generator.clone());
                Some(pre.clone())
            })
            .collect();
        precomputedTable.insert(0, p.clone());
        precomputedTable
    }

    /**
     * Performs scalar multiplication (secretBits must comply with the
     * conditions above)
     */
    pub fn mul(
        p: &AffinePoint,
        secretBits: &Vec<Option<WireType>>,
        precomputedTable: &Vec<AffinePoint>,
        generator: RcCell<CircuitGenerator>,
    ) -> AffinePoint {
        let mut result = precomputedTable[secretBits.len() - 1].clone();
        for j in (0..=secretBits.len() - 2).rev() {
            let tmp = Self::addAffinePoints(&result, &precomputedTable[j], generator.clone());
            let isOne = secretBits[j].clone().unwrap();
            result.x = result.x.clone().map(|x| {
                x.add(
                    isOne
                        .clone()
                        .mul(tmp.x.clone().unwrap().sub(result.x.as_ref().unwrap())),
                )
            });
            result.y = result.y.clone().map(|y| {
                y.add(
                    isOne
                        .clone()
                        .mul(tmp.y.clone().unwrap().sub(result.y.as_ref().unwrap())),
                )
            });
        }
        result
    }

    pub fn doubleAffinePoint(p: &AffinePoint, generator: RcCell<CircuitGenerator>) -> AffinePoint {
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
            &None,
            generator,
        )
        .getOutputWires()[0]
            .clone()
            .unwrap();
        let l2 = l1.clone().mul(&l1);
        let newX = l2
            .clone()
            .sub(&BigInteger::from(Self::COEFF_A))
            .sub(p.x.as_ref().unwrap())
            .sub(p.x.as_ref().unwrap());
        let newY =
            p.x.clone()
                .unwrap()
                .mul(3)
                .add(&BigInteger::from(Self::COEFF_A))
                .sub(l2)
                .mul(l1)
                .sub(p.y.as_ref().unwrap());
        AffinePoint::new(Some(newX), Some(newY))
    }

    pub fn addAffinePoints(
        p1: &AffinePoint,
        p2: &AffinePoint,
        generator: RcCell<CircuitGenerator>,
    ) -> AffinePoint {
        let diffY = p1.y.clone().unwrap().sub(p2.y.as_ref().unwrap());
        let diffX = p1.x.clone().unwrap().sub(p2.x.as_ref().unwrap());
        let q = FieldDivisionGadget::new(diffY, diffX, &None, generator).getOutputWires()[0]
            .clone()
            .unwrap();
        let q2 = q.clone().mul(&q);
        let q3 = q2.clone().mul(&q);
        let newX = q2
            .clone()
            .sub(&BigInteger::from(Self::COEFF_A))
            .sub(p1.x.as_ref().unwrap())
            .sub(p2.x.as_ref().unwrap());
        let newY =
            p1.x.clone()
                .unwrap()
                .mul(2)
                .add(p2.x.as_ref().unwrap())
                .add(&BigInteger::from(Self::COEFF_A))
                .mul(&q)
                .sub(&q3)
                .sub(p1.y.as_ref().unwrap());
        AffinePoint::new(Some(newX), Some(newY))
    }

    pub fn computeYCoordinate(x: BigInteger) -> BigInteger {
        let xSqred = x.clone().mul(&x).rem(&Configs.field_prime);
        let xCubed = xSqred.clone().mul(&x).rem(&Configs.field_prime);
        let ySqred = xCubed
            .add(BigInteger::from(Self::COEFF_A).mul(&xSqred))
            .add(&x)
            .rem(&Configs.field_prime);
        let y = x; //IntegerFunctions.ressol(ySqred, Configs.field_prime);
        y
    }

    pub fn assertPointOrder(&self, p: &AffinePoint, table: &Vec<AffinePoint>) {
        let generator = &self.t.generators;
        let o = &generator.createConstantWire(
            &BigInteger::parse_bytes(Self::SUBGROUP_ORDER.as_bytes(), 10).unwrap(),
            &None,
        );
        let bits = o
            .getBitWiresi(
                BigInteger::parse_bytes(Self::SUBGROUP_ORDER.as_bytes(), 10)
                    .unwrap()
                    .bits(),
                &None,
            )
            .asArray()
            .clone();

        let mut result = table[bits.len() - 1].clone();
        for j in (1..=bits.len() - 2).rev() {
            let tmp = Self::addAffinePoints(&result, &table[j], self.generator.clone());
            let isOne = bits[j].clone().unwrap();
            result.x = result.x.clone().map(|x| {
                x.add(
                    isOne
                        .clone()
                        .mul(tmp.x.clone().unwrap().sub(result.x.as_ref().unwrap())),
                )
            });
            result.y = result.y.clone().map(|y| {
                y.add(
                    isOne
                        .clone()
                        .mul(tmp.y.clone().unwrap().sub(result.y.as_ref().unwrap())),
                )
            });
        }

        // verify that: result = -p
        generator.addEqualityAssertion(result.x.as_ref().unwrap(), p.x.as_ref().unwrap(), &None);
        generator.addEqualityAssertion(
            result.y.as_ref().unwrap(),
            &p.y.clone().unwrap().mul(-1),
            &None,
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
