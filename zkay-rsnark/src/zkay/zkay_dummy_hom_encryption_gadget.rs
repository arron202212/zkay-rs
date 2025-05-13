
use circuit::operations::gadget;
use circuit::structure::wire;



/**
 * Dummy encryption gadget whose ciphertext is additively homomorphic.
 * Key: Some prime number p smaller than the field prime.
 * Encryption: Enc(msg, p) = msg * p mod field_prime.
 * Decryption: Dec(cipher) = cipher * p^-1 mod field_prime.
 * Additive homomorphism: Enc(m1, p) + Enc(m2, p)     (all mod field_prime)
 *                        = (m1 * p) + (m2 * p)
 *                        = (m1 + m2) * p
 *                        = Enc(m1 + m2, p)
 */
pub struct ZkayDummyHomEncryptionGadget {
	 pk:Wire,
	 plain:Wire,
	 cipher:Vec<Wire>,
}
impl  ZkayDummyHomEncryptionGadget{
	pub  fn new(plain:Wire , pk:Wire , rnd:Vec<Wire>, keyBits:i32 , desc:Vec<String>)->Self {
		super(desc);

		Objects.requireNonNull(plain, "plain");
		Objects.requireNonNull(pk, "pk");
		Objects.requireNonNull(rnd, "rnd");
		if rnd.length > 1) assert!("Randomness wire array too long";

		self.plain = plain;
		self.pk = pk;
		self.cipher = vec![Wire::default();1];
		buildCircuit();
	}
}

impl Gadget for ZkayDummyHomEncryptionGadget{
	  fn buildCircuit() {
		cipher[0] = plain.mul(pk, "plain * pk").add(1);
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return cipher;
	}
}
