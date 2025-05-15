use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::structure::wire;
use util::util;

struct Op<T> {
    inputs: Vec<Wire>,
    outputs: Vec<Wire>,
    desc: String,
    t: T,
}
impl<T> Op<T> {
    fn new(inputs: Vec<Wire>, outputs: Vec<Wire>, desc: Vec<String>) -> eyre::Result<Self> {
        self.set_inputs(inputs);
        self.set_outputs(outputs);
        if desc.len() > 0 {
            self.set_desc(desc[0].clone());
        } else {
            self.set_desc(String::new());
        }

        for w in inputs {
            if w.is_none() {
                println!("One of the input wires is null: {self:?}");
                eyre::bail!("A null wire");
            } else if w.getWireId() == -1 {
                println!("One of the input wires is not packed: {self:?}");
                eyre::bail!("A wire with a negative id");
            }
        }
        for w in outputs {
            if w.is_none() {
                println!("One of the output wires is null {self:?}");
                eyre::bail!("A null wire");
            }
        }
        Ok(Self {})
    }
}
pub trait BasicOp: Instruction {
    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        for w in inputs {
            if assignment[w.getWireId()].is_none() {
                println!("Error - The inWire {w } has not been assigned {self:?}\n");
                panic!("Error During Evaluation");
            }
        }
    }

    fn compute(&self, assignment: Vec<BigInteger>);

    fn checkOutputs(assignment: Vec<BigInteger>) {
        for w in outputs {
            if assignment[w.getWireId()].is_some() {
                println!("Error - The outWire {w} has already been assigned {self:?}\n");
                panic!("Error During Evaluation");
            }
        }
    }

    fn getOpcode(&self) -> String;
    fn getNumMulGates(&self) -> i32;

    fn toString(&self) -> String {
        format!("{} in {} <{}> out  <{}> {}",getOpcode(),inputs.length,Util::arrayToString(inputs, " "),outputs.length,Util::arrayToString(outputs, " "),desc.length() > 0  { " \t\t# ".to_owned() + desc }else {String::new()} )
    }

    fn getInputs(&self) -> Vec<Wire> {
        self.inputs.clone()
    }

    fn getOutputs(&self) -> Vec<Wire> {
        self.outputs.clone()
    }

    fn doneWithinCircuit(&self) -> bool {
        true
    }

    fn hashCode(&self) -> i32 {
        // this method should be overriden when a subclass can have more than one opcode, or have other arguments
        let mut h = getOpcode().hashCode();
        for i in inputs {
            h += i.hashCode();
        }
        h
    }

    fn equals(&self, rhs: &Self) -> bool {
        self == rhs
        // logic moved to subclasses
    }
}
impl<T: BasicOp> Instruction for T {
    fn evaluate(&self, evaluator: CircuitEvaluator) {
        let assignment = evaluator.getAssignment();
        self.checkInputs(assignment);
        self.checkOutputs(assignment);
        self.compute(assignment);
    }
}
