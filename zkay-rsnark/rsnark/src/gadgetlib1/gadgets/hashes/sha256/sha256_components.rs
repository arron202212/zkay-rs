/** @file
 *****************************************************************************

 Declaration of interfaces for gadgets for the SHA256 message schedule and round function.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_COMPONENTS_HPP_
// #define SHA256_COMPONENTS_HPP_

use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::hashes::hash_io;
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_aux;



const SHA256_digest_size:usize =256;
const SHA256_block_size:usize =512;

// 
// pb_linear_combination_array<FieldT> SHA256_default_IV(pb:RcCell<protoboard<FieldT>> );


pub struct sha256_message_schedule_gadget<FieldT> {
//  : public gadget
W_bits:    Vec<pb_variable_array<FieldT> >,
pack_W:    Vec<RcCell<packing_gadget<FieldT> > >,

sigma0:    Vec<pb_variable<FieldT> >,
sigma1:    Vec<pb_variable<FieldT> >,
compute_sigma0:    Vec<RcCell<small_sigma_gadget<FieldT> > >,
compute_sigma1:    Vec<RcCell<small_sigma_gadget<FieldT> > >,
unreduced_W:    Vec<pb_variable<FieldT> >,
mod_reduce_W:    Vec<RcCell<lastbits_gadget<FieldT> > >,

M:    pb_variable_array<FieldT>,
packed_W:    pb_variable_array<FieldT>,

}


pub struct sha256_round_function_gadget <FieldT> {
// : public gadget
sigma0:    pb_variable<FieldT>,
sigma1:    pb_variable<FieldT>,
compute_sigma0:    RcCell<big_sigma_gadget<FieldT> >,
compute_sigma1:    RcCell<big_sigma_gadget<FieldT> >,
choice:    pb_variable<FieldT>,
majority:    pb_variable<FieldT>,
compute_choice:    RcCell<choice_gadget<FieldT> >,
compute_majority:    RcCell<majority_gadget<FieldT> >,
packed_d:    pb_variable<FieldT>,
pack_d:    RcCell<packing_gadget<FieldT> >,
packed_h:    pb_variable<FieldT>,
pack_h:    RcCell<packing_gadget<FieldT> >,
unreduced_new_a:    pb_variable<FieldT>,
unreduced_new_e:    pb_variable<FieldT>,
mod_reduce_new_a:    RcCell<lastbits_gadget<FieldT> >,
mod_reduce_new_e:    RcCell<lastbits_gadget<FieldT> >,
packed_new_a:    pb_variable<FieldT>,
packed_new_e:    pb_variable<FieldT>,

a:    pb_linear_combination_array<FieldT>,
b:    pb_linear_combination_array<FieldT>,
c:    pb_linear_combination_array<FieldT>,
d:    pb_linear_combination_array<FieldT>,
e:    pb_linear_combination_array<FieldT>,
f:    pb_linear_combination_array<FieldT>,
g:    pb_linear_combination_array<FieldT>,
h:    pb_linear_combination_array<FieldT>,
W:    pb_variable<FieldT>,
K:    long,
new_a:    pb_linear_combination_array<FieldT>,
new_e:    pb_linear_combination_array<FieldT>,

    
}



// use crate::gadgetlib1::gadgets::hashes::sha256::sha256_components;

//#endif // SHA256_COMPONENTS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for gadgets for the SHA256 message schedule and round function.

 See sha256_components.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_COMPONENTS_TCC_
// #define SHA256_COMPONENTS_TCC_



const  SHA256_K:[u64;64] =  
    [0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2]
;

const SHA256_H:[u64;8] = 
    [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]
;


 pub fn SHA256_default_IV(pb:RcCell<protoboard<FieldT>> )->pb_linear_combination_array<FieldT>
{
    let mut  result=pb_linear_combination_array::<FieldT>::new();
    result.reserve(SHA256_digest_size);

    for i in 0..SHA256_digest_size
    {
        let  iv_val = (SHA256_H[i / 32] >> (31-(i % 32))) & 1;

        let mut iv_element=pb_linear_combination::<FieldT> ::new();
        iv_element.assign(&pb, iv_val * ONE);
        iv_element.evaluate(pb);

        result.push(iv_element);
    }

    return result;
}

impl sha256_message_schedule_gadget<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                       M:&pb_variable_array<FieldT>,
                                                                       packed_W:&pb_variable_array<FieldT>,
                                                                       annotation_prefix:&String)->Self
    
{
    W_bits.resize(64);

    pack_W.resize(16);
    for i in 0..16
    {
        W_bits[i] = pb_variable_array::<FieldT>(M.rbegin() + (15-i) * 32, M.rbegin() + (16-i) * 32);
        pack_W[i]=RcCell::new(packing_gadget::<FieldT>::new(pb, W_bits[i], packed_W[i], FMT(self.annotation_prefix, " pack_W_{}", i)));
    }

    /* NB: some of those will be un-allocated */
    sigma0.resize(64);
    sigma1.resize(64);
    compute_sigma0.resize(64);
    compute_sigma1.resize(64);
    unreduced_W.resize(64);
    mod_reduce_W.resize(64);

    for i in 16..64
    {
        /* allocate result variables for sigma0/sigma1 invocations */
        sigma0[i].allocate(&pb, FMT(self.annotation_prefix, " sigma0_{}", i));
        sigma1[i].allocate(&pb, FMT(self.annotation_prefix, " sigma1_{}", i));

        /* compute sigma0/sigma1 */
        compute_sigma0[i]=RcCell::new(small_sigma_gadget::<FieldT>::new(pb, W_bits[i-15], sigma0[i], 7, 18, 3, FMT(self.annotation_prefix, " compute_sigma0_{}", i)));
        compute_sigma1[i]=RcCell::new(small_sigma_gadget::<FieldT>::new(pb, W_bits[i-2], sigma1[i], 17, 19, 10, FMT(self.annotation_prefix, " compute_sigma1_{}", i)));

        /* unreduced_W = sigma0(W_{i-15}) + sigma1(W_{i-2}) + W_{i-7} + W_{i-16} before modulo 2^32 */
        unreduced_W[i].allocate(&pb, FMT(self.annotation_prefix, " unreduced_W_{}", i));

        /* allocate the bit representation of packed_W[i] */
        W_bits[i].allocate(&pb, 32, FMT(self.annotation_prefix, " W_bits_{}", i));

        /* and finally reduce this into packed and bit representations */
        mod_reduce_W[i]=RcCell::new(lastbits_gadget::<FieldT>::new(pb, unreduced_W[i], 32+2, packed_W[i], W_bits[i], FMT(self.annotation_prefix, " mod_reduce_W_{}", i)));
    }
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{M,
    packed_W}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..16
    {
        pack_W[i].generate_r1cs_constraints(false); // do not enforce bitness here; caller be aware.
    }

    for i in 16..64
    {
        compute_sigma0[i].generate_r1cs_constraints();
        compute_sigma1[i].generate_r1cs_constraints();

        self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(1,
                                                             sigma0[i] + sigma1[i] + packed_W[i-16] + packed_W[i-7],
                                                             unreduced_W[i]),
          FMT(self.annotation_prefix, " unreduced_W_{}", i));

        mod_reduce_W[i].generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    for i in 0..16
    {
        pack_W[i].generate_r1cs_witness_from_bits();
    }

    for i in 16..64
    {
        compute_sigma0[i].generate_r1cs_witness();
        compute_sigma1[i].generate_r1cs_witness();

        self.pb.borrow().val(&unreduced_W[i]) = self.pb.borrow().val(&sigma0[i]) + self.pb.borrow().val(&sigma1[i]) + self.pb.borrow().val(&packed_W[i-16]) + self.pb.borrow().val(&packed_W[i-7]);
        mod_reduce_W[i].generate_r1cs_witness();
    }
}
}
impl sha256_round_function_gadget<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                   a:&pb_linear_combination_array<FieldT>,
                                                                   b:&pb_linear_combination_array<FieldT>,
                                                                   c:&pb_linear_combination_array<FieldT>,
                                                                   d:&pb_linear_combination_array<FieldT>,
                                                                   e:&pb_linear_combination_array<FieldT>,
                                                                   f:&pb_linear_combination_array<FieldT>,
                                                                   g:&pb_linear_combination_array<FieldT>,
                                                                   h:&pb_linear_combination_array<FieldT>,
                                                                   W:&pb_variable<FieldT>,
                                                                   K:&long,
                                                                   new_a:&pb_linear_combination_array<FieldT>,
                                                                   new_e:&pb_linear_combination_array<FieldT>,
                                                                   annotation_prefix:&String)->Self
   
