
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

pub struct AESSBoxGadgetOptimized1  {

	  i32 Vec<SBox> = AES128CipherGadget.SBox;

	 allCoeffSet:ArrayList<Vec<BigInteger>>,

	 {
		// preprocessing
		solveLinearSystems();
	}

	 input:Wire,
	 output:Wire,
}
impl  AESSBoxGadgetOptimized1{
	pub  fn new(input:Wire, desc:Vec<String>)  ->Self{
		super(desc);
		self.input = input;
		buildCircuit();
	}
}
impl Gadget for AESSBoxGadgetOptimized1{
	pub    solveLinearSystems() {
		allCoeffSet = new ArrayList<Vec<BigInteger>>();
let list = new ArrayList<Integer>();
		for i in 0..=255{
			list.add(256 * i + SBox[i]);
		}

		for i in 0..=15{
let memberValueSet = new HashSet<>();
let mat = vec![BigInteger::default();16][17];

			// used for sanity checks
let polyCoeffs = vec![BigInteger::default();] { BigInteger.ONE };

			for k in 0..mat.length {
let value = list.get(k + i * 16);
				memberValueSet.add(value);
let p = BigInteger.valueOf(value);
				mat[k][0] = BigInteger.ONE;
				for j in 1..=16{
					mat[k][j] = p.multiply(mat[k][j - 1]).mod(
							Config.FIELD_PRIME);
				}
				// negate the last element, just to make things consistent with
				// the paper notations
				mat[k][16] = Config.FIELD_PRIME.subtract(mat[k][16]);
				

				// used for a sanity check (verifying that the output solution
				// is equivalent to coefficients of polynomial that has roots at
				// memberValueSet. see note above)
				polyCoeffs = polyMul(polyCoeffs, vec![BigInteger::default();] {
						Config.FIELD_PRIME.subtract(p), BigInteger.ONE });
			}

			LinearSystemSolver::new(mat).solveInPlace();

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

let coeffs = vec![BigInteger::default();16];
			for ii in 0..16{
				coeffs[ii] = mat[ii][16];
				if !coeffs[ii].equals(polyCoeffs[ii]) {
					panic!("Inconsistency found.");
				}
			}
			allCoeffSet.add(coeffs);
		}

	}

	// method for sanity checks during preprocessing
	 fn polyMul(a1:Vec<BigInteger>, a2:Vec<BigInteger>)-> Vec<BigInteger> {
let out = vec![BigInteger::default();a1.length + a2.length - 1];
		Arrays.fill(out, BigInteger.ZERO);
		for i in 0..a1.length {
			for j in 0..a2.length {
				out[i + j] = out[i + j].add(a1[i].multiply(a2[j])).mod(
						Config.FIELD_PRIME);
			}
		}
		return out;
	}

	  bool checkIfProverCanCheat(Vec<Vec<BigInteger>> mat,
			HashSet<Integer> valueSet) {

let coeffs = vec![BigInteger::default();16];
		for i in 0..16 {
			coeffs[i] = mat[i][16];
		}

let validResults = 0;
let outsidePermissibleSet = 0;

		// loop over the whole permissible domain (recall that input & output
		// are bounded)
		for k in 0..256 * 256{

let result = coeffs[0];
let p = BigInteger.valueOf(k);
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

	  fn buildCircuit() {

		output = generator.createProverWitnessWire();
		input.restrictBitLength(8);
		generator.specifyProverWitnessComputation(Instruction::new() {

			
			pub   evaluate(evaluator:CircuitEvaluator) {
				// TODO Auto-generated method stub
let value = evaluator.getWireValue(input);
				evaluator.setWireValue(output,
						BigInteger.valueOf(SBox[value.intValue()]));
			}
		});

		output.restrictBitLength(8);
let vars = vec![Wire::default();16];
let p = input.mul(256).add(output);
		vars[0] = generator.getOneWire();
		for i in 1..16 {
			vars[i] = vars[i - 1].mul(p);
		}

let product = generator.getOneWire();
		for coeffs in  allCoeffSet {
let accum = generator.getZeroWire();
			for j in 0..vars.length {
				accum = accum.add(vars[j].mul(coeffs[j]));
			}
			accum = accum.add(vars[15].mul(p));
			product = product.mul(accum);
		}
		generator.addZeroAssertion(product);
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![Wire::default();] { output };
	}

}
