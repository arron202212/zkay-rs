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
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::CircuitGenerator, wire::WireConfig, wire_array::WireArray,
            wire_type::WireType,
        },
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
        zkay_baby_jub_jub_gadget::{JubJubPoint, ZkayBabyJubJubGadget},
        zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget,
        zkay_elgamal_add_gadget::ZkayElgamalAddGadget,
        zkay_elgamal_dec_gadget::ZkayElgamalDecGadget,
        zkay_elgamal_enc_gadget::ZkayElgamalEncGadget,
        zkay_elgamal_mul_gadget::ZkayElgamalMulGadget,
        zkay_elgamal_rerand_gadget::ZkayElgamalRerandGadget,
        zkay_type::ZkayType,
    },
};

use rccell::{RcCell, WeakCell};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct ElgamalBackend;

impl ElgamalBackend {
    const EC_COORD_BITS: i32 = 254; // a BabyJubJub affine coordinate fits into 254 bits
    const KEY_CHUNK_SIZE: i32 = 256; // needs to be a multiple of 8
    const RND_CHUNK_SIZE: i32 = 256;

    pub fn new(
        key_bits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        // pub  key must be a BabyJubJub point (two coordinates)
        assert!(
            key_bits == 2 * Self::EC_COORD_BITS,
            "pub  key size mismatch"
        );
        Asymmetric::<Self>::new(key_bits, Self, generator)
    }
}

impl CryptoBackendConfigs for CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn is_symmetric(&self) -> bool {
        false
    }
    fn uses_decryption_gadget(&self) -> bool {
        // randomness is not extractable from an ElGamal ciphertext, so need a separate
        // gadget for decryption
        true
    }

    fn add_key(
        &mut self,
        key_name: &String,
        key_wires: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) {
        // elgamal does not require a bit-representation of the pub  key, so store it directly
        self.t.keys.insert(
            key_name.clone(),
            WireArray::new(key_wires.clone(), generator.downgrade()),
        );
    }
    fn create_decryption_gadget(
        &self,
        plain: &TypedWire,
        cipher: &Vec<Option<WireType>>,
        pk_name: &String,
        sk: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let pk_array = self.get_key_array(pk_name);
        let pk = JubJubPoint::new(pk_array[0].clone().unwrap(), pk_array[1].clone().unwrap());
        let c1 = JubJubPoint::new(cipher[0].clone().unwrap(), cipher[1].clone().unwrap());
        let c2 = JubJubPoint::new(cipher[2].clone().unwrap(), cipher[3].clone().unwrap());
        let sk_bits = WireArray::new(sk.clone(), generator.clone().downgrade())
            .get_bits(ElgamalBackend::RND_CHUNK_SIZE as usize, &None)
            .as_array()
            .clone();
        Box::new(ZkayElgamalDecGadget::new(
            pk,
            sk_bits,
            c1,
            c2,
            plain.wire.clone(),
            generator,
        ))
    }
    fn set_key_pair(
        &mut self,
        my_pk: &WireType,
        my_sk: &WireType,
        generator: RcCell<CircuitGenerator>,
    ) {
        panic!("set_key_pair no in Asymmetric");
    }
}
impl CryptoBackendConfig for CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn get_key_chunk_size(&self) -> i32 {
        ElgamalBackend::KEY_CHUNK_SIZE
    }
    fn create_encryption_gadget(
        &mut self,
        plain: &TypedWire,
        key_name: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let pk_array = self.get_key_array(key_name);
        let pk = JubJubPoint::new(pk_array[0].clone().unwrap(), pk_array[1].clone().unwrap());
        let random_array = WireArray::new(random.clone(), generator.clone().downgrade())
            .get_bits(ElgamalBackend::RND_CHUNK_SIZE as usize, &None)
            .as_array()
            .clone();
        assert!(
            plain.zkay_type.bitwidth <= 32,
            "plaintext must be at most 32 bits for elgamal backend"
        );
        Box::new(ZkayElgamalEncGadget::new(
            plain
                .wire
                .get_bit_wiresi(plain.zkay_type.bitwidth as u64, &None)
                .as_array()
                .clone(),
            pk,
            random_array.clone(),
            generator,
        ))
    }
}
impl CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn to_typed_wire_array(&self, wires: &Vec<Option<WireType>>, name: &String) -> Vec<TypedWire> {
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

    fn from_typed_wire_array(&self, typed_wires: &Vec<TypedWire>) -> Vec<Option<WireType>> {
        let uint256 = ZkayType::zk_uint(256);
        typed_wires
            .iter()
            .map(|w| {
                ZkayType::check_type(&uint256, &w.zkay_type);
                Some(w.wire.clone())
            })
            .collect()
    }

    fn parse_jub_jub_point(&self, wire: &Vec<Option<WireType>>, offset: usize) -> JubJubPoint {
        JubJubPoint::new(
            wire[offset].clone().unwrap(),
            wire[offset + 1].clone().unwrap(),
        )
    }

