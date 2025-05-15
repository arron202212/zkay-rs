use circuit::structure::wire;

pub struct NonZeroCheckBasicOp;
fn newNonZeroCheckBasicOp(
    w: Wire,
    out1: Wire,
    out2: Wire,
    desc: Vec<String>,
) -> Op<NonZeroCheckBasicOp> {
    Op::<NonZeroCheckBasicOp> {
        inputs: vec![w],
        outputs: vec![out1, out2],
        desc: descl.get(0).unwrap_or(&String::new()).clone(),
        t: NonZeroCheckBasicOp,
    }
}
impl BasicOp for Op<NonZeroCheckBasicOp> {
    fn getOpcode(&self) -> String {
        return "zerop";
    }

    fn compute(&self, mut assignment: Vec<BigInteger>) {
        if assignment[self.inputs[0].getWireId()].signum() == 0 {
            assignment[self.outputs[1].getWireId()] = BigInteger.ZERO;
        } else {
            assignment[self.outputs[1].getWireId()] = BigInteger.ONE;
        }
        assignment[self.outputs[0].getWireId()] = BigInteger.ZERO; // a dummy value
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        self.inputs[0].equals(op.self.inputs[0])
    }

    fn getNumMulGates(&self) -> i32 {
        return 2;
    }
}
