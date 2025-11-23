/** @file
 *****************************************************************************

 Declaration of interfaces for Fp3 gadgets.

 The gadgets verify field arithmetic in Fp3 = Fp[U]/(U^3-non_residue),
 where non_residue is in Fp.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP3_GADGETS_HPP_
// #define FP3_GADGETS_HPP_



use crate::gadgetlib1::gadget;



/**
 * Gadget that represents an Fp3 variable.
 */
// 
pub struct Fp3_variable<Fp3T> {
//->Self public gadget<Fp3T::my_Fp> 
    // type FieldT=Fp3T::my_Fp;

c0:    pb_linear_combination<FieldT>,
c1:    pb_linear_combination<FieldT>,
c2:    pb_linear_combination<FieldT>,

all_vars:    pb_linear_combination_array<FieldT>,

    
}

/**
 * Gadget that creates constraints for Fp3 by Fp3 multiplication.
 */

pub struct Fp3_mul_gadget<Fp3T> {
// : public gadget<Fp3T::my_Fp> 
//     type FieldT=Fp3T::my_Fp;

A:    Fp3_variable<Fp3T>,
B:    Fp3_variable<Fp3T>,
result:    Fp3_variable<Fp3T>,

v0:    pb_variable<FieldT>,
v4:    pb_variable<FieldT>,

}

/**
 * Gadget that creates constraints for Fp3 multiplication by a linear combination.
 */
// 
pub struct Fp3_mul_by_lc_gadget<Fp3T>  {
// : public gadget<Fp3T::my_Fp>
//     type FieldT=Fp3T::my_Fp;

A:    Fp3_variable<Fp3T>,
lc:    pb_linear_combination<FieldT>,
result:    Fp3_variable<Fp3T>,

}

/**
 * Gadget that creates constraints for Fp3 squaring.
 */
// 
pub struct Fp3_sqr_gadget<Fp3T> {
// : public gadget<Fp3T::my_Fp> 
//     type FieldT=Fp3T::my_Fp;

A:    Fp3_variable<Fp3T>,
result:    Fp3_variable<Fp3T>,

mul:    RcCell<Fp3_mul_gadget<Fp3T> >,

}




// use crate::gadgetlib1::gadgets::fields::fp3_gadgets;

//#endif // FP3_GADGETS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for Fp3 gadgets.

 See fp3_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP3_GADGETS_TCC_
// #define FP3_GADGETS_TCC_


impl Fp3_variable<Fp3T>{

pub fn new(pb:&protoboard<FieldT>,
                                 annotation_prefix:&String)->Self
    
{
   let ( c0_var, c1_var, c2_var)=( pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());
    c0_var.allocate(pb, FMT(annotation_prefix, " c0"));
    c1_var.allocate(pb, FMT(annotation_prefix, " c1"));
    c2_var.allocate(pb, FMT(annotation_prefix, " c2"));

    c0 = pb_linear_combination::<FieldT>(c0_var);
    c1 = pb_linear_combination::<FieldT>(c1_var);
    c2 = pb_linear_combination::<FieldT>(c2_var);

    all_vars.push(c0);
    all_vars.push(c1);
    all_vars.push(c2);
    // gadget<FieldT>(pb, annotation_prefix)
}


pub fn new2(pb:&protoboard<FieldT>,
                                 el:&Fp3T,
                                 annotation_prefix:&String)->Self
   
{
    c0.assign(pb, el.c0);
    c1.assign(pb, el.c1);
    c2.assign(pb, el.c2);

    c0.evaluate(pb);
    c1.evaluate(pb);
    c2.evaluate(pb);

    all_vars.push(c0);
    all_vars.push(c1);
    all_vars.push(c2);
    //  gadget<FieldT>(pb, annotation_prefix)
}


pub fn new3(pb:&protoboard<FieldT>,
                                 el:&Fp3T,
                                 coeff:&pb_linear_combination<FieldT>,
                                 annotation_prefix:&String)->Self
    
{
    c0.assign(pb, el.c0 * coeff);
    c1.assign(pb, el.c1 * coeff);
    c2.assign(pb, el.c2 * coeff);

    all_vars.push(c0);
    all_vars.push(c1);
    all_vars.push(c2);
// gadget<FieldT>(pb, annotation_prefix)
}


pub fn new4(pb:&protoboard<FieldT>,
                                 c0:&pb_linear_combination<FieldT>,
                                 c1:&pb_linear_combination<FieldT>,
                                 c2:&pb_linear_combination<FieldT>,
                                 annotation_prefix:&String)->Self
    
{
    all_vars.push(c0);
    all_vars.push(c1);
    all_vars.push(c2);
// gadget<FieldT>(pb, annotation_prefix), 
    Self{c0, c1, c2}
}


pub fn generate_r1cs_equals_const_constraints(el:&Fp3T)
{
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, el.c0, c0),
                               FMT(self.annotation_prefix, " c0"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, el.c1, c1),
                               FMT(self.annotation_prefix, " c1"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, el.c2, c2),
                               FMT(self.annotation_prefix, " c2"));
}


