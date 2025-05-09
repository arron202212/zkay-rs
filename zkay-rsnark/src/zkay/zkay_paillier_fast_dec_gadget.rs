

use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::math::long_integer_floor_div_gadget;
use examples::gadgets::math::long_integer_mod_gadget;
use examples::gadgets::math::long_integer_mod_inverse_gadget;
use examples::gadgets::math::long_integer_mod_pow_gadget;

public class ZkayPaillierFastDecGadget extends Gadget {

	 LongElement n;
	 LongElement nSquare;
	 int nBits;
	 LongElement lambda;
	 LongElement cipher;
	private LongElement plain;

	public ZkayPaillierFastDecGadget(LongElement n, int nBits, LongElement lambda, LongElement cipher, String... desc) {
		super(desc);
		this.n = n;
		this.nBits = nBits;
		int nSquareMaxBits = 2 * nBits;
		int maxNumChunks = (nSquareMaxBits + (LongElement.CHUNK_BITWIDTH - 1)) / LongElement.CHUNK_BITWIDTH;
		this.nSquare = n.mul(n).align(maxNumChunks);
		this.lambda = lambda;
		this.cipher = cipher;
		buildCircuit();
	}

	private void buildCircuit() {
		int nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
		LongElement lambdaInverse = new LongIntegerModInverseGadget(lambda, n, false, "lambda^(-1)").getResult();

		// plain = L(cipher^lambda mod n^2) / lambda mod n
		LongElement cPowLambda = new LongIntegerModPowGadget(cipher, lambda, nSquare, nSquareMinBits, "c^lambda").getResult();
		LongElement lOutput = new LongIntegerFloorDivGadget(cPowLambda.subtract(1), n, "(c^lambda - 1) / n").getQuotient();
		LongElement timesLambdaInverse = lOutput.mul(lambdaInverse);
		plain = new LongIntegerModGadget(timesLambdaInverse, n, nBits, true).getRemainder();
	}

	public LongElement getPlaintext() {
		return plain;
	}

	
	public Wire[] getOutputWires() {
		return plain.getArray();
	}
}
