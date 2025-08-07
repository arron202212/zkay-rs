#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_division::LongIntegerDivisionConfig;
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;
use crate::examples::gadgets::math::long_integer_mod_inverse_gadget::LongIntegerModInverseGadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget::LongIntegerModPowGadget;

use crate::zkay::crypto::crypto_backend::Asymmetric;
use crate::zkay::crypto::crypto_backend::AsymmetricConfig;
use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfig;
use crate::zkay::crypto::homomorphic_backend::HomomorphicBackend;
use crate::zkay::homomorphic_input::HomomorphicInput;

use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget;
use crate::zkay::zkay_paillier_fast_enc_gadget::ZkayPaillierFastEncGadget;

use crate::zkay::zkay_type::ZkayType;
use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct PaillierBackend {
    pub minNumCipherChunks: i32,
    pub maxNumCipherChunks: i32,
}
impl PaillierBackend {
    const CHUNK_SIZE: i32 = LongElement::CHUNK_BITWIDTH; //120;
    pub fn new(keyBits: i32) -> CryptoBackend<Asymmetric<Self>> {
        // Same chunk size for key, randomness, and ciphertext

        //  {
        // 	if CHUNK_SIZE != LongElement::CHUNK_BITWIDTH {
        // 		assert!("Paillier chunk size must match LongElement::CHUNK_BITWIDTH.\n" +
        // 				"If LongElement::CHUNK_BITWIDTH needs to be changed, change this _and_ meta.py in jsnark!");
        // 	}
        // }

        // //super(keyBits); // keyBits = bits of n
        assert!(
            keyBits > Self::CHUNK_SIZE,
            "Key size too small ( {keyBits}  <  {}  bits)",
            Self::CHUNK_SIZE
        );

        // n^2 has either length (2 * keyBits - 1) or (2 * keyBits) bits
        // minNumCipherChunks = ceil((2 * keyBits - 1) / CHUNK_SIZE)
        // maxNumCipherChunks = ceil((2 * keyBits) / CHUNK_SIZE)
        let minNSquareBits = 2 * keyBits - 1;
        Asymmetric::<Self>::new(
            keyBits,
            Self {
                minNumCipherChunks: (minNSquareBits + Self::CHUNK_SIZE - 1) / Self::CHUNK_SIZE,
                maxNumCipherChunks: (minNSquareBits + Self::CHUNK_SIZE) / Self::CHUNK_SIZE,
            },
        )
    }
}

impl CryptoBackendConfig for CryptoBackend<Asymmetric<PaillierBackend>> {
    fn getKeyChunkSize() -> i32 {
        PaillierBackend::CHUNK_SIZE
    }

    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        keyName: &String,
        randomWires: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let key = self.getKey(keyName, generator.clone());
        let encodedPlain = self.encodeSignedToModN(plain, key);
        let randArr = LongElement::newa(
            WireArray::new(randomWires.clone(), generator.clone().downgrade())
                .getBits(PaillierBackend::CHUNK_SIZE as usize, &None)
                .adjustLength(None, self.keyBits as usize),
        );
        let random = self.uninitZeroToOne(&randArr); // Also replace randomness 0 with 1 (for uninit ciphers)
        Box::new(ZkayPaillierFastEncGadget::new(
            key,
            self.keyBits,
            encodedPlain,
            random,
            desc,
            generator,
        ))
    }
}
impl HomomorphicBackend for CryptoBackend<Asymmetric<PaillierBackend>> {
    fn doHomomorphicOpu(
        &self,
        op: char,
        arg: &HomomorphicInput,
        keyName: &String,
    ) -> Vec<TypedWire> {
        assert!(!arg.isPlain(), "arg");

        let nSquare = self.getNSquare(keyName);
        let cipherVal = self.toLongElement(arg);

        if op == '-' {
            // Enc(m, r)^(-1) = (g^m * r^n)^(-1) = (g^m)^(-1) * (r^n)^(-1) = g^(-m) * (r^(-1))^n = Enc(-m, r^(-1))
            let result = self.invert(cipherVal, nSquare);
            self.toWireArray(result, &format!("-({})", arg.getName()))
        } else {
            panic!("Unary operation {op} not supported");
        }
    }

