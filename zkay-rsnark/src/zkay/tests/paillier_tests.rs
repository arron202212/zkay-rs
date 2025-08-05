use crate::circuit::auxiliary::long_element;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;

use crate::util::util::{BigInteger, Util};
use zkay::zkay_paillier_dec_gadget;
use zkay::zkay_paillier_enc_gadget;
use zkay::zkay_paillier_fast_dec_gadget;
use zkay::zkay_paillier_fast_enc_gadget;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn testEncryptionExample() {
        let plain = BigInteger.parse_bytes(b"42", 10).unwrap();
        let random = BigInteger.parse_bytes(b"25", 10).unwrap();
        let n = BigInteger.parse_bytes(b"9047", 10).unwrap();
        let mut generator = BigInteger.parse_bytes(b"27", 10).unwrap();
        let enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
        let cipher = enc.computeResult();
        assert_eq!(BigInteger.parse_bytes(b"45106492", 10).unwrap(), cipher);
    }

    #[test]
    pub fn testDecryptionExample() {
        let n = BigInteger.parse_bytes(b"9047", 10).unwrap();
        let cipher = BigInteger.parse_bytes(b"2587834", 10).unwrap();
        let lambda = BigInteger.parse_bytes(b"4428", 10).unwrap();
        let mu = BigInteger.parse_bytes(b"1680", 10).unwrap();
        let dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
        let plain = dec.computeResult();
        assert_eq!(BigInteger.parse_bytes(b"55", 10).unwrap(), plain);
    }

    #[test]
    pub fn test256BitEncryption() {
        let plain = BigInteger
            .parse_bytes(
                b"58620521968995858419238449046464883186412581610038046858008683322252437292505",
                10,
            )
            .unwrap();
        let random = BigInteger
            .parse_bytes(
                b"66895129274476067543864711343178574027057505369800972938068894913816799963509",
                10,
            )
            .unwrap();
        let n = BigInteger
            .parse_bytes(
                b"71705678335151044143714697909938764102247769560297862447809589632641441407751",
                10,
            )
            .unwrap();
        let mut generator = BigInteger.parse_bytes(b"27", 10).unwrap();
        let enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
        let cipher = enc.computeResult();
        assert_eq!(BigInteger.parse_bytes(b"3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963",10).unwrap(), cipher);
    }

    #[test]
    pub fn test256BitDecryption() {
        let n = BigInteger
            .parse_bytes(
                b"71705678335151044143714697909938764102247769560297862447809589632641441407751",
                10,
            )
            .unwrap();
        let cipher = BigInteger.parse_bytes(b"3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963",10).unwrap();
        let lambda = BigInteger
            .parse_bytes(
                b"35852839167575522071857348954969382050854184697828828629810896599748215236036",
                10,
            )
            .unwrap();
        let mu = BigInteger
            .parse_bytes(
                b"38822179779668243734206910236945399376867932682990009748733172869327079310544",
                10,
            )
            .unwrap();
        let dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
        let plain = dec.computeResult();
        assert_eq!(BigInteger.parse_bytes(b"58620521968995858419238449046464883186412581610038046858008683322252437292505",10).unwrap(), plain);
    }

    #[test]
    pub fn test256BitFastEncryption() {
        let plain = BigInteger
            .parse_bytes(
                b"58620521968995858419238449046464883186412581610038046858008683322252437292505",
                10,
            )
            .unwrap();
        let random = BigInteger
            .parse_bytes(
                b"66895129274476067543864711343178574027057505369800972938068894913816799963509",
                10,
            )
            .unwrap();
        let n = BigInteger
            .parse_bytes(
                b"71705678335151044143714697909938764102247769560297862447809589632641441407751",
                10,
            )
            .unwrap();
        let enc = PaillierFastEncCircuitGenerator::new("Paillier Enc", n, plain, random);
        let cipher = enc.computeResult();
        assert_eq!(BigInteger.parse_bytes(b"3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136",10).unwrap(), cipher);
    }

    #[test]
    pub fn test256BitFastDecryption() {
        let n = BigInteger
            .parse_bytes(
                b"71705678335151044143714697909938764102247769560297862447809589632641441407751",
                10,
            )
            .unwrap();
        let lambda = BigInteger
            .parse_bytes(
                b"71705678335151044143714697909938764101708369395657657259621793199496430472072",
                10,
            )
            .unwrap();
        let cipher = BigInteger.parse_bytes(b"3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136",10).unwrap();
        let dec = PaillierFastDecCircuitGenerator::new("Paillier Dec", n, lambda, cipher);
        let plain = dec.computeResult();
        assert_eq!(BigInteger.parse_bytes(b"58620521968995858419238449046464883186412581610038046858008683322252437292505",10).unwrap(), plain);
    }

    // Don't look. Here lies the Land of Copy & Paste

    struct PaillierEncCircuitGenerator {
        plain: BigInteger,
        random: BigInteger,
        n: BigInteger,
        generator: BigInteger,

        plainWire: Option<LongElement>,
        randomWire: Option<LongElement>,
        nWire: Option<LongElement>,
        generatorWire: Option<LongElement>,
    }

    impl PaillierEncCircuitGenerator {
        pub fn new(
            name: &str,
            plain: BigInteger,
            random: BigInteger,
            n: BigInteger,
            generator: BigInteger,
        ) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<Self>::new(
                name,
                Self {
                    plain,
                    random,
                    n,
                    generator,
                    plainWire: None,
                    randomWire: None,
                    nWire: None,
                    generatorWire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierEncCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierEncCircuitGenerator> {
        fn buildCircuit(&mut self) {
            plainWire = self.createLongElementInput(max(plain.bits(), 1), "plain");
            randomWire = self.createLongElementInput(max(random.bits(), 1), "random");
            let nBits = std::cmp::max(n.bits(), 1);
            nWire = self.createLongElementInput(nBits, "n");
            generatorWire = self.createLongElementInput(max(generator.bits(), 1), "generator");
            let enc =
                ZkayPaillierEncGadget::new(nWire, nBits, generatorWire, plainWire, randomWire);
            self.makeOutputArray(enc.getOutputWires(), "cipher");
        }

        pub fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValue(plainWire, plain, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(randomWire, random, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(nWire, n, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(generatorWire, generator, LongElement::CHUNK_BITWIDTH);
        }

        pub fn computeResult(&self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            self.evalCircuit();

            let evaluator = self.getCircuitEvaluator();
            let outValues = evaluator.getWiresValues(self.get_out_wires());
            Util::group(outValues, LongElement::CHUNK_BITWIDTH)
        }
    }

    struct PaillierDecCircuitGenerator {
        cipher: BigInteger,
        n: BigInteger,
        lambda: BigInteger,
        mu: BigInteger,

        cipherWire: Option<LongElement>,
        nWire: Option<LongElement>,
        lambdaWire: Option<LongElement>,
        muWire: Option<LongElement>,
    }

    impl PaillierDecCircuitGenerator {
        pub fn new(
            name: &str,
            cipher: BigInteger,
            n: BigInteger,
            lambda: BigInteger,
            mu: BigInteger,
        ) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<Self>::new(
                name,
                Self {
                    cipher,
                    n,
                    lambda,
                    mu,
                    cipherWire: None,
                    nWire: None,
                    lambdaWire: None,
                    muWire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierDecCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierDecCircuitGenerator> {
        fn buildCircuit(&mut self) {
            cipherWire = self.createLongElementInput(max(cipher.bits(), 1), "cipher");
            let nBits = std::cmp::max(n.bits(), 1);
            nWire = self.createLongElementInput(nBits, "n");
            lambdaWire = self.createLongElementInput(max(lambda.bits(), 1), "lambda");
            muWire = self.createLongElementInput(max(mu.bits(), 1), "mu");
            let dec = ZkayPaillierDecGadget::new(nWire, nBits, lambdaWire, muWire, cipherWire);
            self.makeOutputArray(dec.getOutputWires(), "plain");
        }

        pub fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValue(cipherWire, cipher, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(nWire, n, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(lambdaWire, lambda, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(muWire, mu, LongElement::CHUNK_BITWIDTH);
        }

        pub fn computeResult(self) {
            // let t1 = System.nanoTime();
            self.generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            self.evalCircuit();

            let evaluator = self.getCircuitEvaluator();
            let outValues = evaluator.getWiresValues(get_out_wires().toArray(vec![None; 0]));
            Util::group(outValues, LongElement::CHUNK_BITWIDTH)
        }
    }

    struct PaillierFastEncCircuitGenerator {
        n: BigInteger,
        plain: BigInteger,
        random: BigInteger,

        nWire: Option<LongElement>,
        plainWire: Option<LongElement>,
        randomWire: Option<LongElement>,
    }

    impl PaillierFastEncCircuitGenerator {
        pub fn new(
            name: &str,
            n: BigInteger,
            plain: BigInteger,
            random: BigInteger,
        ) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<Self>::new(
                name,
                Self {
                    n,
                    plain,
                    random,

                    nWire: None,
                    plainWire: None,
                    randomWire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierFastEncCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierFastEncCircuitGenerator> {
        fn buildCircuit(&mut self) {
            let nBits = std::cmp::max(n.bits(), 1);
            nWire = self.createLongElementInput(nBits, "n");
            plainWire = self.createLongElementInput(max(plain.bits(), 1), "plain");
            randomWire = self.createLongElementInput(max(random.bits(), 1), "random");
            let enc = ZkayPaillierFastEncGadget::new(nWire, nBits, plainWire, randomWire);
            self.makeOutputArray(enc.getOutputWires(), "cipher");
        }

        pub fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValue(nWire, n, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(plainWire, plain, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(randomWire, random, LongElement::CHUNK_BITWIDTH);
        }

        pub fn computeResult(self) {
            // let t1 = System.nanoTime();
            self.generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            self.evalCircuit();

            let evaluator = self.getCircuitEvaluator();
            let outValues = evaluator.getWiresValues(get_out_wires().toArray(vec![None; 0]));
            Util::group(outValues, LongElement::CHUNK_BITWIDTH)
        }
    }

    struct PaillierFastDecCircuitGenerator {
        n: BigInteger,
        lambda: BigInteger,
        cipher: BigInteger,

        nWire: Option<LongElement>,
        lambdaWire: Option<LongElement>,
        cipherWire: Option<LongElement>,
    }

    impl PaillierFastDecCircuitGenerator {
        pub fn new(
            name: &str,
            n: BigInteger,
            lambda: BigInteger,
            cipher: BigInteger,
        ) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<Self>::new(
                name,
                Self {
                    n,
                    lambda,
                    cipher,
                    nWire: None,
                    lambdaWire: None,
                    cipherWire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierFastDecCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierFastDecCircuitGenerator> {
        fn buildCircuit(&mut self) {
            let nBits = std::cmp::max(n.bits(), 1);
            nWire = self.createLongElementInput(nBits, "n");
            lambdaWire = self.createLongElementInput(max(lambda.bits(), 1), "lambda");
            cipherWire = self.createLongElementInput(max(cipher.bits(), 1), "cipher");
            let dec = ZkayPaillierFastDecGadget::new(nWire, nBits, lambdaWire, cipherWire);
            self.makeOutputArray(dec.getOutputWires(), "plain");
        }

        pub fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValue(nWire, n, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(lambdaWire, lambda, LongElement::CHUNK_BITWIDTH);
            evaluator.setWireValue(cipherWire, cipher, LongElement::CHUNK_BITWIDTH);
        }

        pub fn computeResult(self) {
            // let t1 = System.nanoTime();
            generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            self.evalCircuit();

            let evaluator = self.getCircuitEvaluator();
            let outValues = evaluator.getWiresValues(get_out_wires().toArray(vec![None; 0]));
            Util::group(outValues, LongElement::CHUNK_BITWIDTH)
        }
    }
}
