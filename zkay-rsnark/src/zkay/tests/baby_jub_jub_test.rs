
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use circuit::structure::wire_array;
use zkay::zkay_baby_jub_jub_gadget;


pub struct BabyJubJubTest {
     class TestGadget extends ZkayBabyJubJubGadget {

        pub  TestGadget() {
            buildCircuit();
        }

          fn buildCircuit() {
            // check native inverse
            Wire a = generator.createConstantWire(BigInteger::new("11985782033876175911769025829561891428638139496693105005957757653258"));
            Wire ainv_expected = generator.createConstantWire(BigInteger::new("20950552912096304742729232452120498732043875737213521271262032500972060322340"));
            Wire ainv = nativeInverse(a);
            generator.addEqualityAssertion(ainv, ainv_expected);

            // check generator on curve
            Wire g_x = generator.createConstantWire(GENERATOR_X);
            Wire g_y = generator.createConstantWire(GENERATOR_Y);
            assertOnCurve(g_x, g_y);

            // check generator + generator on curve
            JubJubPoint g = getGenerator();
            JubJubPoint g2 = addPoints(g, g);
            assertOnCurve(g2.x, g2.y);

            // check generator - generator = INFINITY
            JubJubPoint gneg = negatePoint(g);
            assertOnCurve(gneg.x, gneg.y);
            JubJubPoint inf = addPoints(g, gneg);
            generator.addEqualityAssertion(inf.x, generator.getZeroWire());
            generator.addEqualityAssertion(inf.y, generator.getOneWire());

            // check generator + INFINITY = generator
            JubJubPoint g_expected = addPoints(g, getInfinity());
            generator.addEqualityAssertion(g_expected.x, g.x);
            generator.addEqualityAssertion(g_expected.y, g.y);

            // check scalar multiplication
            Wire scalar = generator.createConstantWire(5);
            WireArray scalarBits = scalar.getBitWires(4);
            JubJubPoint g5 = mulScalar(g, scalarBits.asArray());
            JubJubPoint g5_expected = addPoints(addPoints(addPoints(addPoints(g, g), g), g), g);
            assertOnCurve(g5.x, g5.y);
            generator.addEqualityAssertion(g5.x, g5_expected.x);
            generator.addEqualityAssertion(g5.y, g5_expected.y);
        }

        
        pub  fn getOutputWires()->Vec<Wire>  {
            Wire dummy = generator.getOneWire();
            return vec![Wire::default();]{ dummy };
        }
    }

    @Test
    pub   testBabyJubJubGadget() {
        CircuitGenerator cgen = CircuitGenerator::new("test") {
            
              fn buildCircuit() {
                TestGadget gadget = TestGadget::new();
                makeOutput(gadget.getOutputWires()[0]);
            }

            
            pub   generateSampleInput(CircuitEvaluator evaluator) {}
        };

        cgen.generateCircuit();
        cgen.evalCircuit();
        CircuitEvaluator evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
    }
}
