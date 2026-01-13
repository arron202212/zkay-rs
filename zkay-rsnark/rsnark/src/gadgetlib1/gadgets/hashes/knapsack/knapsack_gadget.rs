// Declaration of interfaces for the knapsack gadget.

// The gadget checks the correct execution of a knapsack (modular subset-sum) over
// the field specified in the template parameter. With suitable choices of parameters
// such knapsacks are collision-resistant hashes (CRHs). See \[Ajt96] and \[GGH96].

// Given two positive integers m (the input length) and d (the dimension),
// and a matrix M over the field F and of dimension dxm, the hash H_M maps {0,1}^m
// to F^d by sending x to M*x. Security of the function (very roughly) depends on
// d*log(|F|).

// Below, we give two different gadgets:
// - knapsack_CRH_with_field_out_gadget, which verifies H_M
// - knapsack_CRH_with_bit_out_gadget, which verifies H_M when its output is "expanded" to bits.
// In both cases, a method ("sample_randomness") allows to sample M.

// The parameter d (the dimension) is fixed at compile time in the struct
// knapsack_dimension below. The parameter m (the input length) can be chosen
// at run time (in either gadget).

// References:

// \[Ajt96]:
// "Generating hard instances of lattice problems",
// Miklos Ajtai,
// STOC 1996

// \[GGH96]:
// "Collision-free hashing from lattice problems",
// Oded Goldreich, Shafi Goldwasser, Shai Halevi,
// ECCC TR95-042
use crate::common::data_structures::merkle_tree::merkle_authentication_path;
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::generate_boolean_r1cs_constraint;
use crate::gadgetlib1::gadgets::hashes::hash_io::{
    block_variable, block_variables, digest_variable, digest_variables,
};
use crate::gadgetlib1::pb_variable::pb_coeff_sum;
use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::variable;
use ffec::FieldTConfig;
use ffec::common::utils::bit_vector;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{self, AtomicUsize, Ordering};
pub type ARcCell<Typ> = Arc<Mutex<Typ>>;
#[macro_export]
macro_rules! arc_cell_new {
    ($exp:expr) => {{ std::sync::Arc::new(parking_lot::Mutex::new($exp)) }};
}
fn SHA512_rng<FieldT: FieldTConfig>(i: usize) -> FieldT {
    FieldT::zero()
}

/************************** Choice of dimension ******************************/
// pub static knapsack_coefficients: ARcCell<Vec<FieldT>> = arc_cell_new!(Vec::new());
trait knapsack_coefficientsConfig<FieldT: FieldTConfig> {
    fn knapsack_coefficients() -> ARcCell<Vec<FieldT>> {
        arc_cell_new!(Vec::new())
    }
}
pub static num_cached_coefficients: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Default)]
pub struct knapsack_dimension<FieldT: FieldTConfig>(PhantomData<FieldT>);
impl<FieldT: FieldTConfig> knapsack_dimension<FieldT> {
    // the size of FieldT should be (approximately) at least 200 bits
    pub const dimension: usize = 1;
}

/*********************** Knapsack with field output **************************/
#[derive(Clone, Default)]
pub struct knapsack_CRH_with_field_out_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<FieldT>
    input_len: usize,
    dimension: usize,
    input_block: block_variables<FieldT, PB>,
    output: pb_linear_combination_array<FieldT, PB>,
}

/********************** Knapsack with binary output **************************/
type hash_value_type = bit_vector;
type merkle_authentication_path_type = merkle_authentication_path;

#[derive(Clone, Default)]
pub struct knapsack_CRH_with_bit_out_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget<FieldT>
    input_len: usize,
    dimension: usize,
    output: pb_linear_combination_array<FieldT, PB>,
    hasher: RcCell<knapsack_CRH_with_field_out_gadgets<FieldT, PB>>,
    input_block: block_variables<FieldT, PB>,
    output_digest: digest_variables<FieldT, PB>,
}

use ffec::algebra::field_utils::field_utils;
use ffec::common::rng;

//
// Vec<FieldT> pub fn knapsack_coefficients;
//
// usize pub fn num_cached_coefficients;

pub type knapsack_CRH_with_field_out_gadgets<FieldT, PB> =
    gadget<FieldT, PB, knapsack_CRH_with_field_out_gadget<FieldT, PB>>;

