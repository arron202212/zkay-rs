use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use examples::gadgets::rsa::rsa_encryption_oaep_gadget;
use examples::gadgets::rsa::rsa_encryption_v1_5_gadget;

use zkay::ZkayUtil::*;
use zkay::crypto::RSABackend::*;
pub enum PaddingType {
    PKCS_1_5,
    OAEP,
}

pub struct ZkayRSAEncryptionGadget {
    paddingType: PaddingType,
    pk: LongElement,
    plain: WireType,
    rnd: Vec<Option<WireType>>,
    keyBits: i32,

    cipher: Vec<Option<WireType>>,
}
impl ZkayRSAEncryptionGadget {
    pub fn new(
        plain: TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        paddingType: PaddingType,
        desc: &Option<String>,
    ) -> Self {
        super(desc);

        Objects.requireNonNull(plain, "plain");
        Objects.requireNonNull(pk, "pk");
        Objects.requireNonNull(rnd, "rnd");
        Objects.requireNonNull(paddingType, "paddingType");

        self.paddingType = paddingType;
        self.plain = plain.wire;
        self.pk = pk;
        self.rnd = rnd;
        self.keyBits = keyBits;

        buildCircuit();
    }
}
impl Gadget for ZkayRSAEncryptionGadget {
    fn buildCircuit() {
        let plainBytes = reverseBytes(plain.getBitWires(256), 8);

        let mut enc;
        match paddingType {
            OAEP => {
                let rndBytes = reverseBytes(WireArray::new(rnd).getBits(OAEP_RND_CHUNK_SIZE), 8);
                let e =
                    RSAEncryptionOAEPGadget::new(pk, plainBytes, rndBytes, keyBits, description);
                e.checkSeedCompliance();
                enc = e;
            }
            PKCS_1_5 => {
                let rndLen = keyBits / 8 - 3 - plainBytes.len();
                let rndBytes = reverseBytes(
                    WireArray::new(rnd)
                        .getBits(PKCS15_RND_CHUNK_SIZE)
                        .adjustLength(rndLen * 8),
                    8,
                );
                enc = RSAEncryptionV1_5_Gadget::new(pk, plainBytes, rndBytes, keyBits, description);
            }
            _ => assert!("Unexpected padding type: " + paddingType),
        }

        cipher =
            WireArray::new(enc.getOutputWires()).packWordsIntoLargerWords(8, CIPHER_CHUNK_SIZE / 8);
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        cipher
    }
}
