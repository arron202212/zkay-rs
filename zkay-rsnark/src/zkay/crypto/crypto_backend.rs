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
        auxiliary::long_element::LongElement,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
                getActiveCircuitGenerator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    zkay::{
        crypto::{
            dummy_backend::DummyBackend, dummy_hom_backend::DummyHomBackend,
            ecdh_backend::ECDHBackend, elgamal_backend::ElgamalBackend,
            homomorphic_backend::HomomorphicBackend, paillier_backend::PaillierBackend,
            rsa_backend::RSABackend,
        },
        typed_wire::TypedWire,
        zkay_cbc_symmetric_enc_gadget::CipherType,
        zkay_ec_pk_derivation_gadget::ZkayEcPkDerivationGadget,
        zkay_ecdh_gadget::ZkayECDHGadget,
        zkay_rsa_encryption_gadget::PaddingType,
    },
};

use enum_dispatch::enum_dispatch;
use rccell::{RcCell, WeakCell};
use std::collections::HashMap;
use strum_macros::{EnumIs, EnumTryAs};

#[enum_dispatch(CryptoBackendConfig, CryptoBackendConfigs)]
#[derive(Debug, Clone, EnumIs, EnumTryAs)]
pub enum Backend {
    Dummy(CryptoBackend<Asymmetric<DummyBackend>>),
    DummyHom(CryptoBackend<Asymmetric<DummyHomBackend>>),
    Ecdh(CryptoBackend<Symmetric<ECDHBackend>>),
    // EcdhChaskey(CryptoBackend<Symmetric<ECDHBackend>>),
    Paillier(CryptoBackend<Asymmetric<PaillierBackend>>),
    Elgamal(CryptoBackend<Asymmetric<ElgamalBackend>>),
    Rsa(CryptoBackend<Asymmetric<RSABackend>>),
    // RsaPkcs15(CryptoBackend<Asymmetric<RSABackend>>),
}
impl Backend {
    pub fn create(name: &str, keyBits: i32, generator: RcCell<CircuitGenerator>) -> Backend {
        match name {
            "dummy" => Backend::Dummy(DummyBackend::new(keyBits, generator)),
            "dummyhom" => Backend::DummyHom(DummyHomBackend::new(keyBits, generator)),
            "ecdhaes" => Backend::Ecdh(ECDHBackend::new(keyBits, CipherType::AES_128, generator)),
            "ecdhchaskey" => {
                Backend::Ecdh(ECDHBackend::new(keyBits, CipherType::CHASKEY, generator))
            }
            "paillier" => Backend::Paillier(PaillierBackend::new(keyBits, generator)),
            "elgamal" => Backend::Elgamal(ElgamalBackend::new(keyBits, generator)),
            "rsaoaep" => Backend::Rsa(RSABackend::new(keyBits, PaddingType::OAEP, generator)),
            "rsapkcs15" => Backend::Rsa(RSABackend::new(keyBits, PaddingType::PKCS_1_5, generator)),
            _ => {
                panic!("{}", name)
            }
        }
    }
    pub fn homomorphic_backend(&self) -> Option<Box<dyn HomomorphicBackend + '_>> {
        match self {
            Self::Dummy(_) => None,
            Self::DummyHom(backend) => Some(Box::new(backend)),
            Self::Ecdh(_) => None,
            Self::Paillier(backend) => Some(Box::new(backend)),
            Self::Elgamal(backend) => Some(Box::new(backend)),
            Self::Rsa(_) => None,
        }
    }
}

#[enum_dispatch]
pub trait CryptoBackendField {
    fn key_bits(&self) -> i32;
}

#[derive(Debug, Clone)]
pub struct CryptoBackend<T> {
    pub keyBits: i32,
    pub t: T,
    pub generator: RcCell<CircuitGenerator>,
}
impl<T> CryptoBackend<T> {
    pub fn new(keyBits: i32, t: T, generator: RcCell<CircuitGenerator>) -> Self {
        Self {
            keyBits,
            t,
            generator,
        }
    }
}
impl<T> CryptoBackendField for CryptoBackend<T> {
    fn key_bits(&self) -> i32 {
        self.keyBits
    }
}

#[enum_dispatch]
pub trait CryptoBackendConfigs {
    fn isSymmetric(&self) -> bool;

    //Whether a separate decryption gadget is used for the backend. For efficiency reasons,
    //decryption is checked via encryption gadgets in many backends. For backends where the randomness
    //cannot be extracted, a separate decryption gadget is needed.

    fn usesDecryptionGadget(&self) -> bool;

    fn addKey(
        &mut self,
        keyName: &String,
        keyWires: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    );

    fn createDecryptionGadget(
        &self,
        plain: &TypedWire,
        cipher: &Vec<Option<WireType>>,
        pkName: &String,
        sk: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        panic!("No separate decryption gadget for backend");
    }
    fn setKeyPair(&mut self, myPk: &WireType, mySk: &WireType, generator: RcCell<CircuitGenerator>);
}

