/** @file
 *****************************************************************************

 Declaration of interfaces for auxiliary gadgets for the SHA256 gadget.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_AUX_HPP_
// #define SHA256_AUX_HPP_

use crate::gadgetlib1::gadgets::basic_gadgets;




pub struct lastbits_gadget<FieldT>  {
// : public gadget<FieldT>
X:    pb_variable<FieldT>,
X_bits:    usize,
result:    pb_variable<FieldT>,
result_bits:    pb_linear_combination_array<FieldT>,

full_bits:    pb_linear_combination_array<FieldT>,
unpack_bits:    RcCell<packing_gadget<FieldT> >,
pack_result:    RcCell<packing_gadget<FieldT> >,

}


pub struct XOR3_gadget<FieldT> {
// : public gadget<FieldT> 
tmp:    pb_variable<FieldT>,

A:    pb_linear_combination<FieldT>,
B:    pb_linear_combination<FieldT>,
C:    pb_linear_combination<FieldT>,
assume_C_is_zero:    bool,
out:    pb_linear_combination<FieldT>,

}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

pub struct small_sigma_gadget<FieldT> {
//  : public gadget
W:    pb_variable_array<FieldT>,
result:    pb_variable<FieldT>,

result_bits:    pb_variable_array<FieldT>,
compute_bits:    Vec<RcCell<XOR3_gadget<FieldT> > >,
pack_result:    RcCell<packing_gadget<FieldT> >,

}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

pub struct big_sigma_gadget<FieldT> {
//  : public gadget
W:    pb_linear_combination_array<FieldT>,
result:    pb_variable<FieldT>,

result_bits:    pb_variable_array<FieldT>,
compute_bits:    Vec<RcCell<XOR3_gadget<FieldT> > >,
pack_result:    RcCell<packing_gadget<FieldT> >,

    
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

pub struct choice_gadget<FieldT> {
// // : public gadget
result_bits:    pb_variable_array<FieldT>,

X:    pb_linear_combination_array<FieldT>,
Y:    pb_linear_combination_array<FieldT>,
Z:    pb_linear_combination_array<FieldT>,
result:    pb_variable<FieldT>,
pack_result:    RcCell<packing_gadget<FieldT> >,

    
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

pub struct majority_gadget<FieldT> {
//  : public gadget
result_bits:    pb_variable_array<FieldT>,
pack_result:    RcCell<packing_gadget<FieldT> >,

X:    pb_linear_combination_array<FieldT>,
Y:    pb_linear_combination_array<FieldT>,
Z:    pb_linear_combination_array<FieldT>,
result:    pb_variable<FieldT>,

    
}



// use crate::gadgetlib1::gadgets::hashes::sha256::sha256_aux;

//#endif // SHA256_AUX_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for auxiliary gadgets for the SHA256 gadget.

 See sha256_aux.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_AUX_TCC_
// #define SHA256_AUX_TCC_


impl lastbits_gadget<FieldT>{

pub fn new(pb:&protoboard<FieldT>,
                                         X:&pb_variable<FieldT>,
                                         X_bits:usize,
                                         result:&pb_variable<FieldT>,
                                         result_bits:&pb_linear_combination_array<FieldT>,
                                         annotation_prefix:&String) ->Self

{
    full_bits = result_bits;
    for i in result_bits.len()..X_bits
    {
        let mut  full_bits_overflow=pb_variable::<FieldT>::new();
        full_bits_overflow.allocate(pb, FMT(self.annotation_prefix, " full_bits_{}", i));
        full_bits.push(full_bits_overflow);
    }

    unpack_bits.reset(packing_gadget::<FieldT>::new(pb, full_bits, X, FMT(self.annotation_prefix, " unpack_bits")));
    pack_result.reset(packing_gadget::<FieldT>::new(pb, result_bits, result, FMT(self.annotation_prefix, " pack_result")));
    Self{
    // gadget<FieldT>(pb, annotation_prefix),
    X,
    X_bits,
    result,
    result_bits
    }
}


pub fn generate_r1cs_constraints()
{
    unpack_bits.generate_r1cs_constraints(true);
    pack_result.generate_r1cs_constraints(false);
}


pub fn generate_r1cs_witness()
{
    unpack_bits.generate_r1cs_witness_from_packed();
    pack_result.generate_r1cs_witness_from_bits();
}
}
impl XOR3_gadget<FieldT>{

pub fn new(pb:&protoboard<FieldT>,
                                 A:&pb_linear_combination<FieldT>,
                                 B:&pb_linear_combination<FieldT>,
                                 C:&pb_linear_combination<FieldT>,
                                 assume_C_is_zero:bool,
                                 out:&pb_linear_combination<FieldT>,
                                 annotation_prefix:&String) ->Self
  
{
    if !assume_C_is_zero
    {
        tmp.allocate(pb, FMT(self.annotation_prefix, " tmp"));
    }
    Self{
    //   gadget<FieldT>(pb, annotation_prefix),
    A,
    B,
    C,
    assume_C_is_zero,
    out
    }
}


pub fn generate_r1cs_constraints()
{
    /*
      tmp = A + B - 2AB i.e. tmp = A xor B
      out = tmp + C - 2tmp C i.e. out = tmp xor C
    */
    if assume_C_is_zero
    {
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(2*A, B, A + B - out), FMT(self.annotation_prefix, " implicit_tmp_equals_out"));
    }
    else
    {
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(2*A, B, A + B - tmp), FMT(self.annotation_prefix, " tmp"));
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(2 * tmp, C, tmp + C - out), FMT(self.annotation_prefix, " out"));
    }
}


