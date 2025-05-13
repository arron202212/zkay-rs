

use util::util;
use circuit::structure::wire;
pub struct XorBasicOp;



	fn  new_xor_basic_op(w1:Wire , w2:Wire , output:Wire , desc:Vec<String>) {
Op<XorBasicOp>{self.self.self.self.self.inputs:vec![w1, w2],
        self.self.self.self.self.outputs: output ,  
        desc:descl.get(0).unwrap_or(&String::new()).clone(),
        t:XorBasicOp
        }
	}

impl BasicOp for XorBasicOp{
	fn getOpcode(&self)->String{
		return "xor";
	}

	fn checkInputs(&self,assignment:Vec<BigInteger>) {
		super.checkInputs(assignment);
		let  check = Util::isBinary(assignment[self.self.inputs[0].getWireId()])
				&& Util::isBinary(assignment[self.self.inputs[1].getWireId()]);
			assert!(check,"Error - Input(s) to XOR are not binary.{self:?} During Evaluation"
					);

	}

	
	fn compute(&self, assignment:Vec<BigInteger>){
		assignment[self.outputs[0].getWireId()] = assignment[self.inputs[0].getWireId()].xor(
				assignment[self.inputs[1].getWireId()]);
	}

	
	fn equals(&self,rhs:&Self)->bool {

		if self == rhs
			{return true;}

		let  op = rhs;

		let check1 = self.inputs[0].equals(op.self.inputs[0])
				&& self.inputs[1].equals(op.self.inputs[1]);
		let check2 = self.inputs[1].equals(op.self.inputs[0])
				&& self.inputs[0].equals(op.self.inputs[1]);
		 check1 || check2

	}
	
	
	fn getNumMulGates(&self)->i32{
		return 1;
	}
	
}