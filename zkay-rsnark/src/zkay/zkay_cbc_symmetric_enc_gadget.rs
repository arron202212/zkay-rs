use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use examples::gadgets::blockciphers::aes128_cipher_gadget;
use examples::gadgets::blockciphers::chaskeylts128_cipher_gadget;
use examples::gadgets::blockciphers::speck128_cipher_gadget;
use crate::util::util::{Util,BigInteger};
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
    keyBits: Vec<WireType>,
    plaintextBits: Vec<WireType>,
    ivBits: Vec<WireType>,

    cipherBits: Vec<WireType>,
}

impl ZkayCBCSymmetricEncGadget {
    pub const BLOCK_SIZE: i32 = 128;
    pub const KEY_SIZE: i32 = 128;

    pub fn new(
        plaintext: TypedWire,
        key: WireType,
        iv: WireType,
        cipherType: CipherType,
        desc: Vec<String>,
    ) -> Self {
        // super(desc);
        let plaintextBits = Util::reverseBytes(plaintext.wire.getBitWires(256).asArray());
        println!("Plain length [bits]: {}", plaintextBits.len());
        Self {
            plaintextBits,
            keyBits: Util::reverseBytes(key.getBitWires(KEY_SIZE).asArray()),
            ivBits: Util::reverseBytes(iv.getBitWires(BLOCK_SIZE).asArray()),
            cipherType,
            cipherBits: buildCircuit(),
        }
    }

    fn buildCircuit() -> Vec<WireType> {
        let numBlocks = (plaintextBits.length * 1.0 / BLOCK_SIZE).ceil() as i32;
        let plaintextArray = WireArray::new(plaintextBits)
            .adjustLength(numBlocks * BLOCK_SIZE)
            .asArray();

        let preparedKey = Self::prepareKey();
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
    }

    fn prepareKey() -> Vec<WireType> {
        let mut preparedKey;
        match cipherType {
            SPECK_128 => {
                let packedKey = WireArray::new(keyBits).packBitsIntoWords(64);
                preparedKey = Speck128CipherGadget.expandKey(packedKey);
            }
            AES_128 => {
                let packedKey = WireArray::new(keyBits).packBitsIntoWords(8);
                preparedKey = AES128CipherGadget.expandKey(packedKey);
            }
            CHASKEY => {
                preparedKey = WireArray::new(keyBits).packBitsIntoWords(32);
            }
            _ => panic!("Other Ciphers not supported in this version!"),
        }
        return preparedKey;
    }
}
impl Gadget for ZkayCBCSymmetricEncGadget {
    pub fn getOutputWires() -> Vec<WireType> {
        println!("Cipher length [bits]: {}", cipherBits.length);
        return WireArray::new(Util::reverseBytes(Util::concat(ivBits, cipherBits)))
            .packBitsIntoWords(CryptoBackend.Symmetric.CIPHER_CHUNK_SIZE);
    }
}
