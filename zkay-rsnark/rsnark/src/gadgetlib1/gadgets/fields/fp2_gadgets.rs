/** @file
 *****************************************************************************

 Declaration of interfaces for Fp2 gadgets.

 The gadgets verify field arithmetic in Fp2 = Fp[U]/(U^2-non_residue),
 where non_residue is in Fp.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP2_GADGETS_HPP_
// #define FP2_GADGETS_HPP_



use crate::gadgetlib1::gadget;



/**
 * Gadget that represents an Fp2 variable.
 */
// 
pub struct Fp2_variable  {
// : public gadget<Fp2T::my_Fp>
//     type FieldT=Fp2T::my_Fp;

c0:    pb_linear_combination<FieldT>,
c1:    pb_linear_combination<FieldT>,

all_vars:    pb_linear_combination_array<FieldT>,

    
}

/**
 * Gadget that creates constraints for Fp2 by Fp2 multiplication.
 */

pub struct Fp2_mul_gadget{
//  : public gadget<Fp2T::my_Fp> 
//     type FieldT=Fp2T::my_Fp;

A:    Fp2_variable<Fp2T>,
B:    Fp2_variable<Fp2T>,
result:    Fp2_variable<Fp2T>,

v1:    pb_variable<FieldT>,

}

/**
 * Gadget that creates constraints for Fp2 multiplication by a linear combination.
 */

pub struct Fp2_mul_by_lc_gadget{
//  : public gadget<Fp2T::my_Fp> 
    // type FieldT=Fp2T::my_Fp;

A:    Fp2_variable<Fp2T>,
lc:    pb_linear_combination<FieldT>,
result:    Fp2_variable<Fp2T>,

}

/**
 * Gadget that creates constraints for Fp2 squaring.
 */
// 
pub struct Fp2_sqr_gadget  {
// : public gadget<Fp2T::my_Fp>
    // type FieldT=Fp2T::my_Fp;

A:    Fp2_variable<Fp2T>,
result:    Fp2_variable<Fp2T>,

}



// use crate::gadgetlib1::gadgets::fields::fp2_gadgets;

//#endif // FP2_GADGETS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for Fp2 gadgets.

 See fp2_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP2_GADGETS_TCC_
// #define FP2_GADGETS_TCC_



impl Fp2_variable<Fp2T>{
pub fn new(pb:RcCell<protoboard<FieldT>> ,
                                 annotation_prefix:&String) ->Self
    
{
   let  ( c0_var, c1_var)=(variable::<FieldT,pb_variable>::new(),variable::<FieldT,pb_variable>::new());
    c0_var.allocate(&pb, FMT(annotation_prefix, " c0"));
    c1_var.allocate(&pb, FMT(annotation_prefix, " c1"));

    c0 = pb_linear_combination::<FieldT>(c0_var);
    c1 = pb_linear_combination::<FieldT>(c1_var);

    all_vars.push(c0);
    all_vars.push(c1);
// gadget<FieldT>(&pb, annotation_prefix)
}


pub fn new2(pb:RcCell<protoboard<FieldT>> ,
                                 el:&Fp2T,
                                 annotation_prefix:&String) ->Self
    
{
    c0.assign(&pb, el.c0);
    c1.assign(&pb, el.c1);

    c0.evaluate(pb);
    c1.evaluate(pb);

    all_vars.push(c0);
    all_vars.push(c1);
// gadget<FieldT>(&pb, annotation_prefix)
}


pub fn new3(pb:RcCell<protoboard<FieldT>> ,
                                 el:&Fp2T,
                                 coeff:&pb_linear_combination<FieldT>,
                                 annotation_prefix:&String) ->Self
   
{
    c0.assign(&pb, el.c0 * coeff);
    c1.assign(&pb, el.c1 * coeff);

    all_vars.push(c0);
    all_vars.push(c1);
//  gadget<FieldT>(&pb, annotation_prefix)
}


pub fn new4(pb:RcCell<protoboard<FieldT>> ,
                                 c0:&pb_linear_combination<FieldT>,
                                 c1:&pb_linear_combination<FieldT>,
                                 annotation_prefix:&String) ->Self
    
{
    all_vars.push(c0);
    all_vars.push(c1);
    // gadget<FieldT>(&pb, annotation_prefix), 
    Self{c0, c1}
}


pub fn generate_r1cs_equals_const_constraints(el:&Fp2T)
{
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(1, el.c0, c0),
                               FMT(self.annotation_prefix, " c0"));
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(1, el.c1, c1),
                               FMT(self.annotation_prefix, " c1"));
}


