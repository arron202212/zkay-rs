use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use examples::gadgets::blockciphers::symmetric_encryptioncbc_gadget;
use examples::gadgets::diffie_hellman_key_exchange::field_extension_dh_key_exchange;
use examples::gadgets::hash::sha256_gadget;
use crate::util::util::{Util,BigInteger};

// This gadget shows a simple example of hybrid encryption for illustration purposes
// It currently uses the field extension key exchange gadget with the speck cipher

pub struct HybridEncryptionCircuitGenerator {
    plaintext: Vec<WireType>,  // as 64-bit words
    plaintextSize: i32,    // number of 64-bit words
    ciphertext: Vec<WireType>, // as 64-bit words

    ciphername: String,
    secExpBits: Vec<WireType>,
}
impl HybridEncryptionCircuitGenerator {
    // Will assume the parameterization used in the test files ~ 80-bits
    // security
    pub const EXPONENT_BITWIDTH: i32 = 397; // in bits
    pub const MU: i32 = 4;
    pub const OMEGA: i32 = 7;
    pub fn new(circuitName: String, plaintextSize: i32, ciphername: String) {
        super(circuitName);
        self.ciphername = ciphername;
        self.plaintextSize = plaintextSize;
    }
}
impl CircuitGenerator for HybridEncryptionCircuitGenerator {
    fn buildCircuit() {
        plaintext = createInputWireArray(plaintextSize, "plaint text");

        // Part I: Exchange a key:

        // The secret exponent is a  input by the prover
        secExpBits = createProverWitnessWireArray(EXPONENT_BITWIDTH, "SecretExponent");
        for i in 0..EXPONENT_BITWIDTH {
            addBinaryAssertion(secExpBits[i]); // verify all bits are binary
        }

        let g = vec![WireType::default(); MU];
        let h = vec![WireType::default(); MU];

        // Hardcode the base and the other party's key (suitable when keys are not expected to change)
        g[0] = createConstantWire(BigInteger::new(
            "16377448892084713529161739182205318095580119111576802375181616547062197291263",
        ));
        g[1] = createConstantWire(BigInteger::new(
            "13687683608888423916085091250849188813359145430644908352977567823030408967189",
        ));
        g[2] = createConstantWire(BigInteger::new(
            "12629166084120705167185476169390021031074363183264910102253898080559854363106",
        ));
        g[3] = createConstantWire(BigInteger::new(
            "19441276922979928804860196077335093208498949640381586557241379549605420212272",
        ));

        h[0] = createConstantWire(BigInteger::new(
            "8252578783913909531884765397785803733246236629821369091076513527284845891757",
        ));
        h[1] = createConstantWire(BigInteger::new(
            "20829599225781884356477513064431048695774529855095864514701692089787151865093",
        ));
        h[2] = createConstantWire(BigInteger::new(
            "1540379511125324102377803754608881114249455137236500477169164628692514244862",
        ));
        h[3] = createConstantWire(BigInteger::new(
            "1294177986177175279602421915789749270823809536595962994745244158374705688266",
        ));

        // To make g and h variable inputs to the circuit, simply do the following
        // instead, and supply the above values using the generateSampleInput()
        // method instead.
        /*
         * Vec<WireType> g = createInputWireArray(mu);
         * Vec<WireType> h = createInputWireArray(mu);
         */

        // Exchange keys
        let exchange = FieldExtensionDHKeyExchange::new(g, h, secExpBits, OMEGA, "");

        // Output g^s
        let g_to_s = exchange.getOutputPublicValue();
        makeOutputArray(g_to_s, "DH Key Exchange Output");

        // Use h^s to generate a symmetric secret key and an initialization
        // vector. Apply a Hash-based KDF.
        let h_to_s = exchange.getSharedSecret();
        let hashGadget = SHA256Gadget::new(h_to_s, 256, 128, true, false);
        let secret = hashGadget.getOutputWires();
        let key = Arrays.copyOfRange(secret, 0, 128);
        let iv = Arrays.copyOfRange(secret, 128, 256);

        // Part II: Apply symmetric Encryption

        let plaintextBits = WireArray::new(plaintext).getBits(64).asArray();
        let symEncGagdet = SymmetricEncryptionCBCGadget::new(plaintextBits, key, iv, ciphername);
        ciphertext = symEncGagdet.getOutputWires();
        makeOutputArray(ciphertext, "Cipher Text");
    }

    pub fn generateSampleInput(evaluator: CircuitEvaluator) {
        // TODO Auto-generated method stub
        for i in 0..plaintextSize {
            evaluator.setWireValue(plaintext[i], Util::nextRandomBigInteger(64));
        }
        for i in 0..EXPONENT_BITWIDTH {
            evaluator.setWireValue(secExpBits[i], Util::nextRandomBigInteger(1));
        }
    }

    pub fn main(args: Vec<String>) {
        let generator = HybridEncryptionCircuitGenerator::new("enc_example", 16, "speck128");
        generator.generateCircuit();
        generator.evalCircuit();
        generator.prepFiles();
        generator.runLibsnark();
    }
}
