#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        auxiliary::long_element::LongElement,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::CircuitGenerator, wire::WireConfig, wire_array::WireArray,
            wire_type::WireType,
        },
    },
    zkay::{crypto::{
        crypto_backend::{Asymmetric, CryptoBackend, CryptoBackendConfig, CryptoBackendConfigs},
        homomorphic_backend::HomomorphicBackend,
    },
    homomorphic_input::HomomorphicInput,
    typed_wire::TypedWire,
    zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget,
    zkay_paillier_fast_enc_gadget::ZkayPaillierFastEncGadget,
    zkay_type::ZkayType},
    examples::gadgets::math::{
        long_integer_division::LongIntegerDivisionConfig,
        long_integer_mod_gadget::LongIntegerModGadget,
        long_integer_mod_inverse_gadget::LongIntegerModInverseGadget,
        long_integer_mod_pow_gadget::LongIntegerModPowGadget,
    },
};

use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct PaillierBackend {
    pub minNumCipherChunks: i32,
    pub maxNumCipherChunks: i32,
}
impl PaillierBackend {
    const CHUNK_SIZE: i32 = LongElement::CHUNK_BITWIDTH; //120;
    pub fn new(
        keyBits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
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
            generator,
        )
    }
}
//impl AsymmetricConfig for CryptoBackend<Asymmetric<PaillierBackend>> {}
crate::impl_crypto_backend_configs_for!(PaillierBackend);
impl CryptoBackendConfig for CryptoBackend<Asymmetric<PaillierBackend>> {
    fn getKeyChunkSize(&self) -> i32 {
        PaillierBackend::CHUNK_SIZE
    }

