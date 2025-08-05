use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;

use zkay::crypto::DummyBackend::CIPHER_CHUNK_SIZE;

pub struct ZkayDummyEncryptionGadget {
    pk: &WireType,
    plain: &WireType,
    cipher: Vec<Option<WireType>>,
}
impl ZkayDummyEncryptionGadget {
    pub fn new(
        plain: &TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        desc: &Option<String>,
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
                cipher: vec![None; ((keyBits as f64) / CIPHER_CHUNK_SIZE as f64).ceil() as usize],
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayDummyEncryptionGadget> {
    fn buildCircuit(&mut self) {
        let res = plain.add(pk, "plain + pk");
        self.t.cipher.fill(res);
    }
}
impl GadgetConfig for Gadget<ZkayDummyEncryptionGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
