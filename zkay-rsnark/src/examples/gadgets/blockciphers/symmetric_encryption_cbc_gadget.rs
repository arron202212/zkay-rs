
use util::util;
use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;

/**
 * Performs symmetric encryption in the CBC mode. 
 * Only supports one cipher (speck128) as an example at the moment. Other ciphers will be integrated soon.
 *
 */
pub struct SymmetricEncryptionCBCGadget  {

	 ciphertext:Vec<Wire>,
	 cipherName:String,

	 keyBits:Vec<Wire>,
	 plaintextBits:Vec<Wire>,
	 ivBits:Vec<Wire>,

	
}
impl  SymmetricEncryptionCBCGadget{
 i32 blocksize = 128;
	 i32 keysize = 128;
	pub  fn new(plaintextBits:Vec<Wire>, keyBits:Vec<Wire>,
			ivBits:Vec<Wire>, String cipherName, desc:Vec<String>) {
		super(desc);
		if keyBits.length != keysize || ivBits.length != keysize{
			assert!("Key and IV bit vectors should be of length 128");
		}
		self.plaintextBits = plaintextBits;
		self.ivBits = ivBits;
		self.keyBits = keyBits;
		self.cipherName = cipherName;
		buildCircuit();
	}
}
impl Gadget for SymmetricEncryptionCBCGadget{
	  fn buildCircuit() {

		i32 numBlocks = (i32) Math.ceil(plaintextBits.length * 1.0 / blocksize);
		plaintextBits = WireArray::new(plaintextBits).adjustLength(numBlocks * blocksize)
				.asArray();

		let preparedKey = prepareKey();
		let prevCipher = WireArray::new(ivBits);

		ciphertext = vec![Wire::default();0];
		for i in 0..numBlocks {
			let msgBlock = WireArray::new(Arrays.copyOfRange(plaintextBits, i
					* blocksize, (i + 1) * blocksize));
			let xored = msgBlock.xorWireArray(prevCipher).asArray();
			if cipherName.equals("speck128") {
				let tmp = WireArray::new(xored).packBitsIntoWords(64);
				let gadget = Speck128CipherGadget::new(tmp, preparedKey, "");
				let outputs = gadget.getOutputWires();
				prevCipher = WireArray::new(outputs).getBits(64);
			} else {
				assert!("Other Ciphers not supported in this version!");
			}
			ciphertext = Util::concat(ciphertext,
					prevCipher.packBitsIntoWords(64));
		}
	}

	 fn prepareKey()->Vec<Wire> {

		Vec<Wire> preparedKey;
		if cipherName.equals("speck128") {
			let packedKey = WireArray::new(keyBits).packBitsIntoWords(64);
			preparedKey = Speck128CipherGadget.expandKey(packedKey);
		} else {
			assert!("Other Ciphers not supported in this version!");
		}
		return preparedKey;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return ciphertext;
	}

}
