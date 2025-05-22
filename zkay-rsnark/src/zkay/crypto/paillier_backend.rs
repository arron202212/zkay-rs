use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use examples::gadgets::math::long_integer_mod_gadget;
use examples::gadgets::math::long_integer_mod_pow_gadget;
use examples::gadgets::math::longIntegerModInverseGadget;
use zkay::HomomorphicInput;
use zkay::typed_wire;
use zkay::zkay_paillier_fast_enc_gadget;
use zkay::zkay_type;

pub struct PaillierBackend {
    minNumCipherChunks: i32,
    maxNumCipherChunks: i32,
}
impl PaillierBackend {
    const CHUNK_SIZE: i32 = LongElement.CHUNK_BITWIDTH; //120;
    pub fn new(keyBits: i32) {
        // Same chunk size for key, randomness, and ciphertext

        //  {
        // 	if CHUNK_SIZE != LongElement.CHUNK_BITWIDTH {
        // 		assert!("Paillier chunk size must match LongElement.CHUNK_BITWIDTH.\n" +
        // 				"If LongElement.CHUNK_BITWIDTH needs to be changed, change this _and_ meta.py in jsnark!");
        // 	}
        // }

        // super(keyBits); // keyBits = bits of n
        assert!(
            keyBits > CHUNK_SIZE,
            "Key size too small ( {keyBits}  <  {CHUNK_SIZE}  bits)"
        );

        // n^2 has either length (2 * keyBits - 1) or (2 * keyBits) bits
        // minNumCipherChunks = ceil((2 * keyBits - 1) / CHUNK_SIZE)
        // maxNumCipherChunks = ceil((2 * keyBits) / CHUNK_SIZE)
        let minNSquareBits = 2 * keyBits - 1;
        Self {
            minNumCipherChunks: (minNSquareBits + CHUNK_SIZE - 1) / CHUNK_SIZE,
            maxNumCipherChunks: (minNSquareBits + CHUNK_SIZE) / CHUNK_SIZE,
        }
    }
}

impl Asymmetric for PaillierBackend {
    pub fn getKeyChunkSize() -> i32 {
        return CHUNK_SIZE;
    }

    pub fn createEncryptionGadget(
        plain: TypedWire,
        keyName: String,
        randomWires: Vec<Option<WireType>>,
        desc: Vec<String>,
    ) -> Gadget {
        let key = getKey(keyName);
        let encodedPlain = encodeSignedToModN(plain, key);
        let randArr = LongElement::new(
            WireArray::new(randomWires)
                .getBits(CHUNK_SIZE)
                .adjustLength(keyBits),
        );
        let random = uninitZeroToOne(randArr); // Also replace randomness 0 with 1 (for uninit ciphers)
        return ZkayPaillierFastEncGadget::new(key, keyBits, encodedPlain, random, desc);
    }
}
impl Asymmetric for HomomorphicBackend {
    pub fn doHomomorphicOp(op: char, arg: HomomorphicInput, keyName: String) -> Vec<TypedWire> {
        assert!(arg.is_some() && !arg.isPlain(), "arg");

        let nSquare = getNSquare(keyName);
        let cipherVal = toLongElement(arg);

        if op == '-' {
            // Enc(m, r)^(-1) = (g^m * r^n)^(-1) = (g^m)^(-1) * (r^n)^(-1) = g^(-m) * (r^(-1))^n = Enc(-m, r^(-1))
            let result = invert(cipherVal, nSquare);
            return toWireArray(result, "-(" + arg.getName() + ")");
        } else {
            panic!("Unary operation {op} not supported");
        }
    }

