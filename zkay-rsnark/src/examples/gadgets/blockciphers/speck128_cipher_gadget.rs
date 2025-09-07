#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::{
    operations::gadget::{Gadget, GadgetConfig},
    structure::{
        circuit_generator::{
            CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
            get_active_circuit_generator,
        },
        wire::WireConfig,
        wire_type::WireType,
    },
};

use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Neg, Rem, Sub},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;
//  * Implements the Speck lightweight block cipher
//  * https://eprint.iacr.org/2015/585.pdf

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct Speck128CipherGadget {
    pub plaintext: Vec<Option<WireType>>,
    pub expanded_key: Vec<Option<WireType>>,
    pub ciphertext: Vec<Option<WireType>>,
}
impl Speck128CipherGadget {
    //
    //@param inputs
    //           : Array of 2 64-bit elements.
    //@param expanded_key
    //           : Array of 32 64-bit elements. (Call expandKey(..))
    //@param desc

    pub fn new(
        plaintext: Vec<Option<WireType>>,
        expanded_key: Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            plaintext.len() == 2 && expanded_key.len() == 32,
            "Invalid Input"
        );
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plaintext,
                expanded_key,
                ciphertext: vec![],
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<Speck128CipherGadget> {
    fn build_circuit(&mut self) {
        let (mut x, mut y) = (
            self.t.plaintext[1].clone().unwrap(),
            self.t.plaintext[0].clone().unwrap(),
        );

        for i in 0..=31 {
            x = x.rotate_right(64, 8, &None).add(&y);
            x = x.trim_bits(65, 64, &None);
            x = x.xor_bitwises(self.t.expanded_key[i].as_ref().unwrap(), 64, &None);
            y = y.rotate_left(64, 3, &None).xor_bitwises(&x, 64, &None);
        }
        self.t.ciphertext = vec![Some(y), Some(x)];
    }

    //
    //@param key
    //           : 2 64-bit words
    //@return

    pub fn expandKey(
        key: &Vec<Option<WireType>>,
        generator: &RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        let mut k = vec![None; 32];
        let mut l = vec![None; 32];
        k[0] = key[0].clone();
        l[0] = key[1].clone();
        for i in 0..=32 - 2 {
            l[i + 1] = Some(
                k[i].clone()
                    .unwrap()
                    .add(l[i].as_ref().unwrap().rotate_left(64, 56, &None)),
            );
            l[i + 1] = Some(l[i + 1].as_ref().unwrap().trim_bits(65, 64, &None));
            l[i + 1] = Some(l[i + 1].as_ref().unwrap().xor_bitwises(
                &CircuitGenerator::create_constant_wirei(generator.clone(), i as i64, &None),
                64,
                &None,
            ));
            k[i + 1] = Some(
                k[i].as_ref()
                    .unwrap()
                    .rotate_left(64, 3, &None)
                    .xor_bitwises(l[i + 1].as_ref().unwrap(), 64, &None),
            );
        }
        k
    }
}
impl GadgetConfig for Gadget<Speck128CipherGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.ciphertext
    }
}