    fn createEncryptionGadget(
        &mut self,
        plain: &TypedWire,
        keyName: &String,
        randomWires: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let key = self.getKey(keyName, generator.clone());
        let encodedPlain = self.encodeSignedToModN(plain, &key, generator.clone());
        let randArr = LongElement::newa(
            WireArray::new(randomWires.clone(), generator.clone().downgrade())
                .getBits(PaillierBackend::CHUNK_SIZE as usize, &None)
                .adjustLength(None, self.keyBits as usize),
            generator.clone().downgrade(),
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
impl HomomorphicBackend for &CryptoBackend<Asymmetric<PaillierBackend>> {
    fn doHomomorphicOpu(
        &self,
        op: char,
        arg: &HomomorphicInput,
        keyName: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        assert!(!arg.isPlain(), "arg");

        let nSquare = self.getNSquare(keyName);
        let cipherVal = self.toLongElement(arg, generator.clone());

        if op == '-' {
            // Enc(m, r)^(-1) = (g^m * r^n)^(-1) = (g^m)^(-1) * (r^n)^(-1) = g^(-m) * (r^(-1))^n = Enc(-m, r^(-1))
            let result = self.invert(&cipherVal, &nSquare, generator);
            self.toWireArray(&result, &format!("-({})", arg.getName()))
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
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        let nSquare = self.getNSquare(keyName);

        match op {
            '+' => {
                // Enc(m1, r1) * Enc(m2, r2) = (g^m1 * r1^n) * (g^m2 * r2^n) = g^(m1 + m2) * (r1 * r2)^n = Enc(m1 + m2, r1 * r2)
                let outputName = format!("({}) + ({})", lhs.getName(), rhs.getName());
                let lhsVal = self.toLongElement(&lhs, generator.clone());
                let rhsVal = self.toLongElement(&rhs, generator.clone());
                let result = self.mulMod(&lhsVal, &rhsVal, &nSquare, generator);
                self.toWireArray(&result, &outputName)
            }
            '-' => {
                // Enc(m1, r1) * Enc(m2, r2)^(-1) = Enc(m1 + (-m2), r1 * r2^(-1)) = Enc(m1 - m2, r1 * r2^(-1))
                let outputName = format!("({}) - ({})", lhs.getName(), rhs.getName());
                let lhsVal = self.toLongElement(&lhs, generator.clone());
                let rhsVal = self.toLongElement(&rhs, generator.clone());
                let result = self.mulMod(
                    &lhsVal,
                    &self.invert(&rhsVal, &nSquare, generator.clone()),
                    &nSquare,
                    generator,
                );
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
                    cipherVal = self.toLongElement(&rhs, generator.clone());
                } else if lhs.isCipher() && rhs.isPlain() {
                    cipherVal = self.toLongElement(&lhs, generator.clone());
                    plainWire = rhs.getPlain();
                } else {
                    panic!("Paillier multiplication requires exactly 1 plaintext argument");
                }

                let plainBits = plainWire.zkay_type.bitwidth;
                let plainBitWires = plainWire.wire.getBitWiresi(plainBits as u64, &None);
                let mut absPlainVal;
                if !plainWire.zkay_type.signed {
                    // Unsigned, easy , just do the multiplication.
                    absPlainVal =
                        LongElement::newa(plainBitWires.clone(), generator.clone().downgrade());
                } else {
                    // Signed. Multiply by the absolute value, later negate result if sign bit was set.
                    let twosComplement = plainWire.wire.invBits(plainBits as u64, &None).add(1);
                    let posValue =
                        LongElement::newa(plainBitWires.clone(), generator.clone().downgrade());
                    let negValue = LongElement::newa(
                        twosComplement.getBitWiresi(plainBits as u64, &None),
                        generator.clone().downgrade(),
                    );
                    let signBit = plainBitWires[plainBits as usize - 1].as_ref().unwrap();
                    absPlainVal = posValue.muxBit(&negValue, signBit);
                }
                let outputName = format!("({}) * ({})", lhs.getName(), rhs.getName());

                // Enc(m1, r1) ^ m2 = (g^m1 * r1^n) ^ m2 = (g^m1)^m2 * (r1^n)^m2 = g^(m1*m2) * (r1^m2)^n = Enc(m1 * m2, r1 ^ m2)
                let mut result = self.modPow(&cipherVal, &absPlainVal, plainBits, &nSquare);

                if plainWire.zkay_type.signed {
                    // Correct for sign
                    let signBit = plainBitWires[plainBits as usize - 1].clone().unwrap();
                    let negResult = self.invert(&result, &nSquare, generator);
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
        let n = self.getKey(keyName, self.generator.clone());
        let nSquareMaxBits = 2 * self.keyBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        n.clone().mul(&n).align(maxNumChunks as usize)
    }

    fn invert(
        &self,
        val: &LongElement,
        nSquare: &LongElement,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
        LongIntegerModInverseGadget::new(
            val.clone(),
            nSquare.clone(),
            true,
            &Some("Paillier negation".to_owned()),
            generator,
        )
        .getResult()
        .clone()
    }

    fn mulMod(
        &self,
        lhs: &LongElement,
        rhs: &LongElement,
        nSquare: &LongElement,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
        LongIntegerModGadget::new(
            lhs.clone().mul(rhs),
            nSquare.clone(),
            2 * self.keyBits,
            true,
            &Some("Paillier addition".to_owned()),
            generator,
        )
        .getRemainder()
        .clone()
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
            nSquare.clone(),
            2 * self.keyBits,
            rhsBits,
            &Some("Paillier multiplication".to_owned()),
            self.generator.clone(),
        )
        .getResult()
        .clone()
    }

    fn toLongElement(
        &self,
        input: &HomomorphicInput,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
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
        //bitWidths.last_mut().unwrap() =
        (2 * self.keyBits - (bitWidths.len() as i32 - 1) * PaillierBackend::CHUNK_SIZE) as u64;

        // Cipher could still be uninitialized-zero, which we need to fix
        self.uninitZeroToOne(&LongElement::new(wires, bitWidths, generator.downgrade()))
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
            .map(|w| {
                TypedWire::new(
                    w.clone().unwrap(),
                    uint256.clone(),
                    name.clone(),
                    &vec![],
                    self.generator.clone(),
                )
            })
            .collect()
    }

    fn uninitZeroToOne(&self, val: &LongElement) -> LongElement {
        // Uninitialized values have a ciphertext of all zeros, which is not a valid Paillier cipher.
        // Instead, replace those values with 1 == g^0 * 1^n = Enc(0, 1)
        let valIsZero = val.checkNonZero().invAsBit(&None);
        let oneIfAllZero = LongElement::new(
            vec![valIsZero],
            vec![1], // bit
            self.generator.clone().downgrade(),
        );
        val.clone().add(&oneIfAllZero)
    }

    fn encodeSignedToModN(
        &self,
        input: &TypedWire,
        key: &LongElement,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
        if input.zkay_type.signed {
            // Signed. Encode positive values as-is, negative values (-v) as (key - v)
            let bits = input.zkay_type.bitwidth;
            let inputBits = input.wire.getBitWiresi(bits as u64, &None);
            let signBit = inputBits[bits as usize - 1].clone().unwrap();

            let posValue = LongElement::newa(inputBits.clone(), generator.downgrade());
            let rawNegValue = LongElement::newa(
                input
                    .wire
                    .invBits(bits as u64, &None)
                    .add(1)
                    .getBitWiresi(bits as u64 + 1, &None),
                generator.downgrade(),
            );
            let negValue = key.clone().sub(&rawNegValue);

            posValue.muxBit(&negValue, &signBit)
        } else {
            // Unsigned, encode as-is, just convert the input wire to a LongElement
            LongElement::newa(
                input
                    .wire
                    .getBitWiresi(input.zkay_type.bitwidth as u64, &None),
                generator.downgrade(),
            )
        }
    }
}
