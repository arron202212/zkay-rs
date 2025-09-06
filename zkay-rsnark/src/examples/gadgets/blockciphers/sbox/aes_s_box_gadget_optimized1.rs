#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::{
    InstanceOf,
    {
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, add_to_evaluation_queue,
                get_active_circuit_generator,
            },
            constant_wire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_ops::{AddWire, MulWire, SubWire},
            wire_type::WireType,
        },
    },
};
use crate::util::util::{
    ARcCell, {BigInteger, Util},
};
// use crate::circuit::config::config::Configs;
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget::{Gadget, GadgetConfig};
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::blockciphers::aes128_cipher_gadget::AES128CipherGadget;
use crate::examples::gadgets::blockciphers::sbox::util::linear_system_solver::LinearSystemSolver;
use rccell::RcCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Rem, Sub};
use std::sync::{
    OnceLock,
    atomic::{self, AtomicU8, Ordering},
};
pub static s_all_coeff_set: OnceLock<Vec<Vec<BigInteger>>> = OnceLock::new();

//  * This gadget implements the efficient read-only memory access from xjsnark
//  * (the generic way). A more efficient variant is implemented in
//  * AESSBoxGadgetOptimized2.java
//  *
//  * Note that we can code the preprocessing of this method using a simpler way
//  * (by finding 16 polynomials with specific root points) instead of computing
//  * the coefficients using a linear system of equations, but this was kept as it
//  * inspired the other optimization in AESSBoxGadgetOptimized2.java, which saves
//  * half of the cost of a single access.

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AESSBoxGadgetOptimized1 {
    pub input: WireType,
    pub output: Vec<Option<WireType>>,
}
impl AESSBoxGadgetOptimized1 {
    pub fn new(
        input: WireType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                input,
                output: vec![],
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<AESSBoxGadgetOptimized1> {
    const SBox: [u8; 256] = Gadget::<AES128CipherGadget>::SBox;
    //static
    fn preprocessing() -> &'static Vec<Vec<BigInteger>> {
        s_all_coeff_set.get_or_init(|| Self::solve_linear_systems())
    }
    pub fn solve_linear_systems() -> Vec<Vec<BigInteger>> {
        let mut all_coeff_set = Vec::new();
        let list: Vec<_> = (0..=255)
            .map(|i| 256 * i as i32 + Self::SBox[i] as i32)
            .collect();

        for i in 0..=15 {
            let mut member_value_set = HashSet::new();
            let mut mat = vec![vec![BigInteger::default(); 17]; 16];

            // used for sanity checks
            let mut poly_coeffs = vec![Util::one()];

            for k in 0..mat.len() {
                let value = list[k + i * 16];
                // println!("==value=={value}====");
                member_value_set.insert(value);
                let p = BigInteger::from(value);
                mat[k][0] = Util::one();
                for j in 1..=16 {
                    mat[k][j] = p.clone().mul(&mat[k][j - 1]).rem(&Configs.field_prime);
                    // if mat[k][j]==BigInteger::ZERO||j==16{
                    // println!("=zero=mat==={k}=={j}==p=={p}=v=={value}======{}======{}==",p.clone().mul(&mat[k][j - 1]),mat[k][j]);
                    // }
                }
                // let old=mat[k][16].clone();
                // negate the last element, just to make things consistent with
                // the paper notations
                mat[k][16] = Configs.field_prime.clone().sub(&mat[k][16]);
                // println!("=={old}=={i}==mat[k][16]=={k}======={}",mat[k][16]);
                // used for a sanity check (verifying that the output solution
                // is equivalent to coefficients of polynomial that has roots at
                // member_value_set. see note above)
                poly_coeffs = Self::poly_mul(
                    poly_coeffs,
                    vec![Configs.field_prime.clone().sub(&p), Util::one()],
                );
            }

            mat = LinearSystemSolver::new(mat).solve_in_place();
            // for ii in 0..16 {
            //     println!( "=mat).solve_in_place==mat[{ii}][16]===={}",mat[ii][16]);
            // }
            // Note that this is just a sanity check here. It should be always
            // the case that the prover cannot cheat using this method,
            // because this method is equivalent to finding a polynomial with
            // \sqrt{n} roots. No other point will satisfy this property.
            // However, when we do further optimizations in
            // AESBoxGadgetOptimized2.java, this check becomes
            // necessary, and other trials could be needed.

            assert!(
                !Self::check_if_prover_can_cheat(&mat, &member_value_set),
                "The prover can cheat."
            );

            let mut coeffs = vec![BigInteger::default(); 16];
            for ii in 0..16 {
                coeffs[ii] = mat[ii][16].clone();

                assert!(&coeffs[ii] == &poly_coeffs[ii], "Inconsistency found.");
            }
            all_coeff_set.push(coeffs);
        }
        all_coeff_set
    }

    // method for sanity checks during preprocessing
    fn poly_mul(a1: Vec<BigInteger>, a2: Vec<BigInteger>) -> Vec<BigInteger> {
        let mut out = vec![BigInteger::ZERO; a1.len() + a2.len() - 1];

        for i in 0..a1.len() {
            for j in 0..a2.len() {
                out[i + j] = out[i + j]
                    .clone()
                    .add(a1[i].clone().mul(&a2[j]))
                    .rem(&Configs.field_prime);
            }
        }
        out
    }

    fn check_if_prover_can_cheat(mat: &Vec<Vec<BigInteger>>, value_set: &HashSet<i32>) -> bool {
        let mut coeffs = vec![BigInteger::default(); 16];
        for i in 0..16 {
            coeffs[i] = mat[i][16].clone();
            // if mat[i][16]==BigInteger::ZERO{
            //         println!("=zero=mat==check_if_prover_can_cheat={i}==16============");
            // }
        }

        let mut valid_results = 0;
        let mut outside_permissible_set = 0;

        // loop over the whole permissible domain (recall that input & output
        // are bounded)
        for k in 0..256 * 256 {
            let mut result = coeffs[0].clone();
            let mut p = BigInteger::from(k);
            let kb = BigInteger::from(k);
            for i in 1..16 {
                // println!("======result===={result}=========p========{p}======{i}=={}==",coeffs[i]);
                result = result.add(p.clone().mul(&coeffs[i]));
                p = p.clone().mul(&kb).rem(&Configs.field_prime);
            }
            result = result.rem(&Configs.field_prime);
            // println!("======result===={result}=====");
            if result == Configs.field_prime.clone().sub(&p) {
                valid_results += 1;
                if !value_set.contains(&k) {
                    outside_permissible_set += 1;
                }
            }
            // else if k==99{
            //     println!("===result===={result}=========p========{p}=====");
            // }
        }
        // println!("valid_results={valid_results},outside_permissible_set={outside_permissible_set}");
        if valid_results != 16 || outside_permissible_set != 0 {
            //println!("Prover can cheat with linear system solution");
            //println!("Num of valid values that the prover can use = " + valid_results);
            //println!("Num of valid values outside permissible set = " + valid_results);
            true
        } else {
            false
        }
    }

    fn build_circuit(&mut self) {
        let generator = self.generator.clone();
        let mut output =
            CircuitGenerator::create_prover_witness_wire(self.generator.clone(), &None);
        self.t.input.restrict_bit_length(8, &None);
        let input = self.t.input.clone();
        let SBox = Self::SBox.clone();
        // CircuitGenerator::specify_prover_witness_computation(generator.clone(),&|evaluator: &mut CircuitEvaluator| {
        //     // TODO Auto-generated method stub
        //     let value = evaluator.get_wire_value(input);
        //     evaluator.set_wire_value(output, &BigInteger::from(SBox[value.intValue()]));
        // });
        let prover = crate::impl_prover!(
                                    eval(  input: WireType,
                                output: WireType
                            )  {
                    impl Instruction for Prover{
                     fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                // TODO Auto-generated method stub
                let value = evaluator.get_wire_value(&self.input);
                evaluator.set_wire_value(&self.output, &BigInteger::from(Gadget::<AESSBoxGadgetOptimized1>::SBox[value.to_str_radix(10).parse::<usize>().unwrap()]));
        Ok(())
                    }
                    }
                                }
                            );
        CircuitGenerator::specify_prover_witness_computation(generator.clone(), prover);
        // {
        //             struct Prover;
        //             impl Instruction for Prover {
        //                 &|evaluator: &mut CircuitEvaluator| {
        //                     // TODO Auto-generated method stub
        //                     let value = evaluator.get_wire_value(input);
        //                     evaluator.set_wire_value(output, BigInteger::from(SBox[value.intValue()]));
        //                 }
        //             }
        //             Prover
        //         });

        output.restrict_bit_length(8, &None);
        let mut vars = vec![None; 16];
        let mut p = input.muli(256, &None).add(&output);
        vars[0] = generator.get_one_wire();
        for i in 1..16 {
            vars[i] = Some(vars[i - 1].clone().unwrap().mul(&p));
        }
        let all_coeff_set = Self::preprocessing();
        let mut product = generator.get_one_wire().unwrap();
        for coeffs in all_coeff_set {
            let mut accum = generator.get_zero_wire().unwrap();
            for j in 0..vars.len() {
                accum = accum.add(vars[j].as_ref().unwrap().mulb(&coeffs[j], &None));
            }
            accum = accum.add(vars[15].clone().unwrap().mul(&p));
            product = product.clone().mul(accum);
        }
        self.t.output = vec![Some(output)];
        CircuitGenerator::add_zero_assertion(generator.clone(), &product, &None);
    }
}
impl GadgetConfig for Gadget<AESSBoxGadgetOptimized1> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
