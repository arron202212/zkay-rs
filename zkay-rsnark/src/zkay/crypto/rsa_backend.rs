

use circuit::operations::gadget;
use circuit::structure::wire;
use zkay::typed_wire;
use zkay::zkay_rsa_encryption_gadget;

public class RSABackend extends CryptoBackend.Asymmetric {

	public static final int CIPHER_CHUNK_SIZE = 232;
	public static final int KEY_CHUNK_SIZE = 232;
	public static final int PKCS15_RND_CHUNK_SIZE = 224;
	public static final int OAEP_RND_CHUNK_SIZE = 128;

	 ZkayRSAEncryptionGadget.PaddingType paddingType;

	public RSABackend(int keyBits, ZkayRSAEncryptionGadget.PaddingType padding) {
		super(keyBits);
		this.paddingType = padding;
	}

	
	public int getKeyChunkSize() {
		return KEY_CHUNK_SIZE;
	}

	
	public Gadget createEncryptionGadget(TypedWire plain, String key, Wire[] random, String... desc) {
		return new ZkayRSAEncryptionGadget(plain, getKey(key), random, keyBits, paddingType, desc);
	}
}
