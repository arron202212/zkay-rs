use circuit::eval::instruction;
use circuit::operations::primitive::add_basic_op;
use circuit::operations::primitive::const_mul_basic_op;
use circuit::operations::primitive::mul_basic_op;
use circuit::operations::primitive::orbasic_op;
use circuit::operations::primitive::xor_basic_op;

pub struct BitWire;

impl BitWire {
    // pub  BitWire(wireId:i32 ) {
    // 	super(wireId);
    // }

    pub fn mul(w: Wire, desc: String) -> Wire {
        if w.instanceof(ConstantWire) {
            return self.mul(w.getConstant(), desc);
        }
        let output = if w.instanceof(BitWire) {
            VariableBitWire::new(self.generator.currentWireId)
        } else {
            output = VariableWire::new(self.generator.currentWireId)
        };
        self.generator.currentWireId += 1;
        let op = MulBasicOp::new(self, w, output, desc);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            return cachedOutputs[0];
        }
        output
    }

    pub fn mul(b: BigInteger, desc: Vec<String>) -> Wire {
        if b.equals(BigInteger.ZERO) {
            return self.generator.zeroWire;
        } else if b.equals(BigInteger.ONE) {
            return self;
        }
        let out = LinearCombinationWire::new(self.generator.currentWireId += 1);
        let op = ConstMulBasicOp::new(self, out, b, desc);
        //			self.generator.addToEvaluationQueue(op);
        //			return out;
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn invAsBit(desc: Vec<String>) -> Wire {
        //		Wire neg = Wire::new(self.generator.currentWireId+=1);
        //		Instruction op = ConstMulBasicOp::new(self, neg, -1, desc);
        //		self.generator.addToEvaluationQueue(op);
        let neg = self.mul(-1, desc);
        let out = LinearCombinationBitWire::new(self.generator.currentWireId);
        self.generator.currentWireId += 1;
        let op = AddBasicOp::new(vec![self.generator.oneWire, neg], out, desc);
        //		self.generator.addToEvaluationQueue(op);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn or(w: Wire, desc: Vec<String>) -> Wire {
        if w.instanceof(ConstantWire) {
            return w.or(self, desc);
        }
        if w.instanceof(BitWire) {
            let out = VariableBitWire::new(self.generator.currentWireId);
            self.generator.currentWireId += 1;
            let op = ORBasicOp::new(self, w, out, desc);
            let cachedOutputs = self.generator.addToEvaluationQueue(op);
            return if let Some(cachedOutputs) = cachedOutputs {
                self.generator.currentWireId -= 1;
                cachedOutputs[0].clone()
            } else {
                out
            };
        }
        return super.or(w, desc);
    }

    pub fn xor(w: Wire, desc: Vec<String>) -> Wire {
        if w.instanceof(ConstantWire) {
            return w.xor(self, desc);
        }
        if w.instanceof(BitWire) {
            let out = VariableBitWire::new(self.generator.currentWireId);
            self.generator.currentWireId += 1;
            let op = XorBasicOp::new(self, w, out, desc);
            let cachedOutputs = self.generator.addToEvaluationQueue(op);
            return if let Some(cachedOutputs) = cachedOutputs {
                self.generator.currentWireId -= 1;
                cachedOutputs[0].clone()
            } else {
                out
            };
        }
        super.xor(w, desc)
    }

    pub fn getBits(w: Wire, bitwidth: i32, desc: Vec<String>) -> WireArray {
        return WireArray::new(vec![self]).adjustLength(bitwidth);
    }
}