pub fn generate_r1cs_witness(el:&Fp3T)
{
    self.pb.lc_val(c0) = el.c0;
    self.pb.lc_val(c1) = el.c1;
    self.pb.lc_val(c2) = el.c2;
}


pub fn get_element()->Fp3T
{
    let mut  el=Fp3T::new();
    el.c0 = self.pb.lc_val(c0);
    el.c1 = self.pb.lc_val(c1);
    el.c2 = self.pb.lc_val(c2);
    return el;
}



pub fn mul_by_X() ->Fp3_variable<Fp3T>
{
    let ( new_c0, new_c1, new_c2)=(pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new());
    new_c0.assign(self.pb, self.c2 * Fp3T::non_residue);
    new_c1.assign(self.pb, self.c0);
    new_c2.assign(self.pb, self.c1);
    return Fp3_variable::<Fp3T>(self.pb, new_c0, new_c1, new_c2, FMT(self.annotation_prefix, " mul_by_X"));
}


pub fn evaluate() 
{
    c0.evaluate(self.pb);
    c1.evaluate(self.pb);
    c2.evaluate(self.pb);
}


pub fn is_constant() ->bool
{
    return (c0.is_constant() && c1.is_constant() && c2.is_constant());
}


pub fn size_in_bits()->usize
{
    return 3 * FieldT::size_in_bits();
}


pub fn num_variables(&self)->usize
{
    return 3;
}
}
impl Fp3_mul_gadget<Fp3T>{

pub fn new(pb:&protoboard<FieldT>,
                                     A:&Fp3_variable<Fp3T>,
                                     B:&Fp3_variable<Fp3T>,
                                     result:&Fp3_variable<Fp3T>,
                                     annotation_prefix:&String)->Self
    
{
    v0.allocate(pb, FMT(annotation_prefix, " v0"));
    v4.allocate(pb, FMT(annotation_prefix, " v4"));
    // gadget<FieldT>(pb, annotation_prefix), 
    Self{A, B, result}
}


pub fn generate_r1cs_constraints()
{
/*
    Tom-Cook-3x for Fp3:
        v0 = A.c0 * B.c0
        v1 = (A.c0 + A.c1 + A.c2) * (B.c0 + B.c1 + B.c2)
        v2 = (A.c0 - A.c1 + A.c2) * (B.c0 - B.c1 + B.c2)
        v3 = (A.c0 + 2*A.c1 + 4*A.c2) * (B.c0 + 2*B.c1 + 4*B.c2)
        v4 = A.c2 * B.c2
        result.c0 = v0 + non_residue * (v0/2 - v1/2 - v2/6 + v3/6 - 2*v4)
        result.c1 = -(1/2) v0 +  v1 - (1/3) v2 - (1/6) v3 + 2 v4 + non_residue*v4
        result.c2 = -v0 + (1/2) v1 + (1/2) v2 - v4

    Enforced with 5 constraints. Doing so requires some care, as we first
    compute two of the v_i explicitly, and then "inline" result.c1/c2/c3
    in computations of teh remaining three v_i.

    Concretely, we first compute v0 and v4 explicitly, via 2 constraints:
        A.c0 * B.c0 = v0
        A.c2 * B.c2 = v4
    Then we use the following 3 additional constraints:
        v1 = result.c1 + result.c2 + (result.c0 - v0)/non_residue + v0 + v4 - non_residue v4
        v2 = -result.c1 + result.c2 + v0 + (-result.c0 + v0)/non_residue + v4 + non_residue v4
        v3 = 2 * result.c1 + 4 result.c2 + (8*(result.c0 - v0))/non_residue + v0 + 16 * v4 - 2 * non_residue * v4

    Reference:
        "Multiplication and Squaring on Pairing-Friendly Fields"
        Devegili, OhEigeartaigh, Scott, Dahab

    NOTE: the expressions above were cherry-picked from the Mathematica result
    of the following command:

    (# -> Solve[{c0 == v0 + non_residue*(v0/2 - v1/2 - v2/6 + v3/6 - 2 v4),
                c1 == -(1/2) v0 + v1 - (1/3) v2 - (1/6) v3 + 2 v4 + non_residue*v4,
                c2 == -v0 + (1/2) v1 + (1/2) v2 - v4}, #] // FullSimplify) & /@
    Subsets[{v0, v1, v2, v3, v4}, {3}]
*/
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0, B.c0, v0), FMT(self.annotation_prefix, " v0"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c2, B.c2, v4), FMT(self.annotation_prefix, " v4"));

    let beta = Fp3T::non_residue;

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0 + A.c1 + A.c2,
                                                         B.c0 + B.c1 + B.c2,
                                                         result.c1 + result.c2 + result.c0 * beta.inverse() + v0 * (FieldT(1) - beta.inverse()) + v4 * (FieldT(1) - beta)),
                               FMT(self.annotation_prefix, " v1"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0 - A.c1 + A.c2,
                                                         B.c0 - B.c1 + B.c2,
                                                         -result.c1 + result.c2 + v0 * (FieldT(1) + beta.inverse()) - result.c0 * beta.inverse() + v4 * (FieldT(1) + beta)),
                               FMT(self.annotation_prefix, " v2"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0 + 2 * A.c1 + 4 * A.c2,
                                                         B.c0 + 2 * B.c1 + 4 * B.c2,
                                                         2 * result.c1 + 4 * result.c2 + result.c0 * (FieldT(8) * beta.inverse()) + v0 * (FieldT(1) - FieldT(8) * beta.inverse()) + v4 * (FieldT(16) - FieldT(2) * beta)),
                               FMT(self.annotation_prefix, " v3"));
}


