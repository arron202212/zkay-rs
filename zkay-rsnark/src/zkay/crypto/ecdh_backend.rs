
use circuit::operations::gadget;
use circuit::structure::wire;
use zkay::typed_wire;
use zkay::zkay_cbc_symmetric_enc_gadget;
use zkay::zkay_cbc_symmetric_enc_gadget::cipher_type;

public class ECDHBackend extends CryptoBackend.Symmetric {

	public static final int KEY_CHUNK_SIZE = 256;

	 CipherType cipherType;

	public ECDHBackend(int keyBits, CipherType cipherType) {
		super(keyBits);
		this.cipherType = cipherType;
	}

	
	public int getKeyChunkSize() {
		return KEY_CHUNK_SIZE;
	}

	
	public Gadget createEncryptionGadget(TypedWire plain, String key, Wire[] ivArr, String... desc) {
		return new ZkayCBCSymmetricEncGadget(plain, getKey(key), extractIV(ivArr), cipherType, desc);
	}
}
