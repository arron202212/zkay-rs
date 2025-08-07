#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;

#[cfg(test)]
mod test {
    use super::*;
    //extends ZkayBabyJubJubGadget
    struct TestGadget {
        dummy: Vec<Option<WireType>>,
    }
    impl TestGadget {
        fn new(generator: RcCell<CircuitGenerator>) -> Gadget<ZkayBabyJubJubGadget<Self>> {
            let _self = ZkayBabyJubJubGadget::<Self>::new(
                &None,
                TestGadget {
                    dummy: generator.get_one_wire(),
                },
                generator,
            );
            _self.buildCircuit();
            _self
        }
    }

    impl Gadget<ZkayBabyJubJubGadget<TestGadget>> {
        fn buildCircuit(&mut self) {
            let generator = self.generator.borrow().clone();
            // check native inverse
            let a = generator.createConstantWire(
                &BigInteger
                    .parse_bytes(
                        b"11985782033876175911769025829561891428638139496693105005957757653258",
                        10,
                    )
                    .unwrap(),
            );
            let ainv_expected = generator.createConstantWire(&BigInteger.parse_bytes(b"20950552912096304742729232452120498732043875737213521271262032500972060322340",10).unwrap());
            let ainv = self.t.nativeInverse(a);
            generator.addEqualityAssertion(ainv, ainv_expected);

            // check generator on curve
            let g_x = generator.createConstantWire(GENERATOR_X);
            let g_y = generator.createConstantWire(GENERATOR_Y);
            assertOnCurve(g_x, g_y);

            // check generator + generator on curve
            let g = self.t.getGenerator();
            let g2 = self.t.addPoints(g, g);
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
    }

    impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<TestGadget>> {
        fn getOutputWires(&self) -> &Vec<Option<WireType>> {
            //  let dummy = generator.get_one_wire();
            &self.t.t.dummy
        }
    }

    #[test]
    pub fn testBabyJubJubGadget() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {}

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let gadget = TestGadget::new();
                self.makeOutput(gadget.getOutputWires()[0]);
            }

            fn generateSampleInput(&self, _evaluator: &mut CircuitEvaluator) {}
        }
        let t = CGTest {};
        let cgen = CircuitGeneratorExtend::<CGTest>::new("test", t);
        cgen.generateCircuit();
        cgen.evalCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        evaluator.evaluate(&generator.cg);
    }
}
