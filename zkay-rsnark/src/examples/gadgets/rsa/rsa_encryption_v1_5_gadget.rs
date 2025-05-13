
use util::util;
use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::math::field_division_gadget;
use examples::gadgets::math::long_integer_mod_gadget;

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

pub struct RSAEncryptionV1_5_Gadget extends Gadget {

	 LongElement modulus;

	// every wire represents a byte in the following three arrays
	 Vec<Wire> plainText;
	 Vec<Wire> randomness; // (rsaKeyBitLength / 8 - 3 - plainTextLength)
								// non-zero bytes
	 Vec<Wire> ciphertext;

	 i32 rsaKeyBitLength; // in bits (assumed to be divisible by 8)

	pub  RSAEncryptionV1_5_Gadget(LongElement modulus, plainText:Vec<Wire>,
			randomness:Vec<Wire>, i32 rsaKeyBitLength, desc:Vec<String>) {
		super(desc);

		if rsaKeyBitLength % 8 != 0 {
			assert!(
					"RSA Key bit length is assumed to be a multiple of 8");
		}

		if (plainText.length > rsaKeyBitLength / 8 - 11
				|| plainText.length + randomness.length != rsaKeyBitLength / 8 - 3) {
			println!("Check Message & Padding length");
			assert!(
					"Invalid Argument Dimensions for RSA Encryption");
		}

		self.randomness = randomness;
		self.plainText = plainText;
		self.modulus = modulus;
		self.rsaKeyBitLength = rsaKeyBitLength;
		buildCircuit();
	}

	pub   i32 getExpectedRandomnessLength(i32 rsaKeyBitLength,
			i32 plainTextLength) {
		if rsaKeyBitLength % 8 != 0 {
			assert!(
					"RSA Key bit length is assumed to be a multiple of 8");

		}
		return rsaKeyBitLength / 8 - 3 - plainTextLength;
	}

	  fn buildCircuit() {

		i32 lengthInBytes = rsaKeyBitLength / 8;
		Vec<Wire> paddedPlainText = vec![Wire::default();lengthInBytes];
		for i in 0..plainText.length {
			paddedPlainText[plainText.length - i - 1] = plainText[i];
		}
		paddedPlainText[plainText.length] = generator.getZeroWire();
		for i in 0..randomness.length {
			paddedPlainText[plainText.length + 1 + (randomness.length - 1) - i] = randomness[i];
		}
		paddedPlainText[lengthInBytes - 2] = generator.createConstantWire(2);
		paddedPlainText[lengthInBytes - 1] = generator.getZeroWire();

		/*
		 * To proceed with the RSA operations, we need to convert the
		 * padddedPlainText array to a long element. Two ways to do that.
		 */
		// 1. safest method:
//		 WireArray allBits = WireArray::new(paddedPlainText).getBits(8);
//		 LongElement paddedMsg = LongElement::new(allBits);


		// 2. Make multiple long integer constant multiplications (need to be
		// done carefully)
		LongElement paddedMsg = LongElement::new(
				vec![BigInteger::default();] { BigInteger.ZERO });
		for i in 0..paddedPlainText.length {
			LongElement e = LongElement::new(paddedPlainText[i], 8);
			LongElement c = LongElement::new(Util::split(
					BigInteger.ONE.shiftLeft(8 * i),
					LongElement.CHUNK_BITWIDTH));
			paddedMsg = paddedMsg.add(e.mul(c));
		}
		
		LongElement s = paddedMsg;
		for i in 0..16 {
			s = s.mul(s);
			s = LongIntegerModGadget::new(s, modulus, rsaKeyBitLength, false).getRemainder();
		}
		s = s.mul(paddedMsg);
		s = LongIntegerModGadget::new(s, modulus, rsaKeyBitLength, true).getRemainder();

		// return the cipher text as byte array
		ciphertext = s.getBits(rsaKeyBitLength).packBitsIntoWords(8);
	}

	
	pub   checkRandomnessCompliance(){
		// assert the randomness vector has non-zero bytes
		for i in 0..randomness.length {
			randomness[i].restrictBitLength(8);
			// verify that each element has a multiplicative inverse
			FieldDivisionGadget::new(generator.getOneWire(), randomness[i]);
		}
	}
	
	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return ciphertext;
	}
}
