use circuit::config::config;
use circuit::structure::wire;
use util::util;

pub struct PackBasicOp;
pub fn newPackBasicOp(inBits: Vec<Wire>, out: Wire, desc: Vec<String>) -> Self {
    Op::<PackBasicOp> {
        inputs: inBits,
        outputs: vec![out],
        desc: descl.get(0).unwrap_or(&String::new()).clone(),
        t: PackBasicOp,
    }
}
impl BasicOp for Op<PackBasicOp> {
    fn getOpcode(&self) -> String {
        return "pack";
    }

    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        super.checkInputs(assignment);

        assert!(
            (0..self.inputs.length).all(|i| Util::isBinary(assignment[self.inputs[i].getWireId()])),
            "Error - Input(s) to Pack are not binary.{self:?} During Evaluation "
        );
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        let mut sum = BigInteger.ZERO;
        for i in 0..self.inputs.length {
            sum = sum
                .add(assignment[self.inputs[i].getWireId()].multiply(BigInteger::new("2").pow(i)));
        }
        assignment[self.outputs[0].getWireId()] = sum.modulo(Config.FIELD_PRIME);
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = obj;
        if op.self.inputs.length != self.inputs.length {
            return false;
        }

        let mut check = true;
        for i in 0..self.inputs.length {
            check = check && self.inputs[i].equals(op.inputs[i]);
        }
        check
    }

    fn getNumMulGates(&self) -> i32 {
        return 0;
    }
}
