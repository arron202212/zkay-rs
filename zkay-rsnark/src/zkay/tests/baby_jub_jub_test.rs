#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
    zkay::zkay_baby_jub_jub_gadget::{ZkayBabyJubJubGadget, ZkayBabyJubJubGadgetConfig},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

#[cfg(test)]
mod test {

    use super::*;
    //extends ZkayBabyJubJubGadget
    #[inline]
    fn pbi(bs: &str) -> BigInteger {
        Util::parse_big_int(bs)
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct TestGadget {
        dummy: Vec<Option<WireType>>,
    }
    impl TestGadget {
        fn new(generator: RcCell<CircuitGenerator>) -> Gadget<ZkayBabyJubJubGadget<Self>> {
            let mut _self = ZkayBabyJubJubGadget::<Self>::new(
                &None,
                TestGadget {
                    dummy: vec![generator.get_one_wire()],
                },
                generator,
            );
            _self.build_circuit();
            _self
        }
    }

    impl Gadget<ZkayBabyJubJubGadget<TestGadget>> {
        fn build_circuit(&mut self) {
            let generator = self.generator.clone();
            // check native inverse

            let a = CircuitGenerator::create_constant_wire(
                generator.clone(),
                &pbi("11985782033876175911769025829561891428638139496693105005957757653258"),
            );

            let ainv_expected = CircuitGenerator::create_constant_wire(
                generator.clone(),
                &pbi(
                    "20950552912096304742729232452120498732043875737213521271262032500972060322340",
                ),
            );

            const BASE_ORDER: &str =
                "21888242871839275222246405745257275088548364400416034343698204186575808495617";
            let inverse_value =
                pbi("11985782033876175911769025829561891428638139496693105005957757653258")
                    .modinv(&Util::parse_big_int(BASE_ORDER))
                    .unwrap();
            assert_eq!(
                inverse_value,
                pbi(
                    "20950552912096304742729232452120498732043875737213521271262032500972060322340",
                )
            );

            let ainv = self.native_inverse(&a);

            CircuitGenerator::add_equality_assertion(generator.clone(), &ainv, &ainv_expected);

            // check generator on curve
            let g_x =
                CircuitGenerator::create_constant_wire(generator.clone(), &pbi(Self::GENERATOR_X));
            let g_y =
                CircuitGenerator::create_constant_wire(generator.clone(), &pbi(Self::GENERATOR_Y));
            self.assert_on_curve(&g_x, &g_y);

            // check generator + generator on curve
            let g = self.get_generator();
            let g2 = self.add_points(&g, &g);
            self.assert_on_curve(&g2.x, &g2.y);

            // check generator - generator = INFINITY
            let gneg = Gadget::<ZkayBabyJubJubGadget<TestGadget>>::negate_point(&g);
            self.assert_on_curve(&gneg.x, &gneg.y);
            let inf = self.add_points(&g, &gneg);
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                &inf.x,
                generator.get_zero_wire().as_ref().unwrap(),
            );
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                &inf.y,
                generator.get_one_wire().as_ref().unwrap(),
            );

            // check generator + INFINITY = generator
            let g_expected = self.add_points(&g, &self.get_infinity());
            CircuitGenerator::add_equality_assertion(generator.clone(), &g_expected.x, &g.x);
            CircuitGenerator::add_equality_assertion(generator.clone(), &g_expected.y, &g.y);

            // check scalar multiplication
            let scalar = CircuitGenerator::create_constant_wirei(generator.clone(), 5);
            let scalar_bits = scalar.get_bit_wiresi(4);
            let g5 = self.mul_scalar(&g, scalar_bits.as_array());
            let g5_expected = self.add_points(
                &self.add_points(&self.add_points(&self.add_points(&g, &g), &g), &g),
                &g,
            );
            self.assert_on_curve(&g5.x, &g5.y);
            CircuitGenerator::add_equality_assertion(generator.clone(), &g5.x, &g5_expected.x);
            CircuitGenerator::add_equality_assertion(generator.clone(), &g5.y, &g5_expected.y);
        }
    }

    impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<TestGadget>> {
        fn get_output_wires(&self) -> &Vec<Option<WireType>> {
            //  let dummy = generator.get_one_wire();
            &self.t.t.dummy
        }
    }

    #[test]
    pub fn test_baby_jub_jub_gadget() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest;

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let gadget = TestGadget::new(self.cg());
                CircuitGenerator::make_output(
                    self.cg(),
                    gadget.get_output_wires()[0].as_ref().unwrap(),
                );
            }

            fn generate_sample_input(&self, _evaluator: &mut CircuitEvaluator) {}
        }
        let t = CGTest;
        let mut cgen = CircuitGeneratorExtend::<CGTest>::new("test", t);
        cgen.generate_circuit();
        cgen.eval_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
    }
}
