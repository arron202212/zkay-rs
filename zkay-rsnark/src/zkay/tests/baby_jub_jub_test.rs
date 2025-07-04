
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{addToEvaluationQueue,CGConfig,CircuitGenerator,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use zkay::zkay_baby_jub_jub_gadget;


pub struct BabyJubJubTest {
     class TestGadget extends ZkayBabyJubJubGadget {

        pub  TestGadget() {
            buildCircuit();
        }

          fn buildCircuit() {
            // check native inverse
            let a = generator.createConstantWire(BigInteger::new("11985782033876175911769025829561891428638139496693105005957757653258"));
            let ainv_expected = generator.createConstantWire(BigInteger::new("20950552912096304742729232452120498732043875737213521271262032500972060322340"));
            let ainv = nativeInverse(a);
            generator.addEqualityAssertion(ainv, ainv_expected);

            // check generator on curve
            let g_x = generator.createConstantWire(GENERATOR_X);
            let g_y = generator.createConstantWire(GENERATOR_Y);
            assertOnCurve(g_x, g_y);

            // check generator + generator on curve
            let g = getGenerator();
            let g2 = addPoints(g, g);
            assertOnCurve(g2.x, g2.y);

            // check generator - generator = INFINITY
            let gneg = negatePoint(g);
            assertOnCurve(gneg.x, gneg.y);
            let inf = addPoints(g, gneg);
            generator.addEqualityAssertion(inf.x, generator.get_zero_wire());
            generator.addEqualityAssertion(inf.y, generator.get_one_wire());

            // check generator + INFINITY = generator
            let g_expected = addPoints(g, getInfinity());
            generator.addEqualityAssertion(g_expected.x, g.x);
            generator.addEqualityAssertion(g_expected.y, g.y);

            // check scalar multiplication
            let scalar = generator.createConstantWire(5);
            let scalarBits = scalar.getBitWires(4);
            let g5 = mulScalar(g, scalarBits.asArray());
            let g5_expected = addPoints(addPoints(addPoints(addPoints(g, g), g), g), g);
            assertOnCurve(g5.x, g5.y);
            generator.addEqualityAssertion(g5.x, g5_expected.x);
            generator.addEqualityAssertion(g5.y, g5_expected.y);
        }

        
        pub  fn getOutputWires()->Vec<Option<WireType>>  {
            let dummy = generator.get_one_wire();
            vec![None;]{ dummy }
        }
    }

    
    pub   testBabyJubJubGadget() {
        CircuitGenerator cgen = CircuitGenerator::new("test") {
            
              fn buildCircuit() {
                let gadget = TestGadget::new();
                makeOutput(gadget.getOutputWires()[0]);
            }

            
            pub  fn generateSampleInput(CircuitEvaluator evaluator) {}
        };

        cgen.generateCircuit();
        cgen.evalCircuit();
        let evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
    }
}
