
use circuit::operations::gadget;
use circuit::structure::wire;
use zkay::typed_wire;
use zkay::zkay_cbc_symmetric_enc_gadget;
use zkay::zkay_cbc_symmetric_enc_gadget::cipher_type;

pub struct ECDHBackend{
CipherType cipherType
}
 impl ECDHBackend CryptoBackend.Symmetric {

	const KEY_CHUNK_SIZE:i32 = 256;

	 

	pub  fn new( keyBits:i32 ,  cipherType:CipherType )->Self {
		super(keyBits);
		self.cipherType = cipherType;
	}

	
	pub  fn getKeyChunkSize()-> i32 {
		return KEY_CHUNK_SIZE;
	}

	 impl Symmetric for ECDHBackend  {
	pub  fn createEncryptionGadget( plain:TypedWire ,  key:String ,  ivArr:Vec<Wire> , desc:Vec<String>)-> Gadget {
		return ZkayCBCSymmetricEncGadget::new(plain, getKey(key), extractIV(ivArr), cipherType, desc);
	}
}
