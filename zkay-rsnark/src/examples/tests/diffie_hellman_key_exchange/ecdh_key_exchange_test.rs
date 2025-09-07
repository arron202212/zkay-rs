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
        operations::gadget::Gadget,
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire_type::WireType,
        },
    },
    examples::gadgets::diffie_hellman_key_exchange::ecdh_key_exchange_gadget::ECDHKeyExchangeGadget,
    util::util::BigInteger,
};
use zkay_derive::ImplStructNameConfig;

// * Tests Key Exchange via Elliptic curve Gadget (ECDHKeyExchangeGadget.java)

#[cfg(test)]
mod test {
    use super::*;
    pub const EXPONENT_BIT_LENGTH: usize = Gadget::<ECDHKeyExchangeGadget>::SECRET_BITWIDTH;
    // The sage script to compute the sample case is commented in the end of the file.
    // TODO: Add more test cases

    #[test]
    pub fn test_variable_input_case() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secret_bits: Vec<Option<WireType>>,
            base_x: Option<WireType>,
            h_x: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let start = std::time::Instant::now();
                let secret_bits = CircuitGenerator::create_input_wire_array(
                    self.cg(),
                    EXPONENT_BIT_LENGTH,
                    &Some("exponent".to_owned()),
                );
                let mut base_x = CircuitGenerator::create_input_wire(self.cg(), &None);
                let mut h_x = CircuitGenerator::create_input_wire(self.cg(), &None);

                let key_exchange_gadget = ECDHKeyExchangeGadget::new(
                    Some(base_x.clone()),
                    None,
                    Some(h_x.clone()),
                    None,
                    secret_bits.clone(),
                    &None,
                    self.cg(),
                );

                CircuitGenerator::make_output(
                    self.cg(),
                    key_exchange_gadget
                        .get_output_public_value()
                        .as_ref()
                        .unwrap(),
                    &None,
                );

                // Just for testing. In real scenarios, this should not be made pub
                CircuitGenerator::make_output(
                    self.cg(),
                    key_exchange_gadget.get_shared_secret().as_ref().unwrap(),
                    &None,
                );

                (self.t.base_x, self.t.h_x, self.t.secret_bits) =
                    (Some(base_x), Some(h_x), secret_bits);
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                let start = std::time::Instant::now();
                evaluator.set_wire_value(self.t.base_x.as_ref().unwrap(), &BigInteger::from(4u8));
                evaluator.set_wire_value(self.t.h_x.as_ref().unwrap(), BigInteger::parse_bytes(b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10).as_ref().unwrap());

