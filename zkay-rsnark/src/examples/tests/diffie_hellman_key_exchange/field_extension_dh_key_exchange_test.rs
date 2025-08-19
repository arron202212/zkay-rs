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
    CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
    addToEvaluationQueue, getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::diffie_hellman_key_exchange::field_extension_dh_key_exchange::FieldExtensionDHKeyExchange;
use crate::util::util::BigInteger;
use zkay_derive::ImplStructNameConfig;
/**
 * Tests Key Exchange via Field Extension Gadget (DHKeyExchangeGadget.java)
 * Parameters used here assumes ~80-bit security
 */
#[cfg(test)]
mod test {
    use super::*;
    pub const mu: usize = 4;
    pub const omega: usize = 7;
    pub const exponentBitlength: usize = 397;
    // This is a very simple example for testing purposes. To see how key exchange gadgets could be used,
    // check the HybridEncryptionCircuitGenerator

    // The sage script to compute the sample case is commented in the end of the file.

    #[test]
    pub fn test_hardcoded_keys() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            exponentBits: Vec<Option<WireType>>,
        }
        impl CGTest {
            const mu: usize = 4;
            const omega: usize = 7;
            const exponentBitlength: usize = 397;
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let exponentBits = CircuitGenerator::createInputWireArray(
                    self.cg(),
                    exponentBitlength,
                    &Some("exponent".to_owned()),
                );

                let mut g = vec![None; CGTest::mu];
                let mut h = vec![None; CGTest::mu];

                let ccw = |s: &str| {
                    Some(self.createConstantWire(BigInteger::parse_bytes(
                    b"16377448892084713529161739182205318095580119111576802375181616547062197291263",10
                ).as_ref().unwrap(),&None))
                };
                // Hardcode the base and the other party's key (suitable when keys are not expected to change)
                g[0] = ccw(
                    "16377448892084713529161739182205318095580119111576802375181616547062197291263",
                );

                g[1] = ccw(
                    "13687683608888423916085091250849188813359145430644908352977567823030408967189",
                );
                g[2] = ccw(
                    "12629166084120705167185476169390021031074363183264910102253898080559854363106",
                );
                g[3] = ccw(
                    "19441276922979928804860196077335093208498949640381586557241379549605420212272",
                );

                h[0] = ccw(
                    "8252578783913909531884765397785803733246236629821369091076513527284845891757",
                );
                h[1] = ccw(
                    "20829599225781884356477513064431048695774529855095864514701692089787151865093",
                );
                h[2] = ccw(
                    "1540379511125324102377803754608881114249455137236500477169164628692514244862",
                );
                h[3] = ccw(
                    "1294177986177175279602421915789749270823809536595962994745244158374705688266",
                );

                let fieldExtensionDHKeyExchange = FieldExtensionDHKeyExchange::new(
                    g,
                    h,
                    exponentBits.clone(),
                    omega as i64,
                    &None,
                    self.cg(),
                );

                let g_to_s = fieldExtensionDHKeyExchange.getOutputPublicValue();
                CircuitGenerator::makeOutputArray(
                    self.cg(),
                    g_to_s,
                    &Some("DH Key Exchange Output".to_owned()),
                );
                let h_to_s = fieldExtensionDHKeyExchange.getSharedSecret();
                CircuitGenerator::makeOutputArray(
                    self.cg(),
                    h_to_s,
                    &Some("Derived Secret Key".to_owned()),
                );
                self.t.exponentBits = exponentBits;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let exponent =BigInteger::parse_bytes(
                    b"151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515",10
                ).unwrap();
                for i in 0..exponentBitlength {
                    evaluator.setWireValuei(
                        self.t.exponentBits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            exponentBits: vec![],
        };

        let mut generator = CircuitGeneratorExtend::<CGTest>::new("FieldExtension_Test1", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();

        let output = generator.get_out_wires();

        assert_eq!(
            evaluator.getWireValue(output[0].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"9327289243415079515318132023689497171271904433099600200400859968177425894580",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[1].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"21312311033900790023937954575527091756377215260488498667283640904465223526236",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[2].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"19883079534945520345012965173409210670280801176341700376612297932480562491904",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[3].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"11262499765857836098986663841690204003097813561305051025968110590253003094192",
                10
            )
            .unwrap(),
        );

        assert_eq!(
            evaluator.getWireValue(output[4].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"2202294410438304085016660740566673536814787951643742901558895317916637664703",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[5].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"18724398730888665000453307259637219298475373267590805228665739285983831525279",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[6].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"21875304682329937834628267681832507202983143541480299478306965773109713498819",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[7].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"12006400062454647262588139453308241334465382550157910424084838650858146672647",
                10
            )
            .unwrap(),
        );
    }

    #[test]
    pub fn test_variable_keys() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            exponentBits: Vec<Option<WireType>>,
            g: Vec<Option<WireType>>,
            h: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let exponentBits = CircuitGenerator::createInputWireArray(
                    self.cg(),
                    exponentBitlength,
                    &Some("exponent".to_owned()),
                );

                let mut g = CircuitGenerator::createInputWireArray(self.cg(), mu, &None);
                let mut h = CircuitGenerator::createInputWireArray(self.cg(), mu, &None);

                let fieldExtensionDHKeyExchange = FieldExtensionDHKeyExchange::new(
                    g,
                    h,
                    exponentBits.clone(),
                    omega as i64,
                    &None,
                    self.cg(),
                );

                let g_to_s = fieldExtensionDHKeyExchange.getOutputPublicValue();
                CircuitGenerator::makeOutputArray(
                    self.cg(),
                    g_to_s,
                    &Some("DH Key Exchange Output".to_owned()),
                );
                let h_to_s = fieldExtensionDHKeyExchange.getSharedSecret();
                CircuitGenerator::makeOutputArray(
                    self.cg(),
                    h_to_s,
                    &Some("Derived Secret Key".to_owned()),
                );
                self.t.exponentBits = exponentBits;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValue(self.t.g[0].as_ref().unwrap(),BigInteger::parse_bytes(b"16377448892084713529161739182205318095580119111576802375181616547062197291263",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.g[1].as_ref().unwrap(),BigInteger::parse_bytes(b"13687683608888423916085091250849188813359145430644908352977567823030408967189",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.g[2].as_ref().unwrap(),BigInteger::parse_bytes(b"12629166084120705167185476169390021031074363183264910102253898080559854363106",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.g[3].as_ref().unwrap(),BigInteger::parse_bytes(b"19441276922979928804860196077335093208498949640381586557241379549605420212272",10).as_ref().unwrap());

                evaluator.setWireValue(self.t.h[0].as_ref().unwrap(),BigInteger::parse_bytes(b"8252578783913909531884765397785803733246236629821369091076513527284845891757",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.h[1].as_ref().unwrap(),BigInteger::parse_bytes(b"20829599225781884356477513064431048695774529855095864514701692089787151865093",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.h[2].as_ref().unwrap(),BigInteger::parse_bytes(b"1540379511125324102377803754608881114249455137236500477169164628692514244862",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.h[3].as_ref().unwrap(),BigInteger::parse_bytes(b"1294177986177175279602421915789749270823809536595962994745244158374705688266",10).as_ref().unwrap());

                let exponent =BigInteger::parse_bytes(
                    b"151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515",10
                ).unwrap();
                for i in 0..exponentBitlength {
                    evaluator.setWireValuei(
                        self.t.exponentBits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };

        impl CGTest {
            const mu: usize = 4;
            const omega: usize = 7;
            const exponentBitlength: usize = 397;
        }
        let t = CGTest {
            exponentBits: vec![],
            g: vec![],
            h: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("FieldExtension_Test2", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();

        let output = generator.get_out_wires();

        assert_eq!(
            evaluator.getWireValue(output[0].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"9327289243415079515318132023689497171271904433099600200400859968177425894580",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[1].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"21312311033900790023937954575527091756377215260488498667283640904465223526236",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[2].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"19883079534945520345012965173409210670280801176341700376612297932480562491904",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[3].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"11262499765857836098986663841690204003097813561305051025968110590253003094192",
                10
            )
            .unwrap(),
        );

        assert_eq!(
            evaluator.getWireValue(output[4].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"2202294410438304085016660740566673536814787951643742901558895317916637664703",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[5].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"18724398730888665000453307259637219298475373267590805228665739285983831525279",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[6].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"21875304682329937834628267681832507202983143541480299478306965773109713498819",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[7].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"12006400062454647262588139453308241334465382550157910424084838650858146672647",
                10
            )
            .unwrap(),
        );
    }

    #[test]
    pub fn test_input_validation() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            exponentBits: Vec<Option<WireType>>,
            g: Vec<Option<WireType>>,
            h: Vec<Option<WireType>>,
        }
        impl CGTest {
            const mu: usize = 4;
            const omega: usize = 7;
            const exponentBitlength: usize = 397;
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let exponentBits = CircuitGenerator::createInputWireArray(
                    self.cg(),
                    exponentBitlength,
                    &Some("exponent".to_owned()),
                );

                let mut g = CircuitGenerator::createInputWireArray(self.cg(), mu, &None);
                let mut h = CircuitGenerator::createInputWireArray(self.cg(), mu, &None);

                let fieldExtensionDHKeyExchange = FieldExtensionDHKeyExchange::new(
                    g,
                    h,
                    exponentBits.clone(),
                    omega as i64,
                    &None,
                    self.cg(),
                );

                // provide prime order subgroup
                fieldExtensionDHKeyExchange.validateInputs(BigInteger::parse_bytes(b"566003748421165623973140684210338877916630960782201693595769129706864925719318115473892932098619423042929922932476493069",10).unwrap());

                let g_to_s = fieldExtensionDHKeyExchange.getOutputPublicValue();
                CircuitGenerator::makeOutputArray(
                    self.cg(),
                    g_to_s,
                    &Some("DH Key Exchange Output".to_owned()),
                );
                let h_to_s = fieldExtensionDHKeyExchange.getSharedSecret();
                CircuitGenerator::makeOutputArray(
                    self.cg(),
                    h_to_s,
                    &Some("Derived Secret Key".to_owned()),
                );
                self.t.exponentBits = exponentBits;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValue(self.t.g[0].as_ref().unwrap(),BigInteger::parse_bytes(b"16377448892084713529161739182205318095580119111576802375181616547062197291263",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.g[1].as_ref().unwrap(),BigInteger::parse_bytes(b"13687683608888423916085091250849188813359145430644908352977567823030408967189",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.g[2].as_ref().unwrap(),BigInteger::parse_bytes(b"12629166084120705167185476169390021031074363183264910102253898080559854363106",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.g[3].as_ref().unwrap(),BigInteger::parse_bytes(b"19441276922979928804860196077335093208498949640381586557241379549605420212272",10).as_ref().unwrap());

                evaluator.setWireValue(self.t.h[0].as_ref().unwrap(),BigInteger::parse_bytes(b"8252578783913909531884765397785803733246236629821369091076513527284845891757",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.h[1].as_ref().unwrap(),BigInteger::parse_bytes(b"20829599225781884356477513064431048695774529855095864514701692089787151865093",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.h[2].as_ref().unwrap(),BigInteger::parse_bytes(b"1540379511125324102377803754608881114249455137236500477169164628692514244862",10).as_ref().unwrap());
                evaluator.setWireValue(self.t.h[3].as_ref().unwrap(),BigInteger::parse_bytes(b"1294177986177175279602421915789749270823809536595962994745244158374705688266",10).as_ref().unwrap());

                let exponent =BigInteger::parse_bytes(
                    b"151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515",10
                ).unwrap();
                for i in 0..exponentBitlength {
                    evaluator.setWireValuei(
                        self.t.exponentBits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            exponentBits: vec![],
            g: vec![],
            h: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("FieldExtension_Test3", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();

        let output = generator.get_out_wires();

        assert_eq!(
            evaluator.getWireValue(output[0].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"9327289243415079515318132023689497171271904433099600200400859968177425894580",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[1].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"21312311033900790023937954575527091756377215260488498667283640904465223526236",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[2].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"19883079534945520345012965173409210670280801176341700376612297932480562491904",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[3].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"11262499765857836098986663841690204003097813561305051025968110590253003094192",
                10
            )
            .unwrap(),
        );

        assert_eq!(
            evaluator.getWireValue(output[4].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"2202294410438304085016660740566673536814787951643742901558895317916637664703",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[5].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"18724398730888665000453307259637219298475373267590805228665739285983831525279",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[6].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"21875304682329937834628267681832507202983143541480299478306965773109713498819",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[7].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"12006400062454647262588139453308241334465382550157910424084838650858146672647",
                10
            )
            .unwrap(),
        );
    }

    /* Sage Script generating the above values:
       F.<x> = GF(21888242871839275222246405745257275088548364400416034343698204186575808495617)[]
       K.<a> = GF(21888242871839275222246405745257275088548364400416034343698204186575808495617**4, name='a', modulus=x^4-7)

       base = 16377448892084713529161739182205318095580119111576802375181616547062197291263*a^0 + 13687683608888423916085091250849188813359145430644908352977567823030408967189*a^1 + 12629166084120705167185476169390021031074363183264910102253898080559854363106*a^2 + 19441276922979928804860196077335093208498949640381586557241379549605420212272*a^3
       h = 1294177986177175279602421915789749270823809536595962994745244158374705688266*a^3 + 1540379511125324102377803754608881114249455137236500477169164628692514244862*a^2 + 20829599225781884356477513064431048695774529855095864514701692089787151865093*a + 8252578783913909531884765397785803733246236629821369091076513527284845891757

       baseOrder = base.multiplicative_order()
       hOrder = h.multiplicative_order()
       print(baseOrder)
       print(hOrder)
       print(is_prime(baseOrder))

       secret = 15403795111253241023778037546088811142494551372365004771691646286925142448620047716
       base_to_secret = base^secret
       h_to_secret = h^secret
       print(base_to_secret)
       print(h_to_secret)
    */
}
