use crate::circuit::operations::gadget::GadgetConfig;
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
    pk: &WireType,
    plain: &WireType,
    cipher: Vec<Option<WireType>>,
}
impl ZkayDummyHomEncryptionGadget {
    pub fn new(
        plain: &WireType,
        pk: &WireType,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        desc: &Option<String>,
    ) -> Gadget<Self> {
        // assert!(plain, "plain");
        // assert!(pk, "pk");
        // assert!(rnd, "rnd");
        assert!(rnd.len() <= 1, "Randomness wire array too long");

        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                plain,
                pk,
                cipher: vec![None; 1],
            },
        };
        _self.buildCircuit();
        _self
    }
}

impl Gadget<ZkayDummyHomEncryptionGadget> {
    fn buildCircuit(&mut self) {
        self.t.cipher[0] = plain.mul(pk, "plain * pk").add(1);
    }
}

impl GadgetConfig for Gadget<ZkayDummyHomEncryptionGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
