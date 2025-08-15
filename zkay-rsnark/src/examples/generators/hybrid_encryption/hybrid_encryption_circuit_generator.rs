#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            gadget::GadgetConfig,
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            },
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::structure::circuit_generator::{
//     CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
//     getActiveCircuitGenerator,
// };
// use crate::circuit::structure::wire_array;
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{BigInteger, Util};
use crate::examples::gadgets::blockciphers::symmetric_encryption_cbc_gadget::SymmetricEncryptionCBCGadget;
use crate::examples::gadgets::diffie_hellman_key_exchange::field_extension_dh_key_exchange::FieldExtensionDHKeyExchange;
use crate::examples::gadgets::hash::sha256_gadget::{Base, SHA256Gadget};
use zkay_derive::ImplStructNameConfig;
// This gadget shows a simple example of hybrid encryption for illustration purposes
// It currently uses the field extension key exchange gadget with the speck cipher
crate::impl_struct_name_for!(CircuitGeneratorExtend<HybridEncryptionCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct HybridEncryptionCircuitGenerator {
    pub plaintext: Vec<Option<WireType>>,  // as 64-bit words
    pub plaintextSize: i32,                // number of 64-bit words
    pub ciphertext: Vec<Option<WireType>>, // as 64-bit words
    pub ciphername: String,
    pub secExpBits: Vec<Option<WireType>>,
}
impl HybridEncryptionCircuitGenerator {
    // Will assume the parameterization used in the test files ~ 80-bits
    // security
    pub const EXPONENT_BITWIDTH: i32 = 397; // in bits
    pub const MU: i32 = 4;
    pub const OMEGA: i32 = 7;
    pub fn new(
        circuit_name: &str,
        plaintextSize: i32,
        ciphername: String,
    ) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                plaintext: vec![],
                plaintextSize,
                ciphertext: vec![],
                ciphername,
                secExpBits: vec![],
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<HybridEncryptionCircuitGenerator> {
    fn buildCircuit(&mut self) {
        let plaintext = self.createInputWireArray(
            self.t.plaintextSize as usize,
            &Some("plaint text".to_owned()),
        );

        // Part I: Exchange a key:

        // The secret exponent is a  input by the prover
        let mut secExpBits = self.createProverWitnessWireArray(
            HybridEncryptionCircuitGenerator::EXPONENT_BITWIDTH as usize,
            &Some("SecretExponent".to_owned()),
        );
        for i in 0..HybridEncryptionCircuitGenerator::EXPONENT_BITWIDTH as usize {
            self.addBinaryAssertion(secExpBits[i].as_ref().unwrap(), &None); // verify all bits are binary
        }

        let mut g = vec![None; HybridEncryptionCircuitGenerator::MU as usize];
        let mut h = vec![None; HybridEncryptionCircuitGenerator::MU as usize];

        // Hardcode the base and the other party's key (suitable when keys are not expected to change)
        g[0] = Some(self.createConstantWire(&BigInteger::parse_bytes(
            b"16377448892084713529161739182205318095580119111576802375181616547062197291263",10
        ).unwrap(),&None));
        g[1] = Some(self.createConstantWire(&BigInteger::parse_bytes(
            b"13687683608888423916085091250849188813359145430644908352977567823030408967189",10
        ).unwrap(),&None));
        g[2] = Some(self.createConstantWire(&BigInteger::parse_bytes(
            b"12629166084120705167185476169390021031074363183264910102253898080559854363106",10
        ).unwrap(),&None));
        g[3] = Some(self.createConstantWire(&BigInteger::parse_bytes(
            b"19441276922979928804860196077335093208498949640381586557241379549605420212272",10
        ).unwrap(),&None));

        h[0] = Some(
            self.createConstantWire(
                &BigInteger::parse_bytes(
                    b"8252578783913909531884765397785803733246236629821369091076513527284845891757",
                    10,
                )
                .unwrap(),
                &None,
            ),
        );
        h[1] = Some(self.createConstantWire(&BigInteger::parse_bytes(
           b"20829599225781884356477513064431048695774529855095864514701692089787151865093",10
        ).unwrap(),&None));
        h[2] = Some(
            self.createConstantWire(
                &BigInteger::parse_bytes(
                    b"1540379511125324102377803754608881114249455137236500477169164628692514244862",
                    10,
                )
                .unwrap(),
                &None,
            ),
        );
        h[3] = Some(
            self.createConstantWire(
                &BigInteger::parse_bytes(
                    b"1294177986177175279602421915789749270823809536595962994745244158374705688266",
                    10,
                )
                .unwrap(),
                &None,
            ),
        );

        // To make g and h variable inputs to the circuit, simply do the following
        // instead, and supply the above values using the generateSampleInput()
        // method instead.
        /*
         * Vec<Option<WireType>> g = self.createInputWireArray(mu);
         * Vec<Option<WireType>> h = self.createInputWireArray(mu);
         */

        // Exchange keys
        let exchange = FieldExtensionDHKeyExchange::new(
            g,
            h,
            secExpBits.clone(),
            HybridEncryptionCircuitGenerator::OMEGA as i64,
            &None,
            self.cg(),
        );

        // Output g^s
        let g_to_s = exchange.getOutputPublicValue();
        self.makeOutputArray(g_to_s, &Some("DH Key Exchange Output".to_owned()));

        // Use h^s to generate a symmetric secret key and an initialization
        // vector. Apply a Hash-based KDF.
        let h_to_s = exchange.getSharedSecret();
        let hashGadget = SHA256Gadget::new(
            h_to_s.clone(),
            256,
            128,
            true,
            false,
            &None,
            self.cg(),
            Base,
        );
        let secret = hashGadget.getOutputWires();
        let key = secret[0..128].to_vec();
        let iv = secret[128..256].to_vec();

        // Part II: Apply symmetric Encryption

        let plaintextBits = WireArray::new(plaintext.clone(), self.cg().downgrade())
            .getBits(64, &None)
            .asArray()
            .clone();
        let symEncGagdet = SymmetricEncryptionCBCGadget::new(
            plaintextBits.clone(),
            key,
            iv,
            self.t.ciphername.clone(),
            &None,
            self.cg(),
        );
        let ciphertext = symEncGagdet.getOutputWires();
        self.makeOutputArray(&ciphertext, &Some("Cipher Text".to_owned()));
        (self.t.plaintext, self.t.secExpBits, self.t.ciphertext) =
            (plaintext, secExpBits, ciphertext.clone());
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        // TODO Auto-generated method stub
        for i in 0..self.t.plaintextSize as usize {
            evaluator.setWireValue(
                self.t.plaintext[i].as_ref().unwrap(),
                &Util::nextRandomBigIntegeri(64),
            );
        }
        for i in 0..HybridEncryptionCircuitGenerator::EXPONENT_BITWIDTH as usize {
            evaluator.setWireValue(
                self.t.secExpBits[i].as_ref().unwrap(),
                &Util::nextRandomBigIntegeri(1),
            );
        }
    }
}
pub fn main(args: Vec<String>) {
    let mut generator =
        HybridEncryptionCircuitGenerator::new("enc_example", 16, "speck128".to_owned());
    generator.generateCircuit();
    let mut evaluator = generator.evalCircuit().ok();
    generator.prepFiles(evaluator);
    generator.runLibsnark();
}
