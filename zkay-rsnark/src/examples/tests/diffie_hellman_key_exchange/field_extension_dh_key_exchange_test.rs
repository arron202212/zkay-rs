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
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire_type::WireType,
        },
    },
    examples::gadgets::diffie_hellman_key_exchange::field_extension_dh_key_exchange::FieldExtensionDHKeyExchange,
    util::util::{BigInteger, Util},
};

use zkay_derive::ImplStructNameConfig;

//  * Tests Key Exchange via Field Extension Gadget (DHKeyExchangeGadget.java)
//  * Parameters used here assumes ~80-bit security

#[cfg(test)]
mod test {
    use super::*;
    pub const mu: usize = 4;
    pub const omega: usize = 7;
    pub const EXPONENT_BIT_LENGTH: usize = 397;
    // This is a very simple example for testing purposes. To see how key exchange gadgets could be used,
    // check the HybridEncryptionCircuitGenerator

    // The sage script to compute the sample case is commented in the end of the file.
    #[derive(Debug, Clone, ImplStructNameConfig)]
    struct CGTest {
        exponent_bits: Vec<Option<WireType>>,
        g: Vec<Option<WireType>>,
        h: Vec<Option<WireType>>,
    }
    impl CGTest {
        pub fn new(name: &str) -> CircuitGeneratorExtend<Self> {
            CircuitGeneratorExtend::<CGTest>::new(
                name,
                Self {
                    exponent_bits: vec![],
                    g: vec![],
                    h: vec![],
                },
            )
        }
    }
    crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
    #[test]
    pub fn test_hardcoded_keys() {
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let exponent_bits = CircuitGenerator::create_input_wire_array_with_str(
                    self.cg(),
                    EXPONENT_BIT_LENGTH,
                    "exponent",
                );

                let ccw = |s: &str| {
                    Some(CircuitGenerator::create_constant_wire(
                        self.cg(),
                        &Util::parse_big_int(s),
                    ))
                };
                // Hardcode the base and the other party's key (suitable when keys are not expected to change)

                let g: Vec<_> = [
                    "16377448892084713529161739182205318095580119111576802375181616547062197291263",
                    "13687683608888423916085091250849188813359145430644908352977567823030408967189",
                    "12629166084120705167185476169390021031074363183264910102253898080559854363106",
                    "19441276922979928804860196077335093208498949640381586557241379549605420212272",
                ]
                .into_iter()
                .map(ccw)
                .collect();

                let h: Vec<_> = [
                    "8252578783913909531884765397785803733246236629821369091076513527284845891757",
                    "20829599225781884356477513064431048695774529855095864514701692089787151865093",
                    "1540379511125324102377803754608881114249455137236500477169164628692514244862",
                    "1294177986177175279602421915789749270823809536595962994745244158374705688266",
                ]
                .into_iter()
                .map(ccw)
                .collect();

                let field_extension_dh_key_exchange = FieldExtensionDHKeyExchange::new(
                    g,
                    h,
                    exponent_bits.clone(),
                    omega as i64,
                    &None,
                    self.cg(),
                );

                let g_to_s = field_extension_dh_key_exchange.get_output_public_value();
                CircuitGenerator::make_output_array_with_str(
                    self.cg(),
                    g_to_s,
                    "DH Key Exchange Output",
                );
                let h_to_s = field_extension_dh_key_exchange.get_shared_secret();
                CircuitGenerator::make_output_array_with_str(
                    self.cg(),
                    h_to_s,
                    "Derived Secret Key",
                );
                self.t.exponent_bits = exponent_bits;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                let exponent = Util::parse_big_int(
                    "151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515",
                );
                for i in 0..EXPONENT_BIT_LENGTH {
                    evaluator.set_wire_valuei(
                        self.t.exponent_bits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };

        let mut generator = CGTest::new("FieldExtension_Test1");
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let output = generator.get_out_wires();

        assert!(
            [
                "9327289243415079515318132023689497171271904433099600200400859968177425894580",
                "21312311033900790023937954575527091756377215260488498667283640904465223526236",
                "19883079534945520345012965173409210670280801176341700376612297932480562491904",
                "11262499765857836098986663841690204003097813561305051025968110590253003094192",
                "2202294410438304085016660740566673536814787951643742901558895317916637664703",
                "18724398730888665000453307259637219298475373267590805228665739285983831525279",
                "21875304682329937834628267681832507202983143541480299478306965773109713498819",
                "12006400062454647262588139453308241334465382550157910424084838650858146672647"
            ]
            .into_iter()
            .enumerate()
            .all(|(i, s)| Util::parse_big_int(s)
                == evaluator.get_wire_value(output[i].as_ref().unwrap()))
        );
    }

    #[test]
    pub fn test_variable_keys() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            exponent_bits: Vec<Option<WireType>>,
            g: Vec<Option<WireType>>,
            h: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let exponent_bits = CircuitGenerator::create_input_wire_array_with_str(
                    self.cg(),
                    EXPONENT_BIT_LENGTH,
                    "exponent",
                );

                let mut g = CircuitGenerator::create_input_wire_array(self.cg(), mu);
                let mut h = CircuitGenerator::create_input_wire_array(self.cg(), mu);

                let field_extension_dh_key_exchange = FieldExtensionDHKeyExchange::new(
                    g.clone(),
                    h.clone(),
                    exponent_bits.clone(),
                    omega as i64,
                    &None,
                    self.cg(),
                );

                let g_to_s = field_extension_dh_key_exchange.get_output_public_value();
                CircuitGenerator::make_output_array_with_str(
                    self.cg(),
                    g_to_s,
                    "DH Key Exchange Output",
                );
                let h_to_s = field_extension_dh_key_exchange.get_shared_secret();
                CircuitGenerator::make_output_array_with_str(
                    self.cg(),
                    h_to_s,
                    "Derived Secret Key",
                );
                (self.t.exponent_bits, self.t.g, self.t.h) = (exponent_bits, g, h);
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                [
                    "16377448892084713529161739182205318095580119111576802375181616547062197291263",
                    "13687683608888423916085091250849188813359145430644908352977567823030408967189",
                    "12629166084120705167185476169390021031074363183264910102253898080559854363106",
                    "19441276922979928804860196077335093208498949640381586557241379549605420212272",
                ]
                .into_iter()
                .enumerate()
                .for_each(|(i, s)| {
                    evaluator
                        .set_wire_value(self.t.g[i].as_ref().unwrap(), &Util::parse_big_int(s));
                });

                [
                    "8252578783913909531884765397785803733246236629821369091076513527284845891757",
                    "20829599225781884356477513064431048695774529855095864514701692089787151865093",
                    "1540379511125324102377803754608881114249455137236500477169164628692514244862",
                    "1294177986177175279602421915789749270823809536595962994745244158374705688266",
                ]
                .into_iter()
                .enumerate()
                .for_each(|(i, s)| {
                    evaluator
                        .set_wire_value(self.t.h[i].as_ref().unwrap(), &Util::parse_big_int(s));
                });
                let exponent = Util::parse_big_int(
                    "151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515",
                );
                for i in 0..EXPONENT_BIT_LENGTH {
                    evaluator.set_wire_valuei(
                        self.t.exponent_bits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };

        let t = CGTest {
            exponent_bits: vec![],
            g: vec![],
            h: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("FieldExtension_Test2", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let output = generator.get_out_wires();

        assert!(
            [
                "9327289243415079515318132023689497171271904433099600200400859968177425894580",
                "21312311033900790023937954575527091756377215260488498667283640904465223526236",
                "19883079534945520345012965173409210670280801176341700376612297932480562491904",
                "11262499765857836098986663841690204003097813561305051025968110590253003094192",
                "2202294410438304085016660740566673536814787951643742901558895317916637664703",
                "18724398730888665000453307259637219298475373267590805228665739285983831525279",
                "21875304682329937834628267681832507202983143541480299478306965773109713498819",
                "12006400062454647262588139453308241334465382550157910424084838650858146672647"
            ]
            .into_iter()
            .enumerate()
            .all(|(i, s)| Util::parse_big_int(s)
                == evaluator.get_wire_value(output[i].as_ref().unwrap()))
        );
    }

    #[test]
    pub fn test_fedhke_input_validation() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            exponent_bits: Vec<Option<WireType>>,
            g: Vec<Option<WireType>>,
            h: Vec<Option<WireType>>,
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let exponent_bits = CircuitGenerator::create_input_wire_array_with_str(
                    self.cg(),
                    EXPONENT_BIT_LENGTH,
                    "exponent",
                );

                let mut g = CircuitGenerator::create_input_wire_array(self.cg(), mu);
                let mut h = CircuitGenerator::create_input_wire_array(self.cg(), mu);

                let field_extension_dh_key_exchange = FieldExtensionDHKeyExchange::new(
                    g.clone(),
                    h.clone(),
                    exponent_bits.clone(),
                    omega as i64,
                    &None,
                    self.cg(),
                );

                // provide prime order subgroup
                field_extension_dh_key_exchange.validate_inputs(Util::parse_big_int("566003748421165623973140684210338877916630960782201693595769129706864925719318115473892932098619423042929922932476493069"));

                let g_to_s = field_extension_dh_key_exchange.get_output_public_value();
                CircuitGenerator::make_output_array_with_str(
                    self.cg(),
                    g_to_s,
                    "DH Key Exchange Output",
                );
                let h_to_s = field_extension_dh_key_exchange.get_shared_secret();
                CircuitGenerator::make_output_array_with_str(
                    self.cg(),
                    h_to_s,
                    "Derived Secret Key",
                );
                (self.t.exponent_bits, self.t.g, self.t.h) = (exponent_bits, g, h);
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                [
                    "16377448892084713529161739182205318095580119111576802375181616547062197291263",
                    "13687683608888423916085091250849188813359145430644908352977567823030408967189",
                    "12629166084120705167185476169390021031074363183264910102253898080559854363106",
                    "19441276922979928804860196077335093208498949640381586557241379549605420212272",
                ]
                .into_iter()
                .enumerate()
                .for_each(|(i, s)| {
                    evaluator
                        .set_wire_value(self.t.g[i].as_ref().unwrap(), &Util::parse_big_int(s));
                });

                [
                    "8252578783913909531884765397785803733246236629821369091076513527284845891757",
                    "20829599225781884356477513064431048695774529855095864514701692089787151865093",
                    "1540379511125324102377803754608881114249455137236500477169164628692514244862",
                    "1294177986177175279602421915789749270823809536595962994745244158374705688266",
                ]
                .into_iter()
                .enumerate()
                .for_each(|(i, s)| {
                    evaluator
                        .set_wire_value(self.t.h[i].as_ref().unwrap(), &Util::parse_big_int(s));
                });

                let exponent = Util::parse_big_int(
                    "151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515",
                );
                for i in 0..EXPONENT_BIT_LENGTH {
                    evaluator.set_wire_valuei(
                        self.t.exponent_bits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            exponent_bits: vec![],
            g: vec![],
            h: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("FieldExtension_Test3", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let output = generator.get_out_wires();
        assert!(
            [
                "9327289243415079515318132023689497171271904433099600200400859968177425894580",
                "21312311033900790023937954575527091756377215260488498667283640904465223526236",
                "19883079534945520345012965173409210670280801176341700376612297932480562491904",
                "11262499765857836098986663841690204003097813561305051025968110590253003094192",
                "2202294410438304085016660740566673536814787951643742901558895317916637664703",
                "18724398730888665000453307259637219298475373267590805228665739285983831525279",
                "21875304682329937834628267681832507202983143541480299478306965773109713498819",
                "12006400062454647262588139453308241334465382550157910424084838650858146672647"
            ]
            .into_iter()
            .enumerate()
            .all(|(i, s)| Util::parse_big_int(s)
                == evaluator.get_wire_value(output[i].as_ref().unwrap()))
        );
    }

    //  Sage Script generating the above values:
    //    F.<x> = GF(21888242871839275222246405745257275088548364400416034343698204186575808495617)[]
    //    K.<a> = GF(21888242871839275222246405745257275088548364400416034343698204186575808495617**4, name='a', modulus=x^4-7)

    //    base = 16377448892084713529161739182205318095580119111576802375181616547062197291263*a^0 + 13687683608888423916085091250849188813359145430644908352977567823030408967189*a^1 + 12629166084120705167185476169390021031074363183264910102253898080559854363106*a^2 + 19441276922979928804860196077335093208498949640381586557241379549605420212272*a^3
    //    h = 1294177986177175279602421915789749270823809536595962994745244158374705688266*a^3 + 1540379511125324102377803754608881114249455137236500477169164628692514244862*a^2 + 20829599225781884356477513064431048695774529855095864514701692089787151865093*a + 8252578783913909531884765397785803733246236629821369091076513527284845891757

    //    baseOrder = base.multiplicative_order()
    //    hOrder = h.multiplicative_order()
    //    print(baseOrder)
    //    print(hOrder)
    //    print(is_prime(baseOrder))

    //    secret = 15403795111253241023778037546088811142494551372365004771691646286925142448620047716
    //    base_to_secret = base^secret
    //    h_to_secret = h^secret
    //    print(base_to_secret)
    //    print(h_to_secret)
}
