use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use zkay::typed_wire;
use zkay::zkay_rsa_encryption_gadget;

pub struct RSABackend {
    paddingType: PaddingType,
}
impl RSABackend {
    const CIPHER_CHUNK_SIZE: i32 = 232;
    const KEY_CHUNK_SIZE: i32 = 232;
    const PKCS15_RND_CHUNK_SIZE: i32 = 224;
    const OAEP_RND_CHUNK_SIZE: i32 = 128;
    pub fn new(keyBits: i32, padding: PaddingType) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(
            keyBits,
            Self {
                paddingType: padding,
            },
        )
    }
}

impl AsymmetricConfig for CryptoBackend<Asymmetric<RSABackend>> {
    pub fn getKeyChunkSize(&self) -> i32 {
        RSABackend::KEY_CHUNK_SIZE
    }

    pub fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        random: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        ZkayRSAEncryptionGadget::new(plain, getKey(key), random, keyBits, paddingType, desc)
    }
}