    pub fn doHomomorphicOp(
        lhs: HomomorphicInput,
        op: char,
        rhs: HomomorphicInput,
        keyName: String,
    ) -> Vec<TypedWire> {
        let nSquare = getNSquare(keyName);

        match op {
            '+' => {
                // Enc(m1, r1) * Enc(m2, r2) = (g^m1 * r1^n) * (g^m2 * r2^n) = g^(m1 + m2) * (r1 * r2)^n = Enc(m1 + m2, r1 * r2)
                let outputName = "(" + lhs.getName() + ") + (" + rhs.getName() + ")";
                let lhsVal = toLongElement(lhs);
                let rhsVal = toLongElement(rhs);
                let result = mulMod(lhsVal, rhsVal, nSquare);
                return toWireArray(result, outputName);
            }
            '-' => {
                // Enc(m1, r1) * Enc(m2, r2)^(-1) = Enc(m1 + (-m2), r1 * r2^(-1)) = Enc(m1 - m2, r1 * r2^(-1))
                let outputName = "(" + lhs.getName() + ") - (" + rhs.getName() + ")";
                let lhsVal = toLongElement(lhs);
                let rhsVal = toLongElement(rhs);
                let result = mulMod(lhsVal, invert(rhsVal, nSquare), nSquare);
                return toWireArray(result, outputName);
            }
            '*' => {
                // Multiplication on additively homomorphic ciphertexts requires 1 ciphertext and 1 plaintext argument
                let mut cipherVal;
                let mut plainWire;

                assert!(lhs.is_some(), "lhs is null");
                assert!(rhs.is_some(), "rhs is null");
                if lhs.isPlain() && rhs.isCipher() {
                    plainWire = lhs.getPlain();
                    cipherVal = toLongElement(rhs);
                } else if lhs.isCipher() && rhs.isPlain() {
                    cipherVal = toLongElement(lhs);
                    plainWire = rhs.getPlain();
                } else {
                    assert!("Paillier multiplication requires exactly 1 plaintext argument");
                }

                let plainBits = plainWire.zkay_type.bitwidth;
                let plainBitWires = plainWire.wire.getBitWires(plainBits);
                let mut absPlainVal;
                if !plainWire.zkay_type.signed {
                    // Unsigned, easy , just do the multiplication.
                    absPlainVal = LongElement::new(plainBitWires);
                } else {
                    // Signed. Multiply by the absolute value, later negate result if sign bit was set.
                    let twosComplement = plainWire.wire.invBits(plainBits).add(1);
                    let posValue = LongElement::new(plainBitWires);
                    let negValue = LongElement::new(twosComplement.getBitWires(plainBits));
                    let signBit = plainBitWires.get(plainBits - 1);
                    absPlainVal = posValue.muxBit(negValue, signBit);
                }
                let outputName = "(" + lhs.getName() + ") * (" + rhs.getName() + ")";

                // Enc(m1, r1) ^ m2 = (g^m1 * r1^n) ^ m2 = (g^m1)^m2 * (r1^n)^m2 = g^(m1*m2) * (r1^m2)^n = Enc(m1 * m2, r1 ^ m2)
                let result = modPow(cipherVal, absPlainVal, plainBits, nSquare);

                if plainWire.zkay_type.signed {
                    // Correct for sign
                    let signBit = plainBitWires.get(plainBits - 1);
                    let negResult = invert(result, nSquare);
                    result = result.muxBit(negResult, signBit);
                }

                return toWireArray(result, outputName);
            }
            _ => panic!("Binary operation  {op} not supported"),
        }
    }

