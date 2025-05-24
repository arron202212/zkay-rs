use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use zkay::typed_wire;
use zkay::zkay_dummy_encryption_gadget;

pub struct DummyBackend;
impl DummyBackend {
    const CIPHER_CHUNK_SIZE: i32 = 256;
    const KEY_CHUNK_SIZE: i32 = 256;

    // pub  DummyBackend(i32 keyBits) {
    // 	super(keyBits);
    // }

    pub fn getKeyChunkSize() -> i32 {
        return KEY_CHUNK_SIZE;
    }
}
impl Asymmetric for DummyBackend {
    fn createEncryptionGadget(
        plain: TypedWire,
        key: String,
        random: Vec<Option<WireType>>,
        desc: &String,
    ) -> Gadget {
        return ZkayDummyEncryptionGadget::new(plain, getKey(key), random, keyBits, desc);
    }
}
