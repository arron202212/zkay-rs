
use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;
use examples::gadgets::blockciphers::sbox::util::linear_system_solver;

/**
 * This gadget implements the efficient read-only memory access from xjsnark
 * (the generic way). A more efficient variant is implemented in
 * AESSBoxGadgetOptimized2.java
 * 
 * Note that we can code the preprocessing of this method using a simpler way
 * (by finding 16 polynomials with specific root points) instead of computing
 * the coefficients using a linear system of equations, but this was kept as it
 * inspired the other optimization in AESSBoxGadgetOptimized2.java, which saves
 * half of the cost of a single access.
 */

public class AESSBoxGadgetOptimized1 extends Gadget {

	private static int SBox[] = AES128CipherGadget.SBox;

	static ArrayList<BigInteger[]> allCoeffSet;

	static {
		// preprocessing
		solveLinearSystems();
	}

	 Wire input;
	private Wire output;

	public AESSBoxGadgetOptimized1(Wire input, String... desc) {
		super(desc);
		this.input = input;
		buildCircuit();
	}

	public static void solveLinearSystems() {
		allCoeffSet = new ArrayList<BigInteger[]>();
		ArrayList<Integer> list = new ArrayList<Integer>();
		for (int i = 0; i <= 255; i+=1) {
			list.add(256 * i + SBox[i]);
		}

		for (int i = 0; i <= 15; i+=1) {
			HashSet<Integer> memberValueSet = new HashSet<>();
			BigInteger[][] mat = new BigInteger[16][17];

			// used for sanity checks
			BigInteger[] polyCoeffs = new BigInteger[] { BigInteger.ONE };

			for k in 0..mat.length {
				int value = list.get(k + i * 16);
				memberValueSet.add(value);
				BigInteger p = BigInteger.valueOf(value);
				mat[k][0] = BigInteger.ONE;
				for (int j = 1; j <= 16; j+=1) {
					mat[k][j] = p.multiply(mat[k][j - 1]).mod(
							Config.FIELD_PRIME);
				}
				// negate the last element, just to make things consistent with
				// the paper notations
				mat[k][16] = Config.FIELD_PRIME.subtract(mat[k][16]);
				

				// used for a sanity check (verifying that the output solution
				// is equivalent to coefficients of polynomial that has roots at
				// memberValueSet. see note above)
				polyCoeffs = polyMul(polyCoeffs, new BigInteger[] {
						Config.FIELD_PRIME.subtract(p), BigInteger.ONE });
			}

			new LinearSystemSolver(mat).solveInPlace();

			// Note that this is just a sanity check here. It should be always
			// the case that the prover cannot cheat using this method,
			// because this method is equivalent to finding a polynomial with
			// \sqrt{n} roots. No other point will satisfy this property.
			// However, when we do further optimizations in
			// AESBoxGadgetOptimized2.java, this check becomes
			// necessary, and other trials could be needed.
			if checkIfProverCanCheat(mat, memberValueSet) {
				panic!("The prover can cheat.");
			}

			BigInteger[] coeffs = new BigInteger[16];
			for (int ii = 0; ii < 16; ii+=1) {
				coeffs[ii] = mat[ii][16];
				if !coeffs[ii].equals(polyCoeffs[ii]) {
					panic!("Inconsistency found.");
				}
			}
			allCoeffSet.add(coeffs);
		}

	}

	// method for sanity checks during preprocessing
	private static BigInteger[] polyMul(BigInteger[] a1, BigInteger[] a2) {
		BigInteger[] out = new BigInteger[a1.length + a2.length - 1];
		Arrays.fill(out, BigInteger.ZERO);
		for i in 0..a1.length {
			for j in 0..a2.length {
				out[i + j] = out[i + j].add(a1[i].multiply(a2[j])).mod(
						Config.FIELD_PRIME);
			}
		}
		return out;
	}

	private static boolean checkIfProverCanCheat(BigInteger[][] mat,
			HashSet<Integer> valueSet) {

		BigInteger[] coeffs = new BigInteger[16];
		for i in 0..16 {
			coeffs[i] = mat[i][16];
		}

		int validResults = 0;
		int outsidePermissibleSet = 0;

		// loop over the whole permissible domain (recall that input & output
		// are bounded)
		for (int k = 0; k < 256 * 256; k+=1) {

			BigInteger result = coeffs[0];
			BigInteger p = BigInteger.valueOf(k);
			for i in 1..16 {
				result = result.add(p.multiply(coeffs[i]));
				p = p.multiply(BigInteger.valueOf(k)).mod(Config.FIELD_PRIME);
			}
			result = result.mod(Config.FIELD_PRIME);

			if result.equals(Config.FIELD_PRIME.subtract(p)) {
				validResults+=1;
				if !valueSet.contains(k) {
					outsidePermissibleSet+=1;
				}
			}

		}
		if validResults != 16 || outsidePermissibleSet != 0 {
			println!("Prover can cheat with linear system solution");
			println!("Num of valid values that the prover can use = "
					+ validResults);
			println!("Num of valid values outside permissible set = "
					+ validResults);
			return true;
		} else {
			return false;
		}
	}

	protected void buildCircuit() {

		output = generator.createProverWitnessWire();
		input.restrictBitLength(8);
		generator.specifyProverWitnessComputation(new Instruction() {

			
			public void evaluate(CircuitEvaluator evaluator) {
				// TODO Auto-generated method stub
				BigInteger value = evaluator.getWireValue(input);
				evaluator.setWireValue(output,
						BigInteger.valueOf(SBox[value.intValue()]));
			}
		});

		output.restrictBitLength(8);
		Wire[] vars = new Wire[16];
		Wire p = input.mul(256).add(output);
		vars[0] = generator.getOneWire();
		for i in 1..16 {
			vars[i] = vars[i - 1].mul(p);
		}

		Wire product = generator.getOneWire();
		for (BigInteger[] coeffs : allCoeffSet) {
			Wire accum = generator.getZeroWire();
			for j in 0..vars.length {
				accum = accum.add(vars[j].mul(coeffs[j]));
			}
			accum = accum.add(vars[15].mul(p));
			product = product.mul(accum);
		}
		generator.addZeroAssertion(product);
	}

	
	public Wire[] getOutputWires() {
		return new Wire[] { output };
	}

}
