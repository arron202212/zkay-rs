

use circuit::config::config;
use circuit::structure::wire;
use util::util;

pub struct ConstMulBasicOp {
	  constInteger:BigInteger;
	 inSign:bool;
}

	pub fn newConstMulBasicOp(w:Wire , out:Wire , mut constInteger:BigInteger ,
			desc:Vec<String>)->Op<ConstMulBasicOp> {
		let inSign = constInteger.signum() == -1;
		if !inSign {
			constInteger = Util::mod(constInteger, Config.FIELD_PRIME);
		} else {
			let mut _constInteger = constInteger.negate();
			_constInteger = Util::mod(_constInteger, Config.FIELD_PRIME);
			constInteger = Config.FIELD_PRIME.subtract(_constInteger);
		}
         Op<ConstMulBasicOp>{self.inputs:vec![w ],
        self.outputs: vec![out] ,  
        desc:desc[0].clone(),
        t:ConstMulBasicOp{constInteger,inSign}
        }
	}
impl   BasicOp for Op<ConstMulBasicOp>{


	fn getOpcode(&self)->String{
		if !inSign {
			fomrat!( "const-mul-{:x}",constInteger)
		} else{
			fomrat!( "const-mul-neg-{:x}",Config.FIELD_PRIME.subtract(constInteger))
		}
	}
	
	
	fn compute(&self, assignment:Vec<BigInteger>){
		let mut  result = assignment[self.inputs[0].getWireId()].multiply(self.t.constInteger);
		if result.bitLength() >= Config.LOG2_FIELD_PRIME {
			result = result.mod(Config.FIELD_PRIME);
		}
		assignment[self.outputs[0].getWireId()] = result;
	}
	
	
	fn equals(&self,rhs:&Self)->bool {
		if self == rhs
			{return true;}
		let  op =  rhs;
		self.inputs[0].equals(op.inputs[0]) && self.t.constInteger.equals(op.t.constInteger)
	}
	
	
	fn getNumMulGates(&self)->i32{
		return 0;
	}

	fn hashCode(&self)->i32{
		let mut  h = self.t.constInteger.hashCode();
		for i in inputs{
			h+=i.hashCode();
		}
		h
	}
	
	
}