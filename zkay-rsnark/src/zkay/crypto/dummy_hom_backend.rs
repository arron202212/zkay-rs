#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        auxiliary::long_element::LongElement,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    zkay::{
        crypto::{
            crypto_backend::Asymmetric,
            crypto_backend::CryptoBackend,
            crypto_backend::{CryptoBackendConfig, CryptoBackendConfigs},
            homomorphic_backend::HomomorphicBackend,
        },
        homomorphic_input::HomomorphicInput,
        typed_wire::TypedWire,
        zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget,
        zkay_dummy_hom_encryption_gadget::ZkayDummyHomEncryptionGadget,
        zkay_type::ZkayType,
    },
};

use rccell::RcCell;
use std::ops::{Add, Mul, Neg, Sub};
#[derive(Debug, Clone)]
pub struct DummyHomBackend;

impl DummyHomBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(
        key_bits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(key_bits, Self, generator)
    }
}
//impl AsymmetricConfig for CryptoBackend<Asymmetric<DummyHomBackend>> {}
crate::impl_crypto_backend_configs_for!(DummyHomBackend);
impl CryptoBackendConfig for CryptoBackend<Asymmetric<DummyHomBackend>> {
    fn get_key_chunk_size(&self) -> i32 {
        DummyHomBackend::KEY_CHUNK_SIZE
    }
    fn create_encryption_gadget(
        &mut self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let encoded_plain = self.encode_plaintext_if_signed(plain);
        Box::new(ZkayDummyHomEncryptionGadget::new_with_option(
            encoded_plain,
            self.get_key_wire(key, generator.clone()),
            random.clone(),
            self.key_bits.clone(),
            desc,
            generator,
        ))
    }
}
impl CryptoBackend<Asymmetric<DummyHomBackend>> {
    fn get_key_wire(&self, key_name: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        let key = self.get_key(key_name, generator.clone());
        let key_arr = key.get_bits().unwrap().pack_bits_into_words(256);
        for i in 1..key_arr.len() {
            CircuitGenerator::add_zero_assertion_with_str(
                generator.clone(),
                key_arr[i].as_ref().unwrap(),
                "Dummy-hom enc pk valid",
            );
        }
        key_arr[0].clone().unwrap()
    }

    fn get_cipher_wire(&self, input: &HomomorphicInput, name: &String) -> WireType {
        // assert!(input.is_some(), "{name} is None");
        assert!(!input.is_plain(), "{name} is not a ciphertext");
        assert!(input.get_length() == 1, "{name} has invalid length");

        // Transform input 0 to ciphertext 0 (= encryption of 0); serialized inputs x+1 to ciphertext x
        let cipher_wire = input.get_cipher()[0].wire.clone();
        let is_non_zero = cipher_wire.check_non_zero();
        cipher_wire.sub(is_non_zero)
    }

    fn encode_plaintext_if_signed(&self, plain: &TypedWire) -> WireType {
        if plain.zkay_type.signed {
            // Signed: wrap negative values around the field prime instead of around 2^n
            let bits = plain.zkay_type.bitwidth as u64;
            let sign_bit = plain.wire.get_bit_wiresi(bits)[bits as usize - 1]
                .clone()
                .unwrap();
            let neg_value = plain.wire.inv_bits(bits).add(1).negate();
            sign_bit.mux(&neg_value, &plain.wire)
        } else {
            // Unsigned values get encoded as-is
            plain.wire.clone()
        }
    }

    fn typed_as_uint(&self, wire: &WireType, name: &String) -> Vec<TypedWire> {
        // Always zkay_type cipher wires as zk_uint(256)
        vec![TypedWire::new(
            wire.clone().add(1),
            ZkayType::zk_uint(256),
            name.clone(),
            &vec![],
            self.generator.clone(),
        )]
    }
}

impl HomomorphicBackend for &CryptoBackend<Asymmetric<DummyHomBackend>> {
    fn do_homomorphic_opu(
        &self,
        op: char,
        arg: &HomomorphicInput,
        key_name: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        let cipher = self.get_cipher_wire(arg, &"arg".to_owned());
        assert!(op == '-', "Unary operation {op} not supported");

        // -Enc(msg, p) = -(msg * p) = (-msg) * p = Enc(-msg, p)
        let minus = cipher.negate();
        self.typed_as_uint(&minus, &format!("-({})", arg.get_name()))
    }

    fn do_homomorphic_op(
        &self,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
        key_name: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        match op {
            '+' => {
                // Enc(m1, p) + Enc(m2, p) = (m1 * p) + (m2 * p) = (m1 + m2) * p = Enc(m1 + m2, p)
                let l = self.get_cipher_wire(lhs, &"lhs".to_owned());
                let r = self.get_cipher_wire(rhs, &"rhs".to_owned());
                let sum = l.add(r);
                self.typed_as_uint(
                    &sum,
                    &format!("({}) + ({})", lhs.get_name(), rhs.get_name()),
                )
            }
            '-' => {
                // Enc(m1, p) - Enc(m2, p) = (m1 * p) - (m2 * p) = (m1 - m2) * p = Enc(m1 - m2, p)
                let l = self.get_cipher_wire(lhs, &"lhs".to_owned());
                let r = self.get_cipher_wire(rhs, &"rhs".to_owned());
                let diff = l.sub(r);
                self.typed_as_uint(
                    &diff,
                    &format!("({}) - ({})", lhs.get_name(), rhs.get_name()),
                )
            }
            '*' => {
                // Multiplication on additively homomorphic ciphertexts requires 1 ciphertext and 1 plaintext argument
                let mut plain;
                let mut cipher;
                // assert!(lhs.is_some(), "lhs is None");
                // assert!(rhs.is_some(), "rhs is None");
                if lhs.is_plain() && rhs.is_cipher() {
                    plain = self.encode_plaintext_if_signed(&lhs.get_plain());
                    cipher = self.get_cipher_wire(rhs, &"rhs".to_owned());
                } else if lhs.is_cipher() && rhs.is_plain() {
                    cipher = self.get_cipher_wire(lhs, &"lhs".to_owned());
                    plain = self.encode_plaintext_if_signed(&rhs.get_plain());
                } else {
                    panic!("DummyHom multiplication requires exactly 1 plaintext argument");
                }

                // Enc(m1, p) * m2 = (m1 * p) * m2 = (m1 * m2) * p = Enc(m1 * m2, p)
                let prod = cipher.mul(plain);
                self.typed_as_uint(
                    &prod,
                    &format!("({}) - ({})", lhs.get_name(), rhs.get_name()),
                )
            }
            _ => panic!("Binary operation {op} not supported"),
        }
    }

    fn do_homomorphic_rerand(
        &self,
        arg: &Vec<TypedWire>,
        key_name: &String,
        randomness: &TypedWire,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        arg.clone()
    }
}
