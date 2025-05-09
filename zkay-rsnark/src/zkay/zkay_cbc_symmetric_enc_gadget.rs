

use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::blockciphers::aes128_cipher_gadget;
use examples::gadgets::blockciphers::chaskeylts128_cipher_gadget;
use examples::gadgets::blockciphers::speck128_cipher_gadget;
use util::util;
use zkay::crypto::crypto_backend;



/**
 * Performs symmetric encryption in the CBC mode.
 */
public class ZkayCBCSymmetricEncGadget extends Gadget {

	public enum CipherType {
		SPECK_128,
		AES_128,
		CHASKEY
	}

	 CipherType cipherType;
	 Wire[] keyBits;
	 Wire[] plaintextBits;
	 Wire[] ivBits;

	private Wire[] cipherBits = null;

	public static final int BLOCK_SIZE = 128;
	public static final int KEY_SIZE = 128;

	public ZkayCBCSymmetricEncGadget(TypedWire plaintext, Wire key, Wire iv, CipherType cipherType, String... desc) {
		super(desc);
		this.plaintextBits = Util.reverseBytes(plaintext.wire.getBitWires(256).asArray());
		this.keyBits = Util.reverseBytes(key.getBitWires(KEY_SIZE).asArray());
		this.ivBits = Util.reverseBytes(iv.getBitWires(BLOCK_SIZE).asArray());
		this.cipherType = cipherType;

		println!("Plain length [bits]: " + this.plaintextBits.length);
		buildCircuit();
	}

	protected void buildCircuit() {

		int numBlocks = (int) Math.ceil(plaintextBits.length * 1.0 / BLOCK_SIZE);
		Wire[] plaintextArray = new WireArray(plaintextBits).adjustLength(numBlocks * BLOCK_SIZE).asArray();

		Wire[] preparedKey = prepareKey();
		WireArray prevCipher = new WireArray(ivBits);

		cipherBits = new Wire[0];
		for i in 0..numBlocks {
			WireArray msgBlock = new WireArray(Arrays.copyOfRange(plaintextArray, i * BLOCK_SIZE, (i + 1) * BLOCK_SIZE));
			Wire[] xored = msgBlock.xorWireArray(prevCipher).asArray();
			switch (cipherType) {
				case SPECK_128: {
					Wire[] tmp = new WireArray(xored).packBitsIntoWords(64);
					Gadget gadget = new Speck128CipherGadget(tmp, preparedKey, description);
					Wire[] outputs = gadget.getOutputWires();
					prevCipher = new WireArray(outputs).getBits(64);
					break;
				}
				case AES_128: {
					Wire[] tmp = new WireArray(xored).packBitsIntoWords(8);
					Gadget gadget = new AES128CipherGadget(tmp, preparedKey, "aes: " + description);
					Wire[] outputs = gadget.getOutputWires();
					prevCipher = new WireArray(outputs).getBits(8);
					break;
				}
				case CHASKEY: {
					Wire[] tmp = new WireArray(xored).packBitsIntoWords(32);
					Gadget gadget = new ChaskeyLTS128CipherGadget(tmp, preparedKey, "chaskey: " + description);
					Wire[] outputs = gadget.getOutputWires();
					prevCipher = new WireArray(outputs).getBits(32);
					break;
				}
				default:
					throw new IllegalStateException("Unknown cipher value: " + cipherType);
			}
			cipherBits = Util.concat(cipherBits, prevCipher.asArray());
		}
	}

	private Wire[] prepareKey() {
		Wire[] preparedKey;
		switch (cipherType) {
			case SPECK_128: {
				Wire[] packedKey = new WireArray(keyBits).packBitsIntoWords(64);
				preparedKey = Speck128CipherGadget.expandKey(packedKey);
				break;
			}
			case AES_128: {
				Wire[] packedKey = new WireArray(keyBits).packBitsIntoWords(8);
				preparedKey = AES128CipherGadget.expandKey(packedKey);
				break;
			}
			case CHASKEY: {
				preparedKey = new WireArray(keyBits).packBitsIntoWords(32);
				break;
			}
			default:
				throw new UnsupportedOperationException("Other Ciphers not supported in this version!");
		}
		return preparedKey;
	}

	
	public Wire[] getOutputWires() {
		println!("Cipher length [bits]: " + cipherBits.length);
		return new WireArray(Util.reverseBytes(Util.concat(ivBits, cipherBits)))
				.packBitsIntoWords(CryptoBackend.Symmetric.CIPHER_CHUNK_SIZE);
	}
}