{
    /* compute sigma0 and sigma1 */
    sigma0.allocate(&pb, FMT(self.annotation_prefix, " sigma0"));
    sigma1.allocate(&pb, FMT(self.annotation_prefix, " sigma1"));
    compute_sigma0=RcCell::new(big_sigma_gadget::<FieldT>::new(pb, a, sigma0, 2, 13, 22, FMT(self.annotation_prefix, " compute_sigma0")));
    compute_sigma1=RcCell::new(big_sigma_gadget::<FieldT>::new(pb, e, sigma1, 6, 11, 25, FMT(self.annotation_prefix, " compute_sigma1")));

    /* compute choice */
    choice.allocate(&pb, FMT(self.annotation_prefix, " choice"));
    compute_choice=RcCell::new(choice_gadget::<FieldT>::new(pb, e, f, g, choice, FMT(self.annotation_prefix, " compute_choice")));

    /* compute majority */
    majority.allocate(&pb, FMT(self.annotation_prefix, " majority"));
    compute_majority=RcCell::new(majority_gadget::<FieldT>::new(pb, a, b, c, majority, FMT(self.annotation_prefix, " compute_majority")));

    /* pack d */
    packed_d.allocate(&pb, FMT(self.annotation_prefix, " packed_d"));
    pack_d=RcCell::new(packing_gadget::<FieldT>::new(pb, d, packed_d, FMT(self.annotation_prefix, " pack_d")));

    /* pack h */
    packed_h.allocate(&pb, FMT(self.annotation_prefix, " packed_h"));
    pack_h=RcCell::new(packing_gadget::<FieldT>::new(pb, h, packed_h, FMT(self.annotation_prefix, " pack_h")));

    /* compute the actual results for the round */
    unreduced_new_a.allocate(&pb, FMT(self.annotation_prefix, " unreduced_new_a"));
    unreduced_new_e.allocate(&pb, FMT(self.annotation_prefix, " unreduced_new_e"));

    packed_new_a.allocate(&pb, FMT(self.annotation_prefix, " packed_new_a"));
    packed_new_e.allocate(&pb, FMT(self.annotation_prefix, " packed_new_e"));

    mod_reduce_new_a=RcCell::new(lastbits_gadget::<FieldT>::new(pb, unreduced_new_a, 32+3, packed_new_a, new_a, FMT(self.annotation_prefix, " mod_reduce_new_a")));
    mod_reduce_new_e=RcCell::new(lastbits_gadget::<FieldT>::new(pb, unreduced_new_e, 32+3, packed_new_e, new_e, FMT(self.annotation_prefix, " mod_reduce_new_e")));
    //  gadget<FieldT>(&pb, annotation_prefix),
    Self{a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    W,
    K,
    new_a,
    new_e}
}


