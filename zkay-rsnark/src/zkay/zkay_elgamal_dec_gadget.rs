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
        structure::{circuit_generator::CircuitGenerator, wire::WireConfig, wire_type::WireType},
    },
    zkay::zkay_baby_jub_jub_gadget::{
        JubJubPoint, ZkayBabyJubJubGadget, ZkayBabyJubJubGadgetConfig,
    },
};

use rccell::RcCell;

//  * Gadget for checking correct exponential ElGamal decryption.
//  * The expected message is provided as an input.

#[derive(Debug, Clone)]
pub struct ZkayElgamalDecGadget {
    pub sk_bits: Vec<Option<WireType>>, // little-endian randomness bits
    pub pk: JubJubPoint,
    pub c1: JubJubPoint,
    pub c2: JubJubPoint,
    pub expected_msg: WireType,
    pub msg_ok: Option<WireType>,
    pub outputs: Vec<Option<WireType>>,
}

impl ZkayElgamalDecGadget {
    pub fn new(
        pk: JubJubPoint,
        sk_bits: Vec<Option<WireType>>,
        c1: JubJubPoint,
        c2: JubJubPoint,
        expected_msg: WireType,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            &None,
            Self {
                sk_bits,
                pk,
                c1,
                c2,
                expected_msg,
                msg_ok: None,
                outputs: vec![],
            },
            generator,
        );
        _self.build_circuit();
        _self
    }
}
// impl ZkayBabyJubJubGadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>>{
// }
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>> {
    fn build_circuit(&mut self) {
        // ensure pk and sk_bits form a key pair
        let pk_expected = self.mul_scalar(&self.get_generator(), &self.t.t.sk_bits);
        let key_ok = pk_expected
            .x
            .is_equal_tos(&self.t.t.pk.x, &None)
            .and(&pk_expected.y.is_equal_tos(&self.t.t.pk.y, &None), &None);

        // decrypt ciphertext (without de-embedding)
        let shared_secret = self.mul_scalar(&self.t.t.c1, &self.t.t.sk_bits);
        let msg_embedded = self.add_points(&self.t.t.c2, &Self::negate_point(&shared_secret));

        // embed expected message and assert equality
        let expected_msg_bits = self
            .t
            .t
            .expected_msg
            .get_bit_wiresi(32, &None)
            .as_array()
            .clone();
        let expected_msg_embedded = self.mul_scalar(&self.get_generator(), &expected_msg_bits);
        self.t.t.msg_ok = Some(
            expected_msg_embedded
                .x
                .is_equal_tos(&msg_embedded.x, &None)
                .and(
                    &expected_msg_embedded.y.is_equal_tos(&msg_embedded.y, &None),
                    &None,
                )
                .and(&key_ok, &None),
        );
        self.t.t.outputs = vec![self.t.t.msg_ok.clone()];
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
