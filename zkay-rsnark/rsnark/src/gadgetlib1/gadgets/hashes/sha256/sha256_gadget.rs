/** @file
 *****************************************************************************

 Declaration of interfaces for top-level SHA256 gadgets.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_GADGET_HPP_
// #define SHA256_GADGET_HPP_

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::hashes::hash_io;
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_components;



/**
 * Gadget for the SHA256 compression function.
 */

pub struct sha256_compression_function_gadget<FieldT> {//gadget<FieldT>

round_a:    Vec<pb_linear_combination_array<FieldT> >,
round_b:    Vec<pb_linear_combination_array<FieldT> >,
round_c:    Vec<pb_linear_combination_array<FieldT> >,
round_d:    Vec<pb_linear_combination_array<FieldT> >,
round_e:    Vec<pb_linear_combination_array<FieldT> >,
round_f:    Vec<pb_linear_combination_array<FieldT> >,
round_g:    Vec<pb_linear_combination_array<FieldT> >,
round_h:    Vec<pb_linear_combination_array<FieldT> >,

packed_W:    pb_variable_array<FieldT>,
message_schedule:    RcCell<sha256_message_schedule_gadget<FieldT> >,
round_functions:    Vec<sha256_round_function_gadget<FieldT> >,

unreduced_output:    pb_variable_array<FieldT>,
reduced_output:    pb_variable_array<FieldT>,
reduce_output:    Vec<lastbits_gadget<FieldT> >,

prev_output:    pb_linear_combination_array<FieldT>,
new_block:    pb_variable_array<FieldT>,
output:    digest_variable<FieldT>,

 
}

/**
 * Gadget for the SHA256 compression function, viewed as a 2-to-1 hash
 * function, and using the same initialization vector as in SHA256
 * specification. Thus, any collision for
 * sha256_two_to_one_hash_gadget trivially extends to a collision for
 * full SHA256 (by appending the same padding).
 */

type hash_value_type=bit_vector;
type merkle_authentication_path_type=merkle_authentication_path;
pub struct sha256_two_to_one_hash_gadget<FieldT> {//gadget<FieldT>

    

f:    RcCell<sha256_compression_function_gadget<FieldT> >,

}



// use crate::gadgetlib1::gadgets::hashes::sha256::sha256_gadget;

//#endif // SHA256_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for top-level SHA256 gadgets.

 See sha256_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_GADGET_TCC_
// #define SHA256_GADGET_TCC_


impl sha256_compression_function_gadget<FieldT>{

pub fn new(pb:protoboard<FieldT>,
                                                                               prev_output:&pb_linear_combination_array<FieldT>,
                                                                               new_block:&pb_variable_array<FieldT>,
                                                                               output:&digest_variable<FieldT>,
                                                                               annotation_prefix:&String)->Self
    
{
    /* message schedule and inputs for it */
    packed_W.allocate(pb, 64, FMT(self.annotation_prefix, " packed_W"));
    message_schedule.reset(sha256_message_schedule_gadget::<FieldT>::new(pb, new_block, packed_W, FMT(self.annotation_prefix, " message_schedule")));

    /* initalize */
    round_a.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 7*32, prev_output.rbegin() + 8*32));
    round_b.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 6*32, prev_output.rbegin() + 7*32));
    round_c.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 5*32, prev_output.rbegin() + 6*32));
    round_d.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 4*32, prev_output.rbegin() + 5*32));
    round_e.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 3*32, prev_output.rbegin() + 4*32));
    round_f.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 2*32, prev_output.rbegin() + 3*32));
    round_g.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 1*32, prev_output.rbegin() + 2*32));
    round_h.push_back(pb_linear_combination_array::<FieldT>::new(prev_output.rbegin() + 0*32, prev_output.rbegin() + 1*32));

    /* do the rounds */
    for i in 0..64
    {
        round_h.push_back(round_g[i]);
        round_g.push_back(round_f[i]);
        round_f.push_back(round_e[i]);
        round_d.push_back(round_c[i]);
        round_c.push_back(round_b[i]);
        round_b.push_back(round_a[i]);

        let mut  new_round_a_variables=pb_variable_array::<FieldT>::new();
        new_round_a_variables.allocate(pb, 32, FMT(self.annotation_prefix, " new_round_a_variables_{}", i+1));
        round_a.push(new_round_a_variables);

        let mut  new_round_e_variables=pb_variable_array::<FieldT>::new();
        new_round_e_variables.allocate(pb, 32, FMT(self.annotation_prefix, " new_round_e_variables_{}", i+1));
        round_e.push(new_round_e_variables);

        round_functions.push_back(sha256_round_function_gadget::<FieldT>(pb,
                                                                       round_a[i], round_b[i], round_c[i], round_d[i],
                                                                       round_e[i], round_f[i], round_g[i], round_h[i],
                                                                       packed_W[i], SHA256_K[i], round_a[i+1], round_e[i+1],
                                                                       FMT(self.annotation_prefix, " round_functions_{}", i)));
    }

    /* finalize */
    unreduced_output.allocate(pb, 8, FMT(self.annotation_prefix, " unreduced_output"));
    reduced_output.allocate(pb, 8, FMT(self.annotation_prefix, " reduced_output"));
    for i in 0..8
    {
        reduce_output.push_back(lastbits_gadget::<FieldT>(pb,
                                                        unreduced_output[i],
                                                        32+1,
                                                        reduced_output[i],
                                                        pb_variable_array::<FieldT>(output.bits.rbegin() + (7-i) * 32, output.bits.rbegin() + (8-i) * 32),
                                                        FMT(self.annotation_prefix, " reduce_output_{}", i)));
    }
    // gadget<FieldT>(pb, annotation_prefix),
   Self{prev_output,
   new_block,
    output}
}


