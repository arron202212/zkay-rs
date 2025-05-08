

public interface Instruction {

	public void evaluate(CircuitEvaluator evaluator);

	public default void emit(CircuitEvaluator evaluator) {
	}

	public default boolean doneWithinCircuit() {
		return false;
	}
}
