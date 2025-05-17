
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;

use zkay::zkay_ecdh_gadget;
use zkay::zkay_ecdh_generator;
use zkay::zkay_ec_pk_derivation_gadget;



pub struct EcdhTests {
    
    pub   testECDH() {
        let sec1 = ZkayECDHGenerator.rnd_to_secret("0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0");
        let sec2 = ZkayECDHGenerator.rnd_to_secret("6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c");

        let pk1 = BigInteger::new(ZkayECDHGenerator.derivePk(sec1), 16);
        let pk2 = BigInteger::new(ZkayECDHGenerator.derivePk(sec2), 16);

        let sk1 = ZkayECDHGenerator.getSharedSecret(pk2, sec1);
        let sk2 = ZkayECDHGenerator.getSharedSecret(pk1, sec2);
        Assert.assertEquals(sk1, sk2);
    }

    
    pub   testSameAsGadget() {
        let sec1 = ZkayECDHGenerator.rnd_to_secret("0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0");
        let sec2 = ZkayECDHGenerator.rnd_to_secret("6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c");

        CircuitGenerator cgen = CircuitGenerator::new("pkder") {
            
              fn buildCircuit() {
               let s = createConstantWire(sec1);
               makeOutput(ZkayEcPkDerivationGadget::new(s, true).getOutputWires()[0]);
            }

            
            pub  fn generateSampleInput(CircuitEvaluator evaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        let evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
        let pk1_circ = evaluator.getWireValue(cgen.getOutWires().get(0));

        cgen = CircuitGenerator::new("pkder") {
            
              fn buildCircuit() {
                let s = createConstantWire(sec2);
                makeOutput(ZkayEcPkDerivationGadget::new(s, true).getOutputWires()[0]);
            }

            
            pub  fn generateSampleInput(CircuitEvaluator evaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
        let pk2_circ = evaluator.getWireValue(cgen.getOutWires().get(0));

        let pk1 = BigInteger::new(ZkayECDHGenerator.derivePk(sec1), 16);
        let pk2 = BigInteger::new(ZkayECDHGenerator.derivePk(sec2), 16);
        Assert.assertEquals(pk1, pk1_circ);
        Assert.assertEquals(pk2, pk2_circ);

        cgen = CircuitGenerator::new("ecdh") {
            
              fn buildCircuit() {
                let p = createConstantWire(pk2);
                let s = createConstantWire(sec1);
                makeOutput(ZkayECDHGadget::new(p, s, false).getOutputWires()[0]);
            }

            
            pub  fn generateSampleInput(CircuitEvaluator evaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
        let sk_circ = evaluator.getWireValue(cgen.getOutWires().get(0));

        let sk_exp = BigInteger::new(ZkayECDHGenerator.getSharedSecret(pk2, sec1), 16);
        Assert.assertEquals(sk_exp, sk_circ);
    }
}
