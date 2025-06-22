use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::circuit_generator::{CGConfig,CircuitGenerator,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use zkay::*;
enum Backend {
    Dummy,
    DummyHom,
    EcdhAes,
    EcdhChaskey,
    Paillier,
    Elgamal,
    RsaOaep,
    RsaPkcs15,
}
impl Backend {
    fn create(name: Self, keyBits: i32) -> CryptoBackend {
        match name {
            Dummy => DummyBackend::new(keyBits),
            DummyHom => DummyHomBackend::new(keyBits),
            EcdhAes => ECDHBackend::new(keyBits, ZkayCBCSymmetricEncGadget.CipherType.AES_128),
            EcdhChaskey => ECDHBackend::new(keyBits, ZkayCBCSymmetricEncGadget.CipherType.CHASKEY),
            Paillier => PaillierBackend::new(keyBits),
            Elgamal => ElgamalBackend::new(keyBits),
            RsaOaep => RSABackend::new(keyBits, ZkayRSAEncryptionGadget.PaddingType.OAEP),
            RsaPkcs15 => RSABackend::new(keyBits, ZkayRSAEncryptionGadget.PaddingType.PKCS_1_5),
        }
    }
}

pub struct CryptoBackend {
    keyBits: i32,
}
impl CryptoBackend {
    pub fn new(keyBits: i32) -> Self {
        Self { keyBits }
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

    fn addKey(&self, keyName: String, keyWires: Vec<Option<WireType>>);

    fn getKeyChunkSize(&self) -> i32;

    fn createEncryptionGadget(
        &self,
        plain: TypedWire,
        key: String,
        random: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget;

    fn createDecryptionGadget(
        &self,
        plain: TypedWire,
        cipher: Vec<Option<WireType>>,
        pkName: String,
        sk: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget;
}

pub struct Symmetric {
    pub Keys: HashMap<String, WireType>,
    sharedKeys: HashMap<String, WireType>,
    myPk: String,
    mySk: String,
}
impl Symmetric {
    pub fn new(keyBits: i32) -> Self {
        // super(keyBits);
        Self {
            Keys: HashMap::new(),
            sharedKeys: HashMap::new(),
        }
    }
}
pub trait SymmetricConfig: CryptoBackendConfig {
    // These chunk sizes assume a plaintext <= 256 (253) bit.
    // If self should change in the future, the optimal chunk size should be computed on demand based on the plaintext size
    // (optimal: pick such that data has 1. least amount of chunks, 2. for that chunk amount least possible bit amount)
    const CIPHER_CHUNK_SIZE: i32 = 192;

    pub fn isSymmetric(&self) -> bool {
        true
    }

    pub fn usesDecryptionGadget(&self) -> bool {
        false
    }

    pub fn createDecryptionGadget(
        &self,
        plain: TypedWire,
        cipher: Vec<Option<WireType>>,
        pkey: String,
        skey: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        panic!("No separate decryption gadget for backend");
    }

    pub fn addKey(&self, keyName: String, keyWires: Vec<Option<WireType>>) {
        assert!(
            keyWires.len() == 1,
            "Expected key size 1uint for symmetric keys"
        );
        Keys.put(keyName, keyWires[0]);
    }

    fn getKey(&self, keyName: String) -> WireType {
        let key = sharedKeys.get(keyName);
        if key == null {
            key = computeKey(keyName);
            sharedKeys.put(keyName, key);
        }
        key
    }

    fn computeKey(&self, keyName: String) -> WireType {
        assert!(
            self.myPk.is_some(),
            "setKeyPair not called on symmetric crypto backend"
        );

        // Get other pub  key
        // In the case of decryption with default-initialization, it is possible that the sender pk stored in the
        // cipher struct is 0. In that case -> replace with any valid pk (my_pk for simplicity), to prevent ecdh gadget
        // from crashing (wrong output is not a problem since decryption enforces (pk_zero || cipher_zero) => all_zero
        // and ignores the ecdh result in that case.
        let actualOtherPk = Keys.get(keyName);
        assert!(
            actualOtherPk.is_some(),
            "Key variable " + keyName + " is absent"
        );
        actualOtherPk = actualOtherPk
            .checkNonZero(keyName + " != 0")
            .mux(actualOtherPk, myPk);

        // Compute shared key with me
        let desc = format!("sha256(ecdh(%s, %s))", keyName, mySk);
        let sharedKeyGadget = ZkayECDHGadget::new(actualOtherPk, mySk, false, desc);
        sharedKeyGadget.validateInputs();
        sharedKeyGadget.getOutputWires()[0]
    }

    pub fn setKeyPair(&self, myPk: WireType, mySk: WireType) {
        Objects.requireNonNull(myPk);
        Objects.requireNonNull(mySk);
        assert!(self.myPk.is_none(), "Key pair already set");

        // Ensure that provided sender keys form a key pair
        let mut generator = CircuitGenerator.getActiveCircuitGenerator();
        let pkDerivationGadget = ZkayEcPkDerivationGadget::new(mySk, true, "getPk(mySk)");
        generator.addEqualityAssertion(myPk, pkDerivationGadget.getOutputWires()[0]);

        self.myPk = myPk;
        self.mySk = mySk;
    }

    fn extractIV(&self, ivCipher: Option<Vec<Option<WireType>>>) -> WireType {
        assert!(
            ivCipher.some() && !ivCipher.as_ref().unwrap().is_empty(),
            "IV cipher must not be empty"
        );
        // This assumes as cipher length of 256 bits
        let lastBlockCipherLen = (256 - (((ivCipher.len() - 1) * CIPHER_CHUNK_SIZE) % 256)) % 256;
        let iv = ivCipher[ivCipher.len() - 1];
        if lastBlockCipherLen > 0 {
            iv = iv.shiftRight(CIPHER_CHUNK_SIZE, lastBlockCipherLen);
        }
        iv
    }
}

pub struct Asymmetric {
    keys: HashMap<String, WireArray>,
}
impl Asymmetric {
    pub fn new(&self, keyBits: i32) -> Self {
        super(keyBits);
        keys = HashMap::new();
    }
}
pub trait AsymmetricConfig: CryptoBackendConfig {
    pub fn isSymmetric(&self) -> bool {
        false
    }

    pub fn usesDecryptionGadget(&self) -> bool {
        false
    }

    pub fn createDecryptionGadget(
        &self,
        plain: TypedWire,
        cipher: Vec<Option<WireType>>,
        pkey: String,
        skey: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        panic!("No separate decryption gadget for backend");
    }

    pub fn addKey(&self, keyName: String, keyWires: Vec<Option<WireType>>) {
        let chunkBits = getKeyChunkSize();
        let keyArray = WireArray::new(keyWires)
            .getBits(chunkBits, keyName + "_bits")
            .adjustLength(keyBits);
        keys.put(keyName, keyArray);
    }

    fn getKey(&self, keyName: String) -> LongElement {
        let keyArr = getKeyArray(keyName);
        LongElement::new(keyArr)
    }

    fn getKeyArray(&self, keyName: String) -> WireArray {
        let keyArr = keys.get(keyName);
        assert!(
            keyArr.is_some(),
            "Key variable " + keyName + " is not associated with a WireArray"
        );
        keyArr
    }
}
