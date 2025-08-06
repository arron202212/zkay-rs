#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::dummy_backend::DummyBackend; //::CIPHER_CHUNK_SIZE;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
use crate::zkay::zkay_dummy_encryption_gadget::long_element::LongElement;
use crate::zkay::zkay_paillier_dec_gadget::long_element::LongElement;
use rccell::RcCell;
pub struct ZkayDummyEncryptionGadget {
    pk: WireType,
    plain: WireType,
    cipher: Vec<Option<WireType>>,
}
impl ZkayDummyEncryptionGadget {
    pub fn new(
        plain: &TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(plain.is_some() && pk.is_some() && rnd.is_some());
        let pkarr = pk.getBits().packBitsIntoWords(256);
        for i in 1..pkarr.len() {
            generator.addZeroAssertion(pkarr[i], "Dummy enc pk valid");
        }

        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                plain: plain.wire.clone(),
                pk: pkarr[0].clone(),
                cipher: vec![
                    None;
                    ((keyBits as f64) / DummyBackend::CIPHER_CHUNK_SIZE as f64).ceil()
                        as usize
                ],
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayDummyEncryptionGadget> {
    fn buildCircuit(&mut self) {
        let res = self.plain.add(&self.pk, &Some("plain + pk".to_owned()));
        self.t.cipher.fill(res);
    }
}
impl GadgetConfig for Gadget<ZkayDummyEncryptionGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