#[enum_dispatch]
pub trait CryptoBackendConfig {
    fn getKeyChunkSize(&self) -> i32;

    fn createEncryptionGadget(
        &mut self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig>;
}

pub trait CircuitGeneratorField {
    fn generators(&self) -> &CircuitGenerator;
}
impl<T> SymmetricField for CryptoBackend<Symmetric<T>> {
    fn public_keys(&self) -> &HashMap<String, WireType> {
        &self.t.publicKeys
    }
    fn shared_keys(&self) -> &HashMap<String, WireType> {
        &self.t.sharedKeys
    }
    fn my_pk(&self) -> &Option<WireType> {
        &self.t.myPk
    }
    fn my_sk(&self) -> &Option<WireType> {
        &self.t.mySk
    }
    fn public_keys_mut(&mut self) -> &mut HashMap<String, WireType> {
        &mut self.t.publicKeys
    }
    fn shared_keys_mut(&mut self) -> &mut HashMap<String, WireType> {
        &mut self.t.sharedKeys
    }
    fn my_pk_mut(&mut self) -> &mut Option<WireType> {
        &mut self.t.myPk
    }
    fn my_sk_mut(&mut self) -> &mut Option<WireType> {
        &mut self.t.mySk
    }
}
#[enum_dispatch]
pub trait SymmetricField {
    fn public_keys(&self) -> &HashMap<String, WireType>;
    fn shared_keys(&self) -> &HashMap<String, WireType>;
    fn my_pk(&self) -> &Option<WireType>;
    fn my_sk(&self) -> &Option<WireType>;
    fn public_keys_mut(&mut self) -> &mut HashMap<String, WireType>;
    fn shared_keys_mut(&mut self) -> &mut HashMap<String, WireType>;
    fn my_pk_mut(&mut self) -> &mut Option<WireType>;
    fn my_sk_mut(&mut self) -> &mut Option<WireType>;
}
#[derive(Debug, Clone)]
pub struct Symmetric<T> {
    pub publicKeys: HashMap<String, WireType>,
    pub sharedKeys: HashMap<String, WireType>,
    pub myPk: Option<WireType>,
    pub mySk: Option<WireType>,
    pub t: T,
}
impl<T> Symmetric<T> {
    pub fn new(keyBits: i32, t: T, generator: RcCell<CircuitGenerator>) -> CryptoBackend<Self> {
        CryptoBackend::new(
            keyBits,
            Self {
                publicKeys: HashMap::new(),
                sharedKeys: HashMap::new(),
                myPk: None,
                mySk: None,
                t,
            },
            generator,
        )
    }
}

// #[enum_dispatch]
// pub trait SymmetricConfig: SymmetricField + CryptoBackendConfig {
// These chunk sizes assume a plaintext <= 256 (253) bit.
// If self should change in the future, the optimal chunk size should be computed on demand based on the plaintext size
// (optimal: pick such that data has 1. least amount of chunks, 2. for that chunk amount least possible bit amount)
impl<T> CryptoBackendConfigs for CryptoBackend<Symmetric<T>> {
    fn isSymmetric(&self) -> bool {
        true
    }

    fn usesDecryptionGadget(&self) -> bool {
        false
    }

    fn addKey(
        &mut self,
        keyName: &String,
        keyWires: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) {
        assert!(
            keyWires.len() == 1,
            "Expected key size 1uint for symmetric keys"
        );
        self.t
            .publicKeys
            .insert(keyName.clone(), keyWires[0].clone().unwrap());
    }
    fn setKeyPair(
        &mut self,
        myPk: &WireType,
        mySk: &WireType,
        generator: RcCell<CircuitGenerator>,
    ) {
        // Objects.requireNonNull(myPk);
        // Objects.requireNonNull(mySk);
        assert!(self.t.myPk.is_none(), "Key pair already set");

        // Ensure that provided sender keys form a key pair

        let pkDerivationGadget = ZkayEcPkDerivationGadget::new(
            mySk.clone(),
            true,
            &Some("getPk(mySk)".to_owned()),
            generator.clone(),
        );
        CircuitGenerator::addEqualityAssertion(
            generator.clone(),
            myPk,
            pkDerivationGadget.getOutputWires()[0].as_ref().unwrap(),
            &None,
        );

        self.t.myPk = Some(myPk.clone());
        self.t.mySk = Some(mySk.clone());
    }
}
pub const CIPHER_CHUNK_SIZE: i32 = 192;
impl<T> CryptoBackend<Symmetric<T>> {
    pub fn getKey(&mut self, keyName: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        let v = self.computeKey(keyName, generator);
        self.t
            .sharedKeys
            .entry(keyName.clone())
            .or_insert(v)
            .clone()
    }

