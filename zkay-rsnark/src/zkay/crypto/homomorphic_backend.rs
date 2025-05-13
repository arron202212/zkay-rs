
use zkay::homomorphic_input;
use zkay::typed_wire;

pub trait  HomomorphicBackend {

	/**
	 * Perform the unary homomorphic operation 'op' on the ciphertext 'cipher'.
	 *
	 * @param op
	 * 		a char identifying the operation; one of {'-', '~', '!'}
	 * @param arg
	 * 		the operand, either a ciphertext or a plain wire
	 * @param keyName
	 * 		the qualified name of the key to be used
	 *
	 * @return the resulting ciphertext
	 *
	 * @throws UnsupportedOperationException
	 * 		if the backend does not support operation 'op'
	 */
	fn  doHomomorphicOp( op:char ,  arg:HomomorphicInput ,  keyName:String ) {
		panic!("Unary operation {op} not supported");
	}

	/**
	 * Perform the binary homomorphic operation 'op' on the ciphertexts 'lhs' and 'rhs'.
	 *
	 * @param lhs
	 * 		the left-hand side operand, either a ciphertext or a plain wire
	 * @param op
	 * 		a char identifying the operation; one of {'+', '-', '*', '/', '%', '|', '&', '^', '<', '>'}
	 * @param rhs
	 * 		the right-hand side operand, either a ciphertext or a plain wire
	 * @param keyName
	 * 		the qualified name of the key to be used
	 *
	 * @return the resulting ciphertext
	 *
	 * @throws UnsupportedOperationException
	 * 		if the backend does not support operation 'op'
	 */
	fn  doHomomorphicOp( lhs:HomomorphicInput ,  op:char ,  rhs:HomomorphicInput ,  keyName:String )->Vec<TypedWire> {
		panic!("Binary operation {op} not supported");
	}

	/**
	 * Perform the bool / comparison homomorphic operation 'op' on the ciphertexts 'lhs' and 'rhs'.
	 *
	 * @param lhs
	 * 		the left-hand side operand, either a ciphertext or a plain wire
	 * @param op
	 * 		a char identifying the operation; one of {"==", "!=", "<=", ">=", "&&", "||"}
	 * @param rhs
	 * 		the right-hand side operand, either a ciphertext or a plain wire
	 * @param keyName
	 * 		the qualified name of the key to be used
	 *
	 * @return the resulting ciphertext
	 *
	 * @throws UnsupportedOperationException
	 * 		if the backend does not support operation 'op'
	 */
	fn  doHomomorphicOp( lhs:HomomorphicInput ,  op:String ,  rhs:HomomorphicInput ,  keyName:String )->Vec<TypedWire> {
		panic!("Boolean / comparison operation {op} not supported");
	}

	/**
	 * Perform the ternary conditional operation on the ciphertexts 'cond', 'trueVal', 'falseVal'.
	 *
	 * @param cond
	 * 		the condition, either a ciphertext or a plain wire
	 * @param trueVal
	 * 		the value if cond is true, either a ciphertext or a plain wire
	 * @param falseVal
	 * 		the value if cond is false, either a ciphertext or a plain wire
	 * @param keyName
	 * 		the qualified name of the key to be used
	 *
	 * @return the resulting ciphertext
	 *
	 * @throws UnsupportedOperationException
	 * 		if the backend does not support operation 'op'
	 */
	fn  doHomomorphicCond( cond:HomomorphicInput ,  trueVal:HomomorphicInput ,  falseVal:HomomorphicInput ,  keyName:String )-> Vec<TypedWire>{
		panic!("Ternary conditional not supported");
	}

	/**
	 * Re-randomizes the ciphertext in 'arg' by 'randomness'.
	 *
	 * @param arg
	 * 		the ciphertext to be re-randomized
	 * @param keyName
	 * 		the qualified name of the key under which arg is encrypted
	 * @param randomness
	 * 		the randomness to use for re-randomization
	 *
	 * @return the re-randomized ciphertext
	 */
	fn  doHomomorphicRerand( arg:Vec<TypedWire> ,  keyName:String ,  randomness:TypedWire )->Vec<TypedWire> {
		panic!("Homomorphic re-randomization not supported");
	}
}
