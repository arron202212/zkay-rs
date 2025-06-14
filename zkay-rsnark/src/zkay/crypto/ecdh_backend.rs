use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use zkay::typed_wire;
use zkay::zkay_cbc_symmetric_enc_gadget;
use zkay::zkay_cbc_symmetric_enc_gadget::cipher_type;

pub struct ECDHBackend {
    cipherType: CipherType,
}
impl ECDHBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(keyBits: i32, cipherType: CipherType) -> Self {
        super(keyBits);
        self.cipherType = cipherType;
    }

    pub fn getKeyChunkSize() -> i32 {
        KEY_CHUNK_SIZE
    }
}
impl Symmetric for ECDHBackend {
    pub fn createEncryptionGadget(
        plain: TypedWire,
        key: String,
        ivArr: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        return ZkayCBCSymmetricEncGadget::new(
            plain,
            getKey(key),
            extractIV(ivArr),
            cipherType,
            desc,
        );
    }
}
