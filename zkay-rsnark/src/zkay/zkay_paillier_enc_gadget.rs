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
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_division::LongIntegerDivisionConfig;
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;
use crate::examples::gadgets::math::long_integer_mod_inverse_gadget::LongIntegerModInverseGadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget::LongIntegerModPowGadget;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct ZkayPaillierEncGadget {
    pub n: LongElement,
    pub nSquare: LongElement,
    pub nBits: i32,
    pub nSquareMaxBits: i32,
    pub g: LongElement,
    pub plain: LongElement,
    pub random: LongElement,
    pub cipher: Option<LongElement>,
}
impl ZkayPaillierEncGadget {
    pub fn new(
        n: LongElement,
        nBits: i32,
        g: LongElement,
        plain: LongElement,
        random: LongElement,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let nSquareMaxBits = 2 * nBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        let nSquare = n.clone().mul(&n).align(maxNumChunks as usize);
        println!("=====random.array========={:?}", random.array);
        let mut _self = Gadget::<Self> {
            generator,
            description: desc.clone().unwrap_or(String::new()),
            t: Self {
                n,
                nSquare,
                nBits,
                nSquareMaxBits,
                g,
                plain,
                random,
                cipher: None,
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayPaillierEncGadget> {
    fn buildCircuit(&mut self) {
        let nSquareMinBits = 2 * self.t.nBits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        println!(
            "===t==random.array===buildCircuit======{:?}",
            self.t.random.array
        );
        let randInv = LongIntegerModInverseGadget::new(
            self.t.random.clone(),
            self.t.n.clone(),
            false,
            &None,
            self.generator.clone(),
        )
        .getResult()
        .clone();
        let generators = self.generator.borrow().clone();
        generators.addOneAssertion(&randInv.checkNonZero(), &None);
        // let c = g^m * r^n mod n^2
        let gPowPlain = LongIntegerModPowGadget::new(
            self.t.g.clone(),
            self.t.plain.clone(),
            self.t.nSquare.clone(),
            nSquareMinBits,
            self.t.nBits,
            &Some("g^m".to_owned()),
            self.generator.clone(),
        )
        .getResult()
        .clone();
        let randPowN = LongIntegerModPowGadget::new(
            self.t.random.clone(),
            self.t.n.clone(),
            self.t.nSquare.clone(),
            nSquareMinBits,
            self.t.nBits,
            &Some("r^m".to_owned()),
            self.generator.clone(),
        )
        .getResult()
        .clone();
        let product = gPowPlain.clone().mul(&randPowN);
        self.t.cipher = Some(
            LongIntegerModGadget::new(
                product,
                self.t.nSquare.clone(),
                nSquareMinBits,
                true,
                &Some("g^m * r^n mod n^2".to_owned()),
                self.generator.clone(),
            )
            .getRemainder()
            .clone(),
        );
    }

    pub fn getCiphertext(&self) -> &Option<LongElement> {
        &self.t.cipher
    }
}
impl GadgetConfig for Gadget<ZkayPaillierEncGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.t.cipher.as_ref().unwrap().getArray()
    }
}
