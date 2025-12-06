/** @file
 *****************************************************************************

 Declaration of interfaces for the knapsack gadget.

 The gadget checks the correct execution of a knapsack (modular subset-sum) over
 the field specified in the template parameter. With suitable choices of parameters
 such knapsacks are collision-resistant hashes (CRHs). See \[Ajt96] and \[GGH96].

 Given two positive integers m (the input length) and d (the dimension),
 and a matrix M over the field F and of dimension dxm, the hash H_M maps {0,1}^m
 to F^d by sending x to M*x. Security of the function (very roughly) depends on
 d*log(|F|).

 Below, we give two different gadgets:
 - knapsack_CRH_with_field_out_gadget, which verifies H_M
 - knapsack_CRH_with_bit_out_gadget, which verifies H_M when its output is "expanded" to bits.
 In both cases, a method ("sample_randomness") allows to sample M.

 The parameter d (the dimension) is fixed at compile time in the struct
 knapsack_dimension below. The parameter m (the input length) can be chosen
 at run time (in either gadget).


 References:

 \[Ajt96]:
 "Generating hard instances of lattice problems",
 Miklos Ajtai,
 STOC 1996

 \[GGH96]:
 "Collision-free hashing from lattice problems",
 Oded Goldreich, Shafi Goldwasser, Shai Halevi,
 ECCC TR95-042

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef KNAPSACK_GADGET_HPP_
// #define KNAPSACK_GADGET_HPP_

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::hashes::hash_io;



/************************** Choice of dimension ******************************/

// 
struct knapsack_dimension<FieldT>;
 impl knapsack_dimension{
    // the size of FieldT should be (approximately) at least 200 bits
     const dimension:usize =1;
}

/*********************** Knapsack with field output **************************/


struct knapsack_CRH_with_field_out_gadget<FieldT>  {
// : public gadget<FieldT> 
knapsack_coefficients:     Vec<FieldT>,
num_cached_coefficients:     usize,


input_len:    usize,
dimension:    usize,

input_block:    block_variable<FieldT>,
output:    pb_linear_combination_array<FieldT>,

   
}

/********************** Knapsack with binary output **************************/


struct knapsack_CRH_with_bit_out_gadget<FieldT> {
//  : public gadget<FieldT> 
//     type hash_value_type=bit_vector;
//     type merkle_authentication_path_type=merkle_authentication_path;

input_len:    usize,
dimension:    usize,

output:    pb_linear_combination_array<FieldT>,

hasher:    RcCell<knapsack_CRH_with_field_out_gadget<FieldT> >,

input_block:    block_variable<FieldT>,
output_digest:    digest_variable<FieldT>,

   
}





// use crate::gadgetlib1::gadgets::hashes::knapsack::knapsack_gadget;

//#endif // KNAPSACK_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the knapsack gadget.

 See knapsack_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef KNAPSACK_GADGET_TCC_
// #define KNAPSACK_GADGET_TCC_

use ffec::algebra::field_utils::field_utils;
use ffec::common::rng;



// 
// Vec<FieldT> pub fn knapsack_coefficients;
// 
// usize pub fn num_cached_coefficients;
impl knapsack_CRH_with_field_out_gadget<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                               input_len:usize,
                                                                               input_block:block_variable<FieldT>,
                                                                               output:pb_linear_combination_array<FieldT>,
                                                                               annotation_prefix:String) ->Self
   
{
    assert!(input_block.bits.len() == input_len);
    if num_cached_coefficients < dimension * input_len
    {
        sample_randomness(input_len);
    }
    assert!(output.len() == self.get_digest_len());
    //  gadget<FieldT>(&pb, annotation_prefix),
    Self{input_len,
    dimension,
    input_block,
    output}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..dimension
    {
        self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(1,
                                                             pb_coeff_sum::<FieldT>(input_block.bits,
                                                                                  knapsack_coefficients[input_len * i.. input_len * (i+1)].to_vec()),
                                                             output[i]), FMT(self.annotation_prefix, " knapsack_{}", i));
    }
}


pub fn generate_r1cs_witness()
{
    let input = input_block.get_block();

    for i in 0..dimension
    {
        let mut  sum = FieldT::zero();
        for k in 0..input_len
        {
            if input[k]
            {
                sum += knapsack_coefficients[input_len*i + k];
            }
        }

        self.pb.borrow().lc_val(output[i]) = sum;
    }
}


 pub fn get_digest_len()->usize
{
    return knapsack_dimension::<FieldT>::dimension;
}


 pub fn get_block_len()->usize
{
    return 0;
}


 pub fn get_hash(input:bit_vector)->Vec<FieldT>
{
    let dimension = knapsack_dimension::<FieldT>::dimension;
    if num_cached_coefficients < dimension * input.len()
    {
        sample_randomness(input.len());
    }

    let mut result=vec![ FieldT::zero();dimension];

    for i in 0..dimension
    {
        for k in 0..input.len()
        {
            if input[k]
            {
                result[i] += knapsack_coefficients[input.len()*i + k];
            }
        }
    }

    return result;
}


 pub fn expected_constraints()->usize
{
    return knapsack_dimension::<FieldT>::dimension;
}


