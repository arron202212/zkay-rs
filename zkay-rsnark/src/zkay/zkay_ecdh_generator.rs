use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{addToEvaluationQueue,CGConfig,CircuitGenerator,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;

use zkay::zkay_util::unsigned_bigint_to_bytes;
use zkay::zkay_util::unsigned_bytes_to_bigint;

pub struct ZkayECDHGenerator {
    secret: BigInteger,
    pk: BigInteger,
    late_eval: bool,

    secret_wire: WireType,
    pk_wire: WireType,
}
impl ZkayECDHGenerator {
    pub fn new(pk: BigInteger, secret: BigInteger, late_eval: bool) -> Self {
        //super("circuit");
        self.pk = pk;
        self.secret = secret;
        self.late_eval = late_eval;
    }
}
impl CGConfig for CircuitGeneratorExtend<ZkayECDHGenerator> {
    fn buildCircuit(&mut self) {
        secret_wire = if late_eval {
            createProverWitnessWire(&None)
        } else {
            createConstantWire(secret)
        };

        if pk == None {
            // If no pub  key specified, compute own pub  key
            makeOutput(ZkayEcPkDerivationGadget::new(secret_wire, true).getOutputWires()[0]);
        } else {
            // Derive shared secret
            pk_wire = if late_eval {
                createInputWire()
            } else {
                createConstantWire(pk)
            };
            let gadget = ZkayECDHGadget::new(pk_wire, secret_wire, true);
            gadget.validateInputs();
            makeOutput(gadget.getOutputWires()[0]);
        }
    }

    fn generateSampleInput(&self,evaluator: &mut CircuitEvaluator) {
        if late_eval {
            evaluator.setWireValue(secret_wire, self.secret);
            if self.pk != None {
                evaluator.setWireValue(pk_wire, self.pk);
            }
        }
    }

    pub fn runLibsnark() {
        panic!("This circuit is only for evaluation");
    }

    fn computeECKey(pk: BigInteger, sk: BigInteger) -> BigInteger {
        let mut generator = ZkayECDHGenerator::new(pk, sk, false);
        generator.generateCircuit();
        generator.evalCircuit();
        return generator
            .getCircuitEvaluator()
            .getWireValue(generator.get_out_wires().get(0));
    }

    pub fn derivePk(secret: BigInteger) -> String {
        computeECKey(None, secret).toString(16)
    }

    pub fn getSharedSecret(public_key: BigInteger, secret: BigInteger) -> String {
        computeECKey(public_key, secret).toString(16)
    }

    pub fn rnd_to_secret(rnd_32: String) -> BigInteger {
        let val = BigInteger::new(rnd_32, 16);
        let arr = unsignedBigintToBytes(val, 32);
        arr[0] &= 0x0f;
        arr[0] |= 0x10;
        arr[31] &= 0xf8;
        unsignedBytesToBigInt(arr)
    }
}

pub fn main(args: Vec<String>) {
    if args.len() == 1 {
        let secret = rnd_to_secret(args[0]);
        //println!("Deriving pub  key from secret key 0x{:x}", secret);
        //println!(derivePk(secret));
        //println!(secret.toString(16));
    } else if args.len() == 2 {
        let secret = BigInteger::new(args[0], 16);
        let pk = BigInteger::new(args[1], 16);
        //println!(
            "Deriving shared key from pub  key 0x{:x} and secret 0x{:x}",
            pk, secret
        );
        //println!(getSharedSecret(pk, secret));
    } else {
        panic!();
    }
}
