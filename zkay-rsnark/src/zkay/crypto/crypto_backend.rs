#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::{
    dummy_backend::DummyBackend, dummy_hom_backend::DummyHomBackend, ecdh_backend::ECDHBackend,
    elgamal_backend::ElgamalBackend, paillier_backend::PaillierBackend, rsa_backend::RSABackend,
};
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_cbc_symmetric_enc_gadget::CipherType;
use crate::zkay::zkay_ec_pk_derivation_gadget::ZkayEcPkDerivationGadget;
use crate::zkay::zkay_ecdh_gadget::ZkayECDHGadget;
use crate::zkay::zkay_rsa_encryption_gadget::PaddingType;
use rccell::{RcCell, WeakCell};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum Backend {
    Dummy(CryptoBackend<Asymmetric<DummyBackend>>),
    DummyHom(CryptoBackend<Asymmetric<DummyHomBackend>>),
    EcdhAes(CryptoBackend<Symmetric<ECDHBackend>>),
    EcdhChaskey(CryptoBackend<Symmetric<ECDHBackend>>),
    Paillier(CryptoBackend<Asymmetric<PaillierBackend>>),
    Elgamal(CryptoBackend<Asymmetric<ElgamalBackend>>),
    RsaOaep(CryptoBackend<Asymmetric<RSABackend>>),
    RsaPkcs15(CryptoBackend<Asymmetric<RSABackend>>),
}
impl Backend {
    fn create(name: &str, keyBits: i32, generator: RcCell<CircuitGenerator>) -> Backend {
        match name {
            "Dummy" => Backend::Dummy(DummyBackend::new(keyBits, generator)),
            "DummyHom" => Backend::DummyHom(DummyHomBackend::new(keyBits, generator)),
            "EcdhAes" => {
                Backend::EcdhAes(ECDHBackend::new(keyBits, CipherType::AES_128, generator))
            }
            "EcdhChaskey" => {
                Backend::EcdhChaskey(ECDHBackend::new(keyBits, CipherType::CHASKEY, generator))
            }
            "Paillier" => Backend::Paillier(PaillierBackend::new(keyBits, generator)),
            "Elgamal" => Backend::Elgamal(ElgamalBackend::new(keyBits, generator)),
            "RsaOaep" => Backend::RsaOaep(RSABackend::new(keyBits, PaddingType::OAEP, generator)),
            "RsaPkcs15" => {
                Backend::RsaPkcs15(RSABackend::new(keyBits, PaddingType::PKCS_1_5, generator))
            }
        }
    }
}
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
pub trait CryptoBackendConfig: CryptoBackendField {
    // fn isSymmetric(&self) -> bool;

    /**
     * Whether a separate decryption gadget is used for the backend. For efficiency reasons,
     * decryption is checked via encryption gadgets in many backends. For backends where the randomness
     * cannot be extracted, a separate decryption gadget is needed.
     */
    // fn usesDecryptionGadget(&self) -> bool;

    // fn addKey(&self, keyName: &String, keyWires: &Vec<Option<WireType>>);

    fn getKeyChunkSize() -> i32;

    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig>;

    // fn createDecryptionGadget(
    //     &self,
    //     plain: &TypedWire,
    //     cipher: &Vec<Option<WireType>>,
    //     pkName: &String,
    //     sk: &Vec<Option<WireType>>,
    //     desc: &Option<String>,
    // ) -> Box<dyn GadgetConfig>;
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

pub const CIPHER_CHUNK_SIZE: i32 = 192;
pub trait SymmetricConfig: SymmetricField + CryptoBackendConfig {
    // These chunk sizes assume a plaintext <= 256 (253) bit.
    // If self should change in the future, the optimal chunk size should be computed on demand based on the plaintext size
    // (optimal: pick such that data has 1. least amount of chunks, 2. for that chunk amount least possible bit amount)
    const CIPHER_CHUNK_SIZE: i32 = 192;

    fn isSymmetric(&self) -> bool {
        true
    }

    fn usesDecryptionGadget(&self) -> bool {
        false
    }

