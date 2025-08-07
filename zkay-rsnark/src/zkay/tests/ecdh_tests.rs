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
use crate::circuit::structure::wire_type::WireType;

use crate::zkay::zkay_ec_pk_derivation_gadget::ZkayEcPkDerivationGadget;
use crate::zkay::zkay_ecdh_gadget::ZkayECDHGadget;
use crate::zkay::zkay_ecdh_generator::ZkayECDHGenerator;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testECDH() {
        let sec1 = ZkayECDHGenerator
            .rnd_to_secret("0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0");
        let sec2 = ZkayECDHGenerator
            .rnd_to_secret("6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c");

        let pk1 = BigInteger::new(ZkayECDHGenerator.derivePk(sec1), 16);
        let pk2 = BigInteger::new(ZkayECDHGenerator.derivePk(sec2), 16);

        let sk1 = ZkayECDHGenerator.getSharedSecret(pk2, sec1);
        let sk2 = ZkayECDHGenerator.getSharedSecret(pk1, sec2);
        assert_eq!(sk1, sk2);
    }

    #[test]
    pub fn testSameAsGadget() {
        let sec1 = ZkayECDHGenerator
            .rnd_to_secret("0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0");
        let sec2 = ZkayECDHGenerator
            .rnd_to_secret("6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c");

        let mut cgen = CircuitGenerator::new("pkder");
        crate::impl_struct_name_for!(CircuitGeneratorExtend<ElgamalDecCircuitGenerator>);
        impl CGConfig for CircuitGeneratorExtend<ElgamalDecCircuitGenerator> {
            fn buildCircuit(&mut self) {
                let s = createConstantWire(sec1);
                makeOutput(ZkayEcPkDerivationGadget::new(s, true).getOutputWires()[0]);
            }

            pub fn generateSampleInput(&self, _evaluator: &mut CircuitEvaluator) {}
        }
        cgen.generateCircuit();
        cgen.evalCircuit();
        let mut evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate(generator.cg());
        let pk1_circ = evaluator.getWireValue(cgen.get_out_wires().get(0));

        let mut cgen = CircuitGenerator::new("pkder");
        crate::impl_struct_name_for!(CircuitGeneratorExtend<ElgamalDecCircuitGenerator>);
        impl CGConfig for CircuitGeneratorExtend<ElgamalDecCircuitGenerator> {
            fn buildCircuit(&mut self) {
                let s = self.createConstantWire(sec2);
                self.makeOutput(ZkayEcPkDerivationGadget::new(s, true).getOutputWires()[0]);
            }

            pub fn generateSampleInput(&self, _evaluator: &mut CircuitEvaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate(generator.cg());
        let pk2_circ = evaluator.getWireValue(cgen.get_out_wires().get(0));

        let pk1 = BigInteger::new(ZkayECDHGenerator.derivePk(sec1), 16);
        let pk2 = BigInteger::new(ZkayECDHGenerator.derivePk(sec2), 16);
        assert_eq!(pk1, pk1_circ);
        assert_eq!(pk2, pk2_circ);

        let mut cgen = CircuitGenerator::new("ecdh");
        crate::impl_struct_name_for!(CircuitGeneratorExtend<ElgamalDecCircuitGenerator>);
        impl CGConfig for CircuitGeneratorExtend<ElgamalDecCircuitGenerator> {
            fn buildCircuit(&mut self) {
                let p = self.createConstantWire(pk2);
                let s = self.createConstantWire(sec1);
                self.makeOutput(ZkayECDHGadget::new(p, s, false).getOutputWires()[0]);
            }

            pub fn generateSampleInput(&self, _evaluator: &mut CircuitEvaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate(generator.cg());
        let sk_circ = evaluator.getWireValue(cgen.get_out_wires().get(0));

        let sk_exp = BigInteger::new(ZkayECDHGenerator.getSharedSecret(pk2, sec1), 16);
        assert_eq!(sk_exp, sk_circ);
    }
}
