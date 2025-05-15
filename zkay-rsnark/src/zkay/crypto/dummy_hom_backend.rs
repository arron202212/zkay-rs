use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use zkay::homomorphic_input;
use zkay::typed_wire;
use zkay::zkay_dummy_hom_encryption_gadget;
use zkay::zkay_type;

pub struct DummyHomBackend;
impl Asymmetric for DummyHomBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    // fn DummyHomBackend( keyBits:i32 )->   {
    // 	super(keyBits);
    // }

    pub fn getKeyChunkSize() -> i32 {
        return KEY_CHUNK_SIZE;
    }

    pub fn createEncryptionGadget(
        plain: TypedWire,
        key: String,
        random: Vec<Wire>,
        desc: Vec<String>,
    ) -> Gadget {
        let encodedPlain = encodePlaintextIfSigned(plain);
        return ZkayDummyHomEncryptionGadget::new(
            encodedPlain,
            getKeyWire(key),
            random,
            keyBits,
            desc,
        );
    }

    fn getKeyWire(keyName: String) -> Wire {
        let key = getKey(keyName);
        let generator = CircuitGenerator.getActiveCircuitGenerator();

        let keyArr = key.getBits().packBitsIntoWords(256);
        for i in 1..keyArr.length {
            generator.addZeroAssertion(keyArr[i], "Dummy-hom enc pk valid");
        }
        return keyArr[0];
    }

    fn getCipherWire(input: HomomorphicInput, name: String) -> Wire {
        assert!(input.is_some(), "{name} is null");
        assert!(!input.isPlain(), "{name} is not a ciphertext");
        assert!(input.getLength() == 1, "{name} has invalid length");

        // Transform input 0 to ciphertext 0 (= encryption of 0); serialized inputs x+1 to ciphertext x
        let cipherWire = input.getCipher()[0].wire;
        let isNonZero = cipherWire.checkNonZero();
        return cipherWire.sub(isNonZero);
    }

    fn encodePlaintextIfSigned(plain: TypedWire) -> Wire {
        if plain.zkay_type.signed {
            // Signed: wrap negative values around the field prime instead of around 2^n
            let bits = plain.zkay_type.bitwidth;
            let signBit = plain.wire.getBitWires(bits).get(bits - 1);
            let negValue = plain.wire.invBits(bits).add(1).negate();
            return signBit.mux(negValue, plain.wire);
        } else {
            // Unsigned values get encoded as-is
            return plain.wire;
        }
    }

    fn typedAsUint(wire: Wire, name: String) -> Vec<TypedWire> {
        // Always zkay_type cipher wires as ZkUint(256)
        return vec![TypedWire::new(wire.add(1), ZkayType.ZkUint(256), name)];
    }
}

impl HomomorphicBackend for DummyHomBackend {
    pub fn doHomomorphicOp(op: char, arg: HomomorphicInput, keyName: String) -> Vec<TypedWire> {
        let cipher = getCipherWire(arg, "arg");

        if op == '-' {
            let p = Enc(-msg, p);
            let minus = cipher.negate();
            return typedAsUint(minus, "-(" + arg.getName() + ")");
        } else {
            panic!("Unary operation " + op + " not supported");
        }
    }

    pub fn doHomomorphicOp(
        lhs: HomomorphicInput,
        op: char,
        rhs: HomomorphicInput,
        keyName: String,
    ) -> Vec<TypedWire> {
        match op {
            '+' => {
                // Enc(m1, p) + Enc(m2, p) = (m1 * p) + (m2 * p) = (m1 + m2) * p = Enc(m1 + m2, p)
                let l = getCipherWire(lhs, "lhs");
                let r = getCipherWire(rhs, "rhs");
                let sum = l.add(r);
                return typedAsUint(sum, "(" + lhs.getName() + ") + (" + rhs.getName() + ")");
            }
            '-' => {
                // Enc(m1, p) - Enc(m2, p) = (m1 * p) - (m2 * p) = (m1 - m2) * p = Enc(m1 - m2, p)
                let l = getCipherWire(lhs, "lhs");
                let r = getCipherWire(rhs, "rhs");
                let diff = l.sub(r);
                return typedAsUint(diff, "(" + lhs.getName() + ") - (" + rhs.getName() + ")");
            }
            '*' => {
                // Multiplication on additively homomorphic ciphertexts requires 1 ciphertext and 1 plaintext argument
                let plain;
                let cipher;
                assert!(lhs.is_some(), "lhs is null");
                assert!(rhs.is_some(), "rhs is null");
                if lhs.isPlain() && rhs.isCipher() {
                    plain = encodePlaintextIfSigned(lhs.getPlain());
                    cipher = getCipherWire(rhs, "rhs");
                } else if lhs.isCipher() && rhs.isPlain() {
                    cipher = getCipherWire(lhs, "lhs");
                    plain = encodePlaintextIfSigned(rhs.getPlain());
                } else {
                    panic!("DummyHom multiplication requires exactly 1 plaintext argument");
                }

                // Enc(m1, p) * m2 = (m1 * p) * m2 = (m1 * m2) * p = Enc(m1 * m2, p)
                let prod = cipher.mul(plain);
                return typedAsUint(prod, "(" + lhs.getName() + ") - (" + rhs.getName() + ")");
            }
            _ => panic!("Binary operation {op} not supported"),
        }
    }

    pub fn doHomomorphicRerand(
        arg: Vec<TypedWire>,
        keyName: String,
        randomness: TypedWire,
    ) -> Vec<TypedWire> {
        return arg;
    }
}