impl<FieldT: FieldTConfig, PB: PBConfig> knapsack_CRH_with_field_out_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        input_len: usize,
        input_block: block_variables<FieldT, PB>,
        output: pb_linear_combination_array<FieldT, PB>,
        annotation_prefix: String,
    ) -> knapsack_CRH_with_field_out_gadgets<FieldT, PB> {
        let dimension = knapsack_dimension::<FieldT>::dimension;
        assert!(input_block.t.bits.len() == input_len);
        if num_cached_coefficients.load(Ordering::Relaxed) < dimension * input_len {
            knapsack_CRH_with_field_out_gadget::<FieldT, PB>::sample_randomness(input_len);
        }
        assert!(output.len() == Self::get_digest_len());
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                input_len,
                dimension,
                input_block,
                output,
            },
        )
    }

    pub fn get_digest_len() -> usize {
        return knapsack_dimension::<FieldT>::dimension;
    }

    pub fn get_block_len() -> usize {
        return 0;
    }
    pub fn expected_constraints() -> usize {
        return knapsack_dimension::<FieldT>::dimension;
    }
    pub fn sample_randomness(input_len: usize) {
        let num_coefficients = knapsack_dimension::<FieldT>::dimension * input_len;
        if num_coefficients > num_cached_coefficients.load(Ordering::Relaxed) {
            Self::knapsack_coefficients()
                .lock()
                .resize(num_coefficients, FieldT::zero());
            for i in num_cached_coefficients.load(Ordering::Relaxed)..num_coefficients {
                Self::knapsack_coefficients().lock()[i] = SHA512_rng::<FieldT>(i);
            }
            num_cached_coefficients.store(num_coefficients, Ordering::Relaxed);
        }
    }
    pub fn get_hash(input: bit_vector) -> Vec<FieldT> {
        let dimension = knapsack_dimension::<FieldT>::dimension;
        if num_cached_coefficients.load(Ordering::Relaxed) < dimension * input.len() {
            Self::sample_randomness(input.len());
        }

        let mut result = vec![FieldT::zero(); dimension];

        for i in 0..dimension {
            for k in 0..input.len() {
                if input[k] {
                    result[i] += Self::knapsack_coefficients().lock()[input.len() * i + k].clone();
                }
            }
        }

        return result;
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> knapsack_coefficientsConfig<FieldT>
    for knapsack_CRH_with_field_out_gadget<FieldT, PB>
{
}

impl<FieldT: FieldTConfig, PB: PBConfig> knapsack_CRH_with_field_out_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..self.t.dimension {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    FieldT::from(1).into(),
                    pb_coeff_sum::<FieldT, PB>(
                        &(self.t.input_block.t.bits.clone().into()),
                        &knapsack_CRH_with_field_out_gadget::<FieldT, PB>::knapsack_coefficients()
                            .lock()[self.t.input_len * i..self.t.input_len * (i + 1)]
                            .to_vec(),
                    ),
                    self.t.output[i].clone(),
                ),
                prefix_format!(self.annotation_prefix, " knapsack_{}", i),
            );
        }
    }

    pub fn generate_r1cs_witness(&self) {
        let input = self.t.input_block.get_block();

        for i in 0..self.t.dimension {
            let mut sum = FieldT::zero();
            for k in 0..self.t.input_len {
                if input[k] {
                    sum +=
                        knapsack_CRH_with_field_out_gadget::<FieldT, PB>::knapsack_coefficients()
                            .lock()[self.t.input_len * i + k]
                            .clone();
                }
            }

            *self.pb.borrow_mut().lc_val_ref(&self.t.output[i]) = sum;
        }
    }
}

pub type knapsack_CRH_with_bit_out_gadgets<FieldT, PB> =
    gadget<FieldT, PB, knapsack_CRH_with_bit_out_gadget<FieldT, PB>>;