                let exponent = BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..EXPONENT_BIT_LENGTH {
                    evaluator.set_wire_valuei(
                        self.t.secret_bits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secret_bits: vec![],
            base_x: None,
            h_x: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let output = generator.get_out_wires();

        assert_eq!(
            evaluator.get_wire_value(output[0].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"13458082339735734368462130456283583571822918321676509705348825437102113182254",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.get_wire_value(output[1].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"4167917227796707610764894996898236918915412447839980711033808347811701875717",
                10
            )
            .unwrap(),
        );
    }

    #[test]
    pub fn test_hardcoded_input_case() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secret_bits: Vec<Option<WireType>>,
            base_x: Option<WireType>,
            h_x: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let secret_bits = CircuitGenerator::create_input_wire_array(
                    self.cg(),
                    EXPONENT_BIT_LENGTH,
                    &Some("exponent".to_owned()),
                );
                let base_x =
                    CircuitGenerator::create_constant_wire(self.cg(), &BigInteger::from(4), &None);
                let h_x =  CircuitGenerator::create_constant_wire(self.cg(),BigInteger::parse_bytes(
                    b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10
                ).as_ref().unwrap(),&None);

                let key_exchange_gadget = ECDHKeyExchangeGadget::new(
                    Some(base_x.clone()),
                    None,
                    Some(h_x.clone()),
                    None,
                    secret_bits.clone(),
                    &None,
                    self.cg(),
                );

                CircuitGenerator::make_output(
                    self.cg(),
                    key_exchange_gadget
                        .get_output_public_value()
                        .as_ref()
                        .unwrap(),
                    &None,
                );

                // Just for testing. In real scenarios, this should not be made pub
                CircuitGenerator::make_output(
                    self.cg(),
                    key_exchange_gadget.get_shared_secret().as_ref().unwrap(),
                    &None,
                );
                (self.t.base_x, self.t.h_x, self.t.secret_bits) =
                    (Some(base_x), Some(h_x), secret_bits);
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                let exponent =BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..EXPONENT_BIT_LENGTH {
                    evaluator.set_wire_valuei(
                        self.t.secret_bits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secret_bits: vec![],
            base_x: None,
            h_x: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test2", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let output = generator.get_out_wires();

        assert_eq!(
            evaluator.get_wire_value(output[0].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"13458082339735734368462130456283583571822918321676509705348825437102113182254",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.get_wire_value(output[1].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"4167917227796707610764894996898236918915412447839980711033808347811701875717",
                10
            )
            .unwrap(),
        );
    }

    #[test]
    pub fn test_input_validation1() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secret_bits: Vec<Option<WireType>>,
            base_x: Option<WireType>,
            h_x: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let secret_bits = CircuitGenerator::create_input_wire_array(
                    self.cg(),
                    EXPONENT_BIT_LENGTH,
                    &Some("exponent".to_owned()),
                );
                let base_x = CircuitGenerator::create_input_wire(self.cg(), &None);
                let h_x = CircuitGenerator::create_input_wire(self.cg(), &None);

                let key_exchange_gadget = ECDHKeyExchangeGadget::new(
                    Some(base_x.clone()),
                    None,
                    Some(h_x.clone()),
                    None,
                    secret_bits.clone(),
                    &None,
                    self.cg(),
                );

                key_exchange_gadget.validate_inputs();
                (self.t.base_x, self.t.h_x, self.t.secret_bits) =
                    (Some(base_x), Some(h_x), secret_bits);
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_value(self.t.base_x.as_ref().unwrap(), &BigInteger::from(4));
                evaluator.set_wire_value(self.t.h_x.as_ref().unwrap(),BigInteger::parse_bytes(b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10).as_ref().unwrap());

                let exponent =BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..EXPONENT_BIT_LENGTH {
                    evaluator.set_wire_valuei(
                        self.t.secret_bits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secret_bits: vec![],
            base_x: None,
            h_x: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test_InputValidation", t);
        generator.generate_circuit();
        generator.eval_circuit();

        // if no exception get thrown we are ok
    }

    #[test]
    pub fn test_input_validation2() {
        // try invalid input

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secret_bits: Vec<Option<WireType>>,
            base_x: Option<WireType>,
            h_x: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let secret_bits = CircuitGenerator::create_input_wire_array(
                    self.cg(),
                    EXPONENT_BIT_LENGTH,
                    &Some("exponent".to_owned()),
                );
                let base_x = CircuitGenerator::create_input_wire(self.cg(), &None);
                let h_x = CircuitGenerator::create_input_wire(self.cg(), &None);

                let key_exchange_gadget = ECDHKeyExchangeGadget::new(
                    Some(base_x.clone()),
                    Some(base_x.clone()),
                    Some(h_x.clone()),
                    Some(h_x.clone()),
                    secret_bits.clone(),
                    &None,
                    self.cg(),
                );

                key_exchange_gadget.validate_inputs();
                (self.t.base_x, self.t.h_x, self.t.secret_bits) =
                    (Some(base_x), Some(h_x), secret_bits);
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                // invalid
                evaluator.set_wire_value(self.t.base_x.as_ref().unwrap(), &BigInteger::from(14));
                evaluator.set_wire_value(self.t.h_x.as_ref().unwrap(),BigInteger::parse_bytes(b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10).as_ref().unwrap());

                let exponent =BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..EXPONENT_BIT_LENGTH {
                    evaluator.set_wire_valuei(
                        self.t.secret_bits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secret_bits: vec![],
            base_x: None,
            h_x: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test_InputValidation2", t);
        generator.generate_circuit();

        // we expect an exception somewhere
        // try{
        assert!(generator.eval_circuit().is_err());
        // assert!(false);
        // } catch(Exception e){
        // 	//println!("Exception Expected!");
        // 	assert!(true);
        // }

        // TODO: test more error conditions
    }

    //		Sage Script generating the above values:
    //
    //		p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
    //		K.<a> = NumberField(x-1)
    //		aa = 126932
    //		E = EllipticCurve(GF(p),[0,aa,0,1,0])
    //		print(E.order())
    //		print(n(log(E.order(),2)))
    //		print(n(log(2736030358979909402780800718157159386074658810754251464600343418943805806723,2)))
    //
    //		secret = 13867691842196510828352345865165018381161315605899394650350519162543016860992
    //
    //		base = E(4,  5854969154019084038134685408453962516899849177257040453511959087213437462470)
    //		print(base*secret)
    //		print(h*secret)
}
