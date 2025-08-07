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
        let plaintextBits = Util::reverseBytes(plaintext.wire.getBitWires(256).asArray());
        println!("Plain length [bits]: {}", plaintextBits.len());

        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                plaintextBits,
                keyBits: Util::reverseBytes(key.getBitWires(Self::KEY_SIZE).asArray()),
                ivBits: Util::reverseBytes(iv.getBitWires(Self::BLOCK_SIZE).asArray()),
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
        let numBlocks = (self.t.plaintextBits.len() as f64 / Self::BLOCK_SIZE as f64).ceil() as i32;
        let plaintextArray = WireArray::new(self.t.plaintextBits.clone())
            .adjustLength(numBlocks * Self::BLOCK_SIZE)
            .asArray();

        let preparedKey = self.prepareKey();
        let prevCipher = WireArray::new(self.t.ivBits.clone());

        let mut cipherBits = vec![];
        for i in 0..numBlocks {
            let msgBlock = WireArray::new(
                plaintextArray[i * Self::BLOCK_SIZE..(i + 1) * Self::BLOCK_SIZE].to_vec(),
            );
            let xored = msgBlock.xorWireArray(prevCipher).asArray();
            match self.t.t.cipherType {
                SPECK_128 => {
                    let tmp = WireArray::new(xored).packBitsIntoWords(64);
                    let gadget = Speck128CipherGadget::new(tmp, preparedKey, &self.description);
                    let outputs = gadget.getOutputWires();
                    prevCipher = WireArray::new(outputs).getBits(64);
                }
                AES_128 => {
                    let tmp = WireArray::new(xored).packBitsIntoWords(8);
                    let gadget = AES128CipherGadget::new(
                        tmp,
                        preparedKey,
                        &Some("aes: ".to_ownded() + &self.description),
                    );
                    let outputs = gadget.getOutputWires();
                    prevCipher = WireArray::new(outputs).getBits(8);
                }
                CHASKEY => {
                    let tmp = WireArray::new(xored).packBitsIntoWords(32);
                    let gadget = ChaskeyLTS128CipherGadget::new(
                        tmp,
                        preparedKey,
                        &Some("chaskey: ".to_ownded() + &self.description),
                    );
                    let outputs = gadget.getOutputWires();
                    prevCipher = WireArray::new(outputs).getBits(32);
                }
                _ => panic!("Unknown cipher value:{} ", self.t.cipherType),
            }
            cipherBits = Util::concat(cipherBits, prevCipher.asArray());
        }
        self.t.outputs = WireArray::new(Util::reverseBytes(Util::concat(
            &self.t.ivBits,
            &cipherBits,
        )))
        .packBitsIntoWords(CIPHER_CHUNK_SIZE);
    }

    fn prepareKey(&self) -> Vec<Option<WireType>> {
        match self.cipherType {
            SPECK_128 => {
                let packedKey = WireArray::new(self.keyBits).packBitsIntoWords(64);
                Speck128CipherGadget::expandKey(packedKey)
            }
            AES_128 => {
                let packedKey = WireArray::new(self.keyBits).packBitsIntoWords(8);
                AES128CipherGadget::expandKey(packedKey)
            }
            CHASKEY => WireArray::new(self.keyBits).packBitsIntoWords(32),
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
