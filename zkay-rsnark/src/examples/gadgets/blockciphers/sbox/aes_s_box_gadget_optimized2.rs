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
        StructNameConfig,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
                getActiveCircuitGenerator, put_active_circuit_generator,
            },
            circuit_generator::{CGConfigFields, CGInstance},
            wire::{GetWireId, WireConfig},
            wire_type::WireType,
        },
    },
    examples::gadgets::{
        hash::{sha256_gadget, sha256_gadget::SHA256Gadget},
        math::{field_division_gadget, field_division_gadget::FieldDivisionGadget},
    },
    util::util::{ARcCell, BigInteger, Util},
};

// use crate::circuit::config::config::Configs;
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::eval::instruction::Instruction;
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::blockciphers::aes128_cipher_gadget::AES128CipherGadget;
use crate::examples::gadgets::blockciphers::sbox::util::linear_system_solver::LinearSystemSolver;
use std::collections::HashSet;
use zkay_derive::ImplStructNameConfig;
// extern crate rand;

use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Rem, Sub};
use std::sync::{
    OnceLock,
    atomic::{self, AtomicU8, Ordering},
};
// use rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_pcg::Pcg64Mcg;
use rip_shuffle::RipShuffleParallel;
pub static s_all_coeff_set: OnceLock<Vec<Vec<BigInteger>>> = OnceLock::new();
/*
 * bitCount represents how many bits are going to be used to construct the
 * linear systems. Setting bitCount to 0 will yield almost the same circuit
 * size as in AESBoxGadgetOptimized1.java. Setting bitcount to 16 will
 * almost make it very hard to find a solution. Setting bitCount to x, where
 * 16 > x > 0, means that x columns from the linear system will be based on
 * the bits of the element (input*256+output), and the rest are based on
 * products (as in AESSBoxGadgetOptimized1). As x increases, the more
 * savings. x cannot increase beyond 16.
 */

