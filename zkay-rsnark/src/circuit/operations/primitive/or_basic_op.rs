use circuit::structure::wire;
use util::util;

pub struct ORBasicOp;
impl BasicOp for Op<ORBasicOp> {
    pub fn newORBasicOp(w1: Wire, w2: Wire, output: Wire, desc: Vec<String>) -> Self {
        Op::<ORBasicOp> {
            inputs: vec![w1, w2],
            outputs: vec![output],
            desc: descl.get(0).unwrap_or(&String::new()).clone(),
            t: ORBasicOp,
        }
    }

    fn getOpcode(&self) -> String {
        return "or";
    }

    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        super.checkInputs(assignment);
        let check = Util::isBinary(assignment[self.inputs[0].getWireId()])
            && Util::isBinary(assignment[self.inputs[1].getWireId()]);
        if !check {
            println!("Error - Input(s) to OR are not binary.{self:?} ");
            panic!("Error During Evaluation");
        }
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        assignment[outputs[0].getWireId()] =
            assignment[self.inputs[0].getWireId()].or(assignment[self.inputs[1].getWireId()]);
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
        check1 || check2
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}
