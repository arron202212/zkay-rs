use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use zkay::typed_wire;
use zkay::zkay_cbc_symmetric_enc_gadget;
use zkay::zkay_cbc_symmetric_enc_gadget::cipher_type;

pub struct ECDHBackend {
    cipherType: CipherType,
}
impl ECDHBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(keyBits: i32, cipherType: CipherType) -> CryptoBackend<Symmetric<Self>> {
        Symmetric::<Self>::new(keyBits, Self { cipherType })
    }

    pub fn getKeyChunkSize() -> i32 {
        KEY_CHUNK_SIZE
    }
}
impl SymmetricConfig for CryptoBackend<Symmetric<ECDHBackend>> {
    pub fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        ivArr: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        ZkayCBCSymmetricEncGadget::new(plain, getKey(key), extractIV(ivArr), cipherType, desc)
    }
}
