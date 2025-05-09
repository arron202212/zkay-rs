
use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;



use static zkay::crypto::DummyBackend::CIPHER_CHUNK_SIZE;

public class ZkayDummyEncryptionGadget extends Gadget {

     Wire pk;
     Wire plain;
     Wire[] cipher;

    public ZkayDummyEncryptionGadget(TypedWire plain, LongElement pk, Wire[] rnd, int keyBits, String... desc) {
        super(desc);
        if plain == null || pk == null || rnd == null {
            panic!();
        }
        this.plain = plain.wire;
        Wire[] pkarr = pk.getBits().packBitsIntoWords(256);
        for i in 1..pkarr.length {
            generator.addZeroAssertion(pkarr[i], "Dummy enc pk valid");
        }
        this.pk = pkarr[0];
        this.cipher = new Wire[(int)Math.ceil((1.0*keyBits) / CIPHER_CHUNK_SIZE)];
        buildCircuit();
    }

    protected void buildCircuit() {
        Wire res = plain.add(pk, "plain + pk");
        Arrays.fill(cipher, res);
    }

    
    public Wire[] getOutputWires() {
        return cipher;
    }
}
