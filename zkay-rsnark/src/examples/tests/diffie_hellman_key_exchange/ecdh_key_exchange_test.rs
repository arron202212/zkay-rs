#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
    addToEvaluationQueue, getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::diffie_hellman_key_exchange::ecdh_key_exchange_gadget::ECDHKeyExchangeGadget;
use crate::util::util::BigInteger;
use zkay_derive::ImplStructNameConfig;
/**
* Tests Key Exchange via Elliptic curve Gadget (ECDHKeyExchangeGadget.java)

*/

#[cfg(test)]
mod test {
    use super::*;
    pub const exponentBitlength: usize = Gadget::<ECDHKeyExchangeGadget>::SECRET_BITWIDTH;
    // The sage script to compute the sample case is commented in the end of the file.
    // TODO: Add more test cases

    #[test]
    pub fn testVariableInputCase() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secretBits: Vec<Option<WireType>>,
            baseX: Option<WireType>,
            hX: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let secretBits =
                    self.createInputWireArray(exponentBitlength, &Some("exponent".to_owned()));
                let mut baseX = self.createInputWire(&None);
                let mut hX = self.createInputWire(&None);

                let keyExchangeGadget = ECDHKeyExchangeGadget::new(
                    Some(baseX.clone()),
                    None,
                    Some(hX.clone()),
                    None,
                    secretBits.clone(),
                    &None,
                    self.cg(),
                );

                self.makeOutput(
                    keyExchangeGadget.getOutputPublicValue().as_ref().unwrap(),
                    &None,
                );

                // Just for testing. In real scenarios, this should not be made pub
                self.makeOutput(keyExchangeGadget.getSharedSecret().as_ref().unwrap(), &None);
                (self.t.baseX, self.t.hX, self.t.secretBits) = (Some(baseX), Some(hX), secretBits);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValue(self.t.baseX.as_ref().unwrap(), &BigInteger::from(4u8));
                evaluator.setWireValue(self.t.hX.as_ref().unwrap(), BigInteger::parse_bytes(b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10).as_ref().unwrap());

                let exponent = BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..exponentBitlength {
                    evaluator.setWireValuei(
                        self.t.secretBits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secretBits: vec![],
            baseX: None,
            hX: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();
        // let evaluator = generator.getCircuitEvaluator();
        let output = generator.get_out_wires();

        assert_eq!(
            evaluator.getWireValue(output[0].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"13458082339735734368462130456283583571822918321676509705348825437102113182254",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[1].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"4167917227796707610764894996898236918915412447839980711033808347811701875717",
                10
            )
            .unwrap(),
        );
    }

    #[test]
    pub fn testHardcodedInputCase() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secretBits: Vec<Option<WireType>>,
            baseX: Option<WireType>,
            hX: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let secretBits =
                    self.createInputWireArray(exponentBitlength, &Some("exponent".to_owned()));
                let baseX = self.createConstantWire(&BigInteger::from(4), &None);
                let hX =  self.createConstantWire(BigInteger::parse_bytes(
                    b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10
                ).as_ref().unwrap(),&None);

                let keyExchangeGadget = ECDHKeyExchangeGadget::new(
                    Some(baseX.clone()),
                    None,
                    Some(hX.clone()),
                    None,
                    secretBits.clone(),
                    &None,
                    self.cg(),
                );

                self.makeOutput(
                    keyExchangeGadget.getOutputPublicValue().as_ref().unwrap(),
                    &None,
                );

                // Just for testing. In real scenarios, this should not be made pub
                self.makeOutput(keyExchangeGadget.getSharedSecret().as_ref().unwrap(), &None);
                (self.t.baseX, self.t.hX, self.t.secretBits) = (Some(baseX), Some(hX), secretBits);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let exponent =BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..exponentBitlength {
                    evaluator.setWireValuei(
                        self.t.secretBits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secretBits: vec![],
            baseX: None,
            hX: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test2", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();
        // let evaluator = generator.getCircuitEvaluator();
        let output = generator.get_out_wires();

        assert_eq!(
            evaluator.getWireValue(output[0].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"13458082339735734368462130456283583571822918321676509705348825437102113182254",
                10
            )
            .unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(output[1].as_ref().unwrap()),
            BigInteger::parse_bytes(
                b"4167917227796707610764894996898236918915412447839980711033808347811701875717",
                10
            )
            .unwrap(),
        );
    }

    #[test]
    pub fn testInputValidation1() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secretBits: Vec<Option<WireType>>,
            baseX: Option<WireType>,
            hX: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let secretBits =
                    self.createInputWireArray(exponentBitlength, &Some("exponent".to_owned()));
                let baseX = self.createInputWire(&None);
                let hX = self.createInputWire(&None);

                let keyExchangeGadget = ECDHKeyExchangeGadget::new(
                    Some(baseX.clone()),
                    None,
                    Some(hX.clone()),
                    None,
                    secretBits.clone(),
                    &None,
                    self.cg(),
                );

                keyExchangeGadget.validateInputs();
                (self.t.baseX, self.t.hX, self.t.secretBits) = (Some(baseX), Some(hX), secretBits);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValue(self.t.baseX.as_ref().unwrap(), &BigInteger::from(4));
                evaluator.setWireValue(self.t.hX.as_ref().unwrap(),BigInteger::parse_bytes(b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10).as_ref().unwrap());

                let exponent =BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..exponentBitlength {
                    evaluator.setWireValuei(
                        self.t.secretBits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secretBits: vec![],
            baseX: None,
            hX: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test_InputValidation", t);
        generator.generateCircuit();
        generator.evalCircuit();

        // if no exception get thrown we are ok
    }

    #[test]
    pub fn testInputValidation2() {
        // try invalid input

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            secretBits: Vec<Option<WireType>>,
            baseX: Option<WireType>,
            hX: Option<WireType>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let secretBits =
                    self.createInputWireArray(exponentBitlength, &Some("exponent".to_owned()));
                let baseX = self.createInputWire(&None);
                let hX = self.createInputWire(&None);

                let keyExchangeGadget = ECDHKeyExchangeGadget::new(
                    Some(baseX.clone()),
                    Some(baseX.clone()),
                    Some(hX.clone()),
                    Some(hX.clone()),
                    secretBits.clone(),
                    &None,
                    self.cg(),
                );

                keyExchangeGadget.validateInputs();
                (self.t.baseX, self.t.hX, self.t.secretBits) = (Some(baseX), Some(hX), secretBits);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                // invalid
                evaluator.setWireValue(self.t.baseX.as_ref().unwrap(), &BigInteger::from(14));
                evaluator.setWireValue(self.t.hX.as_ref().unwrap(),BigInteger::parse_bytes(b"21766081959050939664800904742925354518084319102596785077490863571049214729748",10).as_ref().unwrap());

                let exponent =BigInteger::parse_bytes(
                    b"13867691842196510828352345865165018381161315605899394650350519162543016860992",10
                ).unwrap();
                for i in 0..exponentBitlength {
                    evaluator.setWireValuei(
                        self.t.secretBits[i].as_ref().unwrap(),
                        if exponent.bit(i as u64) { 1 } else { 0 },
                    );
                }
            }
        };
        let t = CGTest {
            secretBits: vec![],
            baseX: None,
            hX: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("ECDH_Test_InputValidation2", t);
        generator.generateCircuit();

        // we expect an exception somewhere
        // try{
        assert!(generator.evalCircuit().is_err());
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
