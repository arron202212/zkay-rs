use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::math::dot_product_gadget;

pub struct DotProductCircuitGenerator {
    a: Vec<Wire>,
    b: Vec<Wire>,
    dimension: i32,
}
impl DotProductCircuitGenerator {
    pub fn new(circuitName: String, dimension: i32) -> Self {
        super(circuitName);
        self.dimension = dimension;
    }
}
impl CircuitGenerator for DotProductCircuitGenerator {
    fn buildCircuit() {
        a = createInputWireArray(dimension, "Input a");
        b = createInputWireArray(dimension, "Input b");

        let dotProductGadget = DotProductGadget::new(a, b);
        let result = dotProductGadget.getOutputWires();
        makeOutput(result[0], "output of dot product a, b");
    }

    pub fn generateSampleInput(circuitEvaluator: CircuitEvaluator) {
        for i in 0..dimension {
            circuitEvaluator.setWireValue(a[i], 10 + i);
            circuitEvaluator.setWireValue(b[i], 20 + i);
        }
    }

    pub fn main(args: Vec<String>) {
        let generator = DotProductCircuitGenerator::new("dot_product", 3);
        generator.generateCircuit();
        generator.evalCircuit();
        generator.prepFiles();
        generator.runLibsnark();
    }
}
