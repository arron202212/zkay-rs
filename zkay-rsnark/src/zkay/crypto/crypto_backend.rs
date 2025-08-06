#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_array;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::crypto_backend::long_element::LongElement;
use crate::zkay::crypto::crypto_backend::wire_array::WireArray;
use crate::zkay::crypto::{
    dummy_backend::DummyBackend, dummy_hom_backend::DummyHomBackend, ecdh_backend::ECDHBackend,
    elgamal_backend::ElgamalBackend, paillier_backend::PaillierBackend, rsa_backend::RSABackend,
};
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_cbc_symmetric_enc_gadget::CipherType;
use crate::zkay::zkay_ec_pk_derivation_gadget::ZkayEcPkDerivationGadget;
use crate::zkay::zkay_ecdh_gadget::ZkayECDHGadget;
use crate::zkay::zkay_rsa_encryption_gadget::PaddingType;
use std::collections::HashMap;
enum Backend {
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
    fn create(name: &str, keyBits: i32) -> Backend {
        match name {
            "Dummy" => Backend::Dummy(DummyBackend::new(keyBits)),
            "DummyHom" => Backend::DummyHom(DummyHomBackend::new(keyBits)),
            "EcdhAes" => Backend::EcdhAes(ECDHBackend::new(keyBits, CipherType::AES_128)),
            "EcdhChaskey" => Backend::EcdhChaskey(ECDHBackend::new(keyBits, CipherType::CHASKEY)),
            "Paillier" => Backend::Paillier(PaillierBackend::new(keyBits)),
            "Elgamal" => Backend::Elgamal(ElgamalBackend::new(keyBits)),
            "RsaOaep" => Backend::RsaOaep(RSABackend::new(keyBits, PaddingType::OAEP)),
            "RsaPkcs15" => Backend::RsaPkcs15(RSABackend::new(keyBits, PaddingType::PKCS_1_5)),
        }
    }
}

pub struct CryptoBackend<T> {
    keyBits: i32,
    t: T,
}
impl<T> CryptoBackend<T> {
    pub fn new(keyBits: i32, t: T) -> Self {
        Self { keyBits, t }
    }
}

pub trait CryptoBackendConfig {
    fn isSymmetric(&self) -> bool;

    /**
     * Whether a separate decryption gadget is used for the backend. For efficiency reasons,
     * decryption is checked via encryption gadgets in many backends. For backends where the randomness
     * cannot be extracted, a separate decryption gadget is needed.
     */
    fn usesDecryptionGadget(&self) -> bool;

    fn addKey(&self, keyName: &String, keyWires: &Vec<Option<WireType>>);

    fn getKeyChunkSize(&self) -> i32;

    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget;

    fn createDecryptionGadget(
        &self,
        plain: &TypedWire,
        cipher: &Vec<Option<WireType>>,
        pkName: &String,
        sk: &Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget;
}
pub trait CircuitGeneratorField {
    fn generators(&self) -> &CircuitGenerator;
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
pub struct Symmetric<T> {
    pub publicKeys: HashMap<String, WireType>,
    sharedKeys: HashMap<String, WireType>,
    myPk: Option<WireType>,
    mySk: Option<WireType>,
    t: T,
}
impl<T> Symmetric<T> {
    pub fn new(keyBits: i32, t: T) -> CryptoBackend<Self> {
        CryptoBackend::new(
            keyBits,
            Self {
                publicKeys: HashMap::new(),
                sharedKeys: HashMap::new(),
                myPk: None,
                mySk: None,
                t,
            },
        )
    }
}
pub trait SymmetricConfig: SymmetricField + CircuitGeneratorField + CryptoBackendConfig {
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
    ) -> Gadget {
        panic!("No separate decryption gadget for backend");
    }

    fn addKey(&self, keyName: &String, keyWires: &Vec<Option<WireType>>) {
        assert!(
            keyWires.len() == 1,
            "Expected key size 1uint for symmetric keys"
        );
        self.public_keys_mut().insert(keyName, keyWires[0].clone());
    }

    fn getKey(&self, keyName: &String) -> WireType {
        self.shared_keys_mut()
            .entry(keyName.clone())
            .or_insert_with_key(|key| self.computeKey(key))
            .clone()
    }

    fn computeKey(&self, keyName: &String) -> WireType {
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
        actualOtherPk = actualOtherPk
            .checkNonZero(keyName + " != 0")
            .mux(actualOtherPk, self.my_pk().as_ref().unwrap());

        // Compute shared key with me
        let desc = format!(
            "sha256(ecdh({}, {}))",
            keyName,
            self.my_sk().as_ref().unwrap()
        );
        let sharedKeyGadget =
            ZkayECDHGadget::new(actualOtherPk, self.my_sk().as_ref().unwrap(), false, desc);
        sharedKeyGadget.validateInputs();
        sharedKeyGadget.getOutputWires()[0]
    }

    fn setKeyPair(&mut self, myPk: &WireType, mySk: &WireType) {
        // Objects.requireNonNull(myPk);
        // Objects.requireNonNull(mySk);
        assert!(self.my_pk().is_none(), "Key pair already set");

        // Ensure that provided sender keys form a key pair

        let pkDerivationGadget = ZkayEcPkDerivationGadget::new(mySk, true, "getPk(mySk)");
        self.generators()
            .addEqualityAssertion(myPk, pkDerivationGadget.getOutputWires()[0]);

        *self.my_pk_mut() = myPk.clone();
        *self.my_sk_mut() = mySk.clone();
    }

    fn extractIV(&self, ivCipher: &Option<Vec<Option<WireType>>>) -> WireType {
        assert!(
            ivCipher.some() && !ivCipher.as_ref().unwrap().is_empty(),
            "IV cipher must not be empty"
        );
        // This assumes as cipher length of 256 bits
        let lastBlockCipherLen =
            (256 - (((ivCipher.len() as i32 - 1) * Self::CIPHER_CHUNK_SIZE) % 256)) % 256;
        let iv = ivCipher[ivCipher.len() - 1];
        if lastBlockCipherLen > 0 {
            iv = iv.shiftRight(Self::CIPHER_CHUNK_SIZE, lastBlockCipherLen);
        }
        iv
    }
}

pub trait AsymmetricField {
    fn keys(&self) -> &HashMap<String, WireArray>;
    fn keys_mut(&mut self) -> &mut HashMap<String, WireArray>;
}
pub struct Asymmetric<T> {
    keys: HashMap<String, WireArray>,
    t: T,
}
impl<T> Asymmetric<T> {
    pub fn new(&self, keyBits: i32, t: T) -> CryptoBackend<Self> {
        CryptoBackend::new(
            keyBits,
            Self {
                keys: HashMap::new(),
                t,
            },
        )
    }
}

pub trait AsymmetricConfig: CryptoBackendConfig {
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
    ) -> Gadget {
        panic!("No separate decryption gadget for backend");
    }

    fn addKey(&self, keyName: &String, keyWires: &Vec<Option<WireType>>) {
        let chunkBits = self.getKeyChunkSize();
        let keyArray = WireArray::new(keyWires)
            .getBits(chunkBits, keyName + "_bits")
            .adjustLength(self.keyBits);
        self.keys_mut().insert(keyName.clone(), keyArray);
    }

    fn getKey(&self, keyName: &String) -> LongElement {
        let keyArr = self.getKeyArray(keyName);
        LongElement::new(keyArr)
    }

    fn getKeyArray(&self, keyName: &String) -> WireArray {
        let keyArr = self.keys().get(keyName);
        assert!(
            keyArr.is_some(),
            "Key variable {keyName} is not associated with a WireArray"
        );
        keyArr
    }
}
