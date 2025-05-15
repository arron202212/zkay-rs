use circuit::config::config;
use circuit::structure::wire;

pub struct AssertBasicOp;
pub fn newAssertBasicOp(w1: Wire, w2: Wire, output: Wire, desc: Vec<String>) -> Op<AssertBasicOp> {
    Op::<AssertBasicOp> {
        inputs: vec![w1, w2],
        outputs: vec![output],
        desc,
        t: AssertBasicOp,
    }
}

impl BasicOp for AssertBasicOp {
    fn compute(&self, assignment: Vec<BigInteger>) {
        let leftSide = assignment[self.inputs[0].getWireId()]
            .multiply(assignment[self.inputs[1].getWireId()])
            .modulo(Config.FIELD_PRIME);
        let rightSide = assignment[self.outputs[0].getWireId()];
        let check = leftSide.equals(rightSide);
        if !check {
            println!("Error - Assertion Failed {self:?}");
            println!(
                "{} * {} != {}",
                assignment[self.inputs[1].getWireId()],
                assignment[self.inputs[0].getWireId()],
                assignment[self.outputs[0].getWireId()]
            );
            panic!("Error During Evaluation");
        }
    }

    fn checkOutputs(assignment: Vec<BigInteger>) {
        // do nothing
    }

    fn getOpcode(&self) -> String {
        return "assert";
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;

        let check1 =
            self.inputs[0].equals(op.self.inputs[0]) && self.inputs[1].equals(op.self.inputs[1]);
        let check2 =
            self.inputs[1].equals(op.self.inputs[0]) && self.inputs[0].equals(op.self.inputs[1]);
        return (check1 || check2) && self.outputs[0].equals(op.self.outputs[0]);
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}
