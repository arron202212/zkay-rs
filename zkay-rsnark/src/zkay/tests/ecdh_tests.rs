#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CGConfigFields;
use crate::circuit::structure::circuit_generator::CGInstance;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::zkay_ec_pk_derivation_gadget::ZkayEcPkDerivationGadget;
use crate::zkay::zkay_ecdh_gadget::ZkayECDHGadget;

use crate::util::util::{BigInteger, Util};
use crate::zkay::zkay_ecdh_generator::ZkayECDHGenerator;
use zkay_derive::ImplStructNameConfig;
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testECDH() {
        let sec1 = CircuitGeneratorExtend::<ZkayECDHGenerator>::rnd_to_secret(
            &"0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0".to_owned(),
        );
        let sec2 = CircuitGeneratorExtend::<ZkayECDHGenerator>::rnd_to_secret(
            &"6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c".to_owned(),
        );

        let pk1 = Util::parse_big_int_x(&CircuitGeneratorExtend::<ZkayECDHGenerator>::derivePk(
            &sec1,
        ));
        let pk2 = Util::parse_big_int_x(&CircuitGeneratorExtend::<ZkayECDHGenerator>::derivePk(
            &sec2,
        ));

        let sk1 = CircuitGeneratorExtend::<ZkayECDHGenerator>::getSharedSecret(&pk2, &sec1);
        let sk2 = CircuitGeneratorExtend::<ZkayECDHGenerator>::getSharedSecret(&pk1, &sec2);
        assert_eq!(sk1, sk2);
    }

    #[test]
    pub fn testSameAsGadget() {
        let sec1 = CircuitGeneratorExtend::<ZkayECDHGenerator>::rnd_to_secret(
            &"0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0".to_owned(),
        );
        let sec2 = CircuitGeneratorExtend::<ZkayECDHGenerator>::rnd_to_secret(
            &"6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c".to_owned(),
        );

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub sec1: BigInteger,
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let s = CircuitGenerator::createConstantWire(self.cg(), &self.t.sec1, &None);
                CircuitGenerator::makeOutput(
                    self.cg(),
                    ZkayEcPkDerivationGadget::new(s, true, &None, self.cg()).getOutputWires()[0]
                        .as_ref()
                        .unwrap(),
                    &None,
                );
            }

            fn generateSampleInput(&self, _evaluator: &mut CircuitEvaluator) {}
        }
        let t = CGTest { sec1: sec1.clone() };
        let mut cgen = CircuitGeneratorExtend::<CGTest>::new("pkder", t);
        cgen.generateCircuit();
        cgen.evalCircuit();
        let mut evaluator = CircuitEvaluator::new("pkder", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
        let pk1_circ = evaluator.getWireValue(cgen.get_out_wires()[0].as_ref().unwrap());

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTestpkder {
            pub sec2: BigInteger,
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTestpkder>);
        impl CGConfig for CircuitGeneratorExtend<CGTestpkder> {
            fn buildCircuit(&mut self) {
                let s = CircuitGenerator::createConstantWire(self.cg(), &self.t.sec2, &None);
                CircuitGenerator::makeOutput(
                    self.cg(),
                    ZkayEcPkDerivationGadget::new(s, true, &None, self.cg()).getOutputWires()[0]
                        .as_ref()
                        .unwrap(),
                    &None,
                );
            }

            fn generateSampleInput(&self, _evaluator: &mut CircuitEvaluator) {}
        };
        let t = CGTestpkder { sec2: sec2.clone() };
        let mut cgen = CircuitGeneratorExtend::<CGTestpkder>::new("pkder", t);
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new("pkder", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
        let pk2_circ = evaluator.getWireValue(cgen.get_out_wires()[0].as_ref().unwrap());

        let pk1 = Util::parse_big_int_x(&CircuitGeneratorExtend::<ZkayECDHGenerator>::derivePk(
            &sec1,
        ));
        let pk2 = Util::parse_big_int_x(&CircuitGeneratorExtend::<ZkayECDHGenerator>::derivePk(
            &sec2,
        ));
        assert_eq!(pk1, pk1_circ);
        assert_eq!(pk2, pk2_circ);

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTestecdh {
            pub pk2: BigInteger,
            pub sec1: BigInteger,
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTestecdh>);
        impl CGConfig for CircuitGeneratorExtend<CGTestecdh> {
            fn buildCircuit(&mut self) {
                let p = CircuitGenerator::createConstantWire(self.cg(), &self.t.pk2, &None);
                let s = CircuitGenerator::createConstantWire(self.cg(), &self.t.sec1, &None);
                CircuitGenerator::makeOutput(
                    self.cg(),
                    ZkayECDHGadget::new(p, s, false, &None, self.cg()).getOutputWires()[0]
                        .as_ref()
                        .unwrap(),
                    &None,
                );
            }

            fn generateSampleInput(&self, _evaluator: &mut CircuitEvaluator) {}
        };
        let t = CGTestecdh {
            pk2: pk2.clone(),
            sec1: sec1.clone(),
        };
        let mut cgen = CircuitGeneratorExtend::<CGTestecdh>::new("ecdh", t);
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new("ecdh", &cgen.cg);
        evaluator.evaluate(&cgen.cg);
        let sk_circ = evaluator.getWireValue(cgen.get_out_wires()[0].as_ref().unwrap());

        let sk_exp = Util::parse_big_int_x(
            &CircuitGeneratorExtend::<ZkayECDHGenerator>::getSharedSecret(&pk2, &sec1),
        );
        assert_eq!(sk_exp, sk_circ);
    }
}