pub static atomic_bit_count: AtomicU8 = AtomicU8::new(15);
/**
 * This gadget implements the efficient read-only memory access from xjsnark,
 * while making use of some properties of the AES circuit to gain more savings.
 *
 * Instead of constructing the linear systems using vector of powers like the
 * AESSBoxGadgetOptimized1, this gadget relies on the observation that the bits
 * of the input and output (to the lookup operations) are already available or
 * will be needed later in the circuit. The gadget uses these bits partially to
 * construct the linear systems, but this has to be done carefully to make sure
 * that the prover cannot cheat. This might require shuffling and multiple
 * attempts, while checking all other possibilities that a prover could use to
 * cheat. See the bitCount parameter below.
 *
 */
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AESSBoxGadgetOptimized2 {
    input: WireType,
    output: Vec<Option<WireType>>,
}
impl AESSBoxGadgetOptimized2 {
    pub fn new(
        input: WireType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                output: vec![],
                input,
            },
        };

        _self.buildCircuit();
        _self
    }
}
impl Gadget<AESSBoxGadgetOptimized2> {
    const SBox: [u8; 256] = Gadget::<AES128CipherGadget>::SBox;
    //static
    fn preprocessing() -> &'static Vec<Vec<BigInteger>> {
        // preprocessing
        let all_coeff_set = s_all_coeff_set.get_or_init(|| Self::solve_linear_systems(15));
        all_coeff_set
    }

    pub fn set_bit_count(x: i32) {
        assert!(x >= 0 && x <= 16);
        atomic_bit_count.store(x as u8, Ordering::Relaxed);
    }

    pub fn solve_linear_systems(bit_count: u8) -> Vec<Vec<BigInteger>> {
        println!("======bit_count========={bit_count}");
        let mut seed = 1;
        let mut allCoeffSet = Vec::new();
        let mut list = Vec::new();
        for i in 0..=255 {
            list.push(256 * i as i32 + Self::SBox[i] as i32);
        }
        let mut done = false;
        let mut trialCounter = 0;
        'loop1: while (!done) {
            trialCounter += 1;
            assert!(
                trialCounter < 100,
                "Was not possible to find an adequate solution to the current setting of the AES gadget sbox"
            );

            println!(
                "Attempting to solve linear systems for efficient S-Box Access: Attempt#{trialCounter},{seed}"
            );
            seed += 1;
            // let slice: &mut [u32] = &mut list;
            let mut rng = SmallRng::seed_from_u64(seed);
            // let mut rng=Pcg64Mcg::from_rng(&mut rng);
            // list.par_shuffle_seed_with(&mut rng);
            list.shuffle(&mut rng);
            // Collections.shuffle(list, Random::new(seed));
            allCoeffSet.clear();

            for i in 0..=15 {
                let mut mat = vec![vec![BigInteger::default(); 17]; 16];
                let mut memberValueSet = HashSet::new();

                for k in 0..mat.len() {
                    let memberValue = list[k + i * 16];
                    memberValueSet.insert(memberValue);
                    mat[k][16] = Util::one();

                    // now extract the values that correspond to memberValue
                    // the method getVariableValues takes the bitCount settings
                    // into account
                    let variableValues = Self::getVariableValues(memberValue, bit_count);
                    for j in 0..=15 {
                        mat[k][j] = variableValues[j].clone();
                    }
                }

                mat = LinearSystemSolver::new(mat).solveInPlace();

                if Self::checkIfProverCanCheat(&mat, &memberValueSet, bit_count) {
                    println!("Invalid solution {bit_count} {i} {}", allCoeffSet.len());
                    for ii in 0..16 {
                        if mat[ii][16] == BigInteger::ZERO {
                            println!(
                                "Possibly invalid due to having zero coefficient(s) {i} {}",
                                allCoeffSet.len()
                            );
                            break;
                        }
                    }

                    continue 'loop1;
                }

                let mut coeffs = vec![BigInteger::default(); 16];
                for ii in 0..16 {
                    coeffs[ii] = mat[ii][16].clone();
                }
                allCoeffSet.push(coeffs);
            }
            done = true;
            // AESSBoxGadgetOptimized2.allCoeffSet = allCoeffSet;
            //println!("Solution found!");
        }
        allCoeffSet
    }

    fn buildCircuit(&mut self) {
        let generator = self.generator.borrow().clone();
        let mut output = generator.createProverWitnessWire(&None);
        let input = &self.t.input;
        // generator.specifyProverWitnessComputation(&|evaluator: &mut CircuitEvaluator| {
        //     // TODO Auto-generated method stub
        //     let value = evaluator.getWireValue(input);
        //     evaluator.setWireValue(output, &BigInteger::from(SBox[value.intValue()]));
        // });
        let prover = crate::impl_prover!(
                            eval(  input: WireType,
                        output: WireType
                    )  {
            impl Instruction for Prover{
             fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                        // TODO Auto-generated method stub
        let value = evaluator.getWireValue(&self.input);
        evaluator.setWireValue(&self.output, &BigInteger::from(Gadget::<AESSBoxGadgetOptimized2>::SBox[value.to_str_radix(10).parse::<usize>().unwrap()]));
            }
            }
                        }
                    );
        generator.specifyProverWitnessComputation(prover);
        // {
        //             struct Prover;
        //             impl Instruction for Prover {
        //                 &|evaluator: &mut CircuitEvaluator| {
        //                     // TODO Auto-generated method stub
        //                     let value = evaluator.getWireValue(input);
        //                     evaluator.setWireValue(output, BigInteger::from(SBox[value.intValue()]));
        //                 }
        //             }
        //             Prover
        //         });

        // Although we are getting the bits below anyway (which implicitly
        // restricts the bitwidth), it's a safer practice to call
        // restrictBitLength() explicitly to avoid some special cases with
        // getBitWires().
        // Similar operations get filtered later, so this won't add extra
        // constraints.
        output.restrictBitLength(8, &None);
        input.restrictBitLength(8, &None);

        let bitsIn = input.getBitWiresi(8, &None).asArray().clone();
        let bitsOut = output.getBitWiresi(8, &None).asArray().clone();
        let mut vars = vec![None; 16];
        let mut p = input.muli(256, &None).add(&output).add(1);
        let mut currentProduct = p.clone();
        let bit_count = atomic_bit_count.load(Ordering::Relaxed);
        if bit_count != 0 && bit_count != 16 {
            currentProduct = currentProduct.clone().mul(&currentProduct);
        }
        for i in 0..16 {
            if i < bit_count as usize {
                if i < 8 {
                    vars[i] = bitsOut[i].clone();
                } else {
                    vars[i] = bitsIn[i - 8].clone();
                }
            } else {
                vars[i] = Some(currentProduct.clone());
                if i != 15 {
                    currentProduct = currentProduct.mul(&p);
                }
            }
        }
        let all_coeff_set = Self::preprocessing();
        let mut product = generator.get_one_wire().unwrap();
        for coeffs in all_coeff_set {
            let mut accum = generator.get_zero_wire().unwrap();
            for j in 0..vars.len() {
                accum = accum.add(vars[j].as_ref().unwrap().mulb(&coeffs[j], &None));
            }
            accum = accum.sub(1);
            product = product.mul(accum);
        }
        self.t.output = vec![Some(output)];
        generator.addZeroAssertion(&product, &None);
    }

    fn getVariableValues(k: i32, bit_count: u8) -> Vec<BigInteger> {
        let mut vars = vec![BigInteger::default(); 16];
        let mut v = BigInteger::from(k).add(Util::one());
        let mut product = v.clone();
        if bit_count != 0 {
            product = product.mul(&v).rem(&Configs.field_prime);
        }
        for j in 0..16 {
            if j < bit_count as usize {
                vars[j] = if ((k >> j) & 0x01) == 1 {
                    Util::one()
                } else {
                    BigInteger::ZERO
                };
            } else {
                vars[j] = product.clone();
                product = product.mul(&v).rem(&Configs.field_prime);
            }
        }
        vars
    }

    fn checkIfProverCanCheat(
        mat: &Vec<Vec<BigInteger>>,
        valueSet: &HashSet<i32>,
        bit_count: u8,
    ) -> bool {
        let mut coeffs = vec![BigInteger::default(); 16];
        for i in 0..16 {
            coeffs[i] = mat[i][16].clone();
        }

        let mut validResults = 0;
        let mut outsidePermissibleSet = 0;

        // loop over the whole permissible domain (recall that input & output
        // are bounded)

        for k in 0..256 * 256 {
            let mut variableValues = Self::getVariableValues(k, bit_count);
            let mut result = BigInteger::ZERO;
            for i in 0..16 {
                result = result.add(variableValues[i].clone().mul(&coeffs[i]));
            }
            result = result.rem(&Configs.field_prime);
            if result == Util::one() {
                validResults += 1;
                if !valueSet.contains(&k) {
                    outsidePermissibleSet += 1;
                }
            }
        }
        if validResults != 16 || outsidePermissibleSet != 0 {
            //println!("Prover can cheat with linear system solution");
            //println!("Num of valid values that the prover can use = " + validResults);
            //println!("Num of valid values outside permissible set = " + validResults);
            true
        } else {
            false
        }
    }
}

impl GadgetConfig for Gadget<AESSBoxGadgetOptimized2> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
