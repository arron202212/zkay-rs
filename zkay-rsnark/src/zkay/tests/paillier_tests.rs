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
        auxiliary::long_element::LongElement,
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                addToEvaluationQueue, getActiveCircuitGenerator,
            },
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
    zkay::{
        zkay_paillier_dec_gadget::ZkayPaillierDecGadget,
        zkay_paillier_enc_gadget::ZkayPaillierEncGadget,
        zkay_paillier_fast_dec_gadget::ZkayPaillierFastDecGadget,
        zkay_paillier_fast_enc_gadget::ZkayPaillierFastEncGadget,
    },
};

use zkay_derive::ImplStructNameConfig;
#[cfg(test)]
mod test {
    use super::*;
    #[inline]
    fn pbi(bs: &str) -> BigInteger {
        Util::parse_big_int(bs)
    }
    #[test]
    pub fn testEncryptionExample() {
        let plain = BigInteger::from(42);
        let random = BigInteger::from(25);
        let n = BigInteger::from(9047);
        let mut generator = BigInteger::from(27);
        let mut enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
        let cipher = enc.computeResult();
        assert_eq!(BigInteger::from(45106492), cipher);
    }

    #[test]
    pub fn testDecryptionExample() {
        let n = BigInteger::from(9047);
        let cipher = BigInteger::from(2587834);
        let lambda = BigInteger::from(4428);
        let mu = BigInteger::from(1680);
        let mut dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
        let plain = dec.computeResult();
        assert_eq!(BigInteger::from(55), plain);
    }

    #[test]
    pub fn test256BitEncryption() {
        let plain =
            pbi("58620521968995858419238449046464883186412581610038046858008683322252437292505");

        let random =
            pbi("66895129274476067543864711343178574027057505369800972938068894913816799963509");

        let n =
            pbi("71705678335151044143714697909938764102247769560297862447809589632641441407751");

        let mut generator = BigInteger::from(27);
        let mut enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
        let cipher = enc.computeResult();
        assert_eq!(
            pbi(
                "3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963"
            ),
            cipher
        );
    }

    #[test]
    pub fn test256BitDecryption() {
        let n =
            pbi("71705678335151044143714697909938764102247769560297862447809589632641441407751");

        let cipher = pbi(
            "3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963",
        );
        let lambda =
            pbi("35852839167575522071857348954969382050854184697828828629810896599748215236036");

        let mu =
            pbi("38822179779668243734206910236945399376867932682990009748733172869327079310544");

        let mut dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
        let plain = dec.computeResult();
        assert_eq!(
            pbi("58620521968995858419238449046464883186412581610038046858008683322252437292505"),
            plain
        );
    }

    #[test]
    pub fn test256BitFastEncryption() {
        let plain =
            pbi("58620521968995858419238449046464883186412581610038046858008683322252437292505");

        let random =
            pbi("66895129274476067543864711343178574027057505369800972938068894913816799963509");

        let n =
            pbi("71705678335151044143714697909938764102247769560297862447809589632641441407751");

        let mut enc = PaillierFastEncCircuitGenerator::new("Paillier Enc", n, plain, random);
        let cipher = enc.computeResult();
        assert_eq!(
            pbi(
                "3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136"
            ),
            cipher
        );
    }

    #[test]
    pub fn test256BitFastDecryption() {
        let n =
            pbi("71705678335151044143714697909938764102247769560297862447809589632641441407751");

        let lambda =
            pbi("71705678335151044143714697909938764101708369395657657259621793199496430472072");

        let cipher = pbi(
            "3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136",
        );
        let mut dec = PaillierFastDecCircuitGenerator::new("Paillier Dec", n, lambda, cipher);
        let plain = dec.computeResult();
        assert_eq!(
            pbi("58620521968995858419238449046464883186412581610038046858008683322252437292505"),
            plain
        );
    }

