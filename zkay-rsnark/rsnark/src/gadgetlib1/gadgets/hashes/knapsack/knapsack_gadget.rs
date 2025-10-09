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
use libsnark/gadgetlib1/gadgets/basic_gadgets;
use libsnark/gadgetlib1/gadgets/hashes/hash_io;



/************************** Choice of dimension ******************************/

template<typename FieldT>
struct knapsack_dimension {
    // the size of FieldT should be (approximately) at least 200 bits
    static const size_t dimension = 1;
};

/*********************** Knapsack with field output **************************/

template<typename FieldT>
class knapsack_CRH_with_field_out_gadget : public gadget<FieldT> {
private:
    static std::vector<FieldT> knapsack_coefficients;
    static size_t num_cached_coefficients;

public:
    size_t input_len;
    size_t dimension;

    block_variable<FieldT> input_block;
    pb_linear_combination_array<FieldT> output;

    knapsack_CRH_with_field_out_gadget(protoboard<FieldT> &pb,
                                       const size_t input_len,
                                       const block_variable<FieldT> &input_block,
                                       const pb_linear_combination_array<FieldT> &output,
                                       const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();

    static size_t get_digest_len();
    static size_t get_block_len(); /* return 0 as block length, as the hash function is variable-input */
    static std::vector<FieldT> get_hash(const ffec::bit_vector &input);
    static void sample_randomness(const size_t input_len);

    /* for debugging */
    static size_t expected_constraints();
};

/********************** Knapsack with binary output **************************/

template<typename FieldT>
class knapsack_CRH_with_bit_out_gadget : public gadget<FieldT> {
public:
    type ffec::bit_vector hash_value_type;
    type merkle_authentication_path merkle_authentication_path_type;

    size_t input_len;
    size_t dimension;

    pb_linear_combination_array<FieldT> output;

    std::shared_ptr<knapsack_CRH_with_field_out_gadget<FieldT> > hasher;

    block_variable<FieldT> input_block;
    digest_variable<FieldT> output_digest;

    knapsack_CRH_with_bit_out_gadget(protoboard<FieldT> &pb,
                                     const size_t input_len,
                                     const block_variable<FieldT> &input_block,
                                     const digest_variable<FieldT> &output_digest,
                                     const std::string &annotation_prefix);
    void generate_r1cs_constraints(const bool enforce_bitness=true);
    void generate_r1cs_witness();

    static size_t get_digest_len();
    static size_t get_block_len(); /* return 0 as block length, as the hash function is variable-input */
    static hash_value_type get_hash(const ffec::bit_vector &input);
    static void sample_randomness(const size_t input_len);

    /* for debugging */
    static size_t expected_constraints(const bool enforce_bitness=true);
};

template<typename FieldT>
void test_knapsack_CRH_with_bit_out_gadget();



use libsnark/gadgetlib1/gadgets/hashes/knapsack/knapsack_gadget;

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

use ffec::algebra::fields::field_utils;
use ffec::common::rng;



template<typename FieldT>
std::vector<FieldT> knapsack_CRH_with_field_out_gadget<FieldT>::knapsack_coefficients;
template<typename FieldT>
size_t knapsack_CRH_with_field_out_gadget<FieldT>::num_cached_coefficients;

template<typename FieldT>
knapsack_CRH_with_field_out_gadget<FieldT>::knapsack_CRH_with_field_out_gadget(protoboard<FieldT> &pb,
                                                                               const size_t input_len,
                                                                               const block_variable<FieldT> &input_block,
                                                                               const pb_linear_combination_array<FieldT> &output,
                                                                               const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    input_len(input_len),
    dimension(knapsack_dimension<FieldT>::dimension),
    input_block(input_block),
    output(output)
{
    assert!(input_block.bits.size() == input_len);
    if (num_cached_coefficients < dimension * input_len)
    {
        sample_randomness(input_len);
    }
    assert!(output.size() == self.get_digest_len());
}

template<typename FieldT>
void knapsack_CRH_with_field_out_gadget<FieldT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < dimension; ++i)
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1,
                                                             pb_coeff_sum<FieldT>(input_block.bits,
                                                                                  std::vector<FieldT>(knapsack_coefficients.begin() + input_len * i,
                                                                                                      knapsack_coefficients.begin() + input_len * (i+1))),
                                                             output[i]), FMT(self.annotation_prefix, " knapsack_{}", i));
    }
}

