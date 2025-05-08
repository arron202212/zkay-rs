

use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::math::long_integer_mod_gadget;
use examples::gadgets::math::long_integer_mod_inverse_gadget;
use examples::gadgets::math::long_integer_mod_pow_gadget;

public class ZkayPaillierFastEncGadget extends Gadget {

	private final LongElement n;
	private final LongElement nSquare;
	private final int nBits;
	private final int nSquareMaxBits;
	private final LongElement plain;
	private final LongElement random;
	private LongElement cipher;

	public ZkayPaillierFastEncGadget(LongElement n, int nBits, LongElement plain, LongElement random, String... desc) {
		super(desc);
		this.n = n;
		this.nBits = nBits;
		this.nSquareMaxBits = 2 * nBits; // Maximum bit length of n^2
		int maxNumChunks = (nSquareMaxBits + (LongElement.CHUNK_BITWIDTH - 1)) / LongElement.CHUNK_BITWIDTH;
		this.nSquare = n.mul(n).align(maxNumChunks);
		this.plain = plain;
		this.random = random;
		buildCircuit();
	}

	private void buildCircuit() {
		int nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
		// Prove that random is in Z_n* by checking that random has an inverse mod n
		LongElement randInv = new LongIntegerModInverseGadget(random, n, false).getResult();
		generator.addOneAssertion(randInv.checkNonZero());
		// Compute c = g^m * r^n mod n^2
		LongElement gPowPlain = n.mul(plain).add(1).align(nSquare.getSize());
		LongElement randPowN = new LongIntegerModPowGadget(random, n, nBits, nSquare, nSquareMinBits, "r^n").getResult();
		LongElement product = gPowPlain.mul(randPowN);
		cipher = new LongIntegerModGadget(product, nSquare, nSquareMinBits, true, "g^m * r^n mod n^2").getRemainder();
	}

	public LongElement getCiphertext() {
		return cipher;
	}

	@Override
	public Wire[] getOutputWires() {
		return cipher.getArray();
	}
}
