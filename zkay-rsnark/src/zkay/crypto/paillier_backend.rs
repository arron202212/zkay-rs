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
            circuit_generator::CircuitGenerator, wire::WireConfig, wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::math::{
        long_integer_division::LongIntegerDivisionConfig,
        long_integer_mod_gadget::LongIntegerModGadget,
        long_integer_mod_inverse_gadget::LongIntegerModInverseGadget,
        long_integer_mod_pow_gadget::LongIntegerModPowGadget,
    },
    zkay::{
        crypto::{
            crypto_backend::{
                Asymmetric, CryptoBackend, CryptoBackendConfig, CryptoBackendConfigs,
            },
            homomorphic_backend::HomomorphicBackend,
        },
        homomorphic_input::HomomorphicInput,
        typed_wire::TypedWire,
        zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget,
        zkay_paillier_fast_enc_gadget::ZkayPaillierFastEncGadget,
        zkay_type::ZkayType,
    },
};

use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct PaillierBackend {
    pub min_num_cipher_chunks: i32,
    pub max_num_cipher_chunks: i32,
}
impl PaillierBackend {
    const CHUNK_SIZE: i32 = LongElement::CHUNK_BITWIDTH; //120;
    pub fn new(
        key_bits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        // Same chunk size for key, randomness, and ciphertext

        //  {
        // 	if CHUNK_SIZE != LongElement::CHUNK_BITWIDTH {
        // 		assert!("Paillier chunk size must match LongElement::CHUNK_BITWIDTH.\n" +
        // 				"If LongElement::CHUNK_BITWIDTH needs to be changed, change this _and_ meta.py in jsnark!");
        // 	}
        // }

        // //super(key_bits); // key_bits = bits of n
        assert!(
            key_bits > Self::CHUNK_SIZE,
            "Key size too small ( {key_bits}  <  {}  bits)",
            Self::CHUNK_SIZE
        );

        // n^2 has either length (2 * key_bits - 1) or (2 * key_bits) bits
        // min_num_cipher_chunks = ceil((2 * key_bits - 1) / CHUNK_SIZE)
        // max_num_cipher_chunks = ceil((2 * key_bits) / CHUNK_SIZE)
        let min_n_square_bits = 2 * key_bits - 1;
        Asymmetric::<Self>::new(
            key_bits,
            Self {
                min_num_cipher_chunks: (min_n_square_bits + Self::CHUNK_SIZE - 1)
                    / Self::CHUNK_SIZE,
                max_num_cipher_chunks: (min_n_square_bits + Self::CHUNK_SIZE) / Self::CHUNK_SIZE,
            },
            generator,
        )
    }
}
//impl AsymmetricConfig for CryptoBackend<Asymmetric<PaillierBackend>> {}
crate::impl_crypto_backend_configs_for!(PaillierBackend);
impl CryptoBackendConfig for CryptoBackend<Asymmetric<PaillierBackend>> {
    fn get_key_chunk_size(&self) -> i32 {
        PaillierBackend::CHUNK_SIZE
    }

    fn create_encryption_gadget(
        &mut self,
        plain: &TypedWire,
        key_name: &String,
        random_wires: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let key = self.get_key(key_name, generator.clone());
        let encoded_plain = self.encode_signed_to_mod_n(plain, &key, generator.clone());
        let rand_arr = LongElement::newa(
            WireArray::new(random_wires.clone(), generator.clone().downgrade())
                .get_bits(PaillierBackend::CHUNK_SIZE as usize, &None)
                .adjust_length(None, self.key_bits as usize),
            generator.clone().downgrade(),
        );
        let random = self.uninit_zero_to_one(&rand_arr); // Also replace randomness 0 with 1 (for uninit ciphers)
        Box::new(ZkayPaillierFastEncGadget::new(
            key,
            self.key_bits,
            encoded_plain,
            random,
            desc,
            generator,
        ))
    }
}
impl HomomorphicBackend for &CryptoBackend<Asymmetric<PaillierBackend>> {
    fn do_homomorphic_opu(
        &self,
        op: char,
        arg: &HomomorphicInput,
        key_name: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        assert!(!arg.is_plain(), "arg");

        let n_square = self.get_n_square(key_name);
        let cipher_val = self.to_long_element(arg, generator.clone());

        if op == '-' {
            // Enc(m, r)^(-1) = (g^m * r^n)^(-1) = (g^m)^(-1) * (r^n)^(-1) = g^(-m) * (r^(-1))^n = Enc(-m, r^(-1))
            let result = self.invert(&cipher_val, &n_square, generator);
            self.to_wire_array(&result, &format!("-({})", arg.get_name()))
        } else {
            panic!("Unary operation {op} not supported");
        }
    }

