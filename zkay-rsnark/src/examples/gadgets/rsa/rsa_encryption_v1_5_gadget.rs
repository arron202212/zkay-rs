use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::math::field_division_gadget;
use examples::gadgets::math::long_integer_mod_gadget;
use crate::util::util::{Util,BigInteger};

/**
 * A gadget for RSA encryption according to PKCS#1 v1.5. A future version will
 * have the RSA-OAEP method according to PKCS#1 v2.x. The gadget assumes a
 * hardcoded pub  exponent of 0x10001.
 * This gadget can accept a hardcoded or a variable RSA modulus. See the
 * corresponding generator example.
 *
 * Implemented according to the standard specs here:
 * https://www.emc.com/collateral/white-
 * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf
 *
 */

pub struct RSAEncryptionV1_5_Gadget {
    modulus: LongElement,

    // every wire represents a byte in the following three arrays
    plainText: Vec<Option<WireType>>,
    randomness: Vec<Option<WireType>>, // (rsaKeyBitLength / 8 - 3 - plainTextLength)
    // non-zero bytes
    ciphertext: Vec<Option<WireType>>,

    rsaKeyBitLength: i32, // in bits (assumed to be divisible by 8)
}
impl RSAEncryptionV1_5_Gadget {
    pub fn new(
        modulus: LongElement,
        plainText: Vec<Option<WireType>>,
        randomness: Vec<Option<WireType>>,
        rsaKeyBitLength: i32,
        desc: &Option<String>,
    ) {
        super(desc);

        if rsaKeyBitLength % 8 != 0 {
            assert!("RSA Key bit length is assumed to be a multiple of 8");
        }

        if (plainText.len() > rsaKeyBitLength / 8 - 11
            || plainText.len() + randomness.len() != rsaKeyBitLength / 8 - 3)
        {
            //println!("Check Message & Padding length");
            assert!("Invalid Argument Dimensions for RSA Encryption");
        }

        self.randomness = randomness;
        self.plainText = plainText;
        self.modulus = modulus;
        self.rsaKeyBitLength = rsaKeyBitLength;
        buildCircuit();
    }
}
impl Gadget for RSAEncryptionV1_5_Gadget {
    pub fn getExpectedRandomnessLength(rsaKeyBitLength: i32, plainTextLength: i32) -> i32 {
        assert!(
            rsaKeyBitLength % 8 == 0,
            "RSA Key bit length is assumed to be a multiple of 8"
        );

        rsaKeyBitLength / 8 - 3 - plainTextLength
    }

    fn buildCircuit() {
        let lengthInBytes = rsaKeyBitLength / 8;
        let paddedPlainText = vec![None; lengthInBytes];
        for i in 0..plainText.len() {
            paddedPlainText[plainText.len() - i - 1] = plainText[i];
        }
        paddedPlainText[plainText.len()] = generator.get_zero_wire();
        for i in 0..randomness.len() {
            paddedPlainText[plainText.len() + 1 + (randomness.len() - 1) - i] = randomness[i];
        }
        paddedPlainText[lengthInBytes - 2] = generator.createConstantWire(2);
        paddedPlainText[lengthInBytes - 1] = generator.get_zero_wire();

        /*
         * To proceed with the RSA operations, we need to convert the
         * padddedPlainText array to a long element. Two ways to do that.
         */
        // 1. safest method:
        //		 WireArray allBits = WireArray::new(paddedPlainText).getBits(8);
        //		 LongElement paddedMsg = LongElement::new(allBits);

        // 2. Make multiple long integer constant multiplications (need to be
        // done carefully)
        let paddedMsg = LongElement::new(vec![BigInteger::ZERO]);
        for i in 0..paddedPlainText.len() {
            let e = LongElement::new(paddedPlainText[i], 8);
            let c = LongElement::new(Util::split(
                Util::one().shl(8 * i),
                LongElement.CHUNK_BITWIDTH,
            ));
            paddedMsg = paddedMsg.add(e.mul(c));
        }

        let s = paddedMsg;
        for i in 0..16 {
            s = s.mul(s);
            s = LongIntegerModGadget::new(s, modulus, rsaKeyBitLength, false).getRemainder();
        }
        s = s.mul(paddedMsg);
        s = LongIntegerModGadget::new(s, modulus, rsaKeyBitLength, true).getRemainder();

        // return the cipher text as byte array
        ciphertext = s.getBits(rsaKeyBitLength).packBitsIntoWords(8);
    }

    pub fn checkRandomnessCompliance() {
        // assert the randomness vector has non-zero bytes
        for i in 0..randomness.len() {
            randomness[i].restrictBitLength(8);
            // verify that each element has a multiplicative inverse
            FieldDivisionGadget::new(generator.get_one_wire(), randomness[i]);
        }
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        ciphertext
    }
}
