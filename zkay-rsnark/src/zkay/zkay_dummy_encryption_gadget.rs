use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::GadgetConfig;
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
        desc: &Option<String>,
    ) -> Self {
        //super(desc);
        assert!(plain.is_some() && pk.is_some() && rnd.is_some());
   let pkarr = pk.getBits().packBitsIntoWords(256);
        for i in 1..pkarr.len() {
            generator.addZeroAssertion(pkarr[i], "Dummy enc pk valid");
        }
        let mut _self=Self{plain : plain.wire;
        pk : pkarr[0],
        cipher : vec![None; ((1.0 * keyBits) / CIPHER_CHUNK_SIZE).ceil() as i32]
        }
        ;
           _self.buildCircuit();
        _self
    }

    fn buildCircuit(&mut self) {
        let res = plain.add(pk, "plain + pk");
        cipher.fill(res);
    }
}
impl GadgetConfig for Gadget<ZkayDummyEncryptionGadget> {
    fn getOutputWires() -> Vec<Option<WireType>> {
        cipher
    }
}
