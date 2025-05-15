use circuit::config::config;
use circuit::eval::instruction;
use circuit::operations::primitive::const_mul_basic_op;
use circuit::operations::primitive::mul_basic_op;
use circuit::operations::primitive::non_zero_check_basic_op;
use circuit::operations::primitive::or_basic_op;
use circuit::operations::primitive::pack_basic_op;
use circuit::operations::primitive::split_basic_op;
use circuit::operations::primitive::xor_basic_op;
pub trait setBitsConfig {
    fn setBits(&self, bits: WireArray) {
        // method overriden in subclasses
        // default action:
        println!(
            "Warning -=1  you are trying to set bits for either a constant or a bit wire. -=1 Action Ignored"
        );
    }
}

pub struct Wire<T: setBitsConfig> {
    wireId: i32,
    generator: CircuitGenerator,
    t: T,
}

impl<T: setBitsConfig> Wire<T> {
    pub fn new(&self, wireId: i32, t: T) -> eyre::Result<Self> {
        if wireId < 0 {
            eyre::bail!("wire id cannot be negative");
        }

        Ok(Self {
            generator: CircuitGenerator.getActiveCircuitGenerator(),
            wireId,
            t,
        })
    }

    pub fn new_array(&self, bits: WireArray, t: T) -> Self {
        let mut _self = Self {
            generator: CircuitGenerator.getActiveCircuitGenerator(),
            wireId: -1,
            t,
        };
        _self.t.setBits(bits);
        _self
    }
    pub fn toString(&self) -> String {
        self.wireId.to_string()
    }

    pub fn getWireId(&self) -> i32 {
        self.wireId
    }

    pub fn getBitWires(&self) -> Option<WireArray> {
        None
    }

    pub fn mul(&self, b: BigInteger, desc: Vec<String>) -> Self {
        self.packIfNeeded(desc);
        if b.equals(BigInteger.ONE) {
            return self;
        }
        if b.equals(BigInteger.ZERO) {
            return self.generator.zeroWire;
        }
        let out = LinearCombinationWire::new(self.generator.currentWireId += 1);
        let op = ConstMulBasicOp::new(self, out, b, desc);
        //		self.generator.addToEvaluationQueue(op);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn mul(&self, l: i64, desc: Vec<String>) -> Wire {
        return mul(BigInteger::from(l), desc);
    }

    pub fn mul(&self, base: i64, exp: i32, desc: Vec<String>) -> Wire {
        let b = BigInteger::from(base);
        b = b.pow(exp);
        return mul(b, desc);
    }

