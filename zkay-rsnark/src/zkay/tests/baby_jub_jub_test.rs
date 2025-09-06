#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
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
            // println!(
            //     "===self.get_current_wire_id()======test=={}==={}",
            //     generator.get_current_wire_id(),
            //     self.generator.get_current_wire_id()
            // );

            let a = CircuitGenerator::create_constant_wire(
                generator.clone(),
                &pbi("11985782033876175911769025829561891428638139496693105005957757653258"),
                &None,
            );
            // println!(
            //     "===self.get_current_wire_id()=====test==={}=={}",
            //     generator.get_current_wire_id(),
            //     self.generator.get_current_wire_id()
            // );

            let ainv_expected = CircuitGenerator::create_constant_wire(
                generator.clone(),
                &pbi(
                    "20950552912096304742729232452120498732043875737213521271262032500972060322340",
                ),
                &None,
            );
            // println!(
            //     "===self.get_current_wire_id()=====test=={}==={}",
            //     generator.get_current_wire_id(),
            //     self.generator.get_current_wire_id()
            // );

            const BASE_ORDER: &str =
                "21888242871839275222246405745257275088548364400416034343698204186575808495617";
            let inverseValue =
                pbi("11985782033876175911769025829561891428638139496693105005957757653258")
                    .modinv(&Util::parse_big_int(BASE_ORDER))
                    .unwrap();
            assert_eq!(
                inverseValue,
                pbi(
                    "20950552912096304742729232452120498732043875737213521271262032500972060322340",
                )
            );
            // println!(
            //     "===self.get_current_wire_id()======nativeInverse==test====before=={}===={}",
            //     generator.get_current_wire_id(),
            //     self.generator.get_current_wire_id()
            // );
            let ainv = self.nativeInverse(&a);
            // println!(
            //     "===self.get_current_wire_id()======nativeInverse===test===after=={}===={}",
            //     generator.get_current_wire_id(),
            //     self.generator.get_current_wire_id()
            // );
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                &ainv,
                &ainv_expected,
                &None,
            );

            // check generator on curve
            let g_x = CircuitGenerator::create_constant_wire(
                generator.clone(),
                &pbi(Self::GENERATOR_X),
                &None,
            );
            let g_y = CircuitGenerator::create_constant_wire(
                generator.clone(),
                &pbi(Self::GENERATOR_Y),
                &None,
            );
            self.assertOnCurve(&g_x, &g_y);

            // check generator + generator on curve
            let g = self.getGenerator();
            let g2 = self.addPoints(&g, &g);
            self.assertOnCurve(&g2.x, &g2.y);

            // check generator - generator = INFINITY
            let gneg = Gadget::<ZkayBabyJubJubGadget<TestGadget>>::negatePoint(&g);
            self.assertOnCurve(&gneg.x, &gneg.y);
            let inf = self.addPoints(&g, &gneg);
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                &inf.x,
                generator.get_zero_wire().as_ref().unwrap(),
                &None,
            );
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                &inf.y,
                generator.get_one_wire().as_ref().unwrap(),
                &None,
            );

            // check generator + INFINITY = generator
            let g_expected = self.addPoints(&g, &self.getInfinity());
            CircuitGenerator::add_equality_assertion(generator.clone(), &g_expected.x, &g.x, &None);
            CircuitGenerator::add_equality_assertion(generator.clone(), &g_expected.y, &g.y, &None);

            // check scalar multiplication
            let scalar = CircuitGenerator::create_constant_wirei(generator.clone(), 5, &None);
            let scalarBits = scalar.get_bit_wiresi(4, &None);
            let g5 = self.mulScalar(&g, scalarBits.as_array());
            let g5_expected = self.addPoints(
                &self.addPoints(&self.addPoints(&self.addPoints(&g, &g), &g), &g),
                &g,
            );
            self.assertOnCurve(&g5.x, &g5.y);
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                &g5.x,
                &g5_expected.x,
                &None,
            );
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                &g5.y,
                &g5_expected.y,
                &None,
            );
        }
    }

    impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<TestGadget>> {
        fn get_output_wires(&self) -> &Vec<Option<WireType>> {
            //  let dummy = generator.get_one_wire();
            &self.t.t.dummy
        }
    }

    #[test]
    pub fn testBabyJubJubGadget() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest;

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let gadget = TestGadget::new(self.cg());
                CircuitGenerator::make_output(
                    self.cg(),
                    gadget.get_output_wires()[0].as_ref().unwrap(),
                    &None,
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
