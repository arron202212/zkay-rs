
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;

use zkay::zkay_ecdh_gadget;
use zkay::zkay_ecdh_generator;
use zkay::zkay_ec_pk_derivation_gadget;



pub struct EcdhTests {
    @Test
    pub   testECDH() {
        BigInteger sec1 = ZkayECDHGenerator.rnd_to_secret("0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0");
        BigInteger sec2 = ZkayECDHGenerator.rnd_to_secret("6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c");

        BigInteger pk1 = BigInteger::new(ZkayECDHGenerator.derivePk(sec1), 16);
        BigInteger pk2 = BigInteger::new(ZkayECDHGenerator.derivePk(sec2), 16);

        String sk1 = ZkayECDHGenerator.getSharedSecret(pk2, sec1);
        String sk2 = ZkayECDHGenerator.getSharedSecret(pk1, sec2);
        Assert.assertEquals(sk1, sk2);
    }

    @Test
    pub   testSameAsGadget() {
        BigInteger sec1 = ZkayECDHGenerator.rnd_to_secret("0032f06dfe06a7f7d1a4f4292c136ee78b5d4b4bb26904b2363330bd213ccea0");
        BigInteger sec2 = ZkayECDHGenerator.rnd_to_secret("6c0f17e169532e67f0fa96999f652bca942bd97617295a025eaa6c5d1cd3fd5c");

        CircuitGenerator cgen = CircuitGenerator::new("pkder") {
            
              fn buildCircuit() {
               Wire s = createConstantWire(sec1);
               makeOutput(ZkayEcPkDerivationGadget::new(s, true).getOutputWires()[0]);
            }

            
            pub   generateSampleInput(CircuitEvaluator evaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        CircuitEvaluator evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
        BigInteger pk1_circ = evaluator.getWireValue(cgen.getOutWires().get(0));

        cgen = CircuitGenerator::new("pkder") {
            
              fn buildCircuit() {
                Wire s = createConstantWire(sec2);
                makeOutput(ZkayEcPkDerivationGadget::new(s, true).getOutputWires()[0]);
            }

            
            pub   generateSampleInput(CircuitEvaluator evaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
        BigInteger pk2_circ = evaluator.getWireValue(cgen.getOutWires().get(0));

        BigInteger pk1 = BigInteger::new(ZkayECDHGenerator.derivePk(sec1), 16);
        BigInteger pk2 = BigInteger::new(ZkayECDHGenerator.derivePk(sec2), 16);
        Assert.assertEquals(pk1, pk1_circ);
        Assert.assertEquals(pk2, pk2_circ);

        cgen = CircuitGenerator::new("ecdh") {
            
              fn buildCircuit() {
                Wire p = createConstantWire(pk2);
                Wire s = createConstantWire(sec1);
                makeOutput(ZkayECDHGadget::new(p, s, false).getOutputWires()[0]);
            }

            
            pub   generateSampleInput(CircuitEvaluator evaluator) {}
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
        BigInteger sk_circ = evaluator.getWireValue(cgen.getOutWires().get(0));

        BigInteger sk_exp = BigInteger::new(ZkayECDHGenerator.getSharedSecret(pk2, sec1), 16);
        Assert.assertEquals(sk_exp, sk_circ);
    }
}
