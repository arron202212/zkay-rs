use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_array;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::blockciphers::aes128_cipher_gadget;
use crate::examples::gadgets::blockciphers::chaskeylts128_cipher_gadget;
use crate::examples::gadgets::blockciphers::speck128_cipher_gadget;
use crate::util::util::{BigInteger, Util};
use zkay::crypto::crypto_backend;

pub enum CipherType {
    SPECK_128,
    AES_128,
    CHASKEY,
}

/**
 * Performs symmetric encryption in the CBC mode.
 */
pub struct ZkayCBCSymmetricEncGadget {
    cipherType: CipherType,
    keyBits: Vec<Option<WireType>>,
    plaintextBits: Vec<Option<WireType>>,
    ivBits: Vec<Option<WireType>>,
    cipherBits: Vec<Option<WireType>>,
    outputs: Vec<Option<WireType>>,
}

impl ZkayCBCSymmetricEncGadget {
    pub const BLOCK_SIZE: i32 = 128;
    pub const KEY_SIZE: i32 = 128;

    pub fn new(
        plaintext: &TypedWire,
        key: &WireType,
        iv: &WireType,
        cipherType: CipherType,
        desc: &Option<String>,
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
                keyBits: Util::reverseBytes(key.getBitWires(KEY_SIZE).asArray()),
                ivBits: Util::reverseBytes(iv.getBitWires(BLOCK_SIZE).asArray()),
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
        let numBlocks = (plaintextBits.len() as f64 / BLOCK_SIZE as f64).ceil() as i32;
        let plaintextArray = WireArray::new(plaintextBits)
            .adjustLength(numBlocks * BLOCK_SIZE)
            .asArray();

        let preparedKey = self.prepareKey();
        let prevCipher = WireArray::new(ivBits);

        let mut cipherBits = vec![];
        for i in 0..numBlocks {
            let msgBlock =
                WireArray::new(plaintextArray[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].to_vec());
            let xored = msgBlock.xorWireArray(prevCipher).asArray();
            match cipherType {
                SPECK_128 => {
                    let tmp = WireArray::new(xored).packBitsIntoWords(64);
                    let gadget = Speck128CipherGadget::new(tmp, preparedKey, description);
                    let outputs = gadget.getOutputWires();
                    prevCipher = WireArray::new(outputs).getBits(64);
                }
                AES_128 => {
                    let tmp = WireArray::new(xored).packBitsIntoWords(8);
                    let gadget = AES128CipherGadget::new(tmp, preparedKey, "aes: " + description);
                    let outputs = gadget.getOutputWires();
                    prevCipher = WireArray::new(outputs).getBits(8);
                }
                CHASKEY => {
                    let tmp = WireArray::new(xored).packBitsIntoWords(32);
                    let gadget =
                        ChaskeyLTS128CipherGadget::new(tmp, preparedKey, "chaskey: " + description);
                    let outputs = gadget.getOutputWires();
                    prevCipher = WireArray::new(outputs).getBits(32);
                }
                _ => panic!("Unknown cipher value:{cipherType} "),
            }
            cipherBits = Util::concat(cipherBits, prevCipher.asArray());
        }
        self.t.outputs = WireArray::new(Util::reverseBytes(Util::concat(ivBits, cipherBits)))
            .packBitsIntoWords(CryptoBackend.Symmetric.CIPHER_CHUNK_SIZE);
    }

    fn prepareKey(&self) -> Vec<Option<WireType>> {
        match self.cipherType {
            SPECK_128 => {
                let packedKey = WireArray::new(self.keyBits).packBitsIntoWords(64);
                Speck128CipherGadget.expandKey(packedKey)
            }
            AES_128 => {
                let packedKey = WireArray::new(self.keyBits).packBitsIntoWords(8);
                AES128CipherGadget.expandKey(packedKey)
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