    fn getNSquare(keyName: String) -> LongElement {
        let n = getKey(keyName);
        let nSquareMaxBits = 2 * keyBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement.CHUNK_BITWIDTH - 1)) / LongElement.CHUNK_BITWIDTH;
        return n.mul(n).align(maxNumChunks);
    }

    fn invert(val: LongElement, nSquare: LongElement) -> LongElement {
        return LongIntegerModInverseGadget::new(val, nSquare, true, "Paillier negation")
            .getResult();
    }

    fn mulMod(lhs: LongElement, rhs: LongElement, nSquare: LongElement) -> LongElement {
        return LongIntegerModGadget::new(
            lhs.mul(rhs),
            nSquare,
            2 * keyBits,
            true,
            "Paillier addition",
        )
        .getRemainder();
    }

    fn modPow(
        lhs: LongElement,
        rhs: LongElement,
        rhsBits: i32,
        nSquare: LongElement,
    ) -> LongElement {
        return LongIntegerModPowGadget::new(
            lhs,
            rhs,
            rhsBits,
            nSquare,
            2 * keyBits,
            "Paillier multiplication",
        )
        .getResult();
    }

    fn toLongElement(input: HomomorphicInput) -> LongElement {
        assert!(
            input.is_some() && !input.isPlain(),
            "Input null or not ciphertext"
        );
        let cipher = input.getCipher();
        assert!(
            cipher.len() >= minNumCipherChunks && cipher.len() <= maxNumCipherChunks,
            "Ciphertext has invalid length {}",
            cipher.len()
        );

        // Ciphertext inputs seem to be passed as ZkUint(256); sanity check to make sure we got that.
        let uint256 = ZkayType.ZkUint(256);
        for cipherWire in cipher {
            ZkayType.checkType(uint256, cipherWire.zkay_type);
        }

        // Input is a Paillier ciphertext - front-end must already check that this is the
        let wires = vec![None; cipher.len()];
        for i in 0..cipher.len() {
            wires[i] = cipher[i].wire;
        }
        let mut bitWidths = vec![CHUNK_SIZE; wires.len()];
        bitWidths[bitWidths.len() - 1] = 2 * keyBits - (bitWidths.len() - 1) * CHUNK_SIZE;

        // Cipher could still be uninitialized-zero, which we need to fix
        return uninitZeroToOne(LongElement::new(wires, bitWidths));
    }

    fn toWireArray(value: LongElement, name: String) -> Vec<TypedWire> {
        // First, sanity check that the result has at most maxNumCipherChunks wires of at most CHUNK_SIZE bits each
        assert!(
            value.getSize() <= maxNumCipherChunks,
            "Paillier output contains too many wires"
        );
        assert!(
            value
                .getCurrentBitwidth()
                .iter()
                .all(|bitWidth| bitWidth <= CHUNK_SIZE),
            "Paillier output cipher bit width too large"
        );

        // If ok, wrap the output wires in TypedWire. As with the input, treat ciphertexts as ZkUint(256).
        let wires = value.getArray();
        let typedWires = vec![TypedWire::default(); wires.len()];
        let uint256 = ZkayType.ZkUint(256);
        for i in 0..wires.len() {
            typedWires[i] = TypedWire::new(wires[i], uint256, name);
        }
        return typedWires;
    }

    fn uninitZeroToOne(val: LongElement) -> LongElement {
        // Uninitialized values have a ciphertext of all zeros, which is not a valid Paillier cipher.
        // Instead, replace those values with 1 == g^0 * 1^n = Enc(0, 1)
        let valIsZero = val.checkNonZero().invAsBit();
        let oneIfAllZero = LongElement::new(valIsZero, 1 /* bit */);
        return val.add(oneIfAllZero);
    }

    fn encodeSignedToModN(input: TypedWire, key: LongElement) -> LongElement {
        if input.zkay_type.signed {
            // Signed. Encode positive values as-is, negative values (-v) as (key - v)
            let bits = input.zkay_type.bitwidth;
            let inputBits = input.wire.getBitWires(bits);
            let signBit = inputBits.get(bits - 1);

            let posValue = LongElement::new(inputBits);
            let rawNegValue =
                LongElement::new(input.wire.invBits(bits).add(1).getBitWires(bits + 1));
            let negValue = key.sub(rawNegValue);

            return posValue.muxBit(negValue, signBit);
        } else {
            // Unsigned, encode as-is, just convert the input wire to a LongElement
            return LongElement::new(input.wire.getBitWires(input.zkay_type.bitwidth));
        }
    }
}