    fn do_homomorphic_op(
        &self,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
        key_name: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        let n_square = self.get_n_square(key_name);

        match op {
            '+' => {
                // Enc(m1, r1) * Enc(m2, r2) = (g^m1 * r1^n) * (g^m2 * r2^n) = g^(m1 + m2) * (r1 * r2)^n = Enc(m1 + m2, r1 * r2)
                let output_name = format!("({}) + ({})", lhs.get_name(), rhs.get_name());
                let lhs_val = self.to_long_element(&lhs, generator.clone());
                let rhs_val = self.to_long_element(&rhs, generator.clone());
                let result = self.mul_mod(&lhs_val, &rhs_val, &n_square, generator);
                self.to_wire_array(&result, &output_name)
            }
            '-' => {
                // Enc(m1, r1) * Enc(m2, r2)^(-1) = Enc(m1 + (-m2), r1 * r2^(-1)) = Enc(m1 - m2, r1 * r2^(-1))
                let output_name = format!("({}) - ({})", lhs.get_name(), rhs.get_name());
                let lhs_val = self.to_long_element(&lhs, generator.clone());
                let rhs_val = self.to_long_element(&rhs, generator.clone());
                let result = self.mul_mod(
                    &lhs_val,
                    &self.invert(&rhs_val, &n_square, generator.clone()),
                    &n_square,
                    generator,
                );
                self.to_wire_array(&result, &output_name)
            }
            '*' => {
                // Multiplication on additively homomorphic ciphertexts requires 1 ciphertext and 1 plaintext argument
                let mut cipher_val;
                let mut plain_wire;

                // assert!(lhs.is_some(), "lhs is None");
                // assert!(rhs.is_some(), "rhs is None");
                if lhs.is_plain() && rhs.is_cipher() {
                    plain_wire = lhs.get_plain();
                    cipher_val = self.to_long_element(&rhs, generator.clone());
                } else if lhs.is_cipher() && rhs.is_plain() {
                    cipher_val = self.to_long_element(&lhs, generator.clone());
                    plain_wire = rhs.get_plain();
                } else {
                    panic!("Paillier multiplication requires exactly 1 plaintext argument");
                }

                let plain_bits = plain_wire.zkay_type.bitwidth;
                let plain_bit_wires = plain_wire.wire.get_bit_wiresi(plain_bits as u64);
                let mut abs_plain_val;
                if !plain_wire.zkay_type.signed {
                    // Unsigned, easy , just do the multiplication.
                    abs_plain_val =
                        LongElement::newa(plain_bit_wires.clone(), generator.clone().downgrade());
                } else {
                    // Signed. Multiply by the absolute value, later negate result if sign bit was set.
                    let twos_complement = plain_wire.wire.inv_bits(plain_bits as u64).add(1);
                    let pos_value =
                        LongElement::newa(plain_bit_wires.clone(), generator.clone().downgrade());
                    let neg_value = LongElement::newa(
                        twos_complement.get_bit_wiresi(plain_bits as u64),
                        generator.clone().downgrade(),
                    );
                    let sign_bit = plain_bit_wires[plain_bits as usize - 1].as_ref().unwrap();
                    abs_plain_val = pos_value.mux_bit(&neg_value, sign_bit);
                }
                let output_name = format!("({}) * ({})", lhs.get_name(), rhs.get_name());

                // Enc(m1, r1) ^ m2 = (g^m1 * r1^n) ^ m2 = (g^m1)^m2 * (r1^n)^m2 = g^(m1*m2) * (r1^m2)^n = Enc(m1 * m2, r1 ^ m2)
                let mut result = self.mod_pow(&cipher_val, &abs_plain_val, plain_bits, &n_square);

                if plain_wire.zkay_type.signed {
                    // Correct for sign
                    let sign_bit = plain_bit_wires[plain_bits as usize - 1].clone().unwrap();
                    let neg_result = self.invert(&result, &n_square, generator);
                    result = result.mux_bit(&neg_result, &sign_bit);
                }

                self.to_wire_array(&result, &output_name)
            }
            _ => panic!("Binary operation  {op} not supported"),
        }
    }
}
impl CryptoBackend<Asymmetric<PaillierBackend>> {
    fn get_n_square(&self, key_name: &String) -> LongElement {
        let n = self.get_key(key_name, self.generator.clone());
        let n_square_max_bits = 2 * self.key_bits; // Maximum bit length of n^2
        let max_num_chunks =
            (n_square_max_bits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        n.clone().mul(&n).align(max_num_chunks as usize)
    }

    fn invert(
        &self,
        val: &LongElement,
        n_square: &LongElement,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
        LongIntegerModInverseGadget::new(
            val.clone(),
            n_square.clone(),
            true,
            &Some("Paillier negation".to_owned()),
            generator,
        )
        .get_result()
        .clone()
    }

    fn mul_mod(
        &self,
        lhs: &LongElement,
        rhs: &LongElement,
        n_square: &LongElement,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
        LongIntegerModGadget::new(
            lhs.clone().mul(rhs),
            n_square.clone(),
            2 * self.key_bits,
            true,
            &Some("Paillier addition".to_owned()),
            generator,
        )
        .get_remainder()
        .clone()
    }

    fn mod_pow(
        &self,
        lhs: &LongElement,
        rhs: &LongElement,
        rhs_bits: i32,
        n_square: &LongElement,
    ) -> LongElement {
        LongIntegerModPowGadget::new(
            lhs.clone(),
            rhs.clone(),
            n_square.clone(),
            2 * self.key_bits,
            rhs_bits,
            &Some("Paillier multiplication".to_owned()),
            self.generator.clone(),
        )
        .get_result()
        .clone()
    }

    fn to_long_element(
        &self,
        input: &HomomorphicInput,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
        assert!(!input.is_plain(), "Input None or not ciphertext");
        let cipher = input.get_cipher();
        assert!(
            cipher.len() >= self.t.t.min_num_cipher_chunks as usize
                && cipher.len() <= self.t.t.max_num_cipher_chunks as usize,
            "Ciphertext has invalid length {}",
            cipher.len()
        );

        // Ciphertext inputs seem to be passed as zk_uint(256); sanity check to make sure we got that.
        let uint256 = ZkayType::zk_uint(256);
        for cipher_wire in cipher {
            ZkayType::check_type(&uint256, &cipher_wire.zkay_type);
        }

        // Input is a Paillier ciphertext - front-end must already check that this is the
        let wires: Vec<_> = cipher.iter().map(|c| Some(c.wire.clone())).collect();

        let mut bit_widths = vec![PaillierBackend::CHUNK_SIZE as u64; wires.len()];
        //bit_widths.last_mut().unwrap() =
        (2 * self.key_bits - (bit_widths.len() as i32 - 1) * PaillierBackend::CHUNK_SIZE) as u64;

        // Cipher could still be uninitialized-zero, which we need to fix
        self.uninit_zero_to_one(&LongElement::new(wires, bit_widths, generator.downgrade()))
    }

    fn to_wire_array(&self, value: &LongElement, name: &String) -> Vec<TypedWire> {
        // First, sanity check that the result has at most max_num_cipher_chunks wires of at most CHUNK_SIZE bits each
        assert!(
            value.get_size() <= self.t.t.max_num_cipher_chunks as usize,
            "Paillier output contains too many wires"
        );
        assert!(
            value
                .get_current_bitwidth()
                .iter()
                .all(|&bit_width| bit_width <= PaillierBackend::CHUNK_SIZE as u64),
            "Paillier output cipher bit width too large"
        );

        // If ok, wrap the output wires in TypedWire. As with the input, treat ciphertexts as zk_uint(256).
        let wires = value.get_array();
        let uint256 = ZkayType::zk_uint(256);
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

    fn uninit_zero_to_one(&self, val: &LongElement) -> LongElement {
        // Uninitialized values have a ciphertext of all zeros, which is not a valid Paillier cipher.
        // Instead, replace those values with 1 == g^0 * 1^n = Enc(0, 1)
        let val_is_zero = val.check_non_zero().inv_as_bit();
        let one_if_all_zero = LongElement::new(
            vec![val_is_zero],
            vec![1], // bit
            self.generator.clone().downgrade(),
        );
        val.clone().add(&one_if_all_zero)
    }

    fn encode_signed_to_mod_n(
        &self,
        input: &TypedWire,
        key: &LongElement,
        generator: RcCell<CircuitGenerator>,
    ) -> LongElement {
        if input.zkay_type.signed {
            // Signed. Encode positive values as-is, negative values (-v) as (key - v)
            let bits = input.zkay_type.bitwidth;
            let input_bits = input.wire.get_bit_wiresi(bits as u64);
            let sign_bit = input_bits[bits as usize - 1].clone().unwrap();

            let pos_value = LongElement::newa(input_bits.clone(), generator.downgrade());
            let raw_neg_value = LongElement::newa(
                input
                    .wire
                    .inv_bits(bits as u64)
                    .add(1)
                    .get_bit_wiresi(bits as u64 + 1),
                generator.downgrade(),
            );
            let neg_value = key.clone().sub(&raw_neg_value);

            pos_value.mux_bit(&neg_value, &sign_bit)
        } else {
            // Unsigned, encode as-is, just convert the input wire to a LongElement
            LongElement::newa(
                input.wire.get_bit_wiresi(input.zkay_type.bitwidth as u64),
                generator.downgrade(),
            )
        }
    }
}
