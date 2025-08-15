#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::blockciphers::aes128_cipher_gadget::AES128CipherGadget;
use crate::examples::gadgets::blockciphers::chaskey_lts128_cipher_gadget::ChaskeyLTS128CipherGadget;
use crate::examples::gadgets::blockciphers::speck128_cipher_gadget::Speck128CipherGadget;
use crate::util::util::{BigInteger, Util};
use crate::zkay::crypto::crypto_backend::CIPHER_CHUNK_SIZE;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
use rccell::RcCell;

#[derive(Debug, Clone)]
pub enum CipherType {
    SPECK_128,
    AES_128,
    CHASKEY,
}

/**
 * Performs symmetric encryption in the CBC mode.
 */

#[derive(Debug, Clone)]
pub struct ZkayCBCSymmetricEncGadget {
    pub cipherType: CipherType,
    pub keyBits: Vec<Option<WireType>>,
    pub plaintextBits: Vec<Option<WireType>>,
    pub ivBits: Vec<Option<WireType>>,
    pub cipherBits: Vec<Option<WireType>>,
    pub outputs: Vec<Option<WireType>>,
}

impl ZkayCBCSymmetricEncGadget {
    pub const BLOCK_SIZE: i32 = 128;
    pub const KEY_SIZE: i32 = 128;

    pub fn new(
        plaintext: TypedWire,
        key: WireType,
        iv: WireType,
        cipherType: CipherType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let plaintextBits = Util::reverseBytes(plaintext.wire.getBitWiresi(256, &None).asArray());
        println!("Plain length [bits]: {}", plaintextBits.len());

        let mut _self = Gadget::<Self> {
            generator,
            description: desc.clone().unwrap_or(String::new()),
            t: Self {
                plaintextBits,
                keyBits: Util::reverseBytes(
                    key.getBitWiresi(Self::KEY_SIZE as u64, &None).asArray(),
                ),
                ivBits: Util::reverseBytes(
                    iv.getBitWiresi(Self::BLOCK_SIZE as u64, &None).asArray(),
                ),
                cipherType,
                cipherBits: vec![],
                outputs: vec![],
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayCBCSymmetricEncGadget> {
    fn buildCircuit(&mut self) {
        let block_size = ZkayCBCSymmetricEncGadget::BLOCK_SIZE as usize;
        let numBlocks = (self.t.plaintextBits.len() as f64 / block_size as f64).ceil() as i32;
        let plaintextArray = WireArray::new(
            self.t.plaintextBits.clone(),
            self.generator.clone().downgrade(),
        )
        .adjustLength(None, (numBlocks * block_size as i32) as usize)
        .asArray()
        .clone();

        let preparedKey = self.prepareKey();
        let mut prevCipher =
            WireArray::new(self.t.ivBits.clone(), self.generator.clone().downgrade());

        let mut cipherBits = vec![];
        for i in 0..numBlocks as usize {
            let msgBlock = WireArray::new(
                plaintextArray[i * block_size..(i + 1) * block_size].to_vec(),
                self.generator.clone().downgrade(),
            );
            let xored = msgBlock.xorWireArrayi(&prevCipher, &None).asArray().clone();
            match &self.t.cipherType {
                CipherType::SPECK_128 => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .packBitsIntoWords(64, &None);
                    let gadget = Speck128CipherGadget::new(
                        tmp,
                        preparedKey.clone(),
                        &Some(self.description.clone()),
                        self.generator.clone(),
                    );
                    let outputs = gadget.getOutputWires().clone();
                    prevCipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .getBits(64, &None);
                }
                CipherType::AES_128 => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .packBitsIntoWords(8, &None);
                    let gadget = AES128CipherGadget::new(
                        tmp,
                        preparedKey.clone(),
                        &Some("aes: ".to_owned() + &self.description),
                        self.generator.clone(),
                    );
                    let outputs = gadget.getOutputWires().clone();
                    prevCipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .getBits(8, &None);
                }
                CipherType::CHASKEY => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .packBitsIntoWords(32, &None);
                    let gadget = ChaskeyLTS128CipherGadget::new(
                        tmp,
                        preparedKey.clone(),
                        &Some("chaskey: ".to_owned() + &self.description),
                        self.generator.clone(),
                    );
                    let outputs = gadget.getOutputWires().clone();
                    prevCipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .getBits(3, &None);
                }
                _ => panic!("Unknown cipher value:{:?} ", self.t.cipherType),
            }
            cipherBits = Util::concat(&cipherBits, prevCipher.asArray());
        }
        self.t.outputs = WireArray::new(
            Util::reverseBytes(&Util::concat(&self.t.ivBits, &cipherBits)),
            self.generator.clone().downgrade(),
        )
        .packBitsIntoWords(CIPHER_CHUNK_SIZE as usize, &None);
    }

    fn prepareKey(&self) -> Vec<Option<WireType>> {
        match &self.t.cipherType {
            CipherType::SPECK_128 => {
                let packedKey =
                    WireArray::new(self.t.keyBits.clone(), self.generator.clone().downgrade())
                        .packBitsIntoWords(64, &None);
                Gadget::<Speck128CipherGadget>::expandKey(&packedKey, &self.generator)
            }
            CipherType::AES_128 => {
                let packedKey =
                    WireArray::new(self.t.keyBits.clone(), self.generator.clone().downgrade())
                        .packBitsIntoWords(8, &None);
                Gadget::<AES128CipherGadget>::expandKey(&packedKey, &self.generator)
            }
            CipherType::CHASKEY => {
                WireArray::new(self.t.keyBits.clone(), self.generator.clone().downgrade())
                    .packBitsIntoWords(32, &None)
            }
            _ => panic!("Other Ciphers not supported in this version!"),
        }
    }
}
impl GadgetConfig for Gadget<ZkayCBCSymmetricEncGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        //println!("Cipher length [bits]: {}", cipherBits.len());
        &self.t.outputs
    }
}
