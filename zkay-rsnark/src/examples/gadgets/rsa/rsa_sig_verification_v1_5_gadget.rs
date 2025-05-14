
use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::math::long_integer_mod_gadget;

/**
 * A gadget to check if an RSA signature is valid according to PKCS 1 v1.5 (A
 * gadget based on the latest standard (PSS) will be added in the future).
 * This gadget assumes SHA256 for the message hash, and a pub  exponent of
 * 0x10001.
 * This gadget can accept a hardcoded or a variable RSA modulus. See the
 * corresponding generator example. 
 * 
 * Implemented according to the standard specs here:
 * https://www.emc.com/collateral/white-
 * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf
 * 
 * 
 * 
 * 
 */
pub struct RSASigVerificationV1_5_Gadget  {

	 modulus:LongElement,
	 signature:LongElement,
	 msgHash:Vec<Wire>, // 32-bit wires (the output of SHA256 gadget)
	 isValidSignature:Wire,
	 rsaKeyBitLength:i32, // in bits


}
impl RSASigVerificationV1_5_Gadget{
	pub  const SHA256_IDENTIFIER: Vec<byte>  = vec![byte::default();] { 0x30, 0x31,
			0x30, 0x0d, 0x06, 0x09, 0x60, (byte) 0x86, 0x48, 0x01, 0x65, 0x03,
			0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20 };

	pub   const SHA256_DIGEST_LENGTH:i32 = 32; // in bytes
	pub  fn new( modulus:LongElement, msgHash:Vec<Wire>,
			 signature:LongElement, rsaKeyBitLength:i32, desc:Vec<String>)->Self {
		super(desc);
		self.modulus = modulus;
		self.msgHash = msgHash;
		self.signature = signature;
		self.rsaKeyBitLength = rsaKeyBitLength;
		buildCircuit();
	}
}
impl Gadget for RSASigVerificationV1_5_Gadget{
	  fn buildCircuit() {

		let s = signature;

		for i in 0..16 {
			s = s.mul(s);
			s = LongIntegerModGadget::new(s, modulus, rsaKeyBitLength, false).getRemainder();
		}
		s = s.mul(signature);
		s = LongIntegerModGadget::new(s, modulus, rsaKeyBitLength, true).getRemainder();
		let sChunks = s.getArray();

		// note that the following can be improved, but for simplicity we
		// are going to compare byte by byte

		// get byte arrays
		let sBytes = WireArray::new(sChunks).getBits(
				LongElement.CHUNK_BITWIDTH).packBitsIntoWords(8);
		let msgHashBytes = WireArray::new(msgHash).getBits(32)
				.packBitsIntoWords(8);

		// reverse the byte array representation of each word of the digest to
		// be compatiable with the endianess
		for i in 0..8 {
			let tmp = msgHashBytes[4 * i];
			msgHashBytes[4 * i] = msgHashBytes[(4 * i + 3)];
			msgHashBytes[4 * i + 3] = tmp;
			tmp = msgHashBytes[4 * i + 1];
			msgHashBytes[4 * i + 1] = msgHashBytes[4 * i + 2];
			msgHashBytes[4 * i + 2] = tmp;
		}

		let lengthInBytes = (rsaKeyBitLength * 1.0 / 8).ceil() as i32;
		let sumChecks = generator.getZeroWire();
		sumChecks = sumChecks.add(sBytes[lengthInBytes - 1].isEqualTo(0));
		sumChecks = sumChecks.add(sBytes[lengthInBytes - 2].isEqualTo(1));
		for  i in  3.. lengthInBytes - SHA256_DIGEST_LENGTH- SHA256_IDENTIFIER.length {
			sumChecks = sumChecks
					.add(sBytes[lengthInBytes - i].isEqualTo(0xff));
		}
		sumChecks = sumChecks.add(sBytes[SHA256_DIGEST_LENGTH
				+ SHA256_IDENTIFIER.length].isEqualTo(0));

		for i in 0..SHA256_IDENTIFIER.length {
			sumChecks = sumChecks.add(sBytes[SHA256_IDENTIFIER.length
					+ SHA256_DIGEST_LENGTH - 1 - i]
					.isEqualTo((i32) (SHA256_IDENTIFIER[i] + 256) % 256));
		}
		for i in (0..=SHA256_DIGEST_LENGTH - 1).rev()
			sumChecks = sumChecks.add(sBytes[SHA256_DIGEST_LENGTH - 1 - i]
					.isEqualTo(msgHashBytes[i]));
		}

		isValidSignature = sumChecks.isEqualTo(lengthInBytes);

	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![isValidSignature];
	}
}