impl<FieldT: FieldTConfig, PB: PBConfig> knapsack_CRH_with_bit_out_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        input_len: usize,
        input_block: block_variables<FieldT, PB>,
        output_digest: digest_variables<FieldT, PB>,
        annotation_prefix: String,
    ) -> knapsack_CRH_with_bit_out_gadgets<FieldT, PB> {
        assert!(output_digest.t.bits.len() == Self::get_digest_len());
        let dimension = knapsack_dimension::<FieldT>::dimension;
        let mut output = pb_linear_combination_array::<FieldT, PB>::default();

        for i in 0..dimension {
            output[i].assign(
                &pb,
                &pb_packing_sum::<FieldT, PB>(
                    &(pb_variable_array::<FieldT, PB>::new(
                        output_digest.t.bits.contents
                            [i * FieldT::size_in_bits()..(i + 1) * FieldT::size_in_bits()]
                            .to_vec(),
                    )
                    .into()),
                ),
            );
        }

        let hasher = RcCell::new(knapsack_CRH_with_field_out_gadget::<FieldT, PB>::new(
            pb.clone(),
            input_len,
            input_block.clone(),
            output.clone(),
            prefix_format!(annotation_prefix, " hasher"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                input_len,
                dimension,
                output,
                hasher,
                input_block,
                output_digest,
            },
        )
    }
    pub fn get_digest_len() -> usize {
        return knapsack_dimension::<FieldT>::dimension * FieldT::size_in_bits();
    }

    pub fn get_block_len() -> usize {
        return 0;
    }

    pub fn get_hash(input: bit_vector) -> bit_vector {
        let hash_elems = Self::get_hash(input);
        let mut result = hash_value_type::new();

        for elt in &hash_elems {
            let mut elt_bits = convert_field_element_to_bit_vector(elt);
            result.extend(elt_bits);
        }

        return result;
    }

    pub fn expected_constraints(enforce_bitness: bool) -> usize {
        let hasher_constraints =
            knapsack_CRH_with_field_out_gadget::<FieldT, PB>::expected_constraints();
        let bitness_constraints = if enforce_bitness {
            Self::get_digest_len()
        } else {
            0
        };
        return hasher_constraints + bitness_constraints;
    }

    pub fn sample_randomness(input_len: usize) {
        knapsack_CRH_with_field_out_gadget::<FieldT, PB>::sample_randomness(input_len);
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> knapsack_CRH_with_bit_out_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self, enforce_bitness: bool) {
        self.t.hasher.borrow().generate_r1cs_constraints();

        if enforce_bitness {
            for k in 0..self.t.output_digest.t.bits.len() {
                generate_boolean_r1cs_constraint::<FieldT, PB>(
                    &self.pb,
                    &(self.t.output_digest.t.bits[k].clone().into()),
                    prefix_format!(self.annotation_prefix, " output_digest_{}", k),
                );
            }
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.hasher.borrow().generate_r1cs_witness();

        /* do unpacking in place */
        let input = self.t.input_block.t.bits.get_bits(&self.pb);
        for i in 0..self.t.dimension {
            let mut va = pb_variable_array::<FieldT, PB>::new(
                self.t.output_digest.t.bits.contents
                    [i * FieldT::size_in_bits()..(i + 1) * FieldT::size_in_bits()]
                    .to_vec(),
            );
            va.fill_with_bits_of_field_element(
                &self.pb,
                &self.pb.borrow().lc_val(&self.t.output[i]),
            );
        }
    }
}

pub fn test_knapsack_CRH_with_bit_out_gadget_internal<FieldT: FieldTConfig, PB: PBConfig>(
    dimension: usize,
    digest_bits: bit_vector,
    input_bits: bit_vector,
) {
    assert!(knapsack_dimension::<FieldT>::dimension == dimension);
    knapsack_CRH_with_bit_out_gadget::<FieldT, PB>::sample_randomness(input_bits.len());
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());

    let mut input_block =
        block_variable::<FieldT, PB>::new(pb.clone(), input_bits.len(), "input_block".to_owned());
    let mut output_digest = digest_variable::<FieldT, PB>::new(
        pb.clone(),
        knapsack_CRH_with_bit_out_gadget::<FieldT, PB>::get_digest_len(),
        "output_digest".to_owned(),
    );
    let mut H = knapsack_CRH_with_bit_out_gadget::<FieldT, PB>::new(
        pb.clone(),
        input_bits.len(),
        input_block.clone(),
        output_digest.clone(),
        "H".to_owned(),
    );

    input_block.generate_r1cs_witness(&input_bits);
    H.generate_r1cs_constraints(false);
    H.generate_r1cs_witness();

    assert!(output_digest.get_digest().len() == digest_bits.len());
    assert!(pb.borrow().is_satisfied());

    let num_constraints = pb.borrow().num_constraints();
    let expected_constraints =
        knapsack_CRH_with_bit_out_gadget::<FieldT, PB>::expected_constraints(false);
    assert!(num_constraints == expected_constraints);
}
