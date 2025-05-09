

use circuit::config::config;
use circuit::structure::wire;

public class SplitBasicOp extends BasicOp {

	public SplitBasicOp(Wire w, Wire[] outs, String...desc) {
		super(new Wire[] { w }, outs, desc);
	}

	public String getOpcode(){
		return "split";
	}
	
	protected void checkInputs(BigInteger[] assignment) {
		super.checkInputs(assignment);
		if outputs.length < assignment[inputs[0].getWireId()].bitLength() {
			println!("Error in Split --- The number of bits does not fit -- Input: "
							+ assignment[inputs[0].getWireId()].toString(16) + "\n\t" + this);

			panic!("Error During Evaluation -- " + this);
		}
	}

	
	protected void compute(BigInteger[] assignment) {

		BigInteger inVal = assignment[inputs[0].getWireId()];
		if inVal.compareTo(Config.FIELD_PRIME) > 0 {
			inVal = inVal.mod(Config.FIELD_PRIME);
		}
		for i in 0..outputs.length {
			assignment[outputs[i].getWireId()] = if inVal.testBit(i)  {BigInteger.ONE}
					else {BigInteger.ZERO};
		}
	}

	
	public boolean equals(Object obj) {

		if this == obj
			return true;
		if !(obj instanceof SplitBasicOp) {
			return false;
		}
		SplitBasicOp op = (SplitBasicOp) obj;
		return inputs[0].equals(op.inputs[0]) && outputs.length == op.outputs.length;

	}
	
	
	public int getNumMulGates() {
		return outputs.length + 1;
	}

}
