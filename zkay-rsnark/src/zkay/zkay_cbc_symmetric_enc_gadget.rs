#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        auxiliary::long_element::LongElement,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::CircuitGenerator, wire::WireConfig, wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::blockciphers::{
        aes128_cipher_gadget::AES128CipherGadget,
        chaskey_lts128_cipher_gadget::ChaskeyLTS128CipherGadget,
        speck128_cipher_gadget::Speck128CipherGadget,
    },
    util::util::{BigInteger, Util},
    zkay::{crypto::crypto_backend::CIPHER_CHUNK_SIZE, typed_wire::TypedWire},
};

use rccell::RcCell;

#[derive(Debug, Clone)]
pub enum CipherType {
    SPECK_128,
    AES_128,
    CHASKEY,
}

//  * Performs symmetric encryption in the CBC mode.

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
        let plaintextBits =
            Util::reverseBytes(plaintext.wire.get_bit_wiresi(256, &None).as_array());
        println!("Plain length [bits]: {}", plaintextBits.len());

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plaintextBits,
                keyBits: Util::reverseBytes(
                    key.get_bit_wiresi(Self::KEY_SIZE as u64, &None).as_array(),
                ),
                ivBits: Util::reverseBytes(
                    iv.get_bit_wiresi(Self::BLOCK_SIZE as u64, &None).as_array(),
                ),
                cipherType,
                cipherBits: vec![],
                outputs: vec![],
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayCBCSymmetricEncGadget> {
    fn build_circuit(&mut self) {
        let block_size = ZkayCBCSymmetricEncGadget::BLOCK_SIZE as usize;
        let numBlocks = (self.t.plaintextBits.len() as f64 / block_size as f64).ceil() as i32;
        let plaintextArray = WireArray::new(
            self.t.plaintextBits.clone(),
            self.generator.clone().downgrade(),
        )
        .adjust_length(None, (numBlocks * block_size as i32) as usize)
        .as_array()
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
            let xored = msgBlock
                .xor_wire_arrayi(&prevCipher, &None)
                .as_array()
                .clone();
            match &self.t.cipherType {
                CipherType::SPECK_128 => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(64, &None);
                    let gadget = Speck128CipherGadget::new(
                        tmp,
                        preparedKey.clone(),
                        &Some(self.description.clone()),
                        self.generator.clone(),
                    );
                    let outputs = gadget.get_output_wires().clone();
                    prevCipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .get_bits(64, &None);
                }
                CipherType::AES_128 => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(8, &None);
                    let gadget = AES128CipherGadget::new(
                        tmp,
                        preparedKey.clone(),
                        &Some("aes: ".to_owned() + &self.description),
                        self.generator.clone(),
                    );
                    let outputs = gadget.get_output_wires().clone();
                    prevCipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .get_bits(8, &None);
                }
                CipherType::CHASKEY => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(32, &None);
                    let gadget = ChaskeyLTS128CipherGadget::new(
                        tmp,
                        preparedKey.clone(),
                        &Some("chaskey: ".to_owned() + &self.description),
                        self.generator.clone(),
                    );
                    let outputs = gadget.get_output_wires().clone();
                    prevCipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .get_bits(3, &None);
                }
                _ => panic!("Unknown cipher value:{:?} ", self.t.cipherType),
            }
            cipherBits = Util::concat(&cipherBits, prevCipher.as_array());
        }
        self.t.outputs = WireArray::new(
            Util::reverseBytes(&Util::concat(&self.t.ivBits, &cipherBits)),
            self.generator.clone().downgrade(),
        )
        .pack_bits_into_words(CIPHER_CHUNK_SIZE as usize, &None);
    }

    fn prepareKey(&self) -> Vec<Option<WireType>> {
        match &self.t.cipherType {
            CipherType::SPECK_128 => {
                let packedKey =
                    WireArray::new(self.t.keyBits.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(64, &None);
                Gadget::<Speck128CipherGadget>::expandKey(&packedKey, &self.generator)
            }
            CipherType::AES_128 => {
                let packedKey =
                    WireArray::new(self.t.keyBits.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(8, &None);
                Gadget::<AES128CipherGadget>::expandKey(&packedKey, &self.generator)
            }
            CipherType::CHASKEY => {
                WireArray::new(self.t.keyBits.clone(), self.generator.clone().downgrade())
                    .pack_bits_into_words(32, &None)
            }
            _ => panic!("Other Ciphers not supported in this version!"),
        }
    }
}
impl GadgetConfig for Gadget<ZkayCBCSymmetricEncGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        //println!("Cipher length [bits]: {}", cipherBits.len());
        &self.t.outputs
    }
}
