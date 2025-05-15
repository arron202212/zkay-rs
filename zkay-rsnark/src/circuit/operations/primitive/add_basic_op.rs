use circuit::config::config;
use circuit::structure::wire;

pub struct AddBasicOp;

pub fn NewAddBasicOp(ws: Vec<Wire>, output: Wire, desc: Vec<String>) -> Op<AddBasicOp> {
    Op::<AddBasicOp> {
        inputs: ws,
        outputs: vec![output],
        desc,
        t: AddBasicOp,
    }
}

impl<T> BasicOp for Op<AddBasicOp> {
    fn getOpcode(&self) -> String {
        return "add";
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        let mut s = BigInteger.ZERO;
        for w in self.inputs {
            s = s.add(assignment[w.getWireId()]);
        }
        assignment[self.outputs[0].getWireId()] = s.modulo(Config.FIELD_PRIME);
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        if op.inputs.len() != self.inputs.len() {
            return false;
        }

        if self.inputs.len() == 2 {
            let check1 = self.inputs[0].equals(op.inputs[0]) && self.inputs[1].equals(op.inputs[1]);
            let check2 = self.inputs[1].equals(op.inputs[0]) && self.inputs[0].equals(op.inputs[1]);
            return check1 || check2;
        }

        self.inputs.iter().zip(&op.inputs).all(|(a, b)| a.equals(b))
    }

    fn getNumMulGates(&self) -> i32 {
        0
    }
}
