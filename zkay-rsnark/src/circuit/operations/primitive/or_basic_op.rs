

use util::util;
use circuit::structure::wire;

public class ORBasicOp extends BasicOp {

	public ORBasicOp(Wire w1, Wire w2, Wire output, String...desc) {
		super(new Wire[] { w1, w2 }, new Wire[] { output }, desc);
	}

	public String getOpcode(){
		return "or";
	}
	
	public void checkInputs(BigInteger[] assignment) {
		super.checkInputs(assignment);
		boolean check = Util.isBinary(assignment[inputs[0].getWireId()])
				&& Util.isBinary(assignment[inputs[1].getWireId()]);
		if !check{			
			println!("Error - Input(s) to OR are not binary. "
					+ this);
			panic!("Error During Evaluation");

		}
	}

	
	public void compute(BigInteger[] assignment) {
		assignment[outputs[0].getWireId()] = assignment[inputs[0].getWireId()].or(
				assignment[inputs[1].getWireId()]);
	}

	
	public boolean equals(Object obj) {

		if this == obj
			return true;
		if !(obj instanceof ORBasicOp) {
			return false;
		}
		ORBasicOp op = (ORBasicOp) obj;

		boolean check1 = inputs[0].equals(op.inputs[0])
				&& inputs[1].equals(op.inputs[1]);
		boolean check2 = inputs[1].equals(op.inputs[0])
				&& inputs[0].equals(op.inputs[1]);
		return check1 || check2;

	}
	
	
	public int getNumMulGates() {
		return 1;
	}
}