pub fn generate_r1cs_witness()
{
    if assume_C_is_zero
    {
        self.pb.lc_val(out) = self.pb.lc_val(A) + self.pb.lc_val(B) - FieldT(2) * self.pb.lc_val(A) * self.pb.lc_val(B);
    }
    else
    {
        self.pb.val(tmp) = self.pb.lc_val(A) + self.pb.lc_val(B) - FieldT(2) * self.pb.lc_val(A) * self.pb.lc_val(B);
        self.pb.lc_val(out) = self.pb.val(tmp) + self.pb.lc_val(C) - FieldT(2) * self.pb.val(tmp) * self.pb.lc_val(C);
    }
}
}
// #define SHA256_GADGET_ROTR(A, i, k) A[((i)+(k)) % 32]
impl small_sigma_gadget<FieldT>{
/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

pub fn new(pb:&protoboard<FieldT>,
                                               W:&pb_variable_array<FieldT>,
                                               result:&pb_variable<FieldT>,
                                               rot1:usize,
                                               rot2:usize,
                                               shift:usize,
                                               annotation_prefix:&String) ->Self
   
{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    compute_bits.resize(32);
    for i in 0..32
    {
        compute_bits[i].reset(XOR3_gadget::<FieldT>::new(pb, SHA256_GADGET_ROTR(W, i, rot1), SHA256_GADGET_ROTR(W, i, rot2),
                                              if i + shift < 32 {W[i+shift]} else{ONE},
                                              (i + shift >= 32), result_bits[i],
                                            FMT(self.annotation_prefix, " compute_bits_{}", i)));
    }
    pack_result.reset(packing_gadget::<FieldT>::new(pb, result_bits, result, FMT(self.annotation_prefix, " pack_result")));
    //  gadget<FieldT>(pb, annotation_prefix),
    Self{
    W,
    result
    }
}


pub fn generate_r1cs_constraints()
{
    for i in 0..32
    {
        compute_bits[i].generate_r1cs_constraints();
    }

    pack_result.generate_r1cs_constraints(false);
}


pub fn generate_r1cs_witness()
{
    for i in 0..32
    {
        compute_bits[i].generate_r1cs_witness();
    }

    pack_result.generate_r1cs_witness_from_bits();
}
}
impl big_sigma_gadget<FieldT>{

pub fn new(pb:&protoboard<FieldT>,
                                           W:&pb_linear_combination_array<FieldT>,
                                           result:&pb_variable<FieldT>,
                                           rot1:usize,
                                           rot2:usize,
                                           rot3:usize,
                                           annotation_prefix:&String) ->Self
   
{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    compute_bits.resize(32);
    for i in 0..32
    {
        compute_bits[i].reset(XOR3_gadget::<FieldT>::new(pb, SHA256_GADGET_ROTR(W, i, rot1), SHA256_GADGET_ROTR(W, i, rot2), SHA256_GADGET_ROTR(W, i, rot3), false, result_bits[i],
                                                    FMT(self.annotation_prefix, " compute_bits_{}", i)));
    }

    pack_result.reset(packing_gadget::<FieldT>::new(pb, result_bits, result, FMT(self.annotation_prefix, " pack_result")));
    Self{
    //  gadget<FieldT>(pb, annotation_prefix),
    W,
    result
    }
}


pub fn generate_r1cs_constraints()
{
    for i in 0..32
    {
        compute_bits[i].generate_r1cs_constraints();
    }

    pack_result.generate_r1cs_constraints(false);
}


pub fn generate_r1cs_witness()
{
    for i in 0..32
    {
        compute_bits[i].generate_r1cs_witness();
    }

    pack_result.generate_r1cs_witness_from_bits();
}
}

