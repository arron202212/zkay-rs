#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
use crate::zkay::zkay_paillier_dec_gadget::long_element::LongElement;
use rccell::RcCell;
/**
 * Gadget for checking correct exponential ElGamal decryption.
 * The expected message is provided as an input.
 */
pub struct ZkayElgamalDecGadget {
    skBits: Vec<Option<WireType>>, // little-endian randomness bits
    pk: JubJubPoint,
    c1: JubJubPoint,
    c2: JubJubPoint,
    expectedMsg: WireType,
    msgOk: Option<WireType>,
    outputs: Vec<Option<WireType>>,
}

impl ZkayElgamalDecGadget {
    pub fn new(
        pk: JubJubPoint,
        skBits: Vec<Option<WireType>>,
        c1: JubJubPoint,
        c2: JubJubPoint,
        expectedMsg: &WireType,
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
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>> {
    fn buildCircuit(&mut self) {
        // ensure pk and skBits form a key pair
        let pkExpected = mulScalar(getGenerator(), skBits);
        let keyOk = pkExpected
            .x
            .isEqualTo(pk.x)
            .and(pkExpected.y.isEqualTo(pk.y));

        // decrypt ciphertext (without de-embedding)
        let sharedSecret = mulScalar(c1, skBits);
        let msgEmbedded = addPoints(c2, negatePoint(sharedSecret));

        // embed expected message and assert equality
        let expectedMsgBits = expectedMsg.getBitWires(32).asArray();
        let expectedMsgEmbedded = mulScalar(getGenerator(), expectedMsgBits);
        self.t.t.msgOk = expectedMsgEmbedded
            .x
            .isEqualTo(msgEmbedded.x)
            .and(expectedMsgEmbedded.y.isEqualTo(msgEmbedded.y))
            .and(keyOk);
        self.t.t.outputs = vec![self.msgOk.clone()];
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalDecGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
