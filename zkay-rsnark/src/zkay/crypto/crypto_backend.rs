#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
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
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator,
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
    pub fn create(name: &str, key_bits: i32, generator: RcCell<CircuitGenerator>) -> Backend {
        match name {
            "dummy" => Backend::Dummy(DummyBackend::new(key_bits, generator)),
            "dummyhom" => Backend::DummyHom(DummyHomBackend::new(key_bits, generator)),
            "ecdhaes" => Backend::Ecdh(ECDHBackend::new(key_bits, CipherType::Aes128, generator)),
            "ecdhchaskey" => {
                Backend::Ecdh(ECDHBackend::new(key_bits, CipherType::Chaskey, generator))
            }
            "paillier" => Backend::Paillier(PaillierBackend::new(key_bits, generator)),
            "elgamal" => Backend::Elgamal(ElgamalBackend::new(key_bits, generator)),
            "rsaoaep" => Backend::Rsa(RSABackend::new(key_bits, PaddingType::Oaep, generator)),
            "rsapkcs15" => {
                Backend::Rsa(RSABackend::new(key_bits, PaddingType::Pkcs_1_5, generator))
            }
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
    pub key_bits: i32,
    pub t: T,
    pub generator: RcCell<CircuitGenerator>,
}
impl<T> CryptoBackend<T> {
    pub fn new(key_bits: i32, t: T, generator: RcCell<CircuitGenerator>) -> Self {
        Self {
            key_bits,
            t,
            generator,
        }
    }
}
impl<T> CryptoBackendField for CryptoBackend<T> {
    fn key_bits(&self) -> i32 {
        self.key_bits
    }
}

#[enum_dispatch]
pub trait CryptoBackendConfigs {
    fn is_symmetric(&self) -> bool;

    //Whether a separate decryption gadget is used for the backend. For efficiency reasons,
    //decryption is checked via encryption gadgets in many backends. For backends where the randomness
    //cannot be extracted, a separate decryption gadget is needed.

    fn uses_decryption_gadget(&self) -> bool;

    fn add_key(
        &mut self,
        key_name: &String,
        key_wires: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    );

    fn create_decryption_gadget(
        &self,
        plain: &TypedWire,
        cipher: &Vec<Option<WireType>>,
        pk_name: &String,
        sk: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        panic!("No separate decryption gadget for backend");
    }
    fn set_key_pair(
        &mut self,
        my_pk: &WireType,
        my_sk: &WireType,
        generator: RcCell<CircuitGenerator>,
    );
}

#[enum_dispatch]
pub trait CryptoBackendConfig {
    fn get_key_chunk_size(&self) -> i32;

    fn create_encryption_gadget(
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
        &self.t.public_keys
    }
    fn shared_keys(&self) -> &HashMap<String, WireType> {
        &self.t.shared_keys
    }
    fn my_pk(&self) -> &Option<WireType> {
        &self.t.my_pk
    }
    fn my_sk(&self) -> &Option<WireType> {
        &self.t.my_sk
    }
    fn public_keys_mut(&mut self) -> &mut HashMap<String, WireType> {
        &mut self.t.public_keys
    }
    fn shared_keys_mut(&mut self) -> &mut HashMap<String, WireType> {
        &mut self.t.shared_keys
    }
    fn my_pk_mut(&mut self) -> &mut Option<WireType> {
        &mut self.t.my_pk
    }
    fn my_sk_mut(&mut self) -> &mut Option<WireType> {
        &mut self.t.my_sk
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
    pub public_keys: HashMap<String, WireType>,
    pub shared_keys: HashMap<String, WireType>,
    pub my_pk: Option<WireType>,
    pub my_sk: Option<WireType>,
    pub t: T,
}
impl<T> Symmetric<T> {
    pub fn new(key_bits: i32, t: T, generator: RcCell<CircuitGenerator>) -> CryptoBackend<Self> {
        CryptoBackend::new(
            key_bits,
            Self {
                public_keys: HashMap::new(),
                shared_keys: HashMap::new(),
                my_pk: None,
                my_sk: None,
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
    fn is_symmetric(&self) -> bool {
        true
    }

    fn uses_decryption_gadget(&self) -> bool {
        false
    }

    fn add_key(
        &mut self,
        key_name: &String,
        key_wires: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) {
        assert!(
            key_wires.len() == 1,
            "Expected key size 1uint for symmetric keys"
        );
        self.t
            .public_keys
            .insert(key_name.clone(), key_wires[0].clone().unwrap());
    }
    fn set_key_pair(
        &mut self,
        my_pk: &WireType,
        my_sk: &WireType,
        generator: RcCell<CircuitGenerator>,
    ) {
        // Objects.requireNonNull(my_pk);
        // Objects.requireNonNull(my_sk);
        assert!(self.t.my_pk.is_none(), "Key pair already set");

        // Ensure that provided sender keys form a key pair

        let pk_derivation_gadget = ZkayEcPkDerivationGadget::new(
            my_sk.clone(),
            true,
            &Some("getPk(my_sk)".to_owned()),
            generator.clone(),
        );
        CircuitGenerator::add_equality_assertion(
            generator.clone(),
            my_pk,
            pk_derivation_gadget.get_output_wires()[0].as_ref().unwrap(),
            &None,
        );

        self.t.my_pk = Some(my_pk.clone());
        self.t.my_sk = Some(my_sk.clone());
    }
}
pub const CIPHER_CHUNK_SIZE: i32 = 192;
impl<T> CryptoBackend<Symmetric<T>> {
    pub fn get_key(&mut self, key_name: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        let v = self.compute_key(key_name, generator);
        self.t
            .shared_keys
            .entry(key_name.clone())
            .or_insert(v)
            .clone()
    }

    pub fn compute_key(&self, key_name: &String, generator: RcCell<CircuitGenerator>) -> WireType {
        assert!(
            self.t.my_pk.is_some(),
            "set_key_pair not called on symmetric crypto backend"
        );

        // Get other pub  key
        // In the case of decryption with default-initialization, it is possible that the sender pk stored in the
        // cipher struct is 0. In that case -> replace with any valid pk (my_pk for simplicity), to prevent ecdh gadget
        // from crashing (wrong output is not a problem since decryption enforces (pk_zero || cipher_zero) => all_zero
        // and ignores the ecdh result in that case.
        let actual_other_pk = self.t.public_keys.get(key_name);
        assert!(
            actual_other_pk.is_some(),
            "Key variable {key_name}  is absent"
        );
        let mut actual_other_pk = actual_other_pk.cloned().unwrap();
        actual_other_pk = actual_other_pk
            .check_non_zero(&Some(key_name.to_owned() + " != 0"))
            .mux(&actual_other_pk, self.my_pk().as_ref().unwrap());

        // Compute shared key with me
        let desc = format!(
            "sha256(ecdh({}, {}))",
            key_name,
            self.t.my_sk.as_ref().unwrap()
        );
        let shared_key_gadget = ZkayECDHGadget::new(
            actual_other_pk,
            self.t.my_sk.clone().unwrap(),
            false,
            &Some(desc),
            generator,
        );
        shared_key_gadget.validate_inputs();
        shared_key_gadget.get_output_wires()[0].clone().unwrap()
    }

    pub fn extract_iv(iv_cipher: &Option<Vec<Option<WireType>>>) -> WireType {
        assert!(
            iv_cipher.as_ref().is_some_and(|v| !v.is_empty()),
            "IV cipher must not be empty"
        );
        // This assumes as cipher length of 256 bits
        let last_block_cipher_len = (256
            - (((iv_cipher.as_ref().unwrap().len() as i32 - 1) * CIPHER_CHUNK_SIZE) % 256))
            % 256;
        let mut iv = iv_cipher.as_ref().unwrap()[iv_cipher.as_ref().unwrap().len() - 1]
            .clone()
            .unwrap();
        if last_block_cipher_len > 0 {
            iv = iv.shift_right(
                CIPHER_CHUNK_SIZE as usize,
                last_block_cipher_len as usize,
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
    pub fn new(key_bits: i32, t: T, generator: RcCell<CircuitGenerator>) -> CryptoBackend<Self> {
        CryptoBackend::new(
            key_bits,
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
            fn is_symmetric(&self) -> bool {
                false
            }

            fn uses_decryption_gadget(&self) -> bool {
                false
            }

            fn add_key(
                &mut self,
                key_name: &String,
                key_wires: &Vec<Option<WireType>>,
                generator: RcCell<CircuitGenerator>,
            ) {
                let chunk_bits = self.get_key_chunk_size();
                let key_array = WireArray::new(key_wires.clone(), generator.downgrade())
                    .get_bits(chunk_bits as usize, &Some(key_name.to_owned() + "_bits"))
                    .adjust_length(None, self.key_bits as usize);
                self.t.keys.insert(key_name.clone(), key_array);
            }
            fn set_key_pair(
                &mut self,
                my_pk: &WireType,
                my_sk: &WireType,
                generator: RcCell<CircuitGenerator>,
            ) {
                panic!("set_key_pair no in Asymmetric");
            }
        }
    };
}

impl<T> CryptoBackend<Asymmetric<T>> {
    pub fn get_key(&self, key_name: &String, generator: RcCell<CircuitGenerator>) -> LongElement {
        let key_arr = self.get_key_array(key_name);
        LongElement::newa(key_arr, generator.downgrade())
    }

    pub fn get_key_array(&self, key_name: &String) -> WireArray {
        let key_arr = self.t.keys.get(key_name);
        assert!(
            key_arr.is_some(),
            "Key variable {key_name} is not associated with a WireArray"
        );
        key_arr.cloned().unwrap()
    }
}
