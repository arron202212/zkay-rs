use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;

use zkay::zkay_util::ZkayUtil;

pub struct ZkayECDHGenerator {
    secret: BigInteger,
    pk: Option<BigInteger>,
    late_eval: bool,

    secret_wire: &Option<WireType>,
    pk_wire: &Option<WireType>,
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
impl CGConfig for CircuitGeneratorExtend<ZkayECDHGenerator> {
    fn buildCircuit(&mut self) {
        let secret_wire = if self.t.late_eval {
            self.createProverWitnessWire(&None)
        } else {
            self.createConstantWire(&self.t.secret)
        };

        if self.t.pk.is_none() {
            // If no pub  key specified, compute own pub  key
            self.makeOutput(
                ZkayEcPkDerivationGadget::new(secret_wire.clone(), true).getOutputWires()[0],
            );
        } else {
            // Derive shared secret
            let pk_wire = if self.t.late_eval {
                self.createInputWire()
            } else {
                self.createConstantWire(self.t.pk.as_ref().unwrap())
            };
            let mut gadget = ZkayECDHGadget::new(pk_wire, secret_wire, true);
            gadget.validateInputs();
            self.makeOutput(gadget.getOutputWires()[0]);
        }
        (self.t.secret_wire, self.t.pk_wire) = (secret_wire, pk_wire);
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        if self.t.late_eval {
            evaluator.setWireValue(self.t.secret_wire.as_ref().unwrap(), &self.t.secret);
            if Some(pk) = self.pk.as_ref() {
                evaluator.setWireValue(self.pk_wire.as_ref().unwrap(), pk);
            }
        }
    }

    pub fn runLibsnark(&self) {
        panic!("This circuit is only for evaluation");
    }

    fn computeECKey(pk: &BigInteger, sk: &BigInteger) -> BigInteger {
        let mut ecdh_generator = ZkayECDHGenerator::new(pk, sk, false);
        ecdh_generator.generateCircuit();
        ecdh_generator.evalCircuit();
        ecdh_generator
            .getCircuitEvaluator()
            .getWireValue(ecdh_generator.get_out_wires()[0])
    }

    pub fn derivePk(secret: BigInteger) -> String {
        Self::computeECKey(None, secret).to_str_radix(16)
    }

    pub fn getSharedSecret(public_key: BigInteger, secret: BigInteger) -> String {
        Self::computeECKey(public_key, secret).to_str_radix(16)
    }

    pub fn rnd_to_secret(rnd_32: &String) -> BigInteger {
        let val = BigInteger::new(rnd_32, 16);
        let mut arr = ZkayUtil::unsignedBigintToBytes(val, 32);
        arr[0] &= 0x0f;
        arr[0] |= 0x10;
        arr[31] &= 0xf8;
        ZkayUtil::unsignedBytesToBigInt(arr)
    }
}

pub fn main(args: Vec<String>) {
    if args.len() == 1 {
        let secret = CircuitGeneratorExtend::<ZkayECDHGenerator>::rnd_to_secret(&args[0]);
        //println!("Deriving pub  key from secret key 0x{:x}", secret);
        //println!(derivePk(secret));
        //println!(secret.toString(16));
    } else if args.len() == 2 {
        let secret = BigInteger::new(args[0], 16);
        let pk = BigInteger::new(args[1], 16);
        println!(
            "Deriving shared key from pub  key 0x{:x} and secret 0x{:x}",
            pk, secret
        );
        println!(CircuitGeneratorExtend::<ZkayECDHGenerator>::getSharedSecret(&pk, &secret));
    } else {
        panic!();
    }
}