pub fn generate_r1cs_constraints()
{
    message_schedule.generate_r1cs_constraints();
    for i in 0..64
    {
        round_functions[i].generate_r1cs_constraints();
    }

    for i in 0..4
    {
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1,
                                                             round_functions[3-i].packed_d + round_functions[63-i].packed_new_a,
                                                             unreduced_output[i]),
            FMT(self.annotation_prefix, " unreduced_output_{}", i));

        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1,
                                                             round_functions[3-i].packed_h + round_functions[63-i].packed_new_e,
                                                             unreduced_output[4+i]),
            FMT(self.annotation_prefix, " unreduced_output_{}", 4+i));
    }

    for i in 0..8
    {
        reduce_output[i].generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    message_schedule.generate_r1cs_witness();

// #ifdef DEBUG
    print!("Input:\n");
    for j in 0..16
    {
        print!("{} ", self.pb.val(packed_W[j]).as_ulong());
    }
    print!("\n");
//#endif

    for i in 0..64
    {
        round_functions[i].generate_r1cs_witness();
    }

    for i in 0..4
    {
        self.pb.val(unreduced_output[i]) = self.pb.val(round_functions[3-i].packed_d) + self.pb.val(round_functions[63-i].packed_new_a);
        self.pb.val(unreduced_output[4+i]) = self.pb.val(round_functions[3-i].packed_h) + self.pb.val(round_functions[63-i].packed_new_e);
    }

    for i in 0..8
    {
        reduce_output[i].generate_r1cs_witness();
    }

// #ifdef DEBUG
    print!("Output:\n");
    for j in 0..8
    {
        print!("{} ", self.pb.val(reduced_output[j]).as_ulong());
    }
    print!("\n");
//#endif
}
}

impl sha256_two_to_one_hash_gadget<FieldT>{
pub fn new(pb:protoboard<FieldT>,
                                                                     left:&digest_variable<FieldT>,
                                                                     right:&digest_variable<FieldT>,
                                                                     output:&digest_variable<FieldT>,
                                                                     annotation_prefix:&String)->Self
   
{
    /* concatenate block = left || right */
    let mut  block=pb_variable_array::<FieldT>::new();
    block.insert(block.end(), left.bits.begin(), left.bits.end());
    block.insert(block.end(), right.bits.begin(), right.bits.end());

    /* compute the hash itself */
    f.reset(sha256_compression_function_gadget::<FieldT>::new(pb, SHA256_default_IV::<FieldT>(pb), block, output, FMT(self.annotation_prefix, " f")));
    //  gadget<FieldT>(pb, annotation_prefix)
}


pub fn new2(pb:protoboard<FieldT>,
                                                                     block_length:usize,
                                                                     input_block:&block_variable<FieldT>,
                                                                     output:&digest_variable<FieldT>,
                                                                     annotation_prefix:&String)->Self
    
{
// gadget<FieldT>(pb, annotation_prefix)
    assert!(block_length == SHA256_block_size);
    assert!(input_block.bits.len() == block_length);
    f.reset(sha256_compression_function_gadget::<FieldT>::new(pb, SHA256_default_IV::<FieldT>(pb), input_block.bits, output, FMT(self.annotation_prefix, " f")));
}


pub fn generate_r1cs_constraints(ensure_output_bitness:bool)
{
    // //ffec::UNUSED(ensure_output_bitness);
    f.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    f.generate_r1cs_witness();
}


pub fn get_block_len()->usize
{
    return SHA256_block_size;
}


 pub fn get_digest_len()->usize
{
    return SHA256_digest_size;
}


 pub fn  get_hash(input:&bit_vector)->bit_vector
{
    let mut  pb=protoboard::<FieldT> ::new();

    let mut  input_variable=block_variable::<FieldT>::new(pb, SHA256_block_size, "input");
    let mut output_variable=digest_variable::<FieldT> ::new(pb, SHA256_digest_size, "output");
    let mut  f=sha256_two_to_one_hash_gadget::<FieldT>::new(pb, SHA256_block_size, input_variable, output_variable, "f");

    input_variable.generate_r1cs_witness(input);
    f.generate_r1cs_witness();

    return output_variable.get_digest();
}


pub fn expected_constraints(ensure_output_bitness:bool)->usize
{
    //ffec::UNUSED(ensure_output_bitness);
    return 27280; /* hardcoded for now */
}

}

//#endif // SHA256_GADGET_TCC_
