#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
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
    Speck128,
    Aes128,
    Chaskey,
}

//  * Performs symmetric encryption in the CBC mode.

#[derive(Debug, Clone)]
pub struct ZkayCBCSymmetricEncGadget {
    pub cipher_type: CipherType,
    pub key_bits: Vec<Option<WireType>>,
    pub plaintext_bits: Vec<Option<WireType>>,
    pub iv_bits: Vec<Option<WireType>>,
    pub cipher_bits: Vec<Option<WireType>>,
    pub outputs: Vec<Option<WireType>>,
}

impl ZkayCBCSymmetricEncGadget {
    pub const BLOCK_SIZE: i32 = 128;
    pub const KEY_SIZE: i32 = 128;

    pub fn new(
        plaintext: TypedWire,
        key: WireType,
        iv: WireType,
        cipher_type: CipherType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let plaintext_bits =
            Util::reverse_bytes(plaintext.wire.get_bit_wiresi(256, &None).as_array());
        println!("Plain length [bits]: {}", plaintext_bits.len());

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plaintext_bits,
                key_bits: Util::reverse_bytes(
                    key.get_bit_wiresi(Self::KEY_SIZE as u64, &None).as_array(),
                ),
                iv_bits: Util::reverse_bytes(
                    iv.get_bit_wiresi(Self::BLOCK_SIZE as u64, &None).as_array(),
                ),
                cipher_type,
                cipher_bits: vec![],
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
        let num_blocks = (self.t.plaintext_bits.len() as f64 / block_size as f64).ceil() as i32;
        let plain_text_array = WireArray::new(
            self.t.plaintext_bits.clone(),
            self.generator.clone().downgrade(),
        )
        .adjust_length(None, (num_blocks * block_size as i32) as usize)
        .as_array()
        .clone();

        let prepared_key = self.prepare_key();
        let mut prev_cipher =
            WireArray::new(self.t.iv_bits.clone(), self.generator.clone().downgrade());

        let mut cipher_bits = vec![];
        for i in 0..num_blocks as usize {
            let msg_block = WireArray::new(
                plain_text_array[i * block_size..(i + 1) * block_size].to_vec(),
                self.generator.clone().downgrade(),
            );
            let xored = msg_block
                .xor_wire_arrayi(&prev_cipher, &None)
                .as_array()
                .clone();
            match &self.t.cipher_type {
                CipherType::Speck128 => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(64, &None);
                    let gadget = Speck128CipherGadget::new(
                        tmp,
                        prepared_key.clone(),
                        &Some(self.description.clone()),
                        self.generator.clone(),
                    );
                    let outputs = gadget.get_output_wires().clone();
                    prev_cipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .get_bits(64, &None);
                }
                CipherType::Aes128 => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(8, &None);
                    let gadget = AES128CipherGadget::new(
                        tmp,
                        prepared_key.clone(),
                        &Some("aes: ".to_owned() + &self.description),
                        self.generator.clone(),
                    );
                    let outputs = gadget.get_output_wires().clone();
                    prev_cipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .get_bits(8, &None);
                }
                CipherType::Chaskey => {
                    let tmp = WireArray::new(xored.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(32, &None);
                    let gadget = ChaskeyLTS128CipherGadget::new(
                        tmp,
                        prepared_key.clone(),
                        &Some("chaskey: ".to_owned() + &self.description),
                        self.generator.clone(),
                    );
                    let outputs = gadget.get_output_wires().clone();
                    prev_cipher = WireArray::new(outputs, self.generator.clone().downgrade())
                        .get_bits(3, &None);
                }
                _ => panic!("Unknown cipher value:{:?} ", self.t.cipher_type),
            }
            cipher_bits = Util::concat(&cipher_bits, prev_cipher.as_array());
        }
        self.t.outputs = WireArray::new(
            Util::reverse_bytes(&Util::concat(&self.t.iv_bits, &cipher_bits)),
            self.generator.clone().downgrade(),
        )
        .pack_bits_into_words(CIPHER_CHUNK_SIZE as usize, &None);
    }

    fn prepare_key(&self) -> Vec<Option<WireType>> {
        match &self.t.cipher_type {
            CipherType::Speck128 => {
                let packed_key =
                    WireArray::new(self.t.key_bits.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(64, &None);
                Gadget::<Speck128CipherGadget>::expandKey(&packed_key, &self.generator)
            }
            CipherType::Aes128 => {
                let packed_key =
                    WireArray::new(self.t.key_bits.clone(), self.generator.clone().downgrade())
                        .pack_bits_into_words(8, &None);
                Gadget::<AES128CipherGadget>::expandKey(&packed_key, &self.generator)
            }
            CipherType::Chaskey => {
                WireArray::new(self.t.key_bits.clone(), self.generator.clone().downgrade())
                    .pack_bits_into_words(32, &None)
            }
            _ => panic!("Other Ciphers not supported in this version!"),
        }
    }
}
impl GadgetConfig for Gadget<ZkayCBCSymmetricEncGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        //println!("Cipher length [bits]: {}", cipher_bits.len());
        &self.t.outputs
    }
}
