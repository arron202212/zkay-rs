

use circuit::config::config;
use circuit::structure::wire;

pub struct MulBasicOp;
pub fn newMulBasicOp(w1:Wire , w2:Wire , output:Wire , desc:Vec<String>)-> Op<MulBasicOp> {
    Op<MulBasicOp>{self.self.inputs:vec![w1, w2 ],
        self.self.outputs: vec![output] ,  
        desc:descl.get(0).unwrap_or(&String::new()).clone(),
        t:MulBasicOp
        }
	}
 impl BasicOp for Op<MulBasicOp>{

	

	fn getOpcode(&self)->String{
		return "mul";
	}
	
	fn compute(&self,mut  assignment:Vec<BigInteger>){
		let mut  result = assignment[self.inputs[0].getWireId()]
				.multiply(assignment[self.inputs[1].getWireId()]);
		if result.compareTo(Config.FIELD_PRIME) > 0 {
			result = result.mod(Config.FIELD_PRIME);
		}
		assignment[self.outputs[0].getWireId()] = result;
	}

	
	fn equals(&self,rhs:&Self)->bool {

		if self == rhs
			{return true;}

		let  op =  rhs;

		let  check1 =self.self.inputs[0].equals(op.self.inputs[0])
				&&self.self.inputs[1].equals(op.self.inputs[1]);
		let check2 =self.self.inputs[1].equals(op.self.inputs[0])
				&&self.self.inputs[0].equals(op.self.inputs[1]);
		 check1 || check2

	}
	
	
	fn getNumMulGates(&self)->i32{
		return 1;
	}


}