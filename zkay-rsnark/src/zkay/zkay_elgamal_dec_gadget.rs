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
    zkay::zkay_baby_jub_jub_gadget::{
        JubJubPoint, ZkayBabyJubJubGadget, ZkayBabyJubJubGadgetConfig,
    },
};

use rccell::RcCell;

//  * Gadget for checking correct exponential ElGamal decryption.
//  * The expected message is provided as an input.

#[derive(Debug, Clone)]
pub struct ZkayElgamalDecGadget {
    pub skBits: Vec<Option<WireType>>, // little-endian randomness bits
    pub pk: JubJubPoint,
    pub c1: JubJubPoint,
    pub c2: JubJubPoint,
    pub expectedMsg: WireType,
    pub msgOk: Option<WireType>,
    pub outputs: Vec<Option<WireType>>,
}

impl ZkayElgamalDecGadget {
    pub fn new(
        pk: JubJubPoint,
        skBits: Vec<Option<WireType>>,
        c1: JubJubPoint,
        c2: JubJubPoint,
        expectedMsg: WireType,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            &None,
            Self {
                skBits,
                pk,
                c1,
                c2,
                expectedMsg,
                msgOk: None,
                outputs: vec![],
            },
            generator,
        );
        _self.buildCircuit();
        _self
    }
}
// impl ZkayBabyJubJubGadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>>{
// }
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>> {
    fn buildCircuit(&mut self) {
        // ensure pk and skBits form a key pair
        let pkExpected = self.mulScalar(&self.getGenerator(), &self.t.t.skBits);
        let keyOk = pkExpected
            .x
            .isEqualTo(&self.t.t.pk.x, &None)
            .and(&pkExpected.y.isEqualTo(&self.t.t.pk.y, &None), &None);

        // decrypt ciphertext (without de-embedding)
        let sharedSecret = self.mulScalar(&self.t.t.c1, &self.t.t.skBits);
        let msgEmbedded = self.addPoints(&self.t.t.c2, &Self::negatePoint(&sharedSecret));

        // embed expected message and assert equality
        let expectedMsgBits = self
            .t
            .t
            .expectedMsg
            .getBitWiresi(32, &None)
            .asArray()
            .clone();
        let expectedMsgEmbedded = self.mulScalar(&self.getGenerator(), &expectedMsgBits);
        self.t.t.msgOk = Some(
            expectedMsgEmbedded
                .x
                .isEqualTo(&msgEmbedded.x, &None)
                .and(
                    &expectedMsgEmbedded.y.isEqualTo(&msgEmbedded.y, &None),
                    &None,
                )
                .and(&keyOk, &None),
        );
        self.t.t.outputs = vec![self.t.t.msgOk.clone()];
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