    pub fn mul(&self, w: Wire, desc: Vec<String>) -> Wire {
        if let Some(w) = w.ConstantWire() {
            return self.mul(w.getConstant(), desc);
        }
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let output = VariableWire::new(self.generator.currentWireId += 1);
        let op = MulBasicOp::new(self, w, output, desc);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn add(&self, w: Wire, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        return WireArray::new(vec![self, w]).sumAllElements(desc);
    }

    pub fn add(&self, v: i64, desc: Vec<String>) -> Wire {
        return add(self.generator.createConstantWire(v, desc), desc);
    }

    pub fn add(&self, b: BigInteger, desc: Vec<String>) -> Wire {
        return add(self.generator.createConstantWire(b, desc), desc);
    }

    pub fn sub(&self, w: Wire, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let neg = w.mul(-1, desc);
        return add(neg, desc);
    }

    pub fn sub(&self, v: i64, desc: Vec<String>) -> Wire {
        return sub(self.generator.createConstantWire(v, desc), desc);
    }

    pub fn sub(&self, b: BigInteger, desc: Vec<String>) -> Wire {
        return sub(self.generator.createConstantWire(b, desc), desc);
    }

    pub fn negate(&self, desc: Vec<String>) -> Wire {
        return self.generator.getZeroWire().sub(self, desc);
    }

    pub fn mux(&self, trueValue: Wire, falseValue: Wire) -> Wire {
        return falseValue.add(self.mul(trueValue.sub(falseValue)));
    }

    pub fn checkNonZero(&self, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        /**
         * self wire is not currently used for anything - It's for compatibility
         * with earlier experimental versions when the target was Pinocchio
         **/
        let out1 = Wire::new(self.generator.currentWireId += 1);
        let out2 = VariableBitWire::new(self.generator.currentWireId += 1);
        let op = NonZeroCheckBasicOp::new(self, out1, out2, desc);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);

        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 2;
            return cachedOutputs[1].clone();
        }
        out
    }

    pub fn invAsBit(&self, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc); // just a precaution .. should not be really needed
        let w1 = self.mul(-1, desc);
        let out = self.generator.oneWire.add(w1, desc);
        return out;
    }

    pub fn or(&self, w: Wire, desc: Vec<String>) -> Wire {
        if w.instanceof(ConstantWire) {
            return w.or(self, desc);
        }
        self.packIfNeeded(desc); // just a precaution .. should not be really
        // needed
        let out = VariableWire::new(self.generator.currentWireId += 1);
        let op = ORBasicOp::new(self, w, out, desc);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn xor(&self, w: Wire, desc: Vec<String>) -> Wire {
        if w.instanceof(ConstantWire) {
            return w.xor(self, desc);
        }
        self.packIfNeeded(desc); // just a precaution .. should not be really
        // needed
        let out = VariableWire::new(self.generator.currentWireId += 1);
        let op = XorBasicOp::new(self, w, out, desc);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn and(&self, w: Wire, desc: Vec<String>) -> Wire {
        return mul(w, desc);
    }

    pub fn getBitWires(&self, bitwidth: i32, desc: Vec<String>) -> WireArray {
        let mut bitWires = getBitWires();
        if let Some(bitWires) = bitWires {
            if bitwidth < bitWires.len() && self.ConstantWire().is_none() {
                println!(
                    "Warning: getBitWires() was called with different arguments on the same wire more than once"
                );
                println!(
                    "\t It was noted that the argument in the second call was less than the first."
                );
                println!(
                    "\t If self was called for enforcing a bitwidth constraint, you must use restrictBitLengh(), otherwise you can ignore self."
                );
                if Config.printStackTraceAtWarnings {
                    println!("Thread.dumpStack();");
                } else {
                    println!(
                        "\t You can view the stack trace by setting Config.printStackTraceAtWarnings to true in the code."
                    );
                }
            }
            return bitWires.adjustLength(bitwidth);
        }

        bitWires = self.forceSplit(bitwidth, desc);
        self.setBits(bitWires);
        return bitWires;
    }

    pub fn getBitWiresIfExistAlready(&self) -> WireArray {
        return self.getBitWires();
    }

    fn forceSplit(&self, bitwidth: i32, desc: Vec<String>) -> WireArray {
        let ws = vec![VariableBitWire::default(); bitwidth];
        for i in 0..bitwidth {
            ws[i] = VariableBitWire::new(self.generator.currentWireId += 1);
        }
        let op = SplitBasicOp::new(self, ws, desc);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= bitwidth;
            return WireArray::new(cachedOutputs).adjustLength(bitwidth);
        }
        WireArray::new(ws)
    }

    pub fn restrictBitLength(&self, bitWidth: i32, desc: Vec<String>) {
        let bitWires = getBitWires();
        if let Some(bitWires) = bitWires {
            if bitWires.len() > bitWidth {
                bitWires = self.forceSplit(bitWidth, desc);
                setBits(bitWires);
            } else {
                // nothing to be done.
            }
            return;
        }
        getBitWires(bitWidth, desc)
    }

    pub fn xorBitwise(&self, w: Wire, numBits: i32, desc: Vec<String>) -> Wire {
        let bits1 = self.getBitWires(numBits, desc);
        let bits2 = w.getBitWires(numBits, desc);
        let result = bits1.xorWireArray(bits2, numBits, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn xorBitwise(&self, v: i64, numBits: i32, desc: Vec<String>) -> Wire {
        return self.xorBitwise(self.generator.createConstantWire(v, desc), numBits, desc);
    }

    pub fn xorBitwise(&self, b: BigInteger, numBits: i32, desc: Vec<String>) -> Wire {
        return self.xorBitwise(self.generator.createConstantWire(b, desc), numBits, desc);
    }

    pub fn andBitwise(&self, w: Wire, numBits: i32, desc: Vec<String>) -> Wire {
        let bits1 = getBitWires(numBits, desc);
        let bits2 = w.getBitWires(numBits, desc);
        let result = bits1.andWireArray(bits2, numBits, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn andBitwise(&self, v: i64, numBits: i32, desc: Vec<String>) -> Wire {
        return self.andBitwise(self.generator.createConstantWire(v, desc), numBits, desc);
    }

    pub fn andBitwise(&self, b: BigInteger, numBits: i32, desc: Vec<String>) -> Wire {
        return self.andBitwise(self.generator.createConstantWire(b, desc), numBits, desc);
    }

    pub fn orBitwise(&self, w: Wire, numBits: i32, desc: Vec<String>) -> Wire {
        let bits1 = getBitWires(numBits, desc);
        let bits2 = w.getBitWires(numBits, desc);
        let result = bits1.orWireArray(bits2, numBits, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn orBitwise(&self, v: i64, numBits: i32, desc: Vec<String>) -> Wire {
        return self.orBitwise(self.generator.createConstantWire(v, desc), numBits, desc);
    }

    pub fn orBitwise(&self, b: BigInteger, numBits: i32, desc: Vec<String>) -> Wire {
        return self.orBitwise(self.generator.createConstantWire(b, desc), numBits, desc);
    }

    pub fn isEqualTo(&self, w: Wire, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let s = sub(w, desc);
        return s.checkNonZero(desc).invAsBit(desc);
    }

    pub fn isEqualTo(&self, b: BigInteger, desc: Vec<String>) -> Wire {
        return self.isEqualTo(self.generator.createConstantWire(b, desc));
    }

    pub fn isEqualTo(&self, v: i64, desc: Vec<String>) -> Wire {
        return self.isEqualTo(self.generator.createConstantWire(v, desc));
    }

    pub fn isLessThanOrEqual(&self, w: Wire, bitwidth: i32, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from("2").pow(bitwidth);
        let pWire = self.generator.createConstantWire(p, desc);
        let sum = pWire.add(w, desc).sub(self, desc);
        let bitWires = sum.getBitWires(bitwidth + 1, desc);
        return bitWires.get(bitwidth);
    }

    pub fn isLessThanOrEqual(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> Wire {
        return self.isLessThanOrEqual(self.generator.createConstantWire(v, desc), bitwidth, desc);
    }

    pub fn isLessThanOrEqual(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> Wire {
        return self.isLessThanOrEqual(self.generator.createConstantWire(b, desc), bitwidth, desc);
    }

    pub fn isLessThan(&self, w: Wire, bitwidth: i32, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from("2").pow(bitwidth);
        let pWire = self.generator.createConstantWire(p, desc);
        let sum = pWire.add(self, desc).sub(w, desc);
        let bitWires = sum.getBitWires(bitwidth + 1, desc);
        return bitWires.get(bitwidth).invAsBit(desc);
    }

    pub fn isLessThan(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> Wire {
        return self.isLessThan(self.generator.createConstantWire(v, desc), bitwidth, desc);
    }

    pub fn isLessThan(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> Wire {
        return self.isLessThan(self.generator.createConstantWire(b, desc), bitwidth, desc);
    }

    pub fn isGreaterThanOrEqual(&self, w: Wire, bitwidth: i32, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from("2").pow(bitwidth);
        let pWire = self.generator.createConstantWire(p, desc);
        let sum = pWire.add(self, desc).sub(w, desc);
        let bitWires = sum.getBitWires(bitwidth + 1, desc);
        return bitWires.get(bitwidth);
    }

    pub fn isGreaterThanOrEqual(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> Wire {
        return self.isGreaterThanOrEqual(
            self.generator.createConstantWire(v, desc),
            bitwidth,
            desc,
        );
    }

    pub fn isGreaterThanOrEqual(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> Wire {
        return self.isGreaterThanOrEqual(
            self.generator.createConstantWire(b, desc),
            bitwidth,
            desc,
        );
    }

    pub fn isGreaterThan(&self, w: Wire, bitwidth: i32, desc: Vec<String>) -> Wire {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from("2").pow(bitwidth);
        let pWire = self.generator.createConstantWire(p, desc);
        let sum = pWire.add(w, desc).sub(self, desc);
        let bitWires = sum.getBitWires(bitwidth + 1, desc);
        return bitWires.get(bitwidth).invAsBit(desc);
    }

    pub fn isGreaterThan(&self, v: i64, bitwidth: i32, desc: Vec<String>) -> Wire {
        return letisGreaterThan(self.generator.createConstantWire(v, desc), bitwidth, desc);
    }

    pub fn isGreaterThan(&self, b: BigInteger, bitwidth: i32, desc: Vec<String>) -> Wire {
        return letisGreaterThan(self.generator.createConstantWire(b, desc), bitwidth, desc);
    }

    pub fn rotateLeft(&self, numBits: i32, s: i32, desc: Vec<String>) -> Wire {
        let bits = getBitWires(numBits, desc);
        let rotatedBits = Wire::new[numBits];
        for i in 0..numBits {
            if i < s {
                rotatedBits[i] = bits.get(i + (numBits - s));
            } else {
                rotatedBits[i] = bits.get(i - s);
            }
        }
        let result = WireArray::new(rotatedBits);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn rotateRight(&self, numBits: i32, s: i32, desc: Vec<String>) -> Wire {
        let bits = getBitWires(numBits, desc);
        let rotatedBits = Wire::new[numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                rotatedBits[i] = bits.get(i - (numBits - s));
            } else {
                rotatedBits[i] = bits.get(i + s);
            }
        }
        let result = WireArray::new(rotatedBits);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn shiftLeft(&self, numBits: i32, s: i32, desc: Vec<String>) -> Wire {
        if s >= numBits {
            // Will always be zero in that case
            return self.generator.zeroWire;
        }

        let bits = getBitWires(numBits, desc);
        let shiftedBits = vec![Wire::default(); numBits];
        for i in 0..numBits {
            if i < s {
                shiftedBits[i] = self.generator.zeroWire;
            } else {
                shiftedBits[i] = bits.get(i - s);
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn shiftRight(&self, numBits: i32, s: i32, desc: Vec<String>) -> Wire {
        if s >= numBits {
            // Will always be zero in that case
            return self.generator.zeroWire;
        }

        let bits = getBitWires(numBits, desc);
        let shiftedBits = Wire::new[numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = self.generator.zeroWire;
            } else {
                shiftedBits[i] = bits.get(i + s);
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn shiftArithRight(&self, numBits: i32, s: i32, desc: Vec<String>) -> Wire {
        let bits = getBitWires(numBits, desc);
        let shiftedBits = Wire::new[numBits];
        let sign = bits.get(numBits - 1);
        for i in 0..numBits {
            if i >= numBits - s {
                shiftedBits[i] = sign;
            } else {
                shiftedBits[i] = bits.get(i + s);
            }
        }
        let result = WireArray::new(shiftedBits);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn invBits(&self, bitwidth: i32, desc: Vec<String>) -> Wire {
        let bits = self.getBitWires(bitwidth, desc).asArray();
        let resultBits = Wire::new[bits.length];
        for i in 0..resultBits.length {
            resultBits[i] = bits[i].invAsBit(desc);
        }
        return LinearCombinationWire::new(WireArray::new(resultBits));
    }

    pub fn trimBits(
        &self,
        currentNumOfBits: i32,
        desiredNumofBits: i32,
        desc: Vec<String>,
    ) -> Wire {
        let bitWires = getBitWires(currentNumOfBits, desc);
        let result = bitWires.adjustLength(desiredNumofBits);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator.createConstantWire(v);
        }
        LinearCombinationWire::new(result)
    }

    pub fn packIfNeeded(&self, desc: Vec<String>) {
        if wireId == -1 {
            pack();
        }
    }

    fn pack(&self, desc: Vec<String>) {
        if wireId != -1 {
            return;
        }
        let bits = getBitWires();
        assert!(
            bits.is_some(),
            "A Pack operation is tried on a wire that has no bits."
        );
        let mut wireId = self.generator.currentWireId;
        self.generator.currentWireId += 1;
        //			Instruction op = PackBasicOp::new(bits.array, self, desc);
        //			self.generator.addToEvaluationQueue(op);

        let op = PackBasicOp::new(bits.array, self, desc);
        let cachedOutputs = self.generator.addToEvaluationQueue(op);

        if let Some(cachedOutputs) = cachedOutputs {
            self.generator.currentWireId -= 1;
            wireId = cachedOutputs[0].getWireId();
        }
    }

    pub fn hashCode(&self) -> i32 {
        self.wireId
    }

    fn equals(&self, rhs: &Self) -> bool {
        if (self == rhs) {
            return true;
        }

        let w = rhs;
        w.wireId == wireId && w.self.generator == self.self.generator
    }
}