pub fn generate_r1cs_witness()
{
    self.pb.val(v0) = self.pb.lc_val(A.c0) * self.pb.lc_val(B.c0);
    self.pb.val(v4) = self.pb.lc_val(A.c2) * self.pb.lc_val(B.c2);

    let Aval = A.get_element();
    let Bval = B.get_element();
    let Rval = Aval * Bval;
    result.generate_r1cs_witness(Rval);
}
}

impl Fp3_mul_by_lc_gadget<Fp3T>{

pub fn new(pb:&protoboard<FieldT>,
                                                 A:&Fp3_variable<Fp3T>,
                                                 lc:&pb_linear_combination<FieldT>,
                                                 result:&Fp3_variable<Fp3T>,
                                                 annotation_prefix:&String)->Self
   
{
//  gadget<FieldT>(pb, annotation_prefix),A,lc,result
}


pub fn generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0, lc, result.c0),
                               FMT(self.annotation_prefix, " result.c0"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c1, lc, result.c1),
                               FMT(self.annotation_prefix, " result.c1"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c2, lc, result.c2),
                               FMT(self.annotation_prefix, " result.c2"));
}


pub fn generate_r1cs_witness()
{
    self.pb.lc_val(result.c0) = self.pb.lc_val(A.c0) * self.pb.lc_val(lc);
    self.pb.lc_val(result.c1) = self.pb.lc_val(A.c1) * self.pb.lc_val(lc);
    self.pb.lc_val(result.c2) = self.pb.lc_val(A.c2) * self.pb.lc_val(lc);
}
}

impl Fp3_sqr_gadget<Fp3T>{

pub fn new(pb:&protoboard<FieldT>,
                                     A:&Fp3_variable<Fp3T>,
                                     result:&Fp3_variable<Fp3T>,
                                     annotation_prefix:&String)->Self
    
{
    mul.reset(Fp3_mul_gadget::<Fp3T>::new(pb, A, A, result, FMT(annotation_prefix, " mul")));
// gadget<FieldT>(pb, annotation_prefix),A,result
}


pub fn generate_r1cs_constraints()
{
    // We can't do better than 5 constraints for squaring, so we just use multiplication.
    mul.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    mul.generate_r1cs_witness();
}

}

//#endif // FP3_GADGETS_TCC_

// 
// Fp3_variable<Fp3T>pub fn operator*(coeff:&FieldT) const
// {
//     pb_linear_combination<FieldT> new_c0, new_c1, new_c2;
//     new_c0.assign(self.pb, self.c0 * coeff);
//     new_c1.assign(self.pb, self.c1 * coeff);
//     new_c2.assign(self.pb, self.c2 * coeff);
//     return Fp3_variable<Fp3T>(self.pb, new_c0, new_c1, new_c2, FMT(self.annotation_prefix, " operator*"));
// }

// 
// Fp3_variable<Fp3T>pub fn operator+(other:&Fp3_variable<Fp3T>) const
// {
//     pb_linear_combination<FieldT> new_c0, new_c1, new_c2;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     new_c2.assign(self.pb, self.c2 + other.c2);
//     return Fp3_variable<Fp3T>(self.pb, new_c0, new_c1, new_c2, FMT(self.annotation_prefix, " operator+"));
// }

// 
// Fp3_variable<Fp3T>pub fn operator+(other:&Fp3T) const
// {
//     pb_linear_combination<FieldT> new_c0, new_c1, new_c2;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     new_c2.assign(self.pb, self.c2 + other.c2);
//     return Fp3_variable<Fp3T>(self.pb, new_c0, new_c1, new_c2, FMT(self.annotation_prefix, " operator+"));
// }