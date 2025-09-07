#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
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
            gadget::{Gadget, GadgetConfig},
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_type::WireType,
        },
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Sub},
};

use zkay_derive::ImplStructNameConfig;

use rccell::RcCell;

//  * This gadget does not apply any lookups in the circuit. Instead, it verifies
//  * the solution using the AES S-Box properties.
//  * (Might need to be revisited in
//  * the future to include other ways that have better circuit representation).

// crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AESSBoxComputeGadget {
    pub input: WireType,
    pub inverse: Option<WireType>,
    pub output: Vec<Option<WireType>>,
}
impl AESSBoxComputeGadget {
    pub fn new(
        input: WireType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                output: vec![],
                inverse: None,
                input,
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<AESSBoxComputeGadget> {
    fn build_circuit(&mut self) {
        let inverse = CircuitGenerator::create_prover_witness_wire(self.generator.clone(), &None);
        let input = &self.t.input;
        let prover = crate::impl_prover!(
                                                eval(  input: WireType,
                                            inverse: WireType
                                        )  {
                                impl Instruction for Prover{
                                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                fn gmuli( mut a: i32, mut  b:  i32) -> i32 {
                        let mut p = 0;
                        for j in 0..8 {
                            if (b & 1) != 0 {
                                p ^= a;
                            }
                            a <<= 1;
                            if (a & 0x100) != 0 {
                                a ^= 0x11b;
                            }
                            b >>= 1;
                        }
                        p
                    }

                    fn find_inv(mut a: i32) -> i32 {
                        if a == 0 {
                            return 0;
                        }
                        for i in 0..256 {
                            if gmuli(i,  a) == 1 {
                                return i;
                            }
                        }
                        -1
                    }
                                            let p = evaluator.get_wire_value(&self.input).to_str_radix(10).parse::<i32>().unwrap();
                                            let q = find_inv(p);
                                            evaluator.set_wire_valuei(&self.inverse, q as i64);
        Ok(())
                                }
                                }
                                            }
                                        );
        CircuitGenerator::specify_prover_witness_computation(self.generator.clone(), prover);

        // &{
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let p = evaluator.get_wire_value(input).intValue();
        //             let q = find_inv(p);
        //             evaluator.set_wire_value(inverse, q);
        //         }
        //     }
        //     Prover
        // });

        inverse.restrict_bit_length(8, &None);

        let v = Self::gmul(
            self.t.input.clone(),
            inverse.clone(),
            self.generator.clone(),
        );
        CircuitGenerator::add_assertion(
            self.generator.clone(),
            &v.sub(self.generator.get_one_wire().as_ref().unwrap()),
            &self.t.input.clone().add(&inverse),
            self.generator.get_zero_wire().as_ref().unwrap(),
            &None,
        );
        let constant = CircuitGenerator::create_constant_wirei(self.generator.clone(), 0x63, &None);
        let mut output = constant.xor_bitwises(&inverse, 8, &None);
        output = output.xor_bitwises(&inverse.rotate_left(8, 1, &None), 8, &None);
        output = output.xor_bitwises(&inverse.rotate_left(8, 2, &None), 8, &None);
        output = output.xor_bitwises(&inverse.rotate_left(8, 3, &None), 8, &None);
        output = output.xor_bitwises(&inverse.rotate_left(8, 4, &None), 8, &None);
        (self.t.output, self.t.inverse) = (vec![Some(output)], Some(inverse));
    }

    fn gmul(mut a: WireType, mut b: WireType, generator: RcCell<CircuitGenerator>) -> WireType {
        let mut p = generator.get_zero_wire().unwrap();
        let ccw = CircuitGenerator::create_constant_wirei(generator.clone(), 0x1b, &None);
        for counter in 0..8 {
            let tmp = p.xor_bitwises(&a, 8, &None);
            let bit = b.get_bit_wiresi(8, &None).get(0).clone().unwrap();
            p = p.clone().add(bit.mul(tmp.sub(&p)));

            let bit2 = a.get_bit_wiresi(8, &None).get(7).clone().unwrap();
            a = a.shift_left(8, 1, &None);

            let tmp2 = a.xor_bitwises(&ccw, 8, &None);
            a = a.clone().add(bit2.mul(tmp2.sub(&a)));
            b = b.shift_right(8, 1, &None);
        }
        p
    }
}
impl GadgetConfig for Gadget<AESSBoxComputeGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
