#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::rsa::rsa_encryption_oaep_gadget::RSAEncryptionOAEPGadget;
use crate::examples::gadgets::rsa::rsa_encryption_v1_5_gadget::RSAEncryptionV1_5_Gadget;

use crate::circuit::operations::gadget::Gadget;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::zkay::crypto::rsa_backend::RSABackend;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_util::ZkayUtil;

use rccell::RcCell;

#[derive(Debug, Clone)]
pub enum PaddingType {
    PKCS_1_5,
    OAEP,
}

#[derive(Debug, Clone)]
pub struct ZkayRSAEncryptionGadget {
    pub paddingType: PaddingType,
    pub pk: LongElement,
    pub plain: WireType,
    pub rnd: Vec<Option<WireType>>,
    pub keyBits: i32,
    pub cipher: Vec<Option<WireType>>,
}
impl ZkayRSAEncryptionGadget {
    pub fn new(
        plain: TypedWire,
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

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                paddingType,
                plain: plain.wire.clone(),
                pk,
                rnd,
                keyBits,
                cipher: vec![],
            },
        );
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayRSAEncryptionGadget> {
    fn buildCircuit(&mut self) {
        let plainBytes = ZkayUtil::reverseBytes(
            self.t.plain.getBitWiresi(256, &None),
            8,
            self.generator.clone(),
        );

        let mut enc: Box<dyn GadgetConfig>;
        match self.t.paddingType {
            PaddingType::OAEP => {
                let rndBytes = ZkayUtil::reverseBytes(
                    WireArray::new(self.t.rnd.clone(), self.generator.clone().downgrade())
                        .getBits(RSABackend::OAEP_RND_CHUNK_SIZE as usize, &None),
                    8,
                    self.generator.clone(),
                );
                let e = RSAEncryptionOAEPGadget::new(
                    self.t.pk.clone(),
                    plainBytes,
                    rndBytes,
                    self.t.keyBits.clone(),
                    &Some(self.description.clone()),
                    self.generator.clone(),
                );
                e.checkSeedCompliance();
                enc = Box::new(e);
            }
            PaddingType::PKCS_1_5 => {
                let rndLen = self.t.keyBits as usize / 8 - 3 - plainBytes.len();
                let rndBytes = ZkayUtil::reverseBytes(
                    WireArray::new(self.t.rnd.clone(), self.generator.clone().downgrade())
                        .getBits(RSABackend::PKCS15_RND_CHUNK_SIZE as usize, &None)
                        .adjustLength(None, rndLen * 8),
                    8,
                    self.generator.clone(),
                );
                enc = Box::new(RSAEncryptionV1_5_Gadget::new(
                    self.t.pk.clone(),
                    plainBytes,
                    rndBytes,
                    self.t.keyBits.clone(),
                    &Some(self.description.clone()),
                    self.generator.clone(),
                ));
            }
            _ => panic!("Unexpected padding type: {:?}", self.t.paddingType),
        }

        self.t.cipher = WireArray::new(
            enc.getOutputWires().clone(),
            self.generator.clone().downgrade(),
        )
        .packWordsIntoLargerWords(8, RSABackend::CIPHER_CHUNK_SIZE / 8, &None);
    }
}
impl GadgetConfig for Gadget<ZkayRSAEncryptionGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
