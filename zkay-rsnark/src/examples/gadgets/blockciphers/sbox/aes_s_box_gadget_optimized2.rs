use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
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

pub struct AESSBoxGadgetOptimized2 {
    allCoeffSet: ArrayList<Vec<BigInteger>>,

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
    bitCount: i32,

    input: WireType,
    output: WireType,
}
impl AESSBoxGadgetOptimized2 {
    fn preprocessing() {
        // preprocessing
        solveLinearSystems();
    }
    const SBox: Vec<i32> = AES128CipherGadget.SBox;
    pub fn new(input: WireType, desc: &Option<String>) -> Self {
        super(desc);
        self.input = input;
        buildCircuit();
    }
    pub fn setBitCount(x: i32) {
        assert!(x >= 0 && x <= 16);
        bitCount = x;
    }
}
impl Gadget for AESSBoxGadgetOptimized2 {
    pub fn solveLinearSystems() {
        let seed = 1;
        let allCoeffSet = Vec::new();
        let list = Vec::new();
        for i in 0..=255 {
            list.add(256 * i + SBox[i]);
        }
        let mut done = false;
        let mut trialCounter = 0;
        'loop1: while (!done) {
            trialCounter += 1;
            assert!(trialCounter < 100
						"Was not possible to find an adequate solution to the current setting of the AES gadget sbox");

            //println!(
                "Attempting to solve linear systems for efficient S-Box Access: Attempt#{trialCounter}"
            );
            seed += 1;
            Collections.shuffle(list, Random::new(seed));
            allCoeffSet.clear();

            for i in 0..=15 {
                let mut mat = vec![vec![BigInteger::default()17]; 16];
                let mut memberValueSet = HashSet::new();

                for k in 0..mat.len() {
                    let memberValue = list.get(k + i * 16);
                    memberValueSet.add(memberValue);
                    mat[k][16] = Util::one();

                    // now extract the values that correspond to memberValue
                    // the method getVariableValues takes the bitCount settings
                    // into account
                    let variableValues = getVariableValues(memberValue);
                    for j in 0..=15 {
                        mat[k][j] = variableValues[j];
                    }
                }

                LinearSystemSolver::new(mat).solveInPlace();

                if checkIfProverCanCheat(mat, memberValueSet) {
                    //println!("Invalid solution");
                    for ii in 0..16 {
                        if mat[ii][16]==BigInteger::ZERO {
                            //println!("Possibly invalid due to having zero coefficient(s)");
                            break;
                        }
                    }

                    continue 'loop1;
                }

                let mut coeffs = vec![BigInteger::default(); 16];
                for ii in 0..16 {
                    coeffs[ii] = mat[ii][16];
                }
                allCoeffSet.add(coeffs);
            }
            done = true;
            AESSBoxGadgetOptimized2.allCoeffSet = allCoeffSet;
            //println!("Solution found!");
        }
    }

    fn buildCircuit() {
        output = generator.createProverWitnessWire();
        generator.specifyProverWitnessComputation(    &|evaluator: &mut CircuitEvaluator| {
                    // TODO Auto-generated method stub
                    let value = evaluator.getWireValue(input);
                    evaluator.setWireValue(output, BigInteger::from(SBox[value.intValue()]));
                });
// {
//             struct Prover;
//             impl Instruction for Prover {
//                 &|evaluator: &mut CircuitEvaluator| {
//                     // TODO Auto-generated method stub
//                     let value = evaluator.getWireValue(input);
//                     evaluator.setWireValue(output, BigInteger::from(SBox[value.intValue()]));
//                 }
//             }
//             Prover
//         });

        // Although we are getting the bits below anyway (which implicitly
        // restricts the bitwidth), it's a safer practice to call
        // restrictBitLength() explicitly to avoid some special cases with
        // getBitWires().
        // Similar operations get filtered later, so this won't add extra
        // constraints.
        output.restrictBitLength(8);
        input.restrictBitLength(8);

        let bitsIn = input.getBitWires(8).asArray();
        let bitsOut = output.getBitWires(8).asArray();
        let vars = vec![None; 16];
        let p = input.mul(256).add(output).add(1);
        let currentProduct = p;
        if bitCount != 0 && bitCount != 16 {
            currentProduct = currentProduct.mul(currentProduct);
        }
        for i in 0..16 {
            if i < bitCount {
                if i < 8 {
                    vars[i] = bitsOut[i];
                } else {
                    vars[i] = bitsIn[i - 8];
                }
            } else {
                vars[i] = currentProduct;
                if i != 15 {
                    currentProduct = currentProduct.mul(p);
                }
            }
        }

        let product = generator.get_one_wire();
        for coeffs in allCoeffSet {
            let accum = generator.get_zero_wire();
            for j in 0..vars.len() {
                accum = accum.add(vars[j].mul(coeffs[j]));
            }
            accum = accum.sub(1);
            product = product.mul(accum);
        }
        generator.addZeroAssertion(product);
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return vec![output];
    }

    fn getVariableValues(k: i32) -> Vec<BigInteger> {
        let vars = vec![BigInteger::default(); 16];
        let v = BigInteger::from(k).add(Util::one());
        let product = v;
        if bitCount != 0 {
            product = product.mul(v).rem(Configs.field_prime.clone());
        }
        for j in 0..16 {
            if j < bitCount {
                vars[j] = if ((k >> j) & 0x01) == 1 {
                    Util::one()
                } else {
                    BigInteger::ZERO
                };
            } else {
                vars[j] = product;
                product = product.mul(v).rem(Configs.field_prime.clone());
            }
        }
        return vars;
    }

    fn checkIfProverCanCheat(mat: Vec<Vec<BigInteger>>, valueSet: HashSet<Integer>) -> bool {
        let coeffs = vec![BigInteger::default(); 16];
        for i in 0..16 {
            coeffs[i] = mat[i][16];
        }

        let validResults = 0;
        let outsidePermissibleSet = 0;

        // loop over the whole permissible domain (recall that input & output
        // are bounded)

        for k in 0..256 * 256 {
            let variableValues = getVariableValues(k);
            let result = BigInteger::ZERO;
            for i in 0..16 {
                result = result.add(variableValues[i].mul(coeffs[i]));
            }
            result = result.rem(Configs.field_prime.clone());
            if result==Util::one() {
                validResults += 1;
                if !valueSet.contains(k) {
                    outsidePermissibleSet += 1;
                }
            }
        }
        if validResults != 16 || outsidePermissibleSet != 0 {
            //println!("Prover can cheat with linear system solution");
            //println!("Num of valid values that the prover can use = " + validResults);
            //println!("Num of valid values outside permissible set = " + validResults);
            return true;
        } else {
            return false;
        }
    }
}
