

use util::util;
use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::hash::sha256_gadget;
use examples::gadgets::math::long_integer_mod_gadget;

/**
 * A gadget for RSA encryption according to PKCS#1 v2.2. The gadget assumes a
 * hardcoded pub  exponent of 0x10001, and uses SHA256 as the hash function
 * for mask generation function (mgf).
 * This gadget can accept a hardcoded or a variable RSA modulus. See the
 * corresponding generator example. 
 * 
 * This gadget is costly in comparison with the PKCS v1.5 RSA encryption gadget
 * due to many SHA256 calls during mask generation.
 * 
 * The implementation of this gadget follows the standard specs in:
 * https://www.emc.com/collateral/white-
 * papers/h11300-pkcs-1v2-2-rsa-cryptography-standard-wp.pdf
 */

pub struct RSAEncryptionOAEPGadget  {

	 modulus:LongElement,

	// every wire represents a byte in the following three arrays
	 plainText:Vec<Wire>,
	 seed:Vec<Wire>,

	 ciphertext:Vec<Wire>,

	 rsaKeyBitLength:i32, // in bits (assumed to be divisible by 8)
	
}
impl  RSAEncryptionOAEPGadget{
pub   const SHA256_DIGEST_LENGTH :i32= 32; // in bytes

	pub  const lSHA256_HASH: Vec<byte>  = vec![ (byte) 0xe3,
			(byte) 0xb0, (byte) 0xc4, 0x42, (byte) 0x98, (byte) 0xfc, 0x1c,
			0x14, (byte) 0x9a, (byte) 0xfb, (byte) 0xf4, (byte) 0xc8,
			(byte) 0x99, 0x6f, (byte) 0xb9, 0x24, 0x27, (byte) 0xae, 0x41,
			(byte) 0xe4, 0x64, (byte) 0x9b, (byte) 0x93, 0x4c, (byte) 0xa4,
			(byte) 0x95, (byte) 0x99, 0x1b, 0x78, 0x52, (byte) 0xb8, 0x55 ];
	pub  fn new(LongElement modulus, plainText:Vec<Wire>,
			seed:Vec<Wire>, rsaKeyBitLength:i32, desc:Vec<String>) ->Self{
		super(desc);

			assert!(rsaKeyBitLength % 8 == 0,
					"RSA Key bit length is assumed to be a multiple of 8");
		


			assert!( plainText.length <= rsaKeyBitLength / 8 - 2 * SHA256_DIGEST_LENGTH - 2,
					"Message too long,Invalid message length for RSA Encryption");
		


			assert!(seed.length == SHA256_DIGEST_LENGTH ,
					"Seed must have the same length as the hash function output,Invalid seed dimension for RSA Encryption");
		

		self.seed = seed;
		self.plainText = plainText;
		self.modulus = modulus;
		self.rsaKeyBitLength = rsaKeyBitLength;
		buildCircuit();
	}
}
impl Gadget for RSAEncryptionOAEPGadget{
	  fn buildCircuit() {

		let mLen = plainText.length;
		let hLen = SHA256_DIGEST_LENGTH;
		let keyLen = rsaKeyBitLength / 8; // in bytes
		let mut paddingString = vec![generator.getZeroWire();keyLen - mLen - 2 * hLen - 2];

		let mut db = vec![Wire::default();keyLen - hLen - 1];
		for i in 0..keyLen - hLen - 1{
			if i < hLen {
				db[i] = generator
						.createConstantWire((lSHA256_HASH[i] + 256) % 256);
			} else if i < hLen + paddingString.length {
				db[i] = paddingString[i - hLen];
			} else if i < hLen + paddingString.length + 1 {
				db[i] = generator.getOneWire();
			} else {
				db[i] = plainText[i - (hLen + paddingString.length + 1)];
			}
		}

		let dbMask = mgf1(seed, keyLen - hLen - 1);
		let maskedDb = vec![Wire::default();keyLen - hLen - 1];
		for i in 0..keyLen - hLen - 1{
			maskedDb[i] = dbMask[i].xorBitwise(db[i], 8);
		}

		let seededMask = mgf1(maskedDb, hLen);
		let maskedSeed = vec![Wire::default();hLen];
		for i in 0..hLen {
			maskedSeed[i] = seededMask[i].xorBitwise(seed[i], 8);
		}
		
		let paddedByteArray = Util::concat(maskedSeed, maskedDb); // Big-Endian
		
		// The LongElement implementation is LittleEndian, so we will process the array in reverse order
		
		let paddedMsg = LongElement::new(
				vec![BigInteger.ZERO ]);
		for i in 0..paddedByteArray.length {
			let e = LongElement::new(paddedByteArray[paddedByteArray.length-i-1], 8);
			let c = LongElement::new(Util::split(
					BigInteger.ONE.shiftLeft(8 * i),
					LongElement.CHUNK_BITWIDTH));
			paddedMsg = paddedMsg.add(e.mul(c));
		}
		
		// do modular exponentiation
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

	pub   checkSeedCompliance() {
		for i in 0..seed.length {
			// Verify that the seed wires are bytes
			// This is also checked already by the sha256 gadget in the mgf1 calls, but added here for clarity
			seed[i].restrictBitLength(8);
		}
	}
	
	 fn mgf1(in:Vec<Wire>, length:i32 )->Vec<Wire> {

		let mut mgfOutputList = vec![];
		for  i in  0..=(length * 1.0/ SHA256_DIGEST_LENGTH).ceil() as i32 - 1 {

			// the standard follows a Big Endian format
			let counter = generator.createConstantWireArray(vec![
					(byte) (i >>> 24), (byte) (i >>> 16), (byte) (i >>> 8),
					(byte) i ]);

			let inputToHash = Util::concat(in, counter);
			let shaGadget = SHA256Gadget::new(inputToHash, 8,
					inputToHash.length, false, true);
			let digest = shaGadget.getOutputWires();

			let msgHashBytes = WireArray::new(digest).getBits(32)
					.packBitsIntoWords(8);
			// reverse the byte array representation of each word of the digest
			// to
			// be compatible with the endianess
			for j in 0..8 {
				let tmp = msgHashBytes[4 * j];
				msgHashBytes[4 * j] = msgHashBytes[(4 * j + 3)];
				msgHashBytes[4 * j + 3] = tmp;
				tmp = msgHashBytes[4 * j + 1];
				msgHashBytes[4 * j + 1] = msgHashBytes[4 * j + 2];
				msgHashBytes[4 * j + 2] = tmp;
			}
			for j in 0..msgHashBytes.length {
				mgfOutputList.add(msgHashBytes[j]);
			}
		}
		let out = mgfOutputList.toArray(vec![]);
		return out[..length].to_vec();
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return ciphertext;
	}

}
