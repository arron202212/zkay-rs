use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

use zkay::crypto::DummyBackend::CIPHER_CHUNK_SIZE;

pub struct ZkayDummyEncryptionGadget {
    pk: WireType,
    plain: WireType,
    cipher: Vec<Option<WireType>>,
}
impl ZkayDummyEncryptionGadget {
    pub fn new(
        plain: TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        desc: &String,
    ) -> Self {
        super(desc);
        assert!(plain.is_some() && pk.is_some() && rnd.is_some());
        self.plain = plain.wire;
        let pkarr = pk.getBits().packBitsIntoWords(256);
        for i in 1..pkarr.len() {
            generator.addZeroAssertion(pkarr[i], "Dummy enc pk valid");
        }
        self.pk = pkarr[0];
        self.cipher = vec![None; ((1.0 * keyBits) / CIPHER_CHUNK_SIZE).ceil() as i32];
        buildCircuit();
    }
}
impl Gadget for ZkayDummyEncryptionGadget {
    fn buildCircuit() {
        let res = plain.add(pk, "plain + pk");
        cipher.fill(res);
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return cipher;
    }
}
