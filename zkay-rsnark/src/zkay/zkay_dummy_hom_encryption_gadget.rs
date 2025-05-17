use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

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
    pk: WireType,
    plain: WireType,
    cipher: Vec<WireType>,
}
impl ZkayDummyHomEncryptionGadget {
    pub fn new(plain: WireType, pk: WireType, rnd: Vec<WireType>, keyBits: i32, desc: Vec<String>) -> Self {
        super(desc);

        Objects.requireNonNull(plain, "plain");
        Objects.requireNonNull(pk, "pk");
        Objects.requireNonNull(rnd, "rnd");
        assert!(rnd.length <= 1, "Randomness wire array too long");

        self.plain = plain;
        self.pk = pk;
        self.cipher = vec![WireType::default(); 1];
        buildCircuit();
    }
}

impl Gadget for ZkayDummyHomEncryptionGadget {
    fn buildCircuit() {
        cipher[0] = plain.mul(pk, "plain * pk").add(1);
    }

    pub fn getOutputWires() -> Vec<WireType> {
        return cipher;
    }
}
