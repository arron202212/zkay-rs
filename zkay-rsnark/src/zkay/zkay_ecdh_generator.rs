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
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::CGConfigFields,
            circuit_generator::CGInstance,
            circuit_generator::{CGConfig, CircuitGenerator, CircuitGeneratorExtend},
            wire_type::WireType,
        },
    },
    util::util::BigInteger,
    zkay::{
        zkay_ec_pk_derivation_gadget::ZkayEcPkDerivationGadget, zkay_ecdh_gadget::ZkayECDHGadget,
        zkay_util::ZkayUtil,
    },
};

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
    fn build_circuit(&mut self) {
        let secret_wire = if self.t.late_eval {
            CircuitGenerator::create_prover_witness_wire(self.cg(), &None)
        } else {
            CircuitGenerator::create_constant_wire(self.cg(), &self.t.secret, &None)
        };

        if let Some(pk) = &self.t.pk {
            // Derive shared secret
            self.t.pk_wire = Some(if self.t.late_eval {
                CircuitGenerator::create_input_wire(self.cg(), &None)
            } else {
                CircuitGenerator::create_constant_wire(self.cg(), pk, &None)
            });
            let mut gadget = ZkayECDHGadget::new(
                self.t.pk_wire.clone().unwrap(),
                secret_wire.clone(),
                true,
                &None,
                self.cg(),
            );
            gadget.validate_inputs();
            CircuitGenerator::make_output(
                self.cg(),
                gadget.get_output_wires()[0].as_ref().unwrap(),
                &None,
            );
        } else {
            // If no pub  key specified, compute own pub  key
            CircuitGenerator::make_output(
                self.cg(),
                ZkayEcPkDerivationGadget::new(secret_wire.clone(), true, &None, self.cg())
                    .get_output_wires()[0]
                    .as_ref()
                    .unwrap(),
                &None,
            );
        }
        self.t.secret_wire = Some(secret_wire);
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        if self.t.late_eval {
            evaluator.set_wire_value(self.t.secret_wire.as_ref().unwrap(), &self.t.secret);
            if let Some(pk) = self.t.pk.as_ref() {
                evaluator.set_wire_value(self.t.pk_wire.as_ref().unwrap(), pk);
            }
        }
    }
}

impl CircuitGeneratorExtend<ZkayECDHGenerator> {
    pub fn run_libsnark(&self) {
        panic!("This circuit is only for evaluation");
    }

    pub fn compute_ec_key(pk: Option<&BigInteger>, sk: &BigInteger) -> BigInteger {
        let mut ecdh_generator = ZkayECDHGenerator::new(pk.cloned(), sk.clone(), false);
        ecdh_generator.generate_circuit();
        let evaluator = ecdh_generator.eval_circuit().unwrap();
        evaluator.get_wire_value(ecdh_generator.get_out_wires()[0].as_ref().unwrap())
    }

    pub fn derive_pk(secret: &BigInteger) -> String {
        Self::compute_ec_key(None, secret).to_str_radix(16)
    }

    pub fn get_shared_secret(public_key: &BigInteger, secret: &BigInteger) -> String {
        Self::compute_ec_key(Some(public_key), secret).to_str_radix(16)
    }

    pub fn rnd_to_secret(rnd_32: &String) -> BigInteger {
        let val = BigInteger::parse_bytes(rnd_32.as_bytes(), 16).unwrap();
        let mut arr = ZkayUtil::unsigned_bigint_to_bytesi(val, 32);
        arr[0] &= 0x0f;
        arr[0] |= 0x10;
        arr[31] &= 0xf8;
        ZkayUtil::unsigned_bytes_to_big_int(&arr)
    }
}

pub fn main(args: Vec<String>) {
    if args.len() == 1 {
        let secret = CircuitGeneratorExtend::<ZkayECDHGenerator>::rnd_to_secret(&args[0]);
        //println!("Deriving pub  key from secret key 0x{:x}", secret);
        //println!(derive_pk(secret));
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
            CircuitGeneratorExtend::<ZkayECDHGenerator>::get_shared_secret(&pk, &secret)
        );
    } else {
        panic!();
    }
}
