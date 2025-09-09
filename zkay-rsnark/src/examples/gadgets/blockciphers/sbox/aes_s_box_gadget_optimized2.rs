#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        StructNameConfig,
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator, put_active_circuit_generator,
            },
            circuit_generator::{CGConfigFields, CGInstance},
            wire::{GetWireId, WireConfig},
            wire_type::WireType,
        },
    },
    examples::gadgets::blockciphers::{
        aes128_cipher_gadget::AES128CipherGadget,
        sbox::util::linear_system_solver::LinearSystemSolver,
    },
    examples::gadgets::{
        hash::{
            sha256_gadget,
            sha256_gadget::{Base, SHA256Gadget},
        },
        math::{field_division_gadget, field_division_gadget::FieldDivisionGadget},
    },
    util::util::{ARcCell, BigInteger, Util},
};

// extern crate rand;

use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Rem, Sub},
    sync::{
        OnceLock,
        atomic::{self, AtomicU8, Ordering},
    },
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;
// use rand_core::SeedableRng;
use rand::{
    prelude::*,
    rngs::SmallRng,
    seq::SliceRandom,
    {Rng, thread_rng},
};
use rand_chacha::ChaCha8Rng;
use rand_pcg::Pcg64Mcg;
use rip_shuffle::RipShuffleParallel;
pub static s_all_coeff_set: OnceLock<Vec<Vec<Vec<BigInteger>>>> = OnceLock::new();

//  * bitCount represents how many bits are going to be used to construct the
//  * linear systems. Setting bitCount to 0 will yield almost the same circuit
//  * size as in AESBoxGadgetOptimized1.java. Setting bitcount to 16 will
//  * almost make it very hard to find a solution. Setting bitCount to x, where
//  * 16 > x > 0, means that x columns from the linear system will be based on
//  * the bits of the element (input*256+output), and the rest are based on
//  * products (as in AESSBoxGadgetOptimized1). As x increases, the more
//  * savings. x cannot increase beyond 16.

pub static atomic_bit_count: AtomicU8 = AtomicU8::new(15);