impl choice_gadget<FieldT>{
/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

pub fn new(pb:&protoboard<FieldT>,
                                     X:&pb_linear_combination_array<FieldT>,
                                     Y:&pb_linear_combination_array<FieldT>,
                                     Z:&pb_linear_combination_array<FieldT>,
                                     result:&pb_variable<FieldT>, annotation_prefix:&String) ->Self
    
{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    pack_result.reset(packing_gadget::<FieldT>::new(pb, result_bits, result, FMT(self.annotation_prefix, " result")));
    // gadget<FieldT>(pb, annotation_prefix),
    Self{X,
    Y,
    Z,
    result}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..32
    {
        /*
          result = x * y + (1-x) * z
          result - z = x * (y - z)
        */
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(X[i], Y[i] - Z[i], result_bits[i] - Z[i]), FMT(self.annotation_prefix, " result_bits_{}", i));
    }
    pack_result.generate_r1cs_constraints(false);
}


pub fn generate_r1cs_witness()
{
    for i in 0..32
    {
        self.pb.val(result_bits[i]) = self.pb.lc_val(X[i]) * self.pb.lc_val(Y[i]) + (FieldT::one() - self.pb.lc_val(X[i])) * self.pb.lc_val(Z[i]);
    }
    pack_result.generate_r1cs_witness_from_bits();
}
}

impl majority_gadget<FieldT>{
/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

pub fn new(pb:&protoboard<FieldT>,
                                         X:&pb_linear_combination_array<FieldT>,
                                         Y:&pb_linear_combination_array<FieldT>,
                                         Z:&pb_linear_combination_array<FieldT>,
                                         result:&pb_variable<FieldT>,
                                         annotation_prefix:&String) ->Self

{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    pack_result.reset(packing_gadget::<FieldT>::new(pb, result_bits, result, FMT(self.annotation_prefix, " result")));
        // gadget<FieldT>(pb, annotation_prefix),
    Self{X,
    Y,
    Z,
    result}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..32
    {
        /*
          2*result + aux = x + y + z
          x, y, z, aux -- bits
          aux = x + y + z - 2*result
        */
        generate_boolean_r1cs_constraint::<FieldT>(self.pb, result_bits[i], FMT(self.annotation_prefix, " result_{}", i));
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(X[i] + Y[i] + Z[i] - 2 * result_bits[i],
                                                             1 - (X[i] + Y[i] + Z[i] -  2 * result_bits[i]),
                                                             0),
                                   FMT(self.annotation_prefix, " result_bits_{}", i));
    }
    pack_result.generate_r1cs_constraints(false);
}


pub fn generate_r1cs_witness()
{
    for i in 0..32
    {
        let  v = (self.pb.lc_val(X[i]) + self.pb.lc_val(Y[i]) + self.pb.lc_val(Z[i])).as_ulong();
        self.pb.val(result_bits[i]) = FieldT(v / 2);
    }

    pack_result.generate_r1cs_witness_from_bits();
}

}

//#endif // SHA256_AUX_TCC_