pub fn sample_randomness(input_len:usize)
{
    let num_coefficients = knapsack_dimension::<FieldT>::dimension * input_len;
    if num_coefficients > num_cached_coefficients
    {
        knapsack_coefficients.resize(num_coefficients);
        for i in num_cached_coefficients..num_coefficients
        {
            knapsack_coefficients[i] = ffec::SHA512_rng::<FieldT>(i);
        }
        num_cached_coefficients = num_coefficients;
    }
}
}

impl knapsack_CRH_with_bit_out_gadget<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                           input_len:usize,
                                                                           input_block:block_variable<FieldT>,
                                                                           output_digest:digest_variable<FieldT>,
                                                                           annotation_prefix:String)->Self
 
{
    assert!(output_digest.bits.len() == self.get_digest_len());

    output.resize(dimension);

    for i in 0..dimension
    {
        output[i].assign(&pb, pb_packing_sum::<FieldT>(pb_variable_array::<FieldT>(output_digest.bits.begin() + i * FieldT::size_in_bits(),
                                                                              output_digest.bits.begin() + (i + 1) * FieldT::size_in_bits())));
    }

    hasher=RcCell::new(knapsack_CRH_with_field_out_gadget::<FieldT>::new(pb, input_len, input_block, output, FMT(annotation_prefix, " hasher")));
    //    gadget<FieldT>(&pb, annotation_prefix),
    Self{input_len,
    dimension,
    input_block,
    output_digest}
}



pub fn generate_r1cs_constraints(enforce_bitness:bool)
{
    hasher.generate_r1cs_constraints();

    if enforce_bitness
    {
        for k in 0..output_digest.bits.len()
        {
            generate_boolean_r1cs_constraint::<FieldT>(self.pb, output_digest.bits[k], FMT(self.annotation_prefix, " output_digest_{}", k));
        }
    }
}


pub fn generate_r1cs_witness()
{
    hasher.generate_r1cs_witness();

    /* do unpacking in place */
    let input = input_block.bits.get_bits(self.pb);
    for i in 0..dimension
    {
       let mut  va=pb_variable_array::<FieldT> (output_digest.bits.begin() + i * FieldT::size_in_bits(),
                                     output_digest.bits.begin() + (i + 1) * FieldT::size_in_bits());
        va.fill_with_bits_of_field_element(self.pb, self.pb.borrow().lc_val(output[i]));
    }
}


pub fn get_digest_len()->usize
{
    return knapsack_dimension::<FieldT>::dimension * FieldT::size_in_bits();
}


pub fn get_block_len()->usize
{
     return 0;
}


pub fn get_hash(input:bit_vector)->bit_vector
{
    let hash_elems = Self::get_hash(input);
    let mut  result=hash_value_type::new();

    for elt in &hash_elems
    {
        let mut  elt_bits = ffec::convert_field_element_to_bit_vector::<FieldT>(elt);
        result.insert(result.end(), elt_bits.begin(), elt_bits.end());
    }

    return result;
}


pub fn expected_constraints(enforce_bitness:bool)->usize
{
    let hasher_constraints = Self::expected_constraints();
    let bitness_constraints = if enforce_bitness {get_digest_len()} else{0};
    return hasher_constraints + bitness_constraints;
}


pub fn sample_randomness(input_len:usize)
{
    knapsack_CRH_with_field_out_gadget::<FieldT>::sample_randomness(input_len);
}
}


pub fn  test_knapsack_CRH_with_bit_out_gadget_internal<FieldT>(dimension:usize , digest_bits:bit_vector, input_bits: bit_vector)
{
    assert!(knapsack_dimension::<FieldT>::dimension == dimension);
    knapsack_CRH_with_bit_out_gadget::<FieldT>::sample_randomness(input_bits.len());
    let mut  pb=protoboard::<FieldT>::new();

    let mut  input_block=block_variable::<FieldT>::new(pb, input_bits.len(), "input_block");
    let mut  output_digest=digest_variable::<FieldT>::new(pb, knapsack_CRH_with_bit_out_gadget::<FieldT>::get_digest_len(), "output_digest");
    let mut H= knapsack_CRH_with_bit_out_gadget::<FieldT>::new(pb, input_bits.len(), input_block, output_digest, "H");

    input_block.generate_r1cs_witness(input_bits);
    H.generate_r1cs_constraints();
    H.generate_r1cs_witness();

    assert!(output_digest.get_digest().len() == digest_bits.len());
    assert!(pb.is_satisfied());

    let num_constraints = pb.num_constraints();
    let expected_constraints = knapsack_CRH_with_bit_out_gadget::<FieldT>::expected_constraints();
    assert!(num_constraints == expected_constraints);
}



//#endif // KNAPSACK_GADGET_TCC_
