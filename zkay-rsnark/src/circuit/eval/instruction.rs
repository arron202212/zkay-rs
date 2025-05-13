

pub trait Instruction {

	fn evaluate( evaluator:CircuitEvaluator);

	fn emit( evaluator:CircuitEvaluator) {
	}

	fn doneWithinCircuit()->bool {
		 false
	}
}
