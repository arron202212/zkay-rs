#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CGConfigFields;
use crate::circuit::structure::circuit_generator::CGInstance;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::BigInteger;
use crate::zkay::zkay_ec_pk_derivation_gadget::ZkayEcPkDerivationGadget;
use crate::zkay::zkay_ecdh_gadget::ZkayECDHGadget;
use crate::zkay::zkay_util::ZkayUtil;
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct ZkayECDHGenerator {
    pub secret: BigInteger,
    pub pk: Option<BigInteger>,
    pub late_eval: bool,

    pub secret_wire: Option<WireType>,
    pub pk_wire: Option<WireType>,
}
impl ZkayECDHGenerator {
    pub fn new(
        pk: Option<BigInteger>,
        secret: BigInteger,
        late_eval: bool,
    ) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::<Self>::new(
            "circuit",
            Self {
                pk,
                secret,
                late_eval,
                secret_wire: None,
                pk_wire: None,
            },
        )
    }
}
crate::impl_struct_name_for!(CircuitGeneratorExtend<ZkayECDHGenerator>);
impl CGConfig for CircuitGeneratorExtend<ZkayECDHGenerator> {
    fn buildCircuit(&mut self) {
        let secret_wire = if self.t.late_eval {
            self.createProverWitnessWire(&None)
        } else {
            self.createConstantWire(&self.t.secret, &None)
        };

        if self.t.pk.is_none() {
            // If no pub  key specified, compute own pub  key
            self.makeOutput(
                ZkayEcPkDerivationGadget::new(secret_wire.clone(), true, &None, self.cg())
                    .getOutputWires()[0]
                    .as_ref()
                    .unwrap(),
                &None,
            );
        } else {
            // Derive shared secret
            self.t.pk_wire = if self.t.late_eval {
                Some(self.createInputWire(&None))
            } else {
                Some(self.createConstantWire(self.t.pk.as_ref().unwrap(), &None))
            };
            let mut gadget = ZkayECDHGadget::new(
                self.t.pk_wire.clone().unwrap(),
                secret_wire.clone(),
                true,
                &None,
                self.cg(),
            );
            gadget.validateInputs();
            self.makeOutput(gadget.getOutputWires()[0].as_ref().unwrap(), &None);
        }
        self.t.secret_wire = Some(secret_wire);
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        if self.t.late_eval {
            evaluator.setWireValue(self.t.secret_wire.as_ref().unwrap(), &self.t.secret);
            if let Some(pk) = self.t.pk.as_ref() {
                evaluator.setWireValue(self.t.pk_wire.as_ref().unwrap(), pk);
            }
        }
    }
}

impl CircuitGeneratorExtend<ZkayECDHGenerator> {
    pub fn runLibsnark(&self) {
        panic!("This circuit is only for evaluation");
    }

    pub fn computeECKey(pk: Option<&BigInteger>, sk: &BigInteger) -> BigInteger {
        let mut ecdh_generator = ZkayECDHGenerator::new(pk.cloned(), sk.clone(), false);
        ecdh_generator.generateCircuit();
        ecdh_generator.evalCircuit();
        let evaluator = ecdh_generator.evalCircuit().unwrap();
        evaluator.getWireValue(ecdh_generator.get_out_wires()[0].as_ref().unwrap())
    }

    pub fn derivePk(secret: &BigInteger) -> String {
        Self::computeECKey(None, secret).to_str_radix(16)
    }

    pub fn getSharedSecret(public_key: &BigInteger, secret: &BigInteger) -> String {
        Self::computeECKey(Some(public_key), secret).to_str_radix(16)
    }

    pub fn rnd_to_secret(rnd_32: &String) -> BigInteger {
        let val = BigInteger::parse_bytes(rnd_32.as_bytes(), 16).unwrap();
        let mut arr = ZkayUtil::unsignedBigintToBytesi(val, 32);
        arr[0] &= 0x0f;
        arr[0] |= 0x10;
        arr[31] &= 0xf8;
        ZkayUtil::unsignedBytesToBigInt(&arr)
    }
}

pub fn main(args: Vec<String>) {
    if args.len() == 1 {
        let secret = CircuitGeneratorExtend::<ZkayECDHGenerator>::rnd_to_secret(&args[0]);
        //println!("Deriving pub  key from secret key 0x{:x}", secret);
        //println!(derivePk(secret));
        //println!(secret.toString(16));
    } else if args.len() == 2 {
        let secret = BigInteger::parse_bytes(args[0].as_bytes(), 16).unwrap();
        let pk = BigInteger::parse_bytes(args[1].as_bytes(), 16).unwrap();
        println!(
            "Deriving shared key from pub  key 0x{:x} and secret 0x{:x}",
            pk, secret
        );
        println!(
            "{}",
            CircuitGeneratorExtend::<ZkayECDHGenerator>::getSharedSecret(&pk, &secret)
        );
    } else {
        panic!();
    }
}