    fn uninit_zero_to_identity(&self, p: &JubJubPoint) -> JubJubPoint {
        // Uninitialized values have a ciphertext of all zeroes, which is not a valid ElGamal cipher.
        // Instead, replace those values with the point at infinity (0, 1).
        let one_if_both_zero =
            p.x.check_non_zero(&None)
                .orw(&p.y.check_non_zero(&None), &None)
                .inv_as_bit(&None)
                .unwrap();
        JubJubPoint::new(p.x.clone(), p.y.clone().add(&one_if_both_zero))
    }
}
impl HomomorphicBackend for &CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn do_homomorphic_op(
        &self,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
        key_name: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        if (op == '+') || (op == '-') {
            // for (c1, c2) = Enc(m1, r1)
            //     (d1, d2) = Enc(m2, r2)
            //     e1 = c1 + d1
            //     e2 = c2 + d2
            // it is (e1, e2) = Enc(m1 + m2, r1 + r2)
            let output_name = format!("({}) + ({})", lhs.get_name(), rhs.get_name());

            let lhs_twires = lhs.get_cipher();
            let rhs_twires = rhs.get_cipher();

            // sanity checks
            assert!(lhs_twires.len() == 4); // 4 BabyJubJub coordinates
            assert!(rhs_twires.len() == 4); // 4 BabyJubJub coordinates
            let lhs_wires = self.from_typed_wire_array(&lhs_twires);
            let rhs_wires = self.from_typed_wire_array(&rhs_twires);

            let mut c1 = self.parse_jub_jub_point(&lhs_wires, 0);
            let mut c2 = self.parse_jub_jub_point(&lhs_wires, 2);
            let mut d1 = self.parse_jub_jub_point(&rhs_wires, 0);
            let mut d2 = self.parse_jub_jub_point(&rhs_wires, 2);

            c1 = self.uninit_zero_to_identity(&c1);
            c2 = self.uninit_zero_to_identity(&c2);
            d1 = self.uninit_zero_to_identity(&d1);
            d2 = self.uninit_zero_to_identity(&d2);

            if op == '-' {
                d1.x = d1.x.negate(&None);
                d2.x = d2.x.negate(&None);
            }

            let gadget = ZkayElgamalAddGadget::new(c1, c2, d1, d2, generator);
            self.to_typed_wire_array(gadget.get_output_wires(), &output_name)
        } else if op == '*' {
            let output_name = format!("({}) * ({})", lhs.get_name(), rhs.get_name());

            let mut plain_wire;
            let mut cipher_twires;
            if lhs.is_plain() && rhs.is_cipher() {
                plain_wire = lhs.get_plain();
                cipher_twires = rhs.get_cipher();
            } else if lhs.is_cipher() && rhs.is_plain() {
                cipher_twires = lhs.get_cipher();
                plain_wire = rhs.get_plain();
            } else {
                panic!("Elgamal multiplication requires exactly 1 plaintext argument");
            }

            let cipher_wires = self.from_typed_wire_array(&cipher_twires);
            let mut c1 = self.parse_jub_jub_point(&cipher_wires, 0);
            let mut c2 = self.parse_jub_jub_point(&cipher_wires, 2);

            c1 = self.uninit_zero_to_identity(&c1);
            c2 = self.uninit_zero_to_identity(&c2);

            let gadget = ZkayElgamalMulGadget::new(
                c1,
                c2,
                plain_wire.wire.get_bit_wiresi(32, &None).as_array().clone(),
                generator,
            );
            self.to_typed_wire_array(gadget.get_output_wires(), &output_name)
        } else {
            panic!("Binary operation {op} not supported");
        }
    }

    fn do_homomorphic_rerand(
        &self,
        arg: &Vec<TypedWire>,
        key_name: &String,
        randomness: &TypedWire,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        let output_name = format!("rerand({})", arg[0].name);

        // parse argument
        let arg_wires = self.from_typed_wire_array(&arg);
        let mut c1 = self.parse_jub_jub_point(&arg_wires, 0);
        let mut c2 = self.parse_jub_jub_point(&arg_wires, 2);
        c1 = self.uninit_zero_to_identity(&c1);
        c2 = self.uninit_zero_to_identity(&c2);

        // parse key and randomness
        let pk_array = self.get_key_array(key_name);
        let pk = JubJubPoint::new(pk_array[0].clone().unwrap(), pk_array[1].clone().unwrap());
        let random_array = randomness
            .wire
            .get_bit_wiresi(ElgamalBackend::RND_CHUNK_SIZE as u64, &None)
            .as_array()
            .clone();

        // create gadget
        let gadget = ZkayElgamalRerandGadget::new(c1, c2, pk, random_array.clone(), generator);
        self.to_typed_wire_array(gadget.get_output_wires(), &output_name)
    }
}