template<typename FieldT>
void knapsack_CRH_with_field_out_gadget<FieldT>::generate_r1cs_witness()
{
    const ffec::bit_vector input = input_block.get_block();

    for (size_t i = 0; i < dimension; ++i)
    {
        FieldT sum = FieldT::zero();
        for (size_t k = 0; k < input_len; ++k)
        {
            if (input[k])
            {
                sum += knapsack_coefficients[input_len*i + k];
            }
        }

        self.pb.lc_val(output[i]) = sum;
    }
}

template<typename FieldT>
size_t knapsack_CRH_with_field_out_gadget<FieldT>::get_digest_len()
{
    return knapsack_dimension<FieldT>::dimension;
}

template<typename FieldT>
size_t knapsack_CRH_with_field_out_gadget<FieldT>::get_block_len()
{
    return 0;
}

template<typename FieldT>
std::vector<FieldT> knapsack_CRH_with_field_out_gadget<FieldT>::get_hash(const ffec::bit_vector &input)
{
    const size_t dimension = knapsack_dimension<FieldT>::dimension;
    if (num_cached_coefficients < dimension * input.size())
    {
        sample_randomness(input.size());
    }

    std::vector<FieldT> result(dimension, FieldT::zero());

    for (size_t i = 0; i < dimension; ++i)
    {
        for (size_t k = 0; k < input.size(); ++k)
        {
            if (input[k])
            {
                result[i] += knapsack_coefficients[input.size()*i + k];
            }
        }
    }

    return result;
}

template<typename FieldT>
size_t knapsack_CRH_with_field_out_gadget<FieldT>::expected_constraints()
{
    return knapsack_dimension<FieldT>::dimension;
}

template<typename FieldT>
void knapsack_CRH_with_field_out_gadget<FieldT>::sample_randomness(const size_t input_len)
{
    const size_t num_coefficients = knapsack_dimension<FieldT>::dimension * input_len;
    if (num_coefficients > num_cached_coefficients)
    {
        knapsack_coefficients.resize(num_coefficients);
        for (size_t i = num_cached_coefficients; i < num_coefficients; ++i)
        {
            knapsack_coefficients[i] = ffec::SHA512_rng<FieldT>(i);
        }
        num_cached_coefficients = num_coefficients;
    }
}