    pub fn computeKey(&self, keyName: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        assert!(
            self.t.myPk.is_some(),
            "setKeyPair not called on symmetric crypto backend"
        );

        // Get other pub  key
        // In the case of decryption with default-initialization, it is possible that the sender pk stored in the
        // cipher struct is 0. In that case -> replace with any valid pk (my_pk for simplicity), to prevent ecdh gadget
        // from crashing (wrong output is not a problem since decryption enforces (pk_zero || cipher_zero) => all_zero
        // and ignores the ecdh result in that case.
        let actualOtherPk = self.t.publicKeys.get(keyName);
        assert!(actualOtherPk.is_some(), "Key variable {keyName}  is absent");
        let mut actualOtherPk = actualOtherPk.cloned().unwrap();
        actualOtherPk = actualOtherPk
            .checkNonZero(&Some(keyName.to_owned() + " != 0"))
            .mux(&actualOtherPk, self.my_pk().as_ref().unwrap());

        // Compute shared key with me
        let desc = format!(
            "sha256(ecdh({}, {}))",
            keyName,
            self.t.mySk.as_ref().unwrap()
        );
        let sharedKeyGadget = ZkayECDHGadget::new(
            actualOtherPk,
            self.t.mySk.clone().unwrap(),
            false,
            &Some(desc),
            generator,
        );
        sharedKeyGadget.validateInputs();
        sharedKeyGadget.getOutputWires()[0].clone().unwrap()
    }

    pub fn extractIV(ivCipher: &Option<Vec<Option<WireType>>>) -> WireType {
        assert!(
            ivCipher.as_ref().is_some_and(|v| !v.is_empty()),
            "IV cipher must not be empty"
        );
        // This assumes as cipher length of 256 bits
        let lastBlockCipherLen = (256
            - (((ivCipher.as_ref().unwrap().len() as i32 - 1) * CIPHER_CHUNK_SIZE) % 256))
            % 256;
        let mut iv = ivCipher.as_ref().unwrap()[ivCipher.as_ref().unwrap().len() - 1]
            .clone()
            .unwrap();
        if lastBlockCipherLen > 0 {
            iv = iv.shiftRight(
                CIPHER_CHUNK_SIZE as usize,
                lastBlockCipherLen as usize,
                &None,
            );
        }
        iv
    }
}

#[enum_dispatch]
pub trait AsymmetricField {
    fn keys(&self) -> &HashMap<String, WireArray>;
    fn keys_mut(&mut self) -> &mut HashMap<String, WireArray>;
}
impl<T> AsymmetricField for CryptoBackend<Asymmetric<T>> {
    fn keys(&self) -> &HashMap<String, WireArray> {
        &self.t.keys
    }
    fn keys_mut(&mut self) -> &mut HashMap<String, WireArray> {
        &mut self.t.keys
    }
}

#[derive(Debug, Clone)]
pub struct Asymmetric<T> {
    pub keys: HashMap<String, WireArray>,
    pub t: T,
}
impl<T> Asymmetric<T> {
    pub fn new(keyBits: i32, t: T, generator: RcCell<CircuitGenerator>) -> CryptoBackend<Self> {
        CryptoBackend::new(
            keyBits,
            Self {
                keys: HashMap::new(),
                t,
            },
            generator,
        )
    }
}

// #[enum_dispatch]
// pub trait AsymmetricConfig: CryptoBackendConfig + AsymmetricField {
#[macro_export]
macro_rules! impl_crypto_backend_configs_for {
    ($impl_type:ty) => {
        impl CryptoBackendConfigs for CryptoBackend<Asymmetric<$impl_type>> {
            fn isSymmetric(&self) -> bool {
                false
            }

            fn usesDecryptionGadget(&self) -> bool {
                false
            }

            fn addKey(
                &mut self,
                keyName: &String,
                keyWires: &Vec<Option<WireType>>,
                generator: RcCell<CircuitGenerator>,
            ) {
                let chunkBits = self.getKeyChunkSize();
                let keyArray = WireArray::new(keyWires.clone(), generator.downgrade())
                    .getBits(chunkBits as usize, &Some(keyName.to_owned() + "_bits"))
                    .adjustLength(None, self.keyBits as usize);
                self.t.keys.insert(keyName.clone(), keyArray);
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
    };
}

impl<T> CryptoBackend<Asymmetric<T>> {
    pub fn getKey(&self, keyName: &String, generator: RcCell<CircuitGenerator>) -> LongElement {
        let keyArr = self.getKeyArray(keyName);
        LongElement::newa(keyArr, generator.downgrade())
    }

    pub fn getKeyArray(&self, keyName: &String) -> WireArray {
        let keyArr = self.t.keys.get(keyName);
        assert!(
            keyArr.is_some(),
            "Key variable {keyName} is not associated with a WireArray"
        );
        keyArr.cloned().unwrap()
    }
}
