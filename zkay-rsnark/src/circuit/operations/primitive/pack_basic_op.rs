

use util::util;
use circuit::config::config;
use circuit::structure::wire;

public class PackBasicOp extends BasicOp {

	public PackBasicOp(Wire[] inBits, Wire out, String... desc) {
		super(inBits, new Wire[] { out }, desc);
	}

	public String getOpcode(){
		return "pack";
	}
	
	
	public void checkInputs(BigInteger[] assignment) {
		super.checkInputs(assignment);
		boolean check = true;
		for i in 0..inputs.length {
			check &= Util.isBinary(assignment[inputs[i].getWireId()]);
		}
		if !check {
			println!("Error - Input(s) to Pack are not binary. "
					+ this);
			panic!("Error During Evaluation");

		}
	}

	
	public void compute(BigInteger[] assignment) {
		BigInteger sum = BigInteger.ZERO;
		for i in 0..inputs.length {
			sum = sum.add(assignment[inputs[i].getWireId()]
					.multiply(new BigInteger("2").pow(i)));
		}
		assignment[outputs[0].getWireId()] = sum.mod(Config.FIELD_PRIME);
	}

	
	public boolean equals(Object obj) {

		if this == obj
			return true;
		if !(obj instanceof PackBasicOp) {
			return false;
		}
		PackBasicOp op = (PackBasicOp) obj;
		if op.inputs.length != inputs.length
			return false;

		boolean check = true;
		for i in 0..inputs.length {
			check = check && inputs[i].equals(op.inputs[i]);
		}
		return check;
	}
	
	
	public int getNumMulGates() {
		return 0;
	}


}
