use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use circuit::structure::wire_array;
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

    fn addKey(&self, keyName: String, keyWires: Vec<Wire>);

    fn getKeyChunkSize(&self) -> i32;

    fn createEncryptionGadget(
        &self,
        plain: TypedWire,
        key: String,
        random: Vec<Wire>,
        desc: Vec<String>,
    ) -> Gadget;

    fn createDecryptionGadget(
        &self,
        plain: TypedWire,
        cipher: Vec<Wire>,
        pkName: String,
        sk: Vec<Wire>,
        desc: Vec<String>,
    ) -> Gadget;
}

pub struct Symmetric {
    pub Keys: HashMap<String, Wire>,
    sharedKeys: HashMap<String, Wire>,
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
        return true;
    }

    pub fn usesDecryptionGadget(&self) -> bool {
        return false;
    }

    pub fn createDecryptionGadget(
        &self,
        plain: TypedWire,
        cipher: Vec<Wire>,
        pkey: String,
        skey: Vec<Wire>,
        desc: Vec<String>,
    ) -> Gadget {
        panic!("No separate decryption gadget for backend");
    }

    pub fn addKey(&self, keyName: String, keyWires: Vec<Wire>) {
        assert!(
            keyWires.length == 1,
            "Expected key size 1uint for symmetric keys"
        );
        Keys.put(keyName, keyWires[0]);
    }

    fn getKey(&self, keyName: String) -> Wire {
        let key = sharedKeys.get(keyName);
        if key == null {
            key = computeKey(keyName);
            sharedKeys.put(keyName, key);
        }
        return key;
    }

    fn computeKey(&self, keyName: String) -> Wire {
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
        return sharedKeyGadget.getOutputWires()[0];
    }

    pub fn setKeyPair(&self, myPk: Wire, mySk: Wire) {
        Objects.requireNonNull(myPk);
        Objects.requireNonNull(mySk);
        assert!(self.myPk.is_none(), "Key pair already set");

        // Ensure that provided sender keys form a key pair
        let generator = CircuitGenerator.getActiveCircuitGenerator();
        let pkDerivationGadget = ZkayEcPkDerivationGadget::new(mySk, true, "getPk(mySk)");
        generator.addEqualityAssertion(myPk, pkDerivationGadget.getOutputWires()[0]);

        self.myPk = myPk;
        self.mySk = mySk;
    }

    fn extractIV(&self, ivCipher: Option<Vec<Wire>>) -> Wire {
        assert!(
            ivCipher.some() && !ivCipher.as_ref().unwrap().is_empty(),
            "IV cipher must not be empty"
        );
        // This assumes as cipher length of 256 bits
        let lastBlockCipherLen = (256 - (((ivCipher.length - 1) * CIPHER_CHUNK_SIZE) % 256)) % 256;
        let iv = ivCipher[ivCipher.length - 1];
        if lastBlockCipherLen > 0 {
            iv = iv.shiftRight(CIPHER_CHUNK_SIZE, lastBlockCipherLen);
        }
        return iv;
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
        return false;
    }

    pub fn usesDecryptionGadget(&self) -> bool {
        return false;
    }

    pub fn createDecryptionGadget(
        &self,
        plain: TypedWire,
        cipher: Vec<Wire>,
        pkey: String,
        skey: Vec<Wire>,
        desc: Vec<String>,
    ) -> Gadget {
        panic!("No separate decryption gadget for backend");
    }

    pub fn addKey(&self, keyName: String, keyWires: Vec<Wire>) {
        let chunkBits = getKeyChunkSize();
        let keyArray = WireArray::new(keyWires)
            .getBits(chunkBits, keyName + "_bits")
            .adjustLength(keyBits);
        keys.put(keyName, keyArray);
    }

    fn getKey(&self, keyName: String) -> LongElement {
        let keyArr = getKeyArray(keyName);
        return LongElement::new(keyArr);
    }

    fn getKeyArray(&self, keyName: String) -> WireArray {
        let keyArr = keys.get(keyName);
        assert!(
            keyArr.is_some(),
            "Key variable " + keyName + " is not associated with a WireArray"
        );
        return keyArr;
    }
}
