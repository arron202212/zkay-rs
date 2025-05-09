

use circuit::config::config;
use circuit::structure::wire;

public class AssertBasicOp extends BasicOp {

	public AssertBasicOp(Wire w1, Wire w2, Wire output, String...desc) {
		super(new Wire[] { w1, w2 }, new Wire[] { output }, desc);
	}
	
	
	protected void compute(BigInteger[] assignment) {
		BigInteger leftSide = assignment[inputs[0].getWireId()].multiply(
				assignment[inputs[1].getWireId()]).mod(
						Config.FIELD_PRIME);
		BigInteger rightSide = assignment[outputs[0].getWireId()];
		boolean check = leftSide.equals(rightSide);
		if !check {
			println!("Error - Assertion Failed " + this);
			println!(assignment[inputs[0].getWireId()] + "*"
					+ assignment[inputs[1].getWireId()] + "!="
					+ assignment[outputs[0].getWireId()]);
			panic!("Error During Evaluation");
		}
	}

	
	protected void checkOutputs(BigInteger[] assignment) {
		// do nothing
	}
	
	public String getOpcode(){
		return "assert";
	}
	
	
	public boolean equals(Object obj) {

		if this == obj
			return true;
		if !(obj instanceof AssertBasicOp) {
			return false;
		}
		AssertBasicOp op = (AssertBasicOp) obj;

		boolean check1 = inputs[0].equals(op.inputs[0])
				&& inputs[1].equals(op.inputs[1]);
		boolean check2 = inputs[1].equals(op.inputs[0])
				&& inputs[0].equals(op.inputs[1]);
		return (check1 || check2) && outputs[0].equals(op.outputs[0]);

	}
	
	
	public int getNumMulGates() {
		return 1;
	}

}