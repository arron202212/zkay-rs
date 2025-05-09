
use circuit::structure::wire;

/**
 * Gadget for homomorphically adding two ElGamal ciphertexts (c1, c2) and (d1, d2).
 */
public class ZkayElgamalAddGadget extends ZkayBabyJubJubGadget {

     JubJubPoint c1;

     JubJubPoint c2;

     JubJubPoint d1;

     JubJubPoint d2;

    private JubJubPoint e1;

    private JubJubPoint e2;

    public ZkayElgamalAddGadget(JubJubPoint c1, JubJubPoint c2, JubJubPoint d1, JubJubPoint d2) {
        this.c1 = c1;
        this.c2 = c2;
        this.d1 = d1;
        this.d2 = d2;
        buildCircuit();
    }

    protected void buildCircuit() {
        e1 = addPoints(c1, d1);
        e2 = addPoints(c2, d2);
    }

    
    public Wire[] getOutputWires() {
        return new Wire[]{ e1.x, e1.y, e2.x, e2.y };
    }
}
