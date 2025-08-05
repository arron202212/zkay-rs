use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_array;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::rsa::rsa_encryption_oaep_gadget;
use crate::examples::gadgets::rsa::rsa_encryption_v1_5_gadget;

use zkay::ZkayUtil::*;
use zkay::crypto::RSABackend::*;
pub enum PaddingType {
    PKCS_1_5,
    OAEP,
}

pub struct ZkayRSAEncryptionGadget {
    paddingType: PaddingType,
    pk: LongElement,
    plain: &WireType,
    rnd: Vec<Option<WireType>>,
    keyBits: i32,
    cipher: Vec<Option<WireType>>,
}
impl ZkayRSAEncryptionGadget {
    pub fn new(
        plain: &TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        paddingType: PaddingType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        // assert!(plain, "plain");
        // assert!(pk, "pk");
        assert!(!rnd.is_empty(), "rnd");
        // assert!(paddingType, "paddingType");

        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                paddingType,
                plain: plain.wire.clone(),
                pk,
                rnd,
                keyBits,
                cipher: vec![],
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayRSAEncryptionGadget> {
    fn buildCircuit(&mut self) {
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
}
impl GadgetConfig for Gadget<ZkayRSAEncryptionGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