pub fn generate_r1cs_witness(el:&Fp2T)
{
    self.pb.lc_val(c0) = el.c0;
    self.pb.lc_val(c1) = el.c1;
}


pub fn get_element()->Fp2T
{
    let mut  el=Fp2T::new();
    el.c0 = self.pb.lc_val(c0);
    el.c1 = self.pb.lc_val(c1);
    return el;
}



pub fn mul_by_X() ->Fp2_variable<Fp2T>
{
   let ( new_c0, new_c1)= (pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new());
    new_c0.assign(self.pb, self.c1 * Fp2T::non_residue);
    new_c1.assign(self.pb, self.c0);
    return Fp2_variable::<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " mul_by_X"));
}


pub fn evaluate() 
{
    c0.evaluate(self.pb);
    c1.evaluate(self.pb);
}


pub fn is_constant() ->bool
{
    return (c0.is_constant() && c1.is_constant());
}


pub fn size_in_bits()->usize
{
    return 2 * FieldT::size_in_bits();
}


pub fn num_variables(&self)->usize
{
    return 2;
}

}

impl Fp2_mul_gadget<Fp2T>{
pub fn new(pb:RcCell<protoboard<FieldT>> ,
                                     A:&Fp2_variable<Fp2T>,
                                     B:&Fp2_variable<Fp2T>,
                                     result:&Fp2_variable<Fp2T>,
                                     annotation_prefix:&String) ->Self
    
{
    v1.allocate(&pb, FMT(annotation_prefix, " v1"));
// gadget<FieldT>(&pb, annotation_prefix),
    Self{ A, B, result}
}


pub fn generate_r1cs_constraints()
{
/*
    Karatsuba multiplication for Fp2:
        v0 = A.c0 * B.c0
        v1 = A.c1 * B.c1
        result.c0 = v0 + non_residue * v1
        result.c1 = (A.c0 + A.c1) * (B.c0 + B.c1) - v0 - v1

    Enforced with 3 constraints:
        A.c1 * B.c1 = v1
        A.c0 * B.c0 = result.c0 - non_residue * v1
        (A.c0+A.c1)*(B.c0+B.c1) = result.c1 + result.c0 + (1 - non_residue) * v1

    Reference:
        "Multiplication and Squaring on Pairing-Friendly Fields"
        Devegili, OhEigeartaigh, Scott, Dahab
*/
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c1, B.c1, v1),
                               FMT(self.annotation_prefix, " v1"));
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0, B.c0, result.c0 + v1 * (-Fp2T::non_residue)),
                               FMT(self.annotation_prefix, " result.c0"));
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0 + A.c1, B.c0 + B.c1,
                                                         result.c1 + result.c0 + v1 * (FieldT::one() - Fp2T::non_residue)),
                               FMT(self.annotation_prefix, " result.c1"));
}


pub fn generate_r1cs_witness()
{
    let aA= self.pb.lc_val(A.c0) * self.pb.lc_val(B.c0);
    self.pb.borrow().val(&v1) = self.pb.lc_val(A.c1) * self.pb.lc_val(B.c1);
    self.pb.lc_val(result.c0) = aA + Fp2T::non_residue * self.pb.borrow().val(&v1);
    self.pb.lc_val(result.c1) = (self.pb.lc_val(A.c0) + self.pb.lc_val(A.c1)) * (self.pb.lc_val(B.c0) + self.pb.lc_val(B.c1)) - aA - self.pb.lc_val(v1);
}

}

