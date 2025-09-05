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
        structure::{circuit_generator::CircuitGenerator, wire::WireConfig, wire_type::WireType},
    },
    zkay::zkay_baby_jub_jub_gadget::{JubJubPoint,ZkayBabyJubJubGadget},
};

use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

//  * Dummy encryption gadget whose ciphertext is additively homomorphic.
//  * Key: Some prime number p smaller than the field prime.
//  * Encryption: Enc(msg, p) = msg * p mod field_prime.
//  * Decryption: Dec(cipher) = cipher * p^-1 mod field_prime.
//  * Additive homomorphism: Enc(m1, p) + Enc(m2, p)     (all mod field_prime)
//  *                        = (m1 * p) + (m2 * p)
//  *                        = (m1 + m2) * p
//  *                        = Enc(m1 + m2, p)

#[derive(Debug, Clone)]
pub struct ZkayDummyHomEncryptionGadget {
    pub pk: WireType,
    pub plain: WireType,
    pub cipher: Vec<Option<WireType>>,
}
impl ZkayDummyHomEncryptionGadget {
    pub fn new(
        plain: WireType,
        pk: WireType,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        // assert!(plain, "plain");
        // assert!(pk, "pk");
        // assert!(rnd, "rnd");
        assert!(rnd.len() <= 1, "Randomness wire array too long");

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plain,
                pk,
                cipher: vec![None; 1],
            },
        );
        _self.buildCircuit();
        _self
    }
}

impl Gadget<ZkayDummyHomEncryptionGadget> {
    fn buildCircuit(&mut self) {
        self.t.cipher[0] = Some(
            self.t
                .plain
                .mulw(&self.t.pk, &Some("plain * pk".to_owned()))
                .add(1),
        );
    }
}

impl GadgetConfig for Gadget<ZkayDummyHomEncryptionGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