pub fn generate_r1cs_constraints()
{
    compute_sigma0.generate_r1cs_constraints();
    compute_sigma1.generate_r1cs_constraints();

    compute_choice.generate_r1cs_constraints();
    compute_majority.generate_r1cs_constraints();

    pack_d.generate_r1cs_constraints(false);
    pack_h.generate_r1cs_constraints(false);

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(1,
                                                         packed_h + sigma1 + choice + K + W + sigma0 + majority,
                                                         unreduced_new_a),
      FMT(self.annotation_prefix, " unreduced_new_a"));

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(1,
                                                         packed_d + packed_h + sigma1 + choice + K + W,
                                                         unreduced_new_e),
      FMT(self.annotation_prefix, " unreduced_new_e"));

    mod_reduce_new_a.generate_r1cs_constraints();
    mod_reduce_new_e.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_sigma0.generate_r1cs_witness();
    compute_sigma1.generate_r1cs_witness();

    compute_choice.generate_r1cs_witness();
    compute_majority.generate_r1cs_witness();

    pack_d.generate_r1cs_witness_from_bits();
    pack_h.generate_r1cs_witness_from_bits();

    self.pb.borrow().val(&unreduced_new_a) = self.pb.borrow().val(&packed_h) + self.pb.borrow().val(&sigma1) + self.pb.borrow().val(&choice) + FieldT(K) + self.pb.borrow().val(&W) + self.pb.borrow().val(&sigma0) + self.pb.borrow().val(&majority);
    self.pb.borrow().val(&unreduced_new_e) = self.pb.borrow().val(&packed_d) + self.pb.borrow().val(&packed_h) + self.pb.borrow().val(&sigma1) + self.pb.borrow().val(&choice) + FieldT(K) + self.pb.borrow().val(&W);

    mod_reduce_new_a.generate_r1cs_witness();
    mod_reduce_new_e.generate_r1cs_witness();
}
}


//#endif // SHA256_COMPONENTS_TCC_
