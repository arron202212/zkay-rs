use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{Util,BigInteger};

/**
 * Performs Key Exchange using a field extension F_p[x]/(x^\mu - \omega), where
 * the polynomial (x^\mu - \omega) is irreducible. The inputs to this gadget:
 * the base g, the other party's input h = g^a, the bits of the secret exponent
 * secExpBits and omega. The outputs of this gadget: the derived key h^s to be
 * used for symmetric key derivation, and g^s which is sent to the other party.
 *
 * A sample parameterization that gives low security (~80 bits of security) can
 * be found in the Junit tests. A sample usage is in:
 * examples/generators/EncryptionCircuitGenerator.java
 *
 *
 */
pub struct FieldExtensionDHKeyExchange {
    g: Vec<Option<WireType>>, // base
    h: Vec<Option<WireType>>, // other party's pub  input (supposedly, h = g^(the
    // other party's secret))
    secretExponentBits: Vec<Option<WireType>>, // the bits of the secret exponent of the
    // party
    // executing this gadget
    omega: i64,
    mu: i32,

    // gadget outputs
    outputPublicValue: Vec<Option<WireType>>, // g^s (to be sent to the other party)
    sharedSecret: Vec<Option<WireType>>,      // the derived secret key h^s
    gPowersTable: Vec<Vec<Option<WireType>>>,
    hPowersTable: Vec<Vec<Option<WireType>>>,
}
impl FieldExtensionDHKeyExchange {
    /**
     * Note: In the default mode, the gadget only validates the secret input
     * provided by the prover, but it does not validate that the base and pub
     * input of the other's party are proper elements. Since these values are
     * pub , they could be checked outside the circuit.
     *
     * If the validation is needed inside, the method "validateInputs()" should
     * be called explicitly. Example is provided in
     * FieldExtensionDHKeyExchange_Test
     *
     */
    pub fn new(
        g: Vec<Option<WireType>>,
        h: Vec<Option<WireType>>,
        secretExponentBits: Vec<Option<WireType>>,
        omega: i64,
        desc: String,
    ) -> Self {
        super(desc);
        self.g = g;
        self.h = h;
        self.secretExponentBits = secretExponentBits;
        self.omega = omega;
        mu = g.len();
        assert!(h.len() == g.len(), "g and h must have the same dimension");

        // since this is typically a  input by the prover,
        // the check is also done here for safety. No need to remove this if
        // done also outside the gadget. The back end takes care of caching
        for w in secretExponentBits {
            generator.addBinaryAssertion(w);
        }

        buildCircuit();
    }
}
impl Gadget for FieldExtensionDHKeyExchange {
    fn buildCircuit() {
        gPowersTable = preparePowersTable(g);
        hPowersTable = preparePowersTable(h);
        outputPublicValue = exp(g, secretExponentBits, gPowersTable);
        sharedSecret = exp(h, secretExponentBits, hPowersTable);
    }

    fn mul(a: Vec<Option<WireType>>, b: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let c = vec![None; mu];

        for i in 0..mu {
            c[i] = generator.get_zero_wire();
        }
        for i in 0..mu {
            for j in 0..mu {
                let k = i + j;
                if k < mu {
                    c[k] = c[k].add(a[i].mul(b[j]));
                }
                k = i + j - mu;
                if k >= 0 {
                    c[k] = c[k].add(a[i].mul(b[j]).mul(omega));
                }
            }
        }
        c
    }

    fn preparePowersTable(base: Vec<Option<WireType>>) -> Vec<Vec<Option<WireType>>> {
        let powersTable = vec![vec![None; mu]; secretExponentBits.len() + 1];
        powersTable[0] = base[..mu].to_vec();
        for j in 1..secretExponentBits.len() + 1 {
            powersTable[j] = mul(powersTable[j - 1], powersTable[j - 1]);
        }
        powersTable
    }

    fn exp(base: Vec<Option<WireType>>, expBits: Vec<Option<WireType>>, powersTable: Vec<Vec<Option<WireType>>>) -> Vec<Option<WireType>> {
        let c = vec![generator.get_zero_wire(); mu];
        c[0] = generator.get_one_wire();
        for j in 0..expBits.len() {
            let tmp = mul(c, powersTable[j]);
            for i in 0..mu {
                c[i] = c[i].add(expBits[j].mul(tmp[i].sub(c[i])));
            }
        }
        c
    }

    // TODO: Test more scenarios
    pub fn validateInputs(subGroupOrder: BigInteger) {
        // g and h are not zero and not one

        // checking the first chunk
        let zeroOrOne1 = g[0].mul(g[0].sub(1));
        let zeroOrOne2 = h[0].mul(h[0].sub(1));

        // checking the rest
        let allZero1 = generator.get_one_wire();
        let allZero2 = generator.get_one_wire();

        for i in 1..mu {
            allZero1 = allZero1.mul(g[i].checkNonZero().invAsBit());
            allZero2 = allZero2.mul(h[i].checkNonZero().invAsBit());
        }

        // assertion
        generator.addZeroAssertion(zeroOrOne1.mul(allZero1));
        generator.addZeroAssertion(zeroOrOne2.mul(allZero2));

        // verify order of points

        let bitLength = subGroupOrder.bits();
        let bits = vec![None; bitLength];
        for i in 0..bitLength {
            if subGroupOrder.bit(i) {
                bits[i] = generator.get_one_wire();
            } else {
                bits[i] = generator.get_zero_wire();
            }
        }

        let result1 = exp(g, bits, gPowersTable);
        let result2 = exp(h, bits, hPowersTable);

        // both should be one

        generator.addOneAssertion(result1[0]);
        generator.addOneAssertion(result2[0]);
        for i in 1..mu {
            generator.addZeroAssertion(result1[i]);
            generator.addZeroAssertion(result1[i]);
        }
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        Util::concat(outputPublicValue, sharedSecret)
    }

    pub fn getOutputPublicValue() -> Vec<Option<WireType>> {
        outputPublicValue
    }

    pub fn getSharedSecret() -> Vec<Option<WireType>> {
        sharedSecret
    }
}
