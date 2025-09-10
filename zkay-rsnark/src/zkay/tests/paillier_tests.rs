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
        auxiliary::long_element::LongElement,
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
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
    pub fn test_encryption_example() {
        let plain = BigInteger::from(42);
        let random = BigInteger::from(25);
        let n = BigInteger::from(9047);
        let mut generator = BigInteger::from(27);
        let mut enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
        let cipher = enc.compute_result();
        assert_eq!(BigInteger::from(45106492), cipher);
    }

    #[test]
    pub fn test_decryption_example() {
        let n = BigInteger::from(9047);
        let cipher = BigInteger::from(2587834);
        let lambda = BigInteger::from(4428);
        let mu = BigInteger::from(1680);
        let mut dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
        let plain = dec.compute_result();
        assert_eq!(BigInteger::from(55), plain);
    }

    #[test]
    pub fn test256_bit_encryption() {
        let plain =
            pbi("58620521968995858419238449046464883186412581610038046858008683322252437292505");

        let random =
            pbi("66895129274476067543864711343178574027057505369800972938068894913816799963509");

        let n =
            pbi("71705678335151044143714697909938764102247769560297862447809589632641441407751");

        let mut generator = BigInteger::from(27);
        let mut enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
        let cipher = enc.compute_result();
        assert_eq!(
            pbi(
                "3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963"
            ),
            cipher
        );
    }

    #[test]
    pub fn test256_bit_decryption() {
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
        let plain = dec.compute_result();
        assert_eq!(
            pbi("58620521968995858419238449046464883186412581610038046858008683322252437292505"),
            plain
        );
    }

    #[test]
    pub fn test256_bit_fast_encryption() {
        let plain =
            pbi("58620521968995858419238449046464883186412581610038046858008683322252437292505");

        let random =
            pbi("66895129274476067543864711343178574027057505369800972938068894913816799963509");

        let n =
            pbi("71705678335151044143714697909938764102247769560297862447809589632641441407751");

        let mut enc = PaillierFastEncCircuitGenerator::new("Paillier Enc", n, plain, random);
        let cipher = enc.compute_result();
        assert_eq!(
            pbi(
                "3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136"
            ),
            cipher
        );
    }

    #[test]
    pub fn test256_bit_fast_decryption() {
        let n =
            pbi("71705678335151044143714697909938764102247769560297862447809589632641441407751");

        let lambda =
            pbi("71705678335151044143714697909938764101708369395657657259621793199496430472072");

        let cipher = pbi(
            "3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136",
        );
        let mut dec = PaillierFastDecCircuitGenerator::new("Paillier Dec", n, lambda, cipher);
        let plain = dec.compute_result();
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
        plain_wire: Option<LongElement>,
        random_wire: Option<LongElement>,
        n_wire: Option<LongElement>,
        generator_wire: Option<LongElement>,
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
                    plain_wire: None,
                    random_wire: None,
                    n_wire: None,
                    generator_wire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierEncCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierEncCircuitGenerator> {
        fn build_circuit(&mut self) {
            let plain_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.plain.bits().max(1) as i32,
                "plain",
            );
            let random_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.random.bits().max(1) as i32,
                "random",
            );
            let n_bits = self.t.n.bits().max(1);
            let n_wire =
                CircuitGenerator::create_long_element_input_with_str(self.cg(), n_bits as i32, "n");

            let generator_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.generator.bits().max(1) as i32,
                "generator",
            );
            let enc = ZkayPaillierEncGadget::new(
                n_wire.clone(),
                n_bits as i32,
                generator_wire.clone(),
                plain_wire.clone(),
                random_wire.clone(),
                self.cg(),
            );
            CircuitGenerator::make_output_array_with_str(
                self.cg(),
                enc.get_output_wires(),
                "cipher",
            );
            (
                self.t.n_wire,
                self.t.generator_wire,
                self.t.random_wire,
                self.t.plain_wire,
            ) = (
                Some(n_wire),
                Some(generator_wire),
                Some(random_wire),
                Some(plain_wire),
            );
        }

        fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.set_wire_valuebi(
                self.t.plain_wire.as_ref().unwrap(),
                &self.t.plain,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.random_wire.as_ref().unwrap(),
                &self.t.random,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.n_wire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.generator_wire.as_ref().unwrap(),
                &self.t.generator,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierEncCircuitGenerator> {
        pub fn compute_result(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generate_circuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.eval_circuit().unwrap();

            let out_values = evaluator.get_wires_values(&self.get_out_wires());
            Util::group(&out_values, LongElement::CHUNK_BITWIDTH)
        }
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct PaillierDecCircuitGenerator {
        cipher: BigInteger,
        n: BigInteger,
        lambda: BigInteger,
        mu: BigInteger,
        cipher_wire: Option<LongElement>,
        n_wire: Option<LongElement>,
        lambda_wire: Option<LongElement>,
        mu_wire: Option<LongElement>,
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
                    cipher_wire: None,
                    n_wire: None,
                    lambda_wire: None,
                    mu_wire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierDecCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierDecCircuitGenerator> {
        fn build_circuit(&mut self) {
            let cipher_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.cipher.bits().max(1) as i32,
                "cipher",
            );
            let n_bits = self.t.n.bits().max(1);
            let n_wire =
                CircuitGenerator::create_long_element_input_with_str(self.cg(), n_bits as i32, "n");
            let lambda_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.lambda.bits().max(1) as i32,
                "lambda",
            );
            let mu_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.mu.bits().max(1) as i32,
                "mu",
            );
            let dec = ZkayPaillierDecGadget::new(
                n_wire.clone(),
                n_bits as i32,
                lambda_wire.clone(),
                mu_wire.clone(),
                cipher_wire.clone(),
                self.cg(),
            );
            CircuitGenerator::make_output_array_with_str(
                self.cg(),
                dec.get_output_wires(),
                "plain",
            );
            (
                self.t.cipher_wire,
                self.t.n_wire,
                self.t.lambda_wire,
                self.t.mu_wire,
            ) = (
                Some(cipher_wire),
                Some(n_wire),
                Some(lambda_wire),
                Some(mu_wire),
            );
        }

        fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.set_wire_valuebi(
                self.t.cipher_wire.as_ref().unwrap(),
                &self.t.cipher,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.n_wire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.lambda_wire.as_ref().unwrap(),
                &self.t.lambda,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.mu_wire.as_ref().unwrap(),
                &self.t.mu,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierDecCircuitGenerator> {
        pub fn compute_result(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generate_circuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.eval_circuit().unwrap();

            let out_values = evaluator.get_wires_values(&self.get_out_wires());
            Util::group(&out_values, LongElement::CHUNK_BITWIDTH)
        }
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct PaillierFastEncCircuitGenerator {
        n: BigInteger,
        plain: BigInteger,
        random: BigInteger,
        n_wire: Option<LongElement>,
        plain_wire: Option<LongElement>,
        random_wire: Option<LongElement>,
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
                    n_wire: None,
                    plain_wire: None,
                    random_wire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierFastEncCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierFastEncCircuitGenerator> {
        fn build_circuit(&mut self) {
            let n_bits = self.t.n.bits().max(1);
            let n_wire =
                CircuitGenerator::create_long_element_input_with_str(self.cg(), n_bits as i32, "n");
            let plain_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.plain.bits().max(1) as i32,
                "plain",
            );
            let random_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.random.bits().max(1) as i32,
                "random",
            );
            let enc = ZkayPaillierFastEncGadget::new(
                n_wire.clone(),
                n_bits as i32,
                plain_wire.clone(),
                random_wire.clone(),
                self.cg(),
            );
            CircuitGenerator::make_output_array_with_str(
                self.cg(),
                enc.get_output_wires(),
                "cipher",
            );
            (self.t.n_wire, self.t.plain_wire, self.t.random_wire) =
                (Some(n_wire), Some(plain_wire), Some(random_wire));
        }

        fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.set_wire_valuebi(
                self.t.n_wire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.plain_wire.as_ref().unwrap(),
                &self.t.plain,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.random_wire.as_ref().unwrap(),
                &self.t.random,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierFastEncCircuitGenerator> {
        pub fn compute_result(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generate_circuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.eval_circuit().unwrap();

            let out_values = evaluator.get_wires_values(&self.get_out_wires());
            Util::group(&out_values, LongElement::CHUNK_BITWIDTH)
        }
    }
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct PaillierFastDecCircuitGenerator {
        n: BigInteger,
        lambda: BigInteger,
        cipher: BigInteger,
        n_wire: Option<LongElement>,
        lambda_wire: Option<LongElement>,
        cipher_wire: Option<LongElement>,
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
                    n_wire: None,
                    lambda_wire: None,
                    cipher_wire: None,
                },
            )
        }
    }

    crate::impl_struct_name_for!(CircuitGeneratorExtend<PaillierFastDecCircuitGenerator>);
    impl CGConfig for CircuitGeneratorExtend<PaillierFastDecCircuitGenerator> {
        fn build_circuit(&mut self) {
            let n_bits = self.t.n.bits().max(1);
            let n_wire =
                CircuitGenerator::create_long_element_input_with_str(self.cg(), n_bits as i32, "n");
            let lambda_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.lambda.bits().max(1) as i32,
                "lambda",
            );
            let cipher_wire = CircuitGenerator::create_long_element_input_with_str(
                self.cg(),
                self.t.cipher.bits().max(1) as i32,
                "cipher",
            );
            let dec = ZkayPaillierFastDecGadget::new(
                n_wire.clone(),
                n_bits as i32,
                lambda_wire.clone(),
                cipher_wire.clone(),
                self.cg(),
            );
            CircuitGenerator::make_output_array_with_str(
                self.cg(),
                dec.get_output_wires(),
                "plain",
            );
            (self.t.n_wire, self.t.lambda_wire, self.t.cipher_wire) =
                (Some(n_wire), Some(lambda_wire), Some(cipher_wire));
        }

        fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
            evaluator.set_wire_valuebi(
                self.t.n_wire.as_ref().unwrap(),
                &self.t.n,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.lambda_wire.as_ref().unwrap(),
                &self.t.lambda,
                LongElement::CHUNK_BITWIDTH,
            );
            evaluator.set_wire_valuebi(
                self.t.cipher_wire.as_ref().unwrap(),
                &self.t.cipher,
                LongElement::CHUNK_BITWIDTH,
            );
        }
    }
    impl CircuitGeneratorExtend<PaillierFastDecCircuitGenerator> {
        pub fn compute_result(&mut self) -> BigInteger {
            // let t1 = System.nanoTime();
            self.generate_circuit();
            // let t2 = System.nanoTime();
            // let ms = 1.e-6 * (t2 - t1);
            // System.out.format("Building took %.3f ms\n", ms);
            let evaluator = self.eval_circuit().unwrap();

            let out_values = evaluator.get_wires_values(&self.get_out_wires());
            Util::group(&out_values, LongElement::CHUNK_BITWIDTH)
        }
    }
}
