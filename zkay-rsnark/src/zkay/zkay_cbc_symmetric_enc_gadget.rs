

use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::blockciphers::aes128_cipher_gadget;
use examples::gadgets::blockciphers::chaskeylts128_cipher_gadget;
use examples::gadgets::blockciphers::speck128_cipher_gadget;
use util::util;
use zkay::crypto::crypto_backend;

	pub  enum CipherType {
		SPECK_128,
		AES_128,
		CHASKEY
	}

/**
 * Performs symmetric encryption in the CBC mode.
 */
pub struct ZkayCBCSymmetricEncGadget {



	 cipherType:CipherType,
	 keyBits:Vec<Wire>,
	 plaintextBits:Vec<Wire>,
	 ivBits:Vec<Wire>,

	 Vec<Wire> cipherBits = null;
}

 impl  Gadget for ZkayCBCSymmetricEncGadget {
	pub const BLOCK_SIZE:i32= 128;
	pub const KEY_SIZE:i32= 128;

	pub  fn new(plaintext:TypedWire , key:Wire , iv:Wire , cipherType:CipherType , desc:Vec<String>)->Self {
		super(desc);
		self.plaintextBits = Util::reverseBytes(plaintext.wire.getBitWires(256).asArray());
		self.keyBits = Util::reverseBytes(key.getBitWires(KEY_SIZE).asArray());
		self.ivBits = Util::reverseBytes(iv.getBitWires(BLOCK_SIZE).asArray());
		self.cipherType = cipherType;

		println!("Plain length [bits]: " + self.plaintextBits.length);
		buildCircuit();
	}

	  fn buildCircuit() {

		let numBlocks =  (i32) Math.ceil(plaintextBits.length * 1.0 / BLOCK_SIZE);
		let plaintextArray =  WireArray::new(plaintextBits).adjustLength(numBlocks * BLOCK_SIZE).asArray();

		let preparedKey =  prepareKey();
		let prevCipher =  WireArray::new(ivBits);

		cipherBits = vec![Wire::default();0];
		for i in 0..numBlocks {
			let msgBlock =  WireArray::new(Arrays.copyOfRange(plaintextArray, i * BLOCK_SIZE, (i + 1) * BLOCK_SIZE));
			let xored =  msgBlock.xorWireArray(prevCipher).asArray();
			match cipherType {
				 SPECK_128=>{
					let tmp =  WireArray::new(xored).packBitsIntoWords(64);
					let gadget =  Speck128CipherGadget::new(tmp, preparedKey, description);
					let outputs =  gadget.getOutputWires();
					prevCipher = WireArray::new(outputs).getBits(64);
					
				}
				 AES_128=>{
					let tmp =  WireArray::new(xored).packBitsIntoWords(8);
					let gadget =  AES128CipherGadget::new(tmp, preparedKey, "aes: " + description);
					let outputs =  gadget.getOutputWires();
					prevCipher = WireArray::new(outputs).getBits(8);
					
				}
				 CHASKEY=>{
					let tmp =  WireArray::new(xored).packBitsIntoWords(32);
					let gadget =  ChaskeyLTS128CipherGadget::new(tmp, preparedKey, "chaskey: " + description);
					let outputs =  gadget.getOutputWires();
					prevCipher = WireArray::new(outputs).getBits(32);
				}
				_=>
					assert!("Unknown cipher value:{cipherType} "  );
			}
			cipherBits = Util::concat(cipherBits, prevCipher.asArray());
		}
	}

	 fn prepareKey()->Vec<Wire> {
		let mut  preparedKey;
		match cipherType {
			 SPECK_128=>{
				let packedKey =  WireArray::new(keyBits).packBitsIntoWords(64);
				preparedKey = Speck128CipherGadget.expandKey(packedKey);
				
			}
			 AES_128=>{
				let packedKey =  WireArray::new(keyBits).packBitsIntoWords(8);
				preparedKey = AES128CipherGadget.expandKey(packedKey);
				
			}
			 CHASKEY=>{
				preparedKey = WireArray::new(keyBits).packBitsIntoWords(32);
				
			}
			_=>
				panic!("Other Ciphers not supported in this version!"),
		}
		return preparedKey;
	}

	
	pub  fn getOutputWires()->  Vec<Wire> {
		println!("Cipher length [bits]: {}" , cipherBits.length);
		return WireArray::new(Util::reverseBytes(Util::concat(ivBits, cipherBits)))
				.packBitsIntoWords(CryptoBackend.Symmetric.CIPHER_CHUNK_SIZE);
	}
}