    fn doHomomorphicOp(
        &self,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
        keyName: &String,
    ) -> Vec<TypedWire> {
        let nSquare = self.getNSquare(keyName);

        match op {
            '+' => {
                // Enc(m1, r1) * Enc(m2, r2) = (g^m1 * r1^n) * (g^m2 * r2^n) = g^(m1 + m2) * (r1 * r2)^n = Enc(m1 + m2, r1 * r2)
                let outputName = format!("({}) + ({})", lhs.getName(), rhs.getName());
                let lhsVal = self.toLongElement(lhs);
                let rhsVal = self.toLongElement(rhs);
                let result = self.mulMod(&lhsVal, &rhsVal, &nSquare);
                self.toWireArray(&result, &outputName)
            }
            '-' => {
                // Enc(m1, r1) * Enc(m2, r2)^(-1) = Enc(m1 + (-m2), r1 * r2^(-1)) = Enc(m1 - m2, r1 * r2^(-1))
                let outputName = format!("({}) - ({})", lhs.getName(), rhs.getName());
                let lhsVal = self.toLongElement(lhs);
                let rhsVal = self.toLongElement(rhs);
                let result = self.mulMod(lhsVal, self.invert(&rhsVal, &nSquare), nSquare);
                self.toWireArray(&result, &outputName)
            }
            '*' => {
                // Multiplication on additively homomorphic ciphertexts requires 1 ciphertext and 1 plaintext argument
                let mut cipherVal;
                let mut plainWire;

                // assert!(lhs.is_some(), "lhs is None");
                // assert!(rhs.is_some(), "rhs is None");
                if lhs.isPlain() && rhs.isCipher() {
                    plainWire = lhs.getPlain();
                    cipherVal = self.toLongElement(rhs);
                } else if lhs.isCipher() && rhs.isPlain() {
                    cipherVal = self.toLongElement(lhs);
                    plainWire = rhs.getPlain();
                } else {
                    assert!("Paillier multiplication requires exactly 1 plaintext argument");
                }

                let plainBits = plainWire.zkay_type.bitwidth;
                let plainBitWires = plainWire.wire.getBitWiresi(plainBits, &None);
                let mut absPlainVal;
                if !plainWire.zkay_type.signed {
                    // Unsigned, easy , just do the multiplication.
                    absPlainVal = LongElement::newa(plainBitWires, generator.clone().downgrade());
                } else {
                    // Signed. Multiply by the absolute value, later negate result if sign bit was set.
                    let twosComplement = plainWire.wire.invBits(plainBits).add(1);
                    let posValue = LongElement::newa(plainBitWires, generator.clone().downgrade());
                    let negValue = LongElement::newa(
                        twosComplement.getBitWiresi(plainBits, &None),
                        generator.clone().downgrade(),
                    );
                    let signBit = plainBitWires.get(plainBits - 1);
                    absPlainVal = posValue.muxBit(negValue, signBit);
                }
                let outputName = format!("({}) * ({})", lhs.getName(), rhs.getName());

                // Enc(m1, r1) ^ m2 = (g^m1 * r1^n) ^ m2 = (g^m1)^m2 * (r1^n)^m2 = g^(m1*m2) * (r1^m2)^n = Enc(m1 * m2, r1 ^ m2)
                let result = self.modPow(&cipherVal, &absPlainVal, &plainBits, &nSquare);

                if plainWire.zkay_type.signed {
                    // Correct for sign
                    let signBit = plainBitWires.get(plainBits - 1);
                    let negResult = self.invert(&result, &nSquare);
                    result = result.muxBit(&negResult, &signBit);
                }

                self.toWireArray(&result, &outputName)
            }
            _ => panic!("Binary operation  {op} not supported"),
        }
    }
}
impl CryptoBackend<Asymmetric<PaillierBackend>> {
    fn getNSquare(&self, keyName: &String) -> LongElement {
        let n = self.getKey(keyName);
        let nSquareMaxBits = 2 * self.keyBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        n.mul(n).align(maxNumChunks)
    }

    fn invert(&self, val: &LongElement, nSquare: &LongElement) -> LongElement {
        return LongIntegerModInverseGadget::new(val, nSquare, true, "Paillier negation")
            .getResult();
    }

