
use circuit::structure::wire;

/**
 * Gadget for homomorphically multiplying an ElGamal ciphertext (c1, c2) by a plaintext scalar
 */
public class ZkayElgamalMulGadget extends ZkayBabyJubJubGadget {

     JubJubPoint c1;

     JubJubPoint c2;

    private Wire[] scalarBits;

    private JubJubPoint e1;

    private JubJubPoint e2;

    public ZkayElgamalMulGadget(JubJubPoint c1, JubJubPoint c2, Wire [] scalarBits) {
        this.c1 = c1;
        this.c2 = c2;
        this.scalarBits = scalarBits;
        buildCircuit();
    }

    protected void buildCircuit() {
        e1 = mulScalar(c1, scalarBits);
        e2 = mulScalar(c2, scalarBits);
    }

    
    public Wire[] getOutputWires() {
        return new Wire[]{ e1.x, e1.y, e2.x, e2.y };
    }
}