template<typename FieldT>
knapsack_CRH_with_bit_out_gadget<FieldT>::knapsack_CRH_with_bit_out_gadget(protoboard<FieldT> &pb,
                                                                           const size_t input_len,
                                                                           const block_variable<FieldT> &input_block,
                                                                           const digest_variable<FieldT> &output_digest,
                                                                           const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    input_len(input_len),
    dimension(knapsack_dimension<FieldT>::dimension),
    input_block(input_block),
    output_digest(output_digest)
{
    assert!(output_digest.bits.size() == self.get_digest_len());

    output.resize(dimension);

    for (size_t i = 0; i < dimension; ++i)
    {
        output[i].assign(pb, pb_packing_sum<FieldT>(pb_variable_array<FieldT>(output_digest.bits.begin() + i * FieldT::size_in_bits(),
                                                                              output_digest.bits.begin() + (i + 1) * FieldT::size_in_bits())));
    }

    hasher.reset(new knapsack_CRH_with_field_out_gadget<FieldT>(pb, input_len, input_block, output, FMT(annotation_prefix, " hasher")));
}


template<typename FieldT>
void knapsack_CRH_with_bit_out_gadget<FieldT>::generate_r1cs_constraints(const bool enforce_bitness)
{
    hasher->generate_r1cs_constraints();

    if (enforce_bitness)
    {
        for (size_t k = 0; k < output_digest.bits.size(); ++k)
        {
            generate_boolean_r1cs_constraint<FieldT>(self.pb, output_digest.bits[k], FMT(self.annotation_prefix, " output_digest_{}", k));
        }
    }
}

template<typename FieldT>
void knapsack_CRH_with_bit_out_gadget<FieldT>::generate_r1cs_witness()
{
    hasher->generate_r1cs_witness();

    /* do unpacking in place */
    const ffec::bit_vector input = input_block.bits.get_bits(self.pb);
    for (size_t i = 0; i < dimension; ++i)
    {
        pb_variable_array<FieldT> va(output_digest.bits.begin() + i * FieldT::size_in_bits(),
                                     output_digest.bits.begin() + (i + 1) * FieldT::size_in_bits());
        va.fill_with_bits_of_field_element(self.pb, self.pb.lc_val(output[i]));
    }
}

template<typename FieldT>
size_t knapsack_CRH_with_bit_out_gadget<FieldT>::get_digest_len()
{
    return knapsack_dimension<FieldT>::dimension * FieldT::size_in_bits();
}

template<typename FieldT>
size_t knapsack_CRH_with_bit_out_gadget<FieldT>::get_block_len()
{
     return 0;
}

template<typename FieldT>
ffec::bit_vector knapsack_CRH_with_bit_out_gadget<FieldT>::get_hash(const ffec::bit_vector &input)
{
    const std::vector<FieldT> hash_elems = knapsack_CRH_with_field_out_gadget<FieldT>::get_hash(input);
    hash_value_type result;

    for (const FieldT &elt : hash_elems)
    {
        ffec::bit_vector elt_bits = ffec::convert_field_element_to_bit_vector<FieldT>(elt);
        result.insert(result.end(), elt_bits.begin(), elt_bits.end());
    }

    return result;
}

template<typename FieldT>
size_t knapsack_CRH_with_bit_out_gadget<FieldT>::expected_constraints(const bool enforce_bitness)
{
    const size_t hasher_constraints = knapsack_CRH_with_field_out_gadget<FieldT>::expected_constraints();
    const size_t bitness_constraints = (enforce_bitness ? get_digest_len() : 0);
    return hasher_constraints + bitness_constraints;
}

template<typename FieldT>
void knapsack_CRH_with_bit_out_gadget<FieldT>::sample_randomness(const size_t input_len)
{
    knapsack_CRH_with_field_out_gadget<FieldT>::sample_randomness(input_len);
}

template<typename FieldT>
void test_knapsack_CRH_with_bit_out_gadget_internal(const size_t dimension, const ffec::bit_vector &input_bits, const ffec::bit_vector &digest_bits)
{
    assert!(knapsack_dimension<FieldT>::dimension == dimension);
    knapsack_CRH_with_bit_out_gadget<FieldT>::sample_randomness(input_bits.size());
    protoboard<FieldT> pb;

    block_variable<FieldT> input_block(pb, input_bits.size(), "input_block");
    digest_variable<FieldT> output_digest(pb, knapsack_CRH_with_bit_out_gadget<FieldT>::get_digest_len(), "output_digest");
    knapsack_CRH_with_bit_out_gadget<FieldT> H(pb, input_bits.size(), input_block, output_digest, "H");

    input_block.generate_r1cs_witness(input_bits);
    H.generate_r1cs_constraints();
    H.generate_r1cs_witness();

    assert!(output_digest.get_digest().size() == digest_bits.size());
    assert!(pb.is_satisfied());

    const size_t num_constraints = pb.num_constraints();
    const size_t expected_constraints = knapsack_CRH_with_bit_out_gadget<FieldT>::expected_constraints();
    assert!(num_constraints == expected_constraints);
}



//#endif // KNAPSACK_GADGET_TCC_
