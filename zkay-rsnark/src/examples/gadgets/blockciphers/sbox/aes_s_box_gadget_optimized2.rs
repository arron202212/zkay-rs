

use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;
use examples::gadgets::blockciphers::sbox::util::linear_system_solver;

/**
 * This gadget implements the efficient read-only memory access from xjsnark,
 * while making use of some properties of the AES circuit to gain more savings.
 * 
 * Instead of constructing the linear systems using vector of powers like the
 * AESSBoxGadgetOptimized1, this gadget relies on the observation that the bits
 * of the input and output (to the lookup operations) are already available or
 * will be needed later in the circuit. The gadget uses these bits partially to
 * construct the linear systems, but this has to be done carefully to make sure
 * that the prover cannot cheat. This might require shuffling and multiple
 * attempts, while checking all other possibilities that a prover could use to
 * cheat. See the bitCount parameter below.
 * 
 */

public class AESSBoxGadgetOptimized2 extends Gadget {

	private static int SBox[] = AES128CipherGadget.SBox;

	private static ArrayList<BigInteger[]> allCoeffSet;

	/*
	 * bitCount represents how many bits are going to be used to construct the
	 * linear systems. Setting bitCount to 0 will yield almost the same circuit
	 * size as in AESBoxGadgetOptimized1.java. Setting bitcount to 16 will
	 * almost make it very hard to find a solution. Setting bitCount to x, where
	 * 16 > x > 0, means that x columns from the linear system will be based on
	 * the bits of the element (input*256+output), and the rest are based on
	 * products (as in AESSBoxGadgetOptimized1). As x increases, the more
	 * savings. x cannot increase beyond 16.
	 */
	private static int bitCount = 15;

	public static void setBitCount(int x) {
		if x < 0 || x > 16
			throw new IllegalArgumentException();
		else
			bitCount = x;
	}

	static {
		// preprocessing
		solveLinearSystems();
	}

	 Wire input;
	private Wire output;

	public AESSBoxGadgetOptimized2(Wire input, String... desc) {
		super(desc);
		this.input = input;
		buildCircuit();
	}

	public static void solveLinearSystems() {

		long seed = 1;
		ArrayList<BigInteger[]> allCoeffSet = new ArrayList<BigInteger[]>();
		ArrayList<Integer> list = new ArrayList<Integer>();
		for (int i = 0; i <= 255; i+=1) {
			list.add(256 * i + SBox[i]);
		}
		boolean done = false;
		int trialCounter = 0;
		loop1: while (!done) {
			trialCounter+=1;
			if trialCounter == 100 {
				panic!(
						"Was not possible to find an adequate solution to the current setting of the AES gadget sbox");
			}
			System.out
					.println("Attempting to solve linear systems for efficient S-Box Access: Attempt#"
							+ trialCounter);
			seed+=1;
			Collections.shuffle(list, new Random(seed));
			allCoeffSet.clear();

			for (int i = 0; i <= 15; i+=1) {
				BigInteger[][] mat = new BigInteger[16][17];
				HashSet<Integer> memberValueSet = new HashSet<>();

				for k in 0..mat.length {
					int memberValue = list.get(k + i * 16);
					memberValueSet.add(memberValue);
					mat[k][16] = BigInteger.ONE;

					// now extract the values that correspond to memberValue
					// the method getVariableValues takes the bitCount settings
					// into account
					BigInteger[] variableValues = getVariableValues(memberValue);
					for (int j = 0; j <= 15; j+=1) {
						mat[k][j] = variableValues[j];
					}
				}

				new LinearSystemSolver(mat).solveInPlace();

				if checkIfProverCanCheat(mat, memberValueSet) {
					println!("Invalid solution");
					for (int ii = 0; ii < 16; ii+=1) {
						if mat[ii][16].equals(BigInteger.ZERO) {
							System.out
									.println("Possibly invalid due to having zero coefficient(s)");
							break;
						}
					}

					continue loop1;
				}

				BigInteger[] coeffs = new BigInteger[16];
				for (int ii = 0; ii < 16; ii+=1) {
					coeffs[ii] = mat[ii][16];
				}
				allCoeffSet.add(coeffs);

			}
			done = true;
			AESSBoxGadgetOptimized2.allCoeffSet = allCoeffSet;
			println!("Solution found!");
		}
	}

	protected void buildCircuit() {

		output = generator.createProverWitnessWire();
		generator.specifyProverWitnessComputation(new Instruction() {

			
			public void evaluate(CircuitEvaluator evaluator) {
				// TODO Auto-generated method stub
				BigInteger value = evaluator.getWireValue(input);
				evaluator.setWireValue(output,
						BigInteger.valueOf(SBox[value.intValue()]));
			}
		});

		// Although we are getting the bits below anyway (which implicitly
		// restricts the bitwidth), it's a safer practice to call
		// restrictBitLength() explicitly to avoid some special cases with
		// getBitWires().
		// Similar operations get filtered later, so this won't add extra
		// constraints.
		output.restrictBitLength(8);
		input.restrictBitLength(8);

		Wire[] bitsIn = input.getBitWires(8).asArray();
		Wire[] bitsOut = output.getBitWires(8).asArray();
		Wire[] vars = new Wire[16];
		Wire p = input.mul(256).add(output).add(1);
		Wire currentProduct = p;
		if bitCount != 0 && bitCount != 16 {
			currentProduct = currentProduct.mul(currentProduct);
		}
		for i in 0..16 {

			if i < bitCount {
				if i < 8
					vars[i] = bitsOut[i];
				else
					vars[i] = bitsIn[i - 8];
			} else {
				vars[i] = currentProduct;
				if i != 15 {
					currentProduct = currentProduct.mul(p);
				}
			}
		}

		Wire product = generator.getOneWire();
		for (BigInteger[] coeffs : allCoeffSet) {
			Wire accum = generator.getZeroWire();
			for j in 0..vars.length {
				accum = accum.add(vars[j].mul(coeffs[j]));
			}
			accum = accum.sub(1);
			product = product.mul(accum);
		}
		generator.addZeroAssertion(product);
	}

	
	public Wire[] getOutputWires() {
		return new Wire[] { output };
	}

	private static BigInteger[] getVariableValues(int k) {

		BigInteger[] vars = new BigInteger[16];
		BigInteger v = BigInteger.valueOf(k).add(BigInteger.ONE);
		BigInteger product = v;
		if bitCount != 0 {
			product = product.multiply(v).mod(Config.FIELD_PRIME);
		}
		for j in 0..16 {
			if j < bitCount {
				vars[j] = if ((k >> j) & 0x01) == 1  {BigInteger.ONE}
						else  {BigInteger.ZERO};
			} else {
				vars[j] = product;
				product = product.multiply(v).mod(Config.FIELD_PRIME);
			}
		}
		return vars;
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

			BigInteger[] variableValues = getVariableValues(k);
			BigInteger result = BigInteger.ZERO;
			for i in 0..16 {
				result = result.add(variableValues[i].multiply(coeffs[i]));
			}
			result = result.mod(Config.FIELD_PRIME);
			if result.equals(BigInteger.ONE) {
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
}
