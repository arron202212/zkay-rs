use circuit::config::config;
use circuit::structure::wire;

pub struct SplitBasicOp;
pub fn newSplitBasicOp(w: Wire, outs: Vec<Wire>, desc: Vec<String>) -> Op<SplitBasicOp> {
    Op::<SplitBasicOp> {
        inputs: vec![w],
        outputs: outs,
        desc: descl.get(0).unwrap_or(&String::new()).clone(),
        t: SplitBasicOp,
    }
}
impl BasicOp for Op<SplitBasicOp> {
    fn getOpcode(&self) -> String {
        return "split";
    }

    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        super.checkInputs(assignment);
        assert!(
            self.outputs.len() >= assignment[self.inputs[0].getWireId()].bitLength(),
            "Error in Split --- The number of bits does not fit -- Input: {:x},{self:?}\n\t",
            assignment[self.inputs[0].getWireId()]
        );
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        let mut inVal = assignment[self.inputs[0].getWireId()];
        if inVal.compareTo(Config.FIELD_PRIME) > 0 {
            inVal = inVal.modulo(Config.FIELD_PRIME);
        }
        for i in 0..self.outputs.length {
            assignment[self.outputs[i].getWireId()] = if inVal.testBit(i) {
                BigInteger.ONE
            } else {
                BigInteger.ZERO
            };
        }
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        self.inputs[0].equals(op.self.inputs[0]) && self.outputs.length == op.self.outputs.length
    }

    fn getNumMulGates(&self) -> i32 {
        self.outputs.len() as i32 + 1
    }
}
