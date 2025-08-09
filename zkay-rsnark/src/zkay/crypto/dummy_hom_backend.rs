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
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::crypto_backend::Asymmetric;
use crate::zkay::crypto::crypto_backend::AsymmetricConfig;
use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfig;
use crate::zkay::crypto::homomorphic_backend::HomomorphicBackend;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget;
use crate::zkay::zkay_dummy_hom_encryption_gadget::ZkayDummyHomEncryptionGadget;
use crate::zkay::zkay_type::ZkayType;
use rccell::RcCell;
use std::ops::{Add, Mul, Neg, Sub};
#[derive(Debug, Clone)]
pub struct DummyHomBackend;

impl DummyHomBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(
        keyBits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(keyBits, Self, generator)
    }
}
impl AsymmetricConfig for CryptoBackend<Asymmetric<DummyHomBackend>> {}

impl CryptoBackendConfig for CryptoBackend<Asymmetric<DummyHomBackend>> {
    fn getKeyChunkSize(&self) -> i32 {
        DummyHomBackend::KEY_CHUNK_SIZE
    }
    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let encodedPlain = self.encodePlaintextIfSigned(plain);
        Box::new(ZkayDummyHomEncryptionGadget::new(
            encodedPlain,
            self.getKeyWire(key, generator.clone()),
            random.clone(),
            self.keyBits.clone(),
            desc,
            generator,
        ))
    }
}
impl CryptoBackend<Asymmetric<DummyHomBackend>> {
    fn getKeyWire(&self, keyName: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        let key = self.getKey(keyName, generator.clone());
        // let mut generator = self.generators();

        let keyArr = key.getBits().unwrap().packBitsIntoWords(256, &None);
        for i in 1..keyArr.len() {
            generator.addZeroAssertion(
                keyArr[i].as_ref().unwrap(),
                &Some("Dummy-hom enc pk valid".to_owned()),
            );
        }
        keyArr[0].clone().unwrap()
    }

    fn getCipherWire(&self, input: &HomomorphicInput, name: &String) -> WireType {
        // assert!(input.is_some(), "{name} is None");
        assert!(!input.isPlain(), "{name} is not a ciphertext");
        assert!(input.getLength() == 1, "{name} has invalid length");

        // Transform input 0 to ciphertext 0 (= encryption of 0); serialized inputs x+1 to ciphertext x
        let cipherWire = input.getCipher()[0].wire;
        let isNonZero = cipherWire.checkNonZero(&None);
        cipherWire.sub(isNonZero)
    }

    fn encodePlaintextIfSigned(&self, plain: &TypedWire) -> WireType {
        if plain.zkay_type.signed {
            // Signed: wrap negative values around the field prime instead of around 2^n
            let bits = plain.zkay_type.bitwidth as u64;
            let signBit = plain.wire.getBitWiresi(bits, &None)[bits as usize - 1]
                .clone()
                .unwrap();
            let negValue = plain.wire.invBits(bits, &None).add(1).negate(&None);
            signBit.mux(&negValue, &plain.wire)
        } else {
            // Unsigned values get encoded as-is
            plain.wire
        }
    }

    fn typedAsUint(&self, wire: &WireType, name: &String) -> Vec<TypedWire> {
        // Always zkay_type cipher wires as ZkUint(256)
        vec![TypedWire::new(
            wire.add(1),
            ZkayType::ZkUint(256),
            name.clone(),
            &vec![],
            self.generator.clone(),
        )]
    }
}

impl HomomorphicBackend for CryptoBackend<Asymmetric<DummyHomBackend>> {
    fn doHomomorphicOpu(
        &self,
        op: char,
        arg: &HomomorphicInput,
        keyName: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        let cipher = self.getCipherWire(arg, &"arg".to_owned());
        assert!(op == '-', "Unary operation {op} not supported");

        // -Enc(msg, p) = -(msg * p) = (-msg) * p = Enc(-msg, p)
        let minus = cipher.negate(&None);
        self.typedAsUint(&minus, &format!("-({})", arg.getName()))
    }

    fn doHomomorphicOp(
        &self,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
        keyName: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        match op {
            '+' => {
                // Enc(m1, p) + Enc(m2, p) = (m1 * p) + (m2 * p) = (m1 + m2) * p = Enc(m1 + m2, p)
                let l = self.getCipherWire(lhs, &"lhs".to_owned());
                let r = self.getCipherWire(rhs, &"rhs".to_owned());
                let sum = l.add(r);
                self.typedAsUint(&sum, &format!("({}) + ({})", lhs.getName(), rhs.getName()))
            }
            '-' => {
                // Enc(m1, p) - Enc(m2, p) = (m1 * p) - (m2 * p) = (m1 - m2) * p = Enc(m1 - m2, p)
                let l = self.getCipherWire(lhs, &"lhs".to_owned());
                let r = self.getCipherWire(rhs, &"rhs".to_owned());
                let diff = l.sub(r);
                self.typedAsUint(&diff, &format!("({}) - ({})", lhs.getName(), rhs.getName()))
            }
            '*' => {
                // Multiplication on additively homomorphic ciphertexts requires 1 ciphertext and 1 plaintext argument
                let mut plain;
                let mut cipher;
                // assert!(lhs.is_some(), "lhs is None");
                // assert!(rhs.is_some(), "rhs is None");
                if lhs.isPlain() && rhs.isCipher() {
                    plain = self.encodePlaintextIfSigned(&lhs.getPlain());
                    cipher = self.getCipherWire(rhs, &"rhs".to_owned());
                } else if lhs.isCipher() && rhs.isPlain() {
                    cipher = self.getCipherWire(lhs, &"lhs".to_owned());
                    plain = self.encodePlaintextIfSigned(&rhs.getPlain());
                } else {
                    panic!("DummyHom multiplication requires exactly 1 plaintext argument");
                }

                // Enc(m1, p) * m2 = (m1 * p) * m2 = (m1 * m2) * p = Enc(m1 * m2, p)
                let prod = cipher.mul(plain);
                self.typedAsUint(&prod, &format!("({}) - ({})", lhs.getName(), rhs.getName()))
            }
            _ => panic!("Binary operation {op} not supported"),
        }
    }

    fn doHomomorphicRerand(
        &self,
        arg: &Vec<TypedWire>,
        keyName: &String,
        randomness: &TypedWire,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        arg.clone()
    }
}
