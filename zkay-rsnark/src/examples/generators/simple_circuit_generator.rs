use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{CGConfig,CircuitGenerator,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;

pub struct SimpleCircuitGenerator {
    inputs: Vec<Option<WireType>>,
}
impl SimpleCircuitGenerator {
    pub fn new(circuitName: String) -> Self {
        super(circuitName);
    }
}
impl CircuitGenerator for SimpleCircuitGenerator {
    fn buildCircuit() {
        // declare input array of length 4.
        inputs = createInputWireArray(4);

        // r1 = in0 * in1
        let r1 = inputs[0].mul(inputs[1]);

        // r2 = in2 + in3
        let r2 = inputs[2].add(inputs[3]);

        // result = (r1+5)*(6*r2)
        let result = r1.add(5).mul(r2.mul(6));

        // mark the wire as output
        makeOutput(result);
    }

    pub fn generateSampleInput(circuitEvaluator: CircuitEvaluator) {
        for i in 0..4 {
            circuitEvaluator.setWireValue(inputs[i], i + 1);
        }
    }
}

pub fn main(args: Vec<String>) {
    let mut generator = SimpleCircuitGenerator::new("simple_example");
    generator.generateCircuit();
    generator.evalCircuit();
    generator.prepFiles();
    generator.runLibsnark();
}
