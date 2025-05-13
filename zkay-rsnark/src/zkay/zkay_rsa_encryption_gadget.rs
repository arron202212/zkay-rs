

use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::rsa::rsa_encryption_oaep_gadget;
use examples::gadgets::rsa::rsa_encryption_v1_5_gadget;



use  zkay.ZkayUtil.*;
use  zkay.crypto.RSABackend.*;
   pub  enum PaddingType {
        PKCS_1_5,
        OAEP
    }

pub struct ZkayRSAEncryptionGadget   {

 
     PaddingType paddingType;
     LongElement pk;
     Wire plain;
     Vec<Wire> rnd;
     i32 keyBits;

     Vec<Wire> cipher = null;
}
impl ZkayRSAEncryptionGadget{
    pub  fn new(plain:TypedWire , pk:LongElement , rnd:Vec<Wire>, keyBits:i32 , paddingType:PaddingType , desc:Vec<String>) ->Self{
        super(desc);

        Objects.requireNonNull(plain, "plain");
        Objects.requireNonNull(pk, "pk");
        Objects.requireNonNull(rnd, "rnd");
        Objects.requireNonNull(paddingType, "paddingType");

        self.paddingType = paddingType;
        self.plain = plain.wire;
        self.pk = pk;
        self.rnd = rnd;
        self.keyBits = keyBits;

        buildCircuit();
    }
}
impl Gadget for ZkayRSAEncryptionGadget{
      fn buildCircuit() {
        let plainBytes = reverseBytes(plain.getBitWires(256), 8);

        let mut  enc;
        match paddingType {
             OAEP=>{
                let rndBytes = reverseBytes(WireArray::new(rnd).getBits(OAEP_RND_CHUNK_SIZE), 8);
                let e = RSAEncryptionOAEPGadget::new(pk, plainBytes, rndBytes, keyBits, description);
                e.checkSeedCompliance();
                enc = e;
                
            }
             PKCS_1_5=>{
                let rndLen = keyBits / 8 - 3 - plainBytes.length;
                let rndBytes = reverseBytes(WireArray::new(rnd).getBits(PKCS15_RND_CHUNK_SIZE).adjustLength(rndLen * 8), 8);
                enc = RSAEncryptionV1_5_Gadget::new(pk, plainBytes, rndBytes, keyBits, description);
                
            }
           _=>
                assert!("Unexpected padding type: " + paddingType)
        }

        cipher = WireArray::new(enc.getOutputWires()).packWordsIntoLargerWords(8, CIPHER_CHUNK_SIZE / 8);
    }

    
    pub  fn getOutputWires()->Vec<Wire>  {
        return cipher;
    }
}
