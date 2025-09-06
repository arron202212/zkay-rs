#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::CircuitGenerator, wire::WireConfig, wire_array::WireArray,
            wire_type::WireType,
        },
    },
    zkay::{
        crypto::{
            crypto_backend::{
                Asymmetric, CryptoBackend, CryptoBackendConfig, CryptoBackendConfigs,
            },
            homomorphic_backend::HomomorphicBackend,
        },
        homomorphic_input::HomomorphicInput,
        typed_wire::TypedWire,
        zkay_baby_jub_jub_gadget::{JubJubPoint, ZkayBabyJubJubGadget},
        zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget,
        zkay_elgamal_add_gadget::ZkayElgamalAddGadget,
        zkay_elgamal_dec_gadget::ZkayElgamalDecGadget,
        zkay_elgamal_enc_gadget::ZkayElgamalEncGadget,
        zkay_elgamal_mul_gadget::ZkayElgamalMulGadget,
        zkay_elgamal_rerand_gadget::ZkayElgamalRerandGadget,
        zkay_type::ZkayType,
    },
};

use rccell::{RcCell, WeakCell};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct ElgamalBackend;

impl ElgamalBackend {
    const EC_COORD_BITS: i32 = 254; // a BabyJubJub affine coordinate fits into 254 bits

    const KEY_CHUNK_SIZE: i32 = 256; // needs to be a multiple of 8

    const RND_CHUNK_SIZE: i32 = 256;

    pub fn new(
        keyBits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        // pub  key must be a BabyJubJub point (two coordinates)
        assert!(keyBits == 2 * Self::EC_COORD_BITS, "pub  key size mismatch");
        Asymmetric::<Self>::new(keyBits, Self, generator)
    }
}