    fn mulMod(&self, lhs: &LongElement, rhs: &LongElement, nSquare: &LongElement) -> LongElement {
        LongIntegerModGadget::new(
            lhs.mul(rhs),
            nSquare.clone(),
            2 * self.keyBits,
            true,
            &None("Paillier addition".to_owned()),
        )
        .getRemainder()
    }

    fn modPow(
        &self,
        lhs: &LongElement,
        rhs: &LongElement,
        rhsBits: i32,
        nSquare: &LongElement,
    ) -> LongElement {
        LongIntegerModPowGadget::new(
            lhs.clone(),
            rhs.clone(),
            rhsBits,
            nSquare.clone(),
            2 * self.keyBits,
            &Some("Paillier multiplication"),
        )
        .getResult()
        .clone()
    }

    fn toLongElement(&self, input: &HomomorphicInput) -> LongElement {
        assert!(!input.isPlain(), "Input None or not ciphertext");
        let cipher = input.getCipher();
        assert!(
            cipher.len() >= self.t.t.minNumCipherChunks as usize
                && cipher.len() <= self.t.t.maxNumCipherChunks as usize,
            "Ciphertext has invalid length {}",
            cipher.len()
        );

        // Ciphertext inputs seem to be passed as ZkUint(256); sanity check to make sure we got that.
        let uint256 = ZkayType::ZkUint(256);
        for cipherWire in cipher {
            ZkayType::checkType(&uint256, &cipherWire.zkay_type);
        }

        // Input is a Paillier ciphertext - front-end must already check that this is the
        let wires: Vec<_> = cipher.iter().map(|c| Some(c.wire.clone())).collect();

        let mut bitWidths = vec![PaillierBackend::CHUNK_SIZE as u64; wires.len()];
        bitWidths[bitWidths.len() - 1] =
            (2 * self.keyBits - (bitWidths.len() as i32 - 1) * PaillierBackend::CHUNK_SIZE) as u64;

        // Cipher could still be uninitialized-zero, which we need to fix
        self.uninitZeroToOne(&LongElement::new(wires, bitWidths))
    }

    fn toWireArray(&self, value: &LongElement, name: &String) -> Vec<TypedWire> {
        // First, sanity check that the result has at most maxNumCipherChunks wires of at most CHUNK_SIZE bits each
        assert!(
            value.getSize() <= self.t.t.maxNumCipherChunks as usize,
            "Paillier output contains too many wires"
        );
        assert!(
            value
                .getCurrentBitwidth()
                .iter()
                .all(|&bitWidth| bitWidth <= PaillierBackend::CHUNK_SIZE as u64),
            "Paillier output cipher bit width too large"
        );

        // If ok, wrap the output wires in TypedWire. As with the input, treat ciphertexts as ZkUint(256).
        let wires = value.getArray();
        let uint256 = ZkayType::ZkUint(256);
        wires
            .iter()
            .map(|w| TypedWire::new(wires[i].clone(), uint256.clone(), name.clone(), &vec![]))
            .collect()
    }

    fn uninitZeroToOne(&self, val: &LongElement) -> LongElement {
        // Uninitialized values have a ciphertext of all zeros, which is not a valid Paillier cipher.
        // Instead, replace those values with 1 == g^0 * 1^n = Enc(0, 1)
        let valIsZero = val.checkNonZero().invAsBit();
        let oneIfAllZero = LongElement::new(vec![valIsZero], vec![1] /* bit */, generator);
        val.add(&oneIfAllZero)
    }

    fn encodeSignedToModN(&self, input: &TypedWire, key: &LongElement) -> LongElement {
        if input.zkay_type.signed {
            // Signed. Encode positive values as-is, negative values (-v) as (key - v)
            let bits = input.zkay_type.bitwidth;
            let inputBits = input.wire.getBitWires(bits);
            let signBit = inputBits.get(bits - 1);

            let posValue = LongElement::new(inputBits);
            let rawNegValue =
                LongElement::new(input.wire.invBits(bits).add(1).getBitWires(bits + 1));
            let negValue = key.sub(rawNegValue);

            posValue.muxBit(negValue, signBit)
        } else {
            // Unsigned, encode as-is, just convert the input wire to a LongElement
            LongElement::new(input.wire.getBitWires(input.zkay_type.bitwidth))
        }
    }
}
