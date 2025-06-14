use crate::circuit::config::config::Configs;
use crate::circuit::operations::gadget;
use crate::circuit::structure::circuit_generator::{CircuitGenerator,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::math::field_division_gadget;

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
pub struct ZkayEcGadget;
impl ZkayEcGadget {
    pub fn new(desc: &Option<String>) {
        super(desc);
    }

    // Note: this parameterization assumes that the underlying field has
    // Configs.field_prime =
    // 21888242871839275222246405745257275088548364400416034343698204186575808495617

    pub const SECRET_BITWIDTH: i32 = 253; // number of bits in the
    // exponent. Note that the
    // most significant bit
    // should
    // be set to 1, and the
    // three least significant
    // bits should be be zero.
    // See
    // the constructor

    pub const COEFF_A: BigInteger = BigInteger::new("126932"); // parameterization
    // in
    // https://eprint.iacr.org/2015/1093.pdf

    pub const CURVE_ORDER: BigInteger = BigInteger::new(
        "21888242871839275222246405745257275088597270486034011716802747351550446453784",
    );

    // As in curve25519, CURVE_ORDER = SUBGROUP_ORDER * 2^3
    pub const SUBGROUP_ORDER: BigInteger = BigInteger::new(
        "2736030358979909402780800718157159386074658810754251464600343418943805806723",
    );

    pub fn checkSecretBits(generator:Box<dyn CGConfig+Send+Sync>, secretBits: Vec<Option<WireType>>) {
        /**
         * The secret key bits must be of length SECRET_BITWIDTH and are
         * expected to follow a little endian order. The most significant bit
         * should be 1, and the three least significant bits should be zero.
         */
        generator.addZeroAssertion(secretBits[0], "Asserting secret bit conditions");
        generator.addZeroAssertion(secretBits[1], "Asserting secret bit conditions");
        generator.addZeroAssertion(secretBits[2], "Asserting secret bit conditions");
        generator.addOneAssertion(
            secretBits[SECRET_BITWIDTH - 1],
            "Asserting secret bit conditions",
        );

        for i in 3..SECRET_BITWIDTH - 1 {
            // verifying all other bit wires are binary (as this is typically a
            // secret
            // witness by the prover)
            generator.addBinaryAssertion(secretBits[i]);
        }
    }

    // this is only called, when WireType y is provided as witness by the prover
    // (not as input to the gadget)
    fn assertValidPointOnEC(x: WireType, y: WireType) {
        let ySqr = y.mul(y);
        let xSqr = x.mul(x);
        let xCube = xSqr.mul(x);
        generator.addEqualityAssertion(ySqr, xCube.add(xSqr.mul(COEFF_A)).add(x));
    }

    fn preprocess(p: AffinePoint) -> Vec<AffinePoint> {
        let precomputedTable = vec![AffinePoint::default(); SECRET_BITWIDTH];
        precomputedTable[0] = p;
        for j in 1..SECRET_BITWIDTH {
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
        let l1 =
            FieldDivisionGadget::new(x_2.mul(3).add(p.x.mul(COEFF_A).mul(2)).add(1), p.y.mul(2))
                .getOutputWires()[0];
        let l2 = l1.mul(l1);
        let newX = l2.sub(COEFF_A).sub(p.x).sub(p.x);
        let newY = p.x.mul(3).add(COEFF_A).sub(l2).mul(l1).sub(p.y);
        AffinePoint::new(newX, newY)
    }

    fn addAffinePoints(p1: AffinePoint, p2: AffinePoint) -> AffinePoint {
        let diffY = p1.y.sub(p2.y);
        let diffX = p1.x.sub(p2.x);
        let q = FieldDivisionGadget::new(diffY, diffX).getOutputWires()[0];
        let q2 = q.mul(q);
        let q3 = q2.mul(q);
        let newX = q2.sub(COEFF_A).sub(p1.x).sub(p2.x);
        let newY = p1.x.mul(2).add(p2.x).add(COEFF_A).mul(q).sub(q3).sub(p1.y);
        AffinePoint::new(newX, newY)
    }
}
impl Gadget for ZkayEcGadget {
    pub fn computeYCoordinate(x: BigInteger) -> BigInteger {
        let xSqred = x.mul(x).rem(Configs.field_prime.clone());
        let xCubed = xSqred.mul(x).rem(Configs.field_prime.clone());
        let ySqred = xCubed
            .add(COEFF_A.mul(xSqred))
            .add(x)
            .rem(Configs.field_prime.clone());
        let y = IntegerFunctions.ressol(ySqred, Configs.field_prime);
        y
    }

    fn assertPointOrder(p: AffinePoint, table: Vec<AffinePoint>) {
        let o = generator.createConstantWire(SUBGROUP_ORDER);
        let bits = o.getBitWires(SUBGROUP_ORDER.bits()).asArray();

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