//  * This gadget implements the efficient read-only memory access from xjsnark,
//  * while making use of some properties of the AES circuit to gain more savings.
//  *
//  * Instead of constructing the linear systems using vector of powers like the
//  * AESSBoxGadgetOptimized1, this gadget relies on the observation that the bits
//  * of the input and output (to the lookup operations) are already available or
//  * will be needed later in the circuit. The gadget uses these bits partially to
//  * construct the linear systems, but this has to be done carefully to make sure
//  * that the prover cannot cheat. This might require shuffling and multiple
//  * attempts, while checking all other possibilities that a prover could use to
//  * cheat. See the bitCount parameter below.

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AESSBoxGadgetOptimized2 {
    pub input: WireType,
    pub output: Vec<Option<WireType>>,
}
impl AESSBoxGadgetOptimized2 {
    #[inline]
    pub fn new(input: WireType, generator: RcCell<CircuitGenerator>) -> Gadget<Self> {
        Self::new_with_option(input, &None, generator)
    }
    pub fn new_with_option(
        input: WireType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                output: vec![],
                input,
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<AESSBoxGadgetOptimized2> {
    const SBOX: [u8; 256] = Gadget::<AES128CipherGadget>::SBOX;
    //static
    fn preprocessing(bit_count: u8) -> &'static Vec<Vec<BigInteger>> {
        // preprocessing
        let all_coeff_set = s_all_coeff_set
            .get_or_init(|| (0..16).map(|b| Self::solve_linear_systems(b)).collect());
        &all_coeff_set[bit_count as usize]
    }

    pub fn set_bit_count(x: i32) {
        assert!(x >= 0 && x <= 16);
        atomic_bit_count.store(x as u8, Ordering::Relaxed);
    }

    pub fn solve_linear_systems(bit_count: u8) -> Vec<Vec<BigInteger>> {
        let mut seed = 1;
        let mut all_coeff_set = Vec::new();
        let mut list = Vec::new();
        for i in 0..=255 {
            list.push(256 * i as i32 + Self::SBOX[i] as i32);
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
            all_coeff_set.clear();

            for i in 0..=15 {
                let mut mat = vec![vec![BigInteger::default(); 17]; 16];
                let mut member_value_set = HashSet::new();

                for k in 0..mat.len() {
                    let memberValue = list[k + i * 16];
                    member_value_set.insert(memberValue);
                    mat[k][16] = Util::one();

                    // now extract the values that correspond to memberValue
                    // the method get_variable_values takes the bitCount settings
                    // into account
                    let variable_values = Self::get_variable_values(memberValue, bit_count);
                    for j in 0..=15 {
                        mat[k][j] = variable_values[j].clone();
                    }
                }

                mat = LinearSystemSolver::new(mat).solve_in_place();

                if Self::check_if_prover_can_cheat(&mat, &member_value_set, bit_count) {
                    println!("Invalid solution {bit_count} {i} {}", all_coeff_set.len());
                    for ii in 0..16 {
                        if mat[ii][16] == BigInteger::ZERO {
                            println!(
                                "Possibly invalid due to having zero coefficient(s) {i} {}",
                                all_coeff_set.len()
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
                all_coeff_set.push(coeffs);
            }
            done = true;
            //println!("Solution found!");
        }
        all_coeff_set
    }

    fn build_circuit(&mut self) {
        let generator = self.generator.clone();
        let mut output = CircuitGenerator::create_prover_witness_wire(self.generator.clone());
        let input = &self.t.input;

        let prover = crate::impl_prover!(
                                    eval(  input: WireType,
                                output: WireType
                            )  {
                    impl Instruction for Prover{
                     fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                // TODO Auto-generated method stub
                let value = evaluator.get_wire_value(&self.input);
                evaluator.set_wire_value(&self.output, &BigInteger::from(Gadget::<AESSBoxGadgetOptimized2>::SBOX[value.to_str_radix(10).parse::<usize>().unwrap()]));
        Ok(())
                    }
                    }
                                }
                            );
        CircuitGenerator::specify_prover_witness_computation(generator.clone(), prover);

        // Although we are getting the bits below anyway (which implicitly
        // restricts the bitwidth), it's a safer practice to call
        // restrict_bit_length() explicitly to avoid some special cases with
        // get_bit_wires().
        // Similar operations get filtered later, so this won't add extra
        // constraints.
        output.restrict_bit_length(8);
        input.restrict_bit_length(8);

        let bits_in = input.get_bit_wiresi(8).as_array().clone();
        let bits_out = output.get_bit_wiresi(8).as_array().clone();
        let mut vars = vec![None; 16];
        let mut p = input.muli(256).add(&output).add(1);
        let mut current_product = p.clone();
        let bit_count = atomic_bit_count.load(Ordering::Relaxed);
        if bit_count != 0 && bit_count != 16 {
            current_product = current_product.clone().mul(&current_product);
        }
        for i in 0..16 {
            if i < bit_count as usize {
                if i < 8 {
                    vars[i] = bits_out[i].clone();
                } else {
                    vars[i] = bits_in[i - 8].clone();
                }
            } else {
                vars[i] = Some(current_product.clone());
                if i != 15 {
                    current_product = current_product.mul(&p);
                }
            }
        }
        let all_coeff_set = Self::preprocessing(bit_count);
        let mut product = generator.get_one_wire().unwrap();
        for coeffs in all_coeff_set {
            let mut accum = generator.get_zero_wire().unwrap();
            for j in 0..vars.len() {
                accum = accum.add(vars[j].as_ref().unwrap().mulb(&coeffs[j]));
            }
            accum = accum.sub(1);
            product = product.mul(accum);
        }
        self.t.output = vec![Some(output)];
        CircuitGenerator::add_zero_assertion(generator.clone(), &product);
    }

    fn get_variable_values(k: i32, bit_count: u8) -> Vec<BigInteger> {
        let mut vars = vec![BigInteger::default(); 16];
        let mut v = BigInteger::from(k).add(Util::one());
        let mut product = v.clone();
        if bit_count != 0 {
            product = product.mul(&v).rem(&CONFIGS.field_prime);
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
                product = product.mul(&v).rem(&CONFIGS.field_prime);
            }
        }
        vars
    }

    fn check_if_prover_can_cheat(
        mat: &Vec<Vec<BigInteger>>,
        value_set: &HashSet<i32>,
        bit_count: u8,
    ) -> bool {
        let mut coeffs = vec![BigInteger::default(); 16];
        for i in 0..16 {
            coeffs[i] = mat[i][16].clone();
        }

        let mut valid_results = 0;
        let mut outside_permissible_set = 0;

        // loop over the whole permissible domain (recall that input & output
        // are bounded)

        for k in 0..256 * 256 {
            let mut variable_values = Self::get_variable_values(k, bit_count);
            let mut result = BigInteger::ZERO;
            for i in 0..16 {
                result = result.add(variable_values[i].clone().mul(&coeffs[i]));
            }
            result = result.rem(&CONFIGS.field_prime);
            if result == Util::one() {
                valid_results += 1;
                if !value_set.contains(&k) {
                    outside_permissible_set += 1;
                }
            }
        }
        if valid_results != 16 || outside_permissible_set != 0 {
            //println!("Prover can cheat with linear system solution");
            //println!("Num of valid values that the prover can use = " + valid_results);
            //println!("Num of valid values outside permissible set = " + valid_results);
            true
        } else {
            false
        }
    }
}

impl GadgetConfig for Gadget<AESSBoxGadgetOptimized2> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
