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
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::field_division_gadget;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
use crate::zkay::zkay_ec_gadget::field_division_gadget::FieldDivisionGadget;
// use crate::zkay::zkay_ec_gadget::AffinePoint;
use crate::util::util::BigInteger;
use rccell::RcCell;
pub struct AffinePoint {
    x: WireType,
    y: WireType,
}
// impl AffinePoint {
//     // AffinePoint(x:WireType ) {
//     //     self.x = x;
//     // }

//     // AffinePoint(x:WireType , y:WireType ) {
//     //     self.x = x;
//     //     self.y = y;
//     // }

//     // AffinePoint(p:AffinePoint ) {
//     //     self.x = p.x;
//     //     self.y = p.y;
//     // }
// }

/** Constants and common functionality defined in jsnark's ECDHKeyExchangeGadget */
pub struct ZkayEcGadget<T> {
    generators: CircuitGenerator,
    t: T,
}

impl<T> ZkayEcGadget<T> {
    pub fn new(desc: &Option<String>, t: T, generator: RcCell<CircuitGenerator>) -> Gadget<Self> {
        let generators = generator.borrow().clone();
        Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: ZkayBabyJubJubGadget::<T> { generators, t },
        }
    }

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

    pub fn checkSecretBits(generator: &CircuitGenerator, secretBits: &Vec<Option<WireType>>) {
        /**
         * The secret key bits must be of length SECRET_BITWIDTH and are
         * expected to follow a little endian order. The most significant bit
         * should be 1, and the three least significant bits should be zero.
         */
        let desc = Some("Asserting secret bit conditions".to_owned());
        generator.addZeroAssertion(&secretBits[0], &desc);
        generator.addZeroAssertion(&secretBits[1], &desc);
        generator.addZeroAssertion(&secretBits[2], &desc);
        generator.addOneAssertion(&secretBits[Self::SECRET_BITWIDTH - 1], &desc);

        for i in 3..Self::SECRET_BITWIDTH - 1 {
            // verifying all other bit wires are binary (as this is typically a
            // secret
            // witness by the prover)
            generator.addBinaryAssertion(&secretBits[i]);
        }
    }

    // this is only called, when WireType y is provided as witness by the prover
    // (not as input to the gadget)
    fn assertValidPointOnEC(&self, x: &WireType, y: &WireType) {
        let ySqr = y.mul(y);
        let xSqr = x.mul(x);
        let xCube = xSqr.mul(x);
        self.generators.addEqualityAssertion(
            ySqr,
            xCube.add(xSqr.mul(BigInteger::from(Self::COEFF_A))).add(x),
        );
    }

    fn preprocess(p: AffinePoint) -> Vec<AffinePoint> {
        let precomputedTable = vec![AffinePoint::default(); Self::SECRET_BITWIDTH];
        precomputedTable[0] = p;
        for j in 1..Self::SECRET_BITWIDTH {
            precomputedTable[j] = doubleAffinePoint(precomputedTable[j - 1]);
        }
        precomputedTable
    }

    /**
     * Performs scalar multiplication (secretBits must comply with the
     * conditions above)
     */
    fn mul(
        p: AffinePoint,
        secretBits: Vec<Option<WireType>>,
        precomputedTable: Vec<AffinePoint>,
    ) -> AffinePoint {
        let result = AffinePoint::new(precomputedTable[secretBits.len() - 1]);
        for j in (0..=secretBits.len() - 2).rev() {
            let tmp = addAffinePoints(result, precomputedTable[j]);
            let isOne = secretBits[j];
            result.x = result.x.add(isOne.mul(tmp.x.sub(result.x)));
            result.y = result.y.add(isOne.mul(tmp.y.sub(result.y)));
        }
        result
    }

    fn doubleAffinePoint(p: AffinePoint) -> AffinePoint {
        let x_2 = p.x.mul(p.x);
        let l1 = FieldDivisionGadget::new(
            x_2.mul(3)
                .add(p.x.mul(BigInteger::from(Self::COEFF_A)).mul(2))
                .add(1),
            p.y.mul(2),
        )
        .getOutputWires()[0];
        let l2 = l1.mul(l1);
        let newX = l2.sub(BigInteger::from(Self::COEFF_A)).sub(p.x).sub(p.x);
        let newY =
            p.x.mul(3)
                .add(BigInteger::from(Self::COEFF_A))
                .sub(l2)
                .mul(l1)
                .sub(p.y);
        AffinePoint::new(newX, newY)
    }

    fn addAffinePoints(p1: AffinePoint, p2: AffinePoint) -> AffinePoint {
        let diffY = p1.y.sub(p2.y);
        let diffX = p1.x.sub(p2.x);
        let q = FieldDivisionGadget::new(diffY, diffX).getOutputWires()[0];
        let q2 = q.mul(q);
        let q3 = q2.mul(q);
        let newX = q2.sub(BigInteger::from(Self::COEFF_A)).sub(p1.x).sub(p2.x);
        let newY =
            p1.x.mul(2)
                .add(p2.x)
                .add(BigInteger::from(Self::COEFF_A))
                .mul(q)
                .sub(q3)
                .sub(p1.y);
        AffinePoint::new(newX, newY)
    }

    pub fn computeYCoordinate(x: BigInteger) -> BigInteger {
        let xSqred = x.mul(x).rem(&Configs.field_prime);
        let xCubed = xSqred.mul(x).rem(&Configs.field_prime);
        let ySqred = xCubed
            .add(BigInteger::from(Self::COEFF_A).mul(xSqred))
            .add(x)
            .rem(&Configs.field_prime);
        let y = x; //IntegerFunctions.ressol(ySqred, Configs.field_prime);
        y
    }

    fn assertPointOrder(&self, p: &AffinePoint, table: &Vec<AffinePoint>) {
        let generator = &self.generators;
        let o = &generator.createConstantWire(
            BigInteger::parse_bytes(Self::SUBGROUP_ORDER.as_bytes(), 10).unwrap(),
        );
        let bits = o
            .getBitWires(
                BigInteger::parse_bytes(Self::SUBGROUP_ORDER.as_bytes(), 10)
                    .unwrap()
                    .bits(),
            )
            .asArray();

        let result = AffinePoint::new(table[bits.len() - 1]);
        for j in (1..=bits.len() - 2).rev() {
            let tmp = addAffinePoints(result, table[j]);
            let isOne = bits[j];
            result.x = result.x.add(isOne.mul(tmp.x.sub(result.x)));
            result.y = result.y.add(isOne.mul(tmp.y.sub(result.y)));
        }

        // verify that: result = -p
        generator.addEqualityAssertion(result.x, p.x);
        generator.addEqualityAssertion(result.y, p.y.mul(-1));

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