    // Don't look. Here lies the Land of Copy & Paste
    #[derive(Debug, Clone, ImplStructNameConfig)]
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
            let plainWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.plain.bits().max(1) as i32,
                &Some("plain".to_owned()),
            );
            let randomWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.random.bits().max(1) as i32,
                &Some("random".to_owned()),
            );
            let nBits = self.t.n.bits().max(1);
            let nWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                nBits as i32,
                &Some("n".to_owned()),
            );

            let generatorWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.generator.bits().max(1) as i32,
                &Some("generator".to_owned()),
            );
            let enc = ZkayPaillierEncGadget::new(
                nWire.clone(),
                nBits as i32,
                generatorWire.clone(),
                plainWire.clone(),
                randomWire.clone(),
                &None,
                self.cg(),
            );
            CircuitGenerator::makeOutputArray(
                self.cg(),
                enc.getOutputWires(),
                &Some("cipher".to_owned()),
            );
            (
                self.t.nWire,
                self.t.generatorWire,
                self.t.randomWire,
                self.t.plainWire,
            ) = (
                Some(nWire),
                Some(generatorWire),
                Some(randomWire),
                Some(plainWire),
            );
        }

        fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValuebi(
                self.t.plainWire.as_ref().unwrap(),
                &self.t.plain,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.randomWire.as_ref().unwrap(),
                &self.t.random,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.nWire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.generatorWire.as_ref().unwrap(),
                &self.t.generator,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierEncCircuitGenerator> {
        pub fn computeResult(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.evalCircuit().unwrap();

            let outValues = evaluator.getWiresValues(&self.get_out_wires());
            Util::group(&outValues, LongElement::CHUNK_BITWIDTH)
        }
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
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
            let cipherWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.cipher.bits().max(1) as i32,
                &Some("cipher".to_owned()),
            );
            let nBits = self.t.n.bits().max(1);
            let nWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                nBits as i32,
                &Some("n".to_owned()),
            );
            let lambdaWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.lambda.bits().max(1) as i32,
                &Some("lambda".to_owned()),
            );
            let muWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.mu.bits().max(1) as i32,
                &Some("mu".to_owned()),
            );
            let dec = ZkayPaillierDecGadget::new(
                nWire.clone(),
                nBits as i32,
                lambdaWire.clone(),
                muWire.clone(),
                cipherWire.clone(),
                &None,
                self.cg(),
            );
            CircuitGenerator::makeOutputArray(
                self.cg(),
                dec.getOutputWires(),
                &Some("plain".to_owned()),
            );
            (
                self.t.cipherWire,
                self.t.nWire,
                self.t.lambdaWire,
                self.t.muWire,
            ) = (
                Some(cipherWire),
                Some(nWire),
                Some(lambdaWire),
                Some(muWire),
            );
        }

        fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValuebi(
                self.t.cipherWire.as_ref().unwrap(),
                &self.t.cipher,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.nWire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.lambdaWire.as_ref().unwrap(),
                &self.t.lambda,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.muWire.as_ref().unwrap(),
                &self.t.mu,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierDecCircuitGenerator> {
        pub fn computeResult(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.evalCircuit().unwrap();

            let outValues = evaluator.getWiresValues(&self.get_out_wires());
            Util::group(&outValues, LongElement::CHUNK_BITWIDTH)
        }
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
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
            let nBits = self.t.n.bits().max(1);
            let nWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                nBits as i32,
                &Some("n".to_owned()),
            );
            let plainWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.plain.bits().max(1) as i32,
                &Some("plain".to_owned()),
            );
            let randomWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.random.bits().max(1) as i32,
                &Some("random".to_owned()),
            );
            let enc = ZkayPaillierFastEncGadget::new(
                nWire.clone(),
                nBits as i32,
                plainWire.clone(),
                randomWire.clone(),
                &None,
                self.cg(),
            );
            CircuitGenerator::makeOutputArray(
                self.cg(),
                enc.getOutputWires(),
                &Some("cipher".to_owned()),
            );
            (self.t.nWire, self.t.plainWire, self.t.randomWire) =
                (Some(nWire), Some(plainWire), Some(randomWire));
        }

        fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValuebi(
                self.t.nWire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.plainWire.as_ref().unwrap(),
                &self.t.plain,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.randomWire.as_ref().unwrap(),
                &self.t.random,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierFastEncCircuitGenerator> {
        pub fn computeResult(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.evalCircuit().unwrap();

            let outValues = evaluator.getWiresValues(&self.get_out_wires());
            Util::group(&outValues, LongElement::CHUNK_BITWIDTH)
        }
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
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
            let nBits = self.t.n.bits().max(1);
            let nWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                nBits as i32,
                &Some("n".to_owned()),
            );
            let lambdaWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.lambda.bits().max(1) as i32,
                &Some("lambda".to_owned()),
            );
            let cipherWire = CircuitGenerator::createLongElementInput(
                self.cg(),
                self.t.cipher.bits().max(1) as i32,
                &Some("cipher".to_owned()),
            );
            let dec = ZkayPaillierFastDecGadget::new(
                nWire.clone(),
                nBits as i32,
                lambdaWire.clone(),
                cipherWire.clone(),
                &None,
                self.cg(),
            );
            CircuitGenerator::makeOutputArray(
                self.cg(),
                dec.getOutputWires(),
                &Some("plain".to_owned()),
            );
            (self.t.nWire, self.t.lambdaWire, self.t.cipherWire) =
                (Some(nWire), Some(lambdaWire), Some(cipherWire));
        }

        fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.setWireValuebi(
                self.t.nWire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.lambdaWire.as_ref().unwrap(),
                &self.t.lambda,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.setWireValuebi(
                self.t.cipherWire.as_ref().unwrap(),
                &self.t.cipher,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierFastDecCircuitGenerator> {
        pub fn computeResult(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generateCircuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.evalCircuit().unwrap();

            let outValues = evaluator.getWiresValues(&self.get_out_wires());
            Util::group(&outValues, LongElement::CHUNK_BITWIDTH)
        }
    }
}
