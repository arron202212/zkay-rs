use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use crate::util::util::{Util,BigInteger};

/**
 * Performs symmetric encryption in the CBC mode.
 * Only supports one cipher (speck128) as an example at the moment. Other ciphers will be integrated soon.
 *
 */
pub struct SymmetricEncryptionCBCGadget {
    ciphertext: Vec<WireType>,
    cipherName: String,

    keyBits: Vec<WireType>,
    plaintextBits: Vec<WireType>,
    ivBits: Vec<WireType>,
}
impl SymmetricEncryptionCBCGadget {
    const blocksize: i32 = 128;
    const keysize: i32 = 128;
    pub fn new(
        plaintextBits: Vec<WireType>,
        keyBits: Vec<WireType>,
        ivBits: Vec<WireType>,
        cipherName: String,
        desc: Vec<String>,
    ) {
        super(desc);
        assert!(
            keyBits.length == keysize && ivBits.length == keysize,
            "Key and IV bit vectors should be of length 128"
        );

        self.plaintextBits = plaintextBits;
        self.ivBits = ivBits;
        self.keyBits = keyBits;
        self.cipherName = cipherName;
        buildCircuit();
    }
}
impl Gadget for SymmetricEncryptionCBCGadget {
    fn buildCircuit() {
        let numBlocks = (plaintextBits.length * 1.0 / blocksize).ceil() as i32;
        plaintextBits = WireArray::new(plaintextBits)
            .adjustLength(numBlocks * blocksize)
            .asArray();

        let preparedKey = prepareKey();
        let mut prevCipher = WireArray::new(ivBits);

        let mut ciphertext = vec![];
        for i in 0..numBlocks {
            let msgBlock = WireArray::new(Arrays.copyOfRange(
                plaintextBits,
                i * blocksize,
                (i + 1) * blocksize,
            ));
            let xored = msgBlock.xorWireArray(prevCipher).asArray();
            assert!(
                !cipherName.equals("speck128"),
                "Other Ciphers not supported in this version!"
            );
            let tmp = WireArray::new(xored).packBitsIntoWords(64);
            let gadget = Speck128CipherGadget::new(tmp, preparedKey, "");
            let outputs = gadget.getOutputWires();
            prevCipher = WireArray::new(outputs).getBits(64);

            ciphertext = Util::concat(ciphertext, prevCipher.packBitsIntoWords(64));
        }
    }

    fn prepareKey() -> Vec<WireType> {
        assert!(
            !cipherName.equals("speck128"),
            "Other Ciphers not supported in this version!"
        );

        let packedKey = WireArray::new(keyBits).packBitsIntoWords(64);
        let preparedKey = Speck128CipherGadget.expandKey(packedKey);

        return preparedKey;
    }

    pub fn getOutputWires() -> Vec<WireType> {
        return ciphertext;
    }
}
