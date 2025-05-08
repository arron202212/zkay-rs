

use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::rsa::rsa_encryption_oaep_gadget;
use examples::gadgets::rsa::rsa_encryption_v1_5_gadget;



use static zkay.ZkayUtil.*;
use static zkay.crypto.RSABackend.*;

public class ZkayRSAEncryptionGadget extends Gadget {

    public enum PaddingType {
        PKCS_1_5,
        OAEP
    }

    private final PaddingType paddingType;
    private final LongElement pk;
    private final Wire plain;
    private final Wire[] rnd;
    private final int keyBits;

    private Wire[] cipher = null;

    public ZkayRSAEncryptionGadget(TypedWire plain, LongElement pk, Wire[] rnd, int keyBits, PaddingType paddingType, String... desc) {
        super(desc);

        Objects.requireNonNull(plain, "plain");
        Objects.requireNonNull(pk, "pk");
        Objects.requireNonNull(rnd, "rnd");
        Objects.requireNonNull(paddingType, "paddingType");

        this.paddingType = paddingType;
        this.plain = plain.wire;
        this.pk = pk;
        this.rnd = rnd;
        this.keyBits = keyBits;

        buildCircuit();
    }

    protected void buildCircuit() {
        Wire[] plainBytes = reverseBytes(plain.getBitWires(256), 8);

        Gadget enc;
        switch (paddingType) {
            case OAEP: {
                Wire[] rndBytes = reverseBytes(new WireArray(rnd).getBits(OAEP_RND_CHUNK_SIZE), 8);
                RSAEncryptionOAEPGadget e = new RSAEncryptionOAEPGadget(pk, plainBytes, rndBytes, keyBits, description);
                e.checkSeedCompliance();
                enc = e;
                break;
            }
            case PKCS_1_5: {
                int rndLen = keyBits / 8 - 3 - plainBytes.length;
                Wire[] rndBytes = reverseBytes(new WireArray(rnd).getBits(PKCS15_RND_CHUNK_SIZE).adjustLength(rndLen * 8), 8);
                enc = new RSAEncryptionV1_5_Gadget(pk, plainBytes, rndBytes, keyBits, description);
                break;
            }
            default:
                throw new IllegalStateException("Unexpected padding type: " + paddingType);
        }

        cipher = new WireArray(enc.getOutputWires()).packWordsIntoLargerWords(8, CIPHER_CHUNK_SIZE / 8);
    }

    @Override
    public Wire[] getOutputWires() {
        return cipher;
    }
}