impl Fp2_mul_by_lc_gadget<Fp2T>{
pub fn new(pb:RcCell<protoboard<FieldT>> ,
                                                 A:&Fp2_variable<Fp2T>,
                                                 lc:&pb_linear_combination<FieldT>,
                                                 result:&Fp2_variable<Fp2T>,
                                                 annotation_prefix:&String) ->Self
   
{
//  gadget<FieldT>(&pb, annotation_prefix),
    Self{ A, lc, result}
}


pub fn generate_r1cs_constraints()
{
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0, lc, result.c0),
                               FMT(self.annotation_prefix, " result.c0"));
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c1, lc, result.c1),
                               FMT(self.annotation_prefix, " result.c1"));
}


pub fn generate_r1cs_witness()
{
    self.pb.lc_val(result.c0) = self.pb.lc_val(A.c0) * self.pb.lc_val(lc);
    self.pb.lc_val(result.c1) = self.pb.lc_val(A.c1) * self.pb.lc_val(lc);
}

}

impl Fp2_sqr_gadget<Fp2T>{
pub fn new(pb:RcCell<protoboard<FieldT>> ,
                                     A:&Fp2_variable<Fp2T>,
                                     result:&Fp2_variable<Fp2T>,
                                     annotation_prefix:&String) ->Self
    
{
// gadget<FieldT>(&pb, annotation_prefix), 
Self{A, result}
}


pub fn generate_r1cs_constraints()
{
/*
    Complex multiplication for Fp2:
        v0 = A.c0 * A.c1
        result.c0 = (A.c0 + A.c1) * (A.c0 + non_residue * A.c1) - (1 + non_residue) * v0
        result.c1 = 2 * v0

    Enforced with 2 constraints:
        (2*A.c0) * A.c1 = result.c1
        (A.c0 + A.c1) * (A.c0 + non_residue * A.c1) = result.c0 + result.c1 * (1 + non_residue)/2

    Reference:
        "Multiplication and Squaring on Pairing-Friendly Fields"
        Devegili, OhEigeartaigh, Scott, Dahab
*/
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(2 * A.c0, A.c1, result.c1),
                               FMT(self.annotation_prefix, " result.c1"));
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(A.c0 + A.c1,
                                                         A.c0 + Fp2T::non_residue * A.c1,
                                                         result.c0 + result.c1 * (FieldT::one() + Fp2T::non_residue) * FieldT(2).inverse()),
                               FMT(self.annotation_prefix, " result.c0"));
}


pub fn generate_r1cs_witness()
{
    let a= self.pb.lc_val(A.c0);
    let b= self.pb.lc_val(A.c1);
    self.pb.lc_val(result.c1) = FieldT(2) * a * b;
    self.pb.lc_val(result.c0) = (a + b) * (a + Fp2T::non_residue * b) - a*b - Fp2T::non_residue * a* b;
}

}

//#endif // FP2_GADGETS_TCC_

// pub fn operator*(coeff:&FieldT) ->Fp2_variable<Fp2T>
// {
//     pb_linear_combination<FieldT> new_c0, new_c1;
//     new_c0.assign(self.pb, self.c0 * coeff);
//     new_c1.assign(self.pb, self.c1 * coeff);
//     return Fp2_variable<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " operator*"));
// }


// pub fn operator+(other:&Fp2_variable<Fp2T>) ->Fp2_variable<Fp2T>
// {
//     pb_linear_combination<FieldT> new_c0, new_c1;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     return Fp2_variable<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " operator+"));
// }


// pub fn operator+(other:&Fp2T) ->Fp2_variable<Fp2T>
// {
//     pb_linear_combination<FieldT> new_c0, new_c1;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     return Fp2_variable<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " operator+"));
// }
