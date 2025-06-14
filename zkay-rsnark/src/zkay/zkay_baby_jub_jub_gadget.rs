use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

pub struct JubJubPoint {
    pub x: WireType,
    pub y: WireType,
}
impl JubJubPoint {
    pub fn new(x: WireType, y: WireType) -> Self {
        self.x = x;
        self.y = y;
    }
}

/**
 * Gadget for operations on the BabyJubJub elliptic curve (Twisted Edwards curve over BN254).
 * Parameters are from:
 * https://iden3-docs.readthedocs.io/en/latest/iden3_repos/research/publications/zkproof-standards-workshop-2/baby-jubjub/baby-jubjub.html
 */

pub struct ZkayBabyJubJubGadget;
impl ZkayBabyJubJubGadget {
    pub fn new(desc: &Option<String>) -> Self {
        super(desc);

        // We assume the underlying field matches the base field of BabyJubJub (so that we can avoid alignment/modulus)
        assert!(Configs.field_prime.toString().equals(
            "21888242871839275222246405745257275088548364400416034343698204186575808495617"
        ));
    }

    pub const BASE_ORDER: BigInteger = BigInteger::new(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617",
    );

    pub const CURVE_ORDER: BigInteger = BigInteger::new(
        "2736030358979909402780800718157159386076813972158567259200215660948447373041",
    );

    pub const COFACTOR: BigInteger = BigInteger::new("8");

    pub const COEFF_A: BigInteger = BigInteger::new("1");

    pub const COEFF_D: BigInteger = BigInteger::new(
        "9706598848417545097372247223557719406784115219466060233080913168975159366771",
    );

    // arbitrary generator
    pub const GENERATOR_X: BigInteger = BigInteger::new(
        "11904062828411472290643689191857696496057424932476499415469791423656658550213",
    );

    pub const GENERATOR_Y: BigInteger = BigInteger::new(
        "9356450144216313082194365820021861619676443907964402770398322487858544118183",
    );

    fn getInfinity() -> JubJubPoint {
        JubJubPoint::new(generator.get_zero_wire(), generator.get_one_wire())
    }

    fn getGenerator() -> JubJubPoint {
        let g_x = generator.createConstantWire(GENERATOR_X);
        let g_y = generator.createConstantWire(GENERATOR_Y);
        JubJubPoint::new(g_x, g_y)
    }

    fn assertOnCurve(x: WireType, y: WireType) {
        // assert COEFF_A*x*x + y*y == 1 + COEFF_D*x*x*y*y
        let xSqr = x.mul(x);
        let ySqr = y.mul(y);
        let prod = xSqr.mul(ySqr);
        let lhs = xSqr.mul(COEFF_A).add(ySqr);
        let rhs = prod.mul(COEFF_D).add(1);
        generator.addEqualityAssertion(lhs, rhs);
    }

    fn addPoints(p1: JubJubPoint, p2: JubJubPoint) -> JubJubPoint {
        // Twisted Edwards addition according to https://en.wikipedia.org/wiki/Twisted_Edwards_curve#Addition_on_twisted_Edwards_curves

        let a1 = p1.x.mul(p2.y).add(p1.y.mul(p2.x));
        let a2 = p1.x.mul(p2.x).mul(p1.y.mul(p2.y)).mul(COEFF_D).add(1);
        let b1 = p1.y.mul(p2.y).sub(p1.x.mul(p2.x).mul(COEFF_A));
        let b2 =
            p1.x.mul(p2.x)
                .mul(p1.y.mul(p2.y))
                .mul(COEFF_D)
                .neg()
                .add(1);

        let x = a1.mul(nativeInverse(a2));
        let y = b1.mul(nativeInverse(b2));
        JubJubPoint::new(x, y)
    }

    fn negatePoint(p: JubJubPoint) -> JubJubPoint {
        let new_x = p.x.neg();
        JubJubPoint::new(new_x, p.y)
    }

    /**
     * @param scalarBits the scalar bit representation in little-endian order
     */
    fn mulScalar(p: JubJubPoint, scalarBits: Vec<Option<WireType>>) -> JubJubPoint {
        // Scalar point multiplication using double-and-add algorithm
        let result = getInfinity();
        let doubling = p;

        for i in 0..scalarBits.len() {
            let q = addPoints(doubling, result);
            let new_x = scalarBits[i].mux(q.x, result.x);
            let new_y = scalarBits[i].mux(q.y, result.y);
            result = JubJubPoint::new(new_x, new_y);
            doubling = addPoints(doubling, doubling);
        }

        result
    }
}
impl Gadget for ZkayBabyJubJubGadget {
    /**
     * Returns a wire holding the inverse of a in the native base field.
     */
    fn nativeInverse(a: WireType) -> WireType {
        let ainv = generator.createProverWitnessWire();
        generator.specifyProverWitnessComputation( &|evaluator: &mut CircuitEvaluator| {
                    let aValue = evaluator.getWireValue(a);
                    let inverseValue = aValue.modInverse(BASE_ORDER);
                    evaluator.setWireValue(ainv, inverseValue);
                });
        // {
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a);
        //             let inverseValue = aValue.modInverse(BASE_ORDER);
        //             evaluator.setWireValue(ainv, inverseValue);
        //         }
        //     }
        //     Prover
        // });

        // check if a * ainv = 1 (natively)
        let test = a.mul(ainv);
        generator.addEqualityAssertion(test, generator.get_one_wire());

        ainv
    }
}
