

use circuit::config::config;
use circuit::eval::instruction;
use circuit::operations::primitive::const_mul_basic_op;

pub struct ConstantWire {
	  constant:BigInteger;
}
impl ConstantWire{
	pub fn new(wireId:i32 , value:BigInteger ) ->Self {
		super(wireId);
		constant = value.mod(Config.FIELD_PRIME);
	}
	
	pub  fn getConstant()-> BigInteger {
		return constant;
	}

	pub  fn isBinary()-> bool {
		return constant.equals(BigInteger.ONE)
				|| constant.equals(BigInteger.ZERO);
	}

	pub  fn mul(w:Wire , desc:Vec<String>)-> Wire {
		if w.instanceof( ConstantWire) {
			return self.generator.createConstantWire(
					constant.multiply(w.constant), desc);
		} else {
			return w.mul(constant, desc);
		}
	}

	pub  fn mul(b:BigInteger , desc:Vec<String>)-> Wire {
		Wire out;
let sign =  b.signum() == -1;
let newConstant =  constant.multiply(b).mod(Config.FIELD_PRIME);
		 	
		let mut out = self.generator.knownConstantWires.get(newConstant);
		if out.is_some() {
			return out;
        }


		out = ConstantWire::new(self.generator.currentWireId, if !sign{newConstant}else{newConstant.subtract(Config.FIELD_PRIME)});
		
self.generator.currentWireId+=1;
let op =  ConstMulBasicOp::new(self, out,
					b, desc);
let cachedOutputs =  self.generator.addToEvaluationQueue(op);
			if let Some(cachedOutputs) = cachedOutputs{
				// self branch might not be needed
				self.generator.currentWireId-=1;
				return cachedOutputs[0];
			}
			
		self.generator.knownConstantWires.put(newConstant, out);
				 out
		
	}

	pub  fn checkNonZero(w:Wire , desc:Vec<String>)-> Wire {
		if constant.equals(BigInteger.ZERO) {
			return self.generator.zeroWire;
		} else {
			return self.generator.oneWire;
		}
	}

	pub  fn invAsBit(desc:Vec<String>)-> Wire {
			assert!(self.isBinary(),
					"Trying to invert a non-binary constant!");
		
		if constant.equals(BigInteger.ZERO) {
			 self.generator.oneWire
		} else {
			 self.generator.zeroWire
		}
	}

	pub  fn or(w:Wire , desc:Vec<String>)-> Wire {
		if let Some(cw)=w.ConstantWire() {
                assert!(self.isBinary() && cw.isBinary() ,
						"Trying to OR two non-binary constants");
			return	if constant.equals(BigInteger.ZERO
						&& cw.getConstant().equals(BigInteger.ZERO)) {
					 self.generator.zeroWire
				} else {
					 self.generator.oneWire
				}
		} 
			if constant.equals(BigInteger.ONE) {
				 self.generator.oneWire
			} else {
				 w
			}
		
	}

	pub  fn xor(w:Wire , desc:Vec<String>)-> Wire {
		if let Some(cw)=w.ConstantWire() {
            assert!(isBinary() && cw.isBinary(),
						"Trying to XOR two non-binary constants");
				return if constant.equals(cw.getConstant()) {
					 self.generator.zeroWire
				} else {
					 self.generator.oneWire
				}
		} 
			if constant.equals(BigInteger.ONE) {
				 w.invAsBit(desc)
			} else {
				 w
			}
		
	}

	pub  fn getBitWires(bitwidth:i32 , desc:Vec<String>)-> WireArray {
			assert!(constant.bitLength() <= bitwidth,"Trying to split a constant of {} bits into  {bitwidth} bits",constant.bitLength() );
let mut bits =  vec![ConstantWire::default();bitwidth];
			for i in 0..bitwidth {
				bits[i] = constant.testBit(i)  { self.generator.oneWire }else { self.generator.zeroWire};
			}
			return WireArray::new(bits);
		
	}
	
	pub  fn restrictBitLength(bitwidth:i32 , desc:Vec<String>) {
		getBitWires(bitwidth, desc);
	}
	
	fn pack(desc:Vec<String>){
	}
	
}