    fn createDecryptionGadget(
        &self,
        plain: &TypedWire,
        cipher: &Vec<Option<WireType>>,
        pkey: &String,
        skey: &Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Box<dyn GadgetConfig> {
        panic!("No separate decryption gadget for backend");
    }

    fn addKey(&self, keyName: &String, keyWires: &Vec<Option<WireType>>) {
        assert!(
            keyWires.len() == 1,
            "Expected key size 1uint for symmetric keys"
        );
        self.public_keys_mut()
            .insert(keyName.clone(), keyWires[0].clone().unwrap());
    }

    fn getKey(&self, keyName: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        self.shared_keys_mut()
            .entry(keyName.clone())
            .or_insert_with_key(|key| self.computeKey(key, generator))
            .clone()
    }

    fn computeKey(&self, keyName: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        assert!(
            self.my_pk().is_some(),
            "setKeyPair not called on symmetric crypto backend"
        );

        // Get other pub  key
        // In the case of decryption with default-initialization, it is possible that the sender pk stored in the
        // cipher struct is 0. In that case -> replace with any valid pk (my_pk for simplicity), to prevent ecdh gadget
        // from crashing (wrong output is not a problem since decryption enforces (pk_zero || cipher_zero) => all_zero
        // and ignores the ecdh result in that case.
        let actualOtherPk = self.public_keys().get(keyName);
        assert!(actualOtherPk.is_some(), "Key variable {keyName}  is absent");
        let mut actualOtherPk = actualOtherPk.cloned().unwrap();
        actualOtherPk = actualOtherPk
            .checkNonZero(&Some(keyName.to_owned() + " != 0"))
            .mux(&actualOtherPk, self.my_pk().as_ref().unwrap());

        // Compute shared key with me
        let desc = format!(
            "sha256(ecdh({}, {}))",
            keyName,
            self.my_sk().as_ref().unwrap()
        );
        let sharedKeyGadget = ZkayECDHGadget::new(
            actualOtherPk,
            self.my_sk().unwrap().clone(),
            false,
            &Some(desc),
            generator,
        );
        sharedKeyGadget.validateInputs();
        sharedKeyGadget.getOutputWires()[0].clone().unwrap()
    }

    fn setKeyPair(
        &mut self,
        myPk: &WireType,
        mySk: &WireType,
        generator: RcCell<CircuitGenerator>,
    ) {
        // Objects.requireNonNull(myPk);
        // Objects.requireNonNull(mySk);
        assert!(self.my_pk().is_none(), "Key pair already set");

        // Ensure that provided sender keys form a key pair

        let pkDerivationGadget = ZkayEcPkDerivationGadget::new(
            mySk.clone(),
            true,
            &Some("getPk(mySk)".to_owned()),
            generator.clone(),
        );
        generator.addEqualityAssertion(
            myPk,
            pkDerivationGadget.getOutputWires()[0].as_ref().unwrap(),
            &None,
        );

        *self.my_pk_mut() = Some(myPk.clone());
        *self.my_sk_mut() = Some(mySk.clone());
    }

    fn extractIV(&self, ivCipher: &Option<Vec<Option<WireType>>>) -> WireType {
        assert!(
            ivCipher.as_ref().is_some_and(|v| !v.is_empty()),
            "IV cipher must not be empty"
        );
        // This assumes as cipher length of 256 bits
        let lastBlockCipherLen = (256
            - (((ivCipher.as_ref().unwrap().len() as i32 - 1) * Self::CIPHER_CHUNK_SIZE) % 256))
            % 256;
        let mut iv = ivCipher.as_ref().unwrap()[ivCipher.as_ref().unwrap().len() - 1]
            .clone()
            .unwrap();
        if lastBlockCipherLen > 0 {
            iv = iv.shiftRight(
                Self::CIPHER_CHUNK_SIZE as usize,
                lastBlockCipherLen as usize,
                &None,
            );
        }
        iv
    }
}

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

pub trait AsymmetricConfig: CryptoBackendConfig + AsymmetricField {
    fn isSymmetric(&self) -> bool {
        false
    }

    fn usesDecryptionGadget(&self) -> bool {
        false
    }

    fn createDecryptionGadget(
        &self,
        plain: &TypedWire,
        cipher: &Vec<Option<WireType>>,
        pkey: &String,
        skey: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        panic!("No separate decryption gadget for backend");
    }

    fn addKey(
        &self,
        keyName: &String,
        keyWires: &Vec<Option<WireType>>,
        generator: WeakCell<CircuitGenerator>,
    ) {
        let chunkBits = Self::getKeyChunkSize();
        let keyArray = WireArray::new(keyWires.clone(), generator)
            .getBits(chunkBits as usize, &Some(keyName.to_owned() + "_bits"))
            .adjustLength(None, self.key_bits() as usize);
        self.keys_mut().insert(keyName.clone(), keyArray);
    }

    fn getKey(&self, keyName: &String, generator: RcCell<CircuitGenerator>) -> LongElement {
        let keyArr = self.getKeyArray(keyName);
        LongElement::newa(keyArr, generator.downgrade())
    }

    fn getKeyArray(&self, keyName: &String) -> WireArray {
        let keyArr = self.keys().get(keyName);
        assert!(
            keyArr.is_some(),
            "Key variable {keyName} is not associated with a WireArray"
        );
        keyArr.cloned().unwrap()
    }
}