impl CryptoBackendConfigs for CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn isSymmetric(&self) -> bool {
        false
    }
    fn usesDecryptionGadget(&self) -> bool {
        // randomness is not extractable from an ElGamal ciphertext, so need a separate
        // gadget for decryption
        true
    }

    fn addKey(
        &mut self,
        keyName: &String,
        keyWires: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) {
        // elgamal does not require a bit-representation of the pub  key, so store it directly
        self.t.keys.insert(
            keyName.clone(),
            WireArray::new(keyWires.clone(), generator.downgrade()),
        );
    }
    fn createDecryptionGadget(
        &self,
        plain: &TypedWire,
        cipher: &Vec<Option<WireType>>,
        pkName: &String,
        sk: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let pkArray = self.getKeyArray(pkName);
        let pk = JubJubPoint::new(pkArray[0].clone().unwrap(), pkArray[1].clone().unwrap());
        let c1 = JubJubPoint::new(cipher[0].clone().unwrap(), cipher[1].clone().unwrap());
        let c2 = JubJubPoint::new(cipher[2].clone().unwrap(), cipher[3].clone().unwrap());
        let skBits = WireArray::new(sk.clone(), generator.clone().downgrade())
            .get_bits(ElgamalBackend::RND_CHUNK_SIZE as usize, &None)
            .as_array()
            .clone();
        Box::new(ZkayElgamalDecGadget::new(
            pk,
            skBits,
            c1,
            c2,
            plain.wire.clone(),
            generator,
        ))
    }
    fn setKeyPair(
        &mut self,
        myPk: &WireType,
        mySk: &WireType,
        generator: RcCell<CircuitGenerator>,
    ) {
        panic!("setKeyPair no in Asymmetric");
    }
}
impl CryptoBackendConfig for CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn getKeyChunkSize(&self) -> i32 {
        ElgamalBackend::KEY_CHUNK_SIZE
    }
    fn createEncryptionGadget(
        &mut self,
        plain: &TypedWire,
        keyName: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        let pkArray = self.getKeyArray(keyName);
        let pk = JubJubPoint::new(pkArray[0].clone().unwrap(), pkArray[1].clone().unwrap());
        let randomArray = WireArray::new(random.clone(), generator.clone().downgrade())
            .get_bits(ElgamalBackend::RND_CHUNK_SIZE as usize, &None)
            .as_array()
            .clone();
        assert!(
            plain.zkay_type.bitwidth <= 32,
            "plaintext must be at most 32 bits for elgamal backend"
        );
        Box::new(ZkayElgamalEncGadget::new(
            plain
                .wire
                .get_bit_wiresi(plain.zkay_type.bitwidth as u64, &None)
                .as_array()
                .clone(),
            pk,
            randomArray.clone(),
            generator,
        ))
    }
}
impl CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn toTypedWireArray(&self, wires: &Vec<Option<WireType>>, name: &String) -> Vec<TypedWire> {
        let uint256 = ZkayType::ZkUint(256);
        wires
            .iter()
            .map(|w| {
                TypedWire::new(
                    w.clone().unwrap(),
                    uint256.clone(),
                    name.clone(),
                    &vec![],
                    self.generator.clone(),
                )
            })
            .collect()
    }

    fn fromTypedWireArray(&self, typedWires: &Vec<TypedWire>) -> Vec<Option<WireType>> {
        let uint256 = ZkayType::ZkUint(256);
        typedWires
            .iter()
            .map(|w| {
                ZkayType::checkType(&uint256, &w.zkay_type);
                Some(w.wire.clone())
            })
            .collect()
    }

    fn parseJubJubPoint(&self, wire: &Vec<Option<WireType>>, offset: usize) -> JubJubPoint {
        JubJubPoint::new(
            wire[offset].clone().unwrap(),
            wire[offset + 1].clone().unwrap(),
        )
    }

    fn uninitZeroToIdentity(&self, p: &JubJubPoint) -> JubJubPoint {
        // Uninitialized values have a ciphertext of all zeroes, which is not a valid ElGamal cipher.
        // Instead, replace those values with the point at infinity (0, 1).
        let oneIfBothZero =
            p.x.check_non_zero(&None)
                .orw(&p.y.check_non_zero(&None), &None)
                .inv_as_bit(&None)
                .unwrap();
        JubJubPoint::new(p.x.clone(), p.y.clone().add(&oneIfBothZero))
    }
}
impl HomomorphicBackend for &CryptoBackend<Asymmetric<ElgamalBackend>> {
    fn doHomomorphicOp(
        &self,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
        keyName: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        if (op == '+') || (op == '-') {
            // for (c1, c2) = Enc(m1, r1)
            //     (d1, d2) = Enc(m2, r2)
            //     e1 = c1 + d1
            //     e2 = c2 + d2
            // it is (e1, e2) = Enc(m1 + m2, r1 + r2)
            let outputName = format!("({}) + ({})", lhs.getName(), rhs.getName());

            let lhs_twires = lhs.getCipher();
            let rhs_twires = rhs.getCipher();

            // sanity checks
            assert!(lhs_twires.len() == 4); // 4 BabyJubJub coordinates
            assert!(rhs_twires.len() == 4); // 4 BabyJubJub coordinates
            let lhs_wires = self.fromTypedWireArray(&lhs_twires);
            let rhs_wires = self.fromTypedWireArray(&rhs_twires);

            let mut c1 = self.parseJubJubPoint(&lhs_wires, 0);
            let mut c2 = self.parseJubJubPoint(&lhs_wires, 2);
            let mut d1 = self.parseJubJubPoint(&rhs_wires, 0);
            let mut d2 = self.parseJubJubPoint(&rhs_wires, 2);

            c1 = self.uninitZeroToIdentity(&c1);
            c2 = self.uninitZeroToIdentity(&c2);
            d1 = self.uninitZeroToIdentity(&d1);
            d2 = self.uninitZeroToIdentity(&d2);

            if op == '-' {
                d1.x = d1.x.negate(&None);
                d2.x = d2.x.negate(&None);
            }

            let gadget = ZkayElgamalAddGadget::new(c1, c2, d1, d2, generator);
            self.toTypedWireArray(gadget.get_output_wires(), &outputName)
        } else if op == '*' {
            let outputName = format!("({}) * ({})", lhs.getName(), rhs.getName());

            let mut plain_wire;
            let mut cipher_twires;
            if lhs.isPlain() && rhs.isCipher() {
                plain_wire = lhs.getPlain();
                cipher_twires = rhs.getCipher();
            } else if lhs.isCipher() && rhs.isPlain() {
                cipher_twires = lhs.getCipher();
                plain_wire = rhs.getPlain();
            } else {
                panic!("Elgamal multiplication requires exactly 1 plaintext argument");
            }

            let cipher_wires = self.fromTypedWireArray(&cipher_twires);
            let mut c1 = self.parseJubJubPoint(&cipher_wires, 0);
            let mut c2 = self.parseJubJubPoint(&cipher_wires, 2);

            c1 = self.uninitZeroToIdentity(&c1);
            c2 = self.uninitZeroToIdentity(&c2);

            let gadget = ZkayElgamalMulGadget::new(
                c1,
                c2,
                plain_wire.wire.get_bit_wiresi(32, &None).as_array().clone(),
                generator,
            );
            self.toTypedWireArray(gadget.get_output_wires(), &outputName)
        } else {
            panic!("Binary operation {op} not supported");
        }
    }

    fn doHomomorphicRerand(
        &self,
        arg: &Vec<TypedWire>,
        keyName: &String,
        randomness: &TypedWire,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        let outputName = format!("rerand({})", arg[0].name);

        // parse argument
        let arg_wires = self.fromTypedWireArray(&arg);
        let mut c1 = self.parseJubJubPoint(&arg_wires, 0);
        let mut c2 = self.parseJubJubPoint(&arg_wires, 2);
        c1 = self.uninitZeroToIdentity(&c1);
        c2 = self.uninitZeroToIdentity(&c2);

        // parse key and randomness
        let pkArray = self.getKeyArray(keyName);
        let pk = JubJubPoint::new(pkArray[0].clone().unwrap(), pkArray[1].clone().unwrap());
        let randomArray = randomness
            .wire
            .get_bit_wiresi(ElgamalBackend::RND_CHUNK_SIZE as u64, &None)
            .as_array()
            .clone();

        // create gadget
        let gadget = ZkayElgamalRerandGadget::new(c1, c2, pk, randomArray.clone(), generator);
        self.toTypedWireArray(gadget.get_output_wires(), &outputName)
    }
}
