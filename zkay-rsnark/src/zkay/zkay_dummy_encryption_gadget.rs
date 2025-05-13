
use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;



use  zkay::crypto::DummyBackend::CIPHER_CHUNK_SIZE;

pub struct ZkayDummyEncryptionGadget   {

     pk:Wire,
     plain:Wire,
     cipher:Vec<Wire>,
}
impl  ZkayDummyEncryptionGadget{
    pub  fn new(plain:TypedWire , pk:LongElement , rnd:Vec<Wire>, keyBits:i32 , desc:Vec<String>)->Self {
        super(desc);
            assert!(plain.is_some() && pk.is_some() && rnd.is_some());
        self.plain = plain.wire;
        let pkarr = pk.getBits().packBitsIntoWords(256);
        for i in 1..pkarr.length {
            generator.addZeroAssertion(pkarr[i], "Dummy enc pk valid");
        }
        self.pk = pkarr[0];
        self.cipher = vec![Wire::default();(i32)Math.ceil((1.0*keyBits) / CIPHER_CHUNK_SIZE)];
        buildCircuit();
    }
}
impl Gadget for ZkayDummyEncryptionGadget{
      fn buildCircuit() {
        let res = plain.add(pk, "plain + pk");
        cipher.fill( res);
    }

    
    pub  fn getOutputWires()->  Vec<Wire> {
        return cipher;
    }
}
