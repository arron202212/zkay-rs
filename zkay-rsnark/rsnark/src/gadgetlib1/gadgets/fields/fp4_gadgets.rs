/** @file
 *****************************************************************************

 Declaration of interfaces for Fp4 gadgets.

 The gadgets verify field arithmetic in Fp4 = Fp2[V]/(V^2-U) where
 Fp2 = Fp[U]/(U^2-non_residue) and non_residue is in Fp.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP4_GADGETS_HPP_
// #define FP4_GADGETS_HPP_

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::fields::fp2_gadgets;



/**
 * Gadget that represents an Fp4 variable.
 */
// 
pub struct Fp4_variable<Fp4T> {
// : public gadget<Fp4T::my_Fp>
//     type FieldT=Fp4T::my_Fp;
//     type Fp2T=Fp4T::my_Fpe;

c0:    Fp2_variable<Fp2T>,
c1:    Fp2_variable<Fp2T>,

   
}

/**
 * Gadget that creates constraints for Fp4 multiplication (towering formulas).
 */

pub struct Fp4_tower_mul_gadget<Fp4T>  {
// : public gadget<Fp4T::my_Fp>
    // type FieldT=Fp4T::my_Fp;
    // type Fp2T=Fp4T::my_Fpe;

A:    Fp4_variable<Fp4T>,
B:    Fp4_variable<Fp4T>,
result:    Fp4_variable<Fp4T>,

v0_c0:    pb_linear_combination<FieldT>,
v0_c1:    pb_linear_combination<FieldT>,

Ac0_plus_Ac1_c0:    pb_linear_combination<FieldT>,
Ac0_plus_Ac1_c1:    pb_linear_combination<FieldT>,
Ac0_plus_Ac1:    RcCell<Fp2_variable<Fp2T> >,

v0:    RcCell<Fp2_variable<Fp2T> >,
v1:    RcCell<Fp2_variable<Fp2T> >,

Bc0_plus_Bc1_c0:    pb_linear_combination<FieldT>,
Bc0_plus_Bc1_c1:    pb_linear_combination<FieldT>,
Bc0_plus_Bc1:    RcCell<Fp2_variable<Fp2T> >,

result_c1_plus_v0_plus_v1_c0:    pb_linear_combination<FieldT>,
result_c1_plus_v0_plus_v1_c1:    pb_linear_combination<FieldT>,

result_c1_plus_v0_plus_v1:    RcCell<Fp2_variable<Fp2T> >,

compute_v0:    RcCell<Fp2_mul_gadget<Fp2T> >,
compute_v1:    RcCell<Fp2_mul_gadget<Fp2T> >,
compute_result_c1:    RcCell<Fp2_mul_gadget<Fp2T> >,

}

/**
 * Gadget that creates constraints for Fp4 multiplication (direct formulas).
 */

pub struct Fp4_direct_mul_gadget<Fp4T> {
// : public gadget<Fp4T::my_Fp> 
//     type FieldT=Fp4T::my_Fp;
//     type Fp2T=Fp4T::my_Fpe;

A:    Fp4_variable<Fp4T>,
B:    Fp4_variable<Fp4T>,
result:    Fp4_variable<Fp4T>,

v1:    pb_variable<FieldT>,
v2:    pb_variable<FieldT>,
v6:    pb_variable<FieldT>,

}

/**
 * Alias default multiplication gadget
 */
// 
type Fp4_mul_gadget<Fp4T> = Fp4_direct_mul_gadget<Fp4T>;

/**
 * Gadget that creates constraints for Fp4 squaring.
 */

pub struct Fp4_sqr_gadget<Fp4T>   {
// : public gadget<Fp4T::my_Fp>
//     type FieldT=Fp4T::my_Fp;
//     type Fp2T=Fp4T::my_Fpe;

A:    Fp4_variable<Fp4T>,
result:    Fp4_variable<Fp4T>,

v1:    RcCell<Fp2_variable<Fp2T> >,

v0_c0:    pb_linear_combination<FieldT>,
v0_c1:    pb_linear_combination<FieldT>,
v0:    RcCell<Fp2_variable<Fp2T> >,

compute_v0:    RcCell<Fp2_sqr_gadget<Fp2T> >,
compute_v1:    RcCell<Fp2_sqr_gadget<Fp2T> >,

Ac0_plus_Ac1_c0:    pb_linear_combination<FieldT>,
Ac0_plus_Ac1_c1:    pb_linear_combination<FieldT>,
Ac0_plus_Ac1:    RcCell<Fp2_variable<Fp2T> >,

result_c1_plus_v0_plus_v1_c0:    pb_linear_combination<FieldT>,
     result_c1_plus_v0_plus_v1_c1:pb_linear_combination<FieldT >,

result_c1_plus_v0_plus_v1:    RcCell<Fp2_variable<Fp2T> >,

compute_result_c1:    RcCell<Fp2_sqr_gadget<Fp2T> >,


}

/**
 * Gadget that creates constraints for Fp4 cyclotomic squaring
 */

pub struct Fp4_cyclotomic_sqr_gadget<Fp4T> {

// : public gadget<Fp4T::my_Fp>
//     type FieldT=Fp4T::my_Fp;
//     type Fp2T=Fp4T::my_Fpe;

A:    Fp4_variable<Fp4T>,
result:    Fp4_variable<Fp4T>,

c0_expr_c0:    pb_linear_combination<FieldT>,
c0_expr_c1:    pb_linear_combination<FieldT>,
c0_expr:    RcCell<Fp2_variable<Fp2T> >,
compute_c0_expr:    RcCell<Fp2_sqr_gadget<Fp2T> >,

A_c0_plus_A_c1_c0:    pb_linear_combination<FieldT>,
A_c0_plus_A_c1_c1:    pb_linear_combination<FieldT>,
A_c0_plus_A_c1:    RcCell<Fp2_variable<Fp2T> >,

c1_expr_c0:    pb_linear_combination<FieldT>,
c1_expr_c1:    pb_linear_combination<FieldT>,
c1_expr:    RcCell<Fp2_variable<Fp2T> >,
compute_c1_expr:    RcCell<Fp2_sqr_gadget<Fp2T> >,

 
}



// use crate::gadgetlib1::gadgets::fields::fp4_gadgets;

//#endif // FP4_GADGETS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for Fp4 gadgets.

 See fp4_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP4_GADGETS_TCC_
// #define FP4_GADGETS_TCC_


impl Fp4_variable<Fp4T>{

pub fn new(pb:&protoboard<FieldT>, annotation_prefix:&String) ->Self
{
//  gadget<FieldT>(pb, annotation_prefix), 
    Self{c0:Fp2_variable::<Fp2T>::new(pb, FMT(annotation_prefix, " c0")), c1:Fp2_variable::<Fp2T>::new(pb, FMT(annotation_prefix, " c1"))}
}


pub fn new2(pb:&protoboard<FieldT>,
                                 el:&Fp4T,
                                 annotation_prefix:&String) ->Self
    
{
// gadget<FieldT>(pb, annotation_prefix), 
    Self{c0:Fp2_variable::<Fp2T>::new(pb, el.c0, FMT(annotation_prefix, " c0")), c1:Fp2_variable::<Fp2T>::new(pb, el.c1, FMT(annotation_prefix, " c1"))}
}


pub fn new3(pb:&protoboard<FieldT>, c0:&Fp2_variable<Fp2T>, c1:&Fp2_variable<Fp2T>, annotation_prefix:&String) ->Self
    
{
// gadget<FieldT>(pb, annotation_prefix), 

    Self{c0, c1}
}


pub fn generate_r1cs_equals_const_constraints(el:&Fp4T)
{
    c0.generate_r1cs_equals_const_constraints(el.c0);
    c1.generate_r1cs_equals_const_constraints(el.c1);
}


pub fn generate_r1cs_witness(el:&Fp4T)
{
    c0.generate_r1cs_witness(el.c0);
    c1.generate_r1cs_witness(el.c1);
}


 pub fn get_element()->Fp4T
{
    let mut  el=Fp4T::default();
    el.c0 = c0.get_element();
    el.c1 = c1.get_element();
    return el;
}


pub fn Frobenius_map( power:usize) ->Fp4_variable<Fp4T> 
{
    let ( new_c0c0, new_c0c1, new_c1c0, new_c1c1)=(pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new());
    new_c0c0.assign(self.pb, c0.c0);
    new_c0c1.assign(self.pb, c0.c1 * Fp2T::Frobenius_coeffs_c1[power % 2]);
    new_c1c0.assign(self.pb, c1.c0 * Fp4T::Frobenius_coeffs_c1[power % 4]);
    new_c1c1.assign(self.pb, c1.c1 * Fp4T::Frobenius_coeffs_c1[power % 4] * Fp2T::Frobenius_coeffs_c1[power % 2]);

    return Fp4_variable::<Fp4T>(self.pb,
                              Fp2_variable::<Fp2T>(self.pb, new_c0c0, new_c0c1, FMT(self.annotation_prefix, " Frobenius_map_c0")),
                              Fp2_variable::<Fp2T>(self.pb, new_c1c0, new_c1c1, FMT(self.annotation_prefix, " Frobenius_map_c1")),
                            FMT(self.annotation_prefix, " Frobenius_map"));
}


pub fn evaluate() 
{
    c0.evaluate();
    c1.evaluate();
}
}
impl Fp4_tower_mul_gadget<Fp4T>{

pub fn new(pb:&protoboard<FieldT>,
                                                 A:&Fp4_variable<Fp4T>,
                                                 B:&Fp4_variable<Fp4T>,
                                                 result:&Fp4_variable<Fp4T>,
                                                 annotation_prefix:&String) ->Self
    
{
/*
  Karatsuba multiplication for Fp4 as a quadratic extension of Fp2:
  v0 = A.c0 * B.c0
  v1 = A.c1 * B.c1
  result.c0 = v0 + non_residue * v1
  result.c1 = (A.c0 + A.c1) * (B.c0 + B.c1) - v0 - v1
  where "non_residue * elem" := (non_residue * elt.c1, elt.c0)

  Enforced with 3 Fp2_mul_gadget's that ensure that:
  A.c1 * B.c1 = v1
  A.c0 * B.c0 = v0
  (A.c0+A.c1)*(B.c0+B.c1) = result.c1 + v0 + v1

  Reference:
  "Multiplication and Squaring on Pairing-Friendly Fields"
  Devegili, OhEigeartaigh, Scott, Dahab
*/
    v1.reset(Fp2_variable::<Fp2T>::new(pb, FMT(annotation_prefix, " v1")));

    compute_v1.reset(Fp2_mul_gadget::<Fp2T>::new(pb, A.c1, B.c1, *v1, FMT(annotation_prefix, " compute_v1")));

    v0_c0.assign(pb, result.c0.c0 - Fp4T::non_residue * v1.c1);
    v0_c1.assign(pb, result.c0.c1 - v1.c0);
    v0.reset(Fp2_variable::<Fp2T>::new(pb, v0_c0, v0_c1, FMT(annotation_prefix, " v0")));

    compute_v0.reset(Fp2_mul_gadget::<Fp2T>::new(pb, A.c0, B.c0, *v0, FMT(annotation_prefix, " compute_v0")));

    Ac0_plus_Ac1_c0.assign(pb, A.c0.c0 + A.c1.c0);
    Ac0_plus_Ac1_c1.assign(pb, A.c0.c1 + A.c1.c1);
    Ac0_plus_Ac1.reset(Fp2_variable::<Fp2T>::new(pb, Ac0_plus_Ac1_c0, Ac0_plus_Ac1_c1, FMT(annotation_prefix, " Ac0_plus_Ac1")));

    Bc0_plus_Bc1_c0.assign(pb, B.c0.c0 + B.c1.c0);
    Bc0_plus_Bc1_c1.assign(pb, B.c0.c1 + B.c1.c1);
    Bc0_plus_Bc1.reset(Fp2_variable::<Fp2T>::new(pb, Bc0_plus_Bc1_c0, Bc0_plus_Bc1_c1, FMT(annotation_prefix, " Bc0_plus_Bc1")));

    result_c1_plus_v0_plus_v1_c0.assign(pb, result.c1.c0 + v0.c0 + v1.c0);
    result_c1_plus_v0_plus_v1_c1.assign(pb, result.c1.c1 + v0.c1 + v1.c1);
    result_c1_plus_v0_plus_v1.reset(Fp2_variable::<Fp2T>::new(pb, result_c1_plus_v0_plus_v1_c0, result_c1_plus_v0_plus_v1_c1, FMT(annotation_prefix, " result_c1_plus_v0_plus_v1")));

    compute_result_c1.reset(Fp2_mul_gadget::<Fp2T>::new(pb, *Ac0_plus_Ac1, *Bc0_plus_Bc1, *result_c1_plus_v0_plus_v1, FMT(annotation_prefix, " compute_result_c1")));
    // gadget<FieldT>(pb, annotation_prefix), 
    Self{A, B, result}
}


pub fn generate_r1cs_constraints()
{
    compute_v0.generate_r1cs_constraints();
    compute_v1.generate_r1cs_constraints();
    compute_result_c1.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_v0.generate_r1cs_witness();
    compute_v1.generate_r1cs_witness();

    Ac0_plus_Ac1_c0.evaluate(self.pb);
    Ac0_plus_Ac1_c1.evaluate(self.pb);

    Bc0_plus_Bc1_c0.evaluate(self.pb);
    Bc0_plus_Bc1_c1.evaluate(self.pb);

    compute_result_c1.generate_r1cs_witness();

    let Aval= A.get_element();
    let Bval= B.get_element();
    let Rval= Aval * Bval;

    result.generate_r1cs_witness(Rval);
}
}
impl Fp4_direct_mul_gadget<Fp4T>{

pub fn new(pb:&protoboard<FieldT>,
                                                   A:&Fp4_variable<Fp4T>,
                                                   B:&Fp4_variable<Fp4T>,
                                                   result:&Fp4_variable<Fp4T>,
                                                   annotation_prefix:&String) ->Self
    
{
/*
    Tom-Cook-4x for Fp4 (beta is the quartic non-residue):
        v0 = a0*b0,
        v1 = (a0+a1+a2+a3)*(b0+b1+b2+b3),
        v2 = (a0-a1+a2-a3)*(b0-b1+b2-b3),
        v3 = (a0+2a1+4a2+8a3)*(b0+2b1+4b2+8b3),
        v4 = (a0-2a1+4a2-8a3)*(b0-2b1+4b2-8b3),
        v5 = (a0+3a1+9a2+27a3)*(b0+3b1+9b2+27b3),
        v6 = a3*b3

        result.c0 = v0+beta((1/4)v0-(1/6)(v1+v2)+(1/24)(v3+v4)-5v6),
        result.c1 = -(1/3)v0+v1-(1/2)v2-(1/4)v3+(1/20)v4+(1/30)v5-12v6+beta(-(1/12)(v0-v1)+(1/24)(v2-v3)-(1/120)(v4-v5)-3v6),
        result.c2 = -(5/4)v0+(2/3)(v1+v2)-(1/24)(v3+v4)+4v6+beta v6,
        result.c3 = (1/12)(5v0-7v1)-(1/24)(v2-7v3+v4+v5)+15v6

    Enforced with 7 constraints. Doing so requires some care, as we first
    compute three of the v_i explicitly, and then "inline" result.c0/c1/c2/c3
    in computations of the remaining four v_i.

    Concretely, we first compute v1, v2 and v6 explicitly, via 3 constraints as above.
        v1 = (a0+a1+a2+a3)*(b0+b1+b2+b3),
        v2 = (a0-a1+a2-a3)*(b0-b1+b2-b3),
        v6 = a3*b3

    Then we use the following 4 additional constraints:
        (1-beta) v0 = c0 + beta c2 - (beta v1)/2 - (beta v2)/ 2 - (-1 + beta) beta v6
        (1-beta) v3 = -15 c0 - 30 c1 - 3 (4 + beta) c2 - 6 (4 + beta) c3 + (24 - (3 beta)/2) v1 + (-8 + beta/2) v2 + 3 (-16 + beta) (-1 + beta) v6
        (1-beta) v4 = -15 c0 + 30 c1 - 3 (4 + beta) c2 + 6 (4 + beta) c3 + (-8 + beta/2) v1 + (24 - (3 beta)/2) v2 + 3 (-16 + beta) (-1 + beta) v6
        (1-beta) v5 = -80 c0 - 240 c1 - 8 (9 + beta) c2 - 24 (9 + beta) c3 - 2 (-81 + beta) v1 + (-81 + beta) v2 + 8 (-81 + beta) (-1 + beta) v6

    The isomorphism between the representation above and towering is:
        (a0, a1, a2, a3) <-> (a.c0.c0, a.c1.c0, a.c0.c1, a.c1.c1)

    Reference:
        "Multiplication and Squaring on Pairing-Friendly Fields"
        Devegili, OhEigeartaigh, Scott, Dahab

    NOTE: the expressions above were cherry-picked from the Mathematica result
    of the following command:

    (# -> Solve[{c0 == v0+beta((1/4)v0-(1/6)(v1+v2)+(1/24)(v3+v4)-5v6),
    c1 == -(1/3)v0+v1-(1/2)v2-(1/4)v3+(1/20)v4+(1/30)v5-12v6+beta(-(1/12)(v0-v1)+(1/24)(v2-v3)-(1/120)(v4-v5)-3v6),
    c2 == -(5/4)v0+(2/3)(v1+v2)-(1/24)(v3+v4)+4v6+beta v6,
    c3 == (1/12)(5v0-7v1)-(1/24)(v2-7v3+v4+v5)+15v6}, #] // FullSimplify) & /@ Subsets[{v0, v1, v2, v3, v4, v5}, {4}]

    and simplified by multiplying the selected result by (1-beta)
*/
    v1.allocate(pb, FMT(annotation_prefix, " v1"));
    v2.allocate(pb, FMT(annotation_prefix, " v2"));
    v6.allocate(pb, FMT(annotation_prefix, " v6"));
    //  gadget<FieldT>(pb, annotation_prefix), 
    Self{A, B, result}
}


pub fn generate_r1cs_constraints()
{
    let beta= Fp4T::non_residue;
    let u= (FieldT::one() - beta).inverse();

    // const pb_linear_combination<FieldT>
        let a0=&A.c0.c0; let a1=&A.c1.c0; let a2=&A.c0.c1; let a3=&A.c1.c1;
        let b0=&B.c0.c0; let b1=&B.c1.c0; let b2=&B.c0.c1; let b3=&B.c1.c1;
        let c0=&result.c0.c0; let c1=&result.c1.c0; let c2=&result.c0.c1; let c3=&result.c1.c1;

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(
        a0 + a1 + a2 + a3,
        b0 + b1 + b2 + b3,
        v1),
                               FMT(self.annotation_prefix, " v1"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(
        a0 - a1 + a2 - a3,
        b0 - b1 + b2 - b3,
        v2),
                               FMT(self.annotation_prefix, " v2"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(
        a3,
        b3,
        v6),
                               FMT(self.annotation_prefix, " v6"));

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(
        a0,
        b0,
        u * c0 + beta * u * c2 - beta * u * FieldT(2).inverse() * v1 - beta * u * FieldT(2).inverse() * v2 + beta * v6),
                               FMT(self.annotation_prefix, " v0"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(
        a0 + FieldT(2)*a1 + FieldT(4)*a2 + FieldT(8)*a3,
        b0 + FieldT(2)*b1 + FieldT(4)*b2 + FieldT(8)*b3,
        - FieldT(15) * u * c0 - FieldT(30) * u * c1 - FieldT(3) * (FieldT(4) + beta) * u * c2 - FieldT(6) * (FieldT(4) + beta) * u * c3 + (FieldT(24) - FieldT(3) * beta * FieldT(2).inverse()) * u * v1 + (-FieldT(8) + beta * FieldT(2).inverse()) * u * v2 - FieldT(3) * (-FieldT(16) + beta) * v6),
                               FMT(self.annotation_prefix, " v3"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(
        a0 - FieldT(2)*a1 + FieldT(4)*a2 - FieldT(8)*a3,
        b0 - FieldT(2)*b1 + FieldT(4)*b2 - FieldT(8)*b3,
        - FieldT(15) * u * c0 + FieldT(30) * u * c1 - FieldT(3) * (FieldT(4) + beta) * u * c2 + FieldT(6) * (FieldT(4) + beta) * u * c3 + (FieldT(24) - FieldT(3) * beta * FieldT(2).inverse()) * u * v2 + (-FieldT(8) + beta * FieldT(2).inverse()) * u * v1
        - FieldT(3) * (-FieldT(16) + beta) * v6),
                               FMT(self.annotation_prefix, " v4"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(
        a0 + FieldT(3)*a1 + FieldT(9)*a2 + FieldT(27)*a3,
        b0 + FieldT(3)*b1 + FieldT(9)*b2 + FieldT(27)*b3,
        - FieldT(80) * u * c0 - FieldT(240) * u * c1 - FieldT(8) * (FieldT(9) + beta) * u * c2 - FieldT(24) * (FieldT(9) + beta) * u * c3 - FieldT(2) * (-FieldT(81) + beta) * u * v1 + (-FieldT(81) + beta) * u * v2 - FieldT(8) * (-FieldT(81) + beta) * v6),
                               FMT(self.annotation_prefix, " v5"));
}


pub fn generate_r1cs_witness()
{
    // const pb_linear_combination<FieldT>
        let a0=&A.c0.c0; let a1=&A.c1.c0; let a2=&A.c0.c1; let a3=&A.c1.c1;
        let b0=&B.c0.c0; let b1=&B.c1.c0; let b2=&B.c0.c1; let b3=& B.c1.c1;

    self.pb.val(v1) = ((self.pb.lc_val(a0) + self.pb.lc_val(a1) + self.pb.lc_val(a2) + self.pb.lc_val(a3)) *
                        (self.pb.lc_val(b0) + self.pb.lc_val(b1) + self.pb.lc_val(b2) + self.pb.lc_val(b3)));
    self.pb.val(v2) = ((self.pb.lc_val(a0) - self.pb.lc_val(a1) + self.pb.lc_val(a2) - self.pb.lc_val(a3)) *
                        (self.pb.lc_val(b0) - self.pb.lc_val(b1) + self.pb.lc_val(b2) - self.pb.lc_val(b3)));
    self.pb.val(v6) = self.pb.lc_val(a3) * self.pb.lc_val(b3);

    let Aval= A.get_element();
    let Bval= B.get_element();
    let Rval= Aval * Bval;

    result.generate_r1cs_witness(Rval);
}
}
impl Fp4_sqr_gadget<Fp4T>{


pub fn new(pb:&protoboard<FieldT>,
                                     A:&Fp4_variable<Fp4T>,
                                     result:&Fp4_variable<Fp4T>,
                                     annotation_prefix:&String) ->Self
   
{
/*
  Karatsuba squaring for Fp4 as a quadratic extension of Fp2:
  v0 = A.c0^2
  v1 = A.c1^2
  result.c0 = v0 + non_residue * v1
  result.c1 = (A.c0 + A.c1)^2 - v0 - v1
  where "non_residue * elem" := (non_residue * elt.c1, elt.c0)

  Enforced with 3 Fp2_sqr_gadget's that ensure that:
  A.c1^2 = v1
  A.c0^2 = v0
  (A.c0+A.c1)^2 = result.c1 + v0 + v1

  Reference:
  "Multiplication and Squaring on Pairing-Friendly Fields"
  Devegili, OhEigeartaigh, Scott, Dahab
*/
    v1.reset(Fp2_variable::<Fp2T>::new(pb, FMT(annotation_prefix, " v1")));
    compute_v1.reset(Fp2_sqr_gadget::<Fp2T>::new(pb, A.c1, *v1, FMT(annotation_prefix, " compute_v1")));

    v0_c0.assign(pb, result.c0.c0 - Fp4T::non_residue * v1.c1);
    v0_c1.assign(pb, result.c0.c1 - v1.c0);
    v0.reset(Fp2_variable::<Fp2T>::new(pb, v0_c0, v0_c1, FMT(annotation_prefix, " v0")));

    compute_v0.reset(Fp2_sqr_gadget::<Fp2T>::new(pb, A.c0, *v0, FMT(annotation_prefix, " compute_v0")));

    Ac0_plus_Ac1_c0.assign(pb, A.c0.c0 + A.c1.c0);
    Ac0_plus_Ac1_c1.assign(pb, A.c0.c1 + A.c1.c1);
    Ac0_plus_Ac1.reset(Fp2_variable::<Fp2T>::new(pb, Ac0_plus_Ac1_c0, Ac0_plus_Ac1_c1, FMT(annotation_prefix, " Ac0_plus_Ac1")));

    result_c1_plus_v0_plus_v1_c0.assign(pb, result.c1.c0 + v0.c0 + v1.c0);
    result_c1_plus_v0_plus_v1_c1.assign(pb, result.c1.c1 + v0.c1 + v1.c1);
    result_c1_plus_v0_plus_v1.reset(Fp2_variable::<Fp2T>::new(pb, result_c1_plus_v0_plus_v1_c0, result_c1_plus_v0_plus_v1_c1, FMT(annotation_prefix, " result_c1_plus_v0_plus_v1")));

    compute_result_c1.reset(Fp2_sqr_gadget::<Fp2T>::new(pb, *Ac0_plus_Ac1, *result_c1_plus_v0_plus_v1, FMT(annotation_prefix, " compute_result_c1")));
    //  gadget<FieldT>(pb, annotation_prefix), 
    Self{A, result}
}


pub fn generate_r1cs_constraints()
{
    compute_v1.generate_r1cs_constraints();
    compute_v0.generate_r1cs_constraints();
    compute_result_c1.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_v1.generate_r1cs_witness();

    v0_c0.evaluate(self.pb);
    v0_c1.evaluate(self.pb);
    compute_v0.generate_r1cs_witness();

    Ac0_plus_Ac1_c0.evaluate(self.pb);
    Ac0_plus_Ac1_c1.evaluate(self.pb);
    compute_result_c1.generate_r1cs_witness();

    let Aval= A.get_element();
    let Rval= Aval.squared();
    result.generate_r1cs_witness(Rval);
}
}
impl Fp4_cyclotomic_sqr_gadget<Fp4T>{

pub fn new(pb:&protoboard<FieldT>,
                                                           A:&Fp4_variable<Fp4T>,
                                                           result:&Fp4_variable<Fp4T>,
                                                           annotation_prefix:&String) ->Self
   
{
/*
  A = elt.c1 ^ 2
  B = elt.c1 + elt.c0;
  C = B ^ 2 - A
  D = Fp2(A.c1 * non_residue, A.c0)
  E = C - D
  F = D + D + Fp2::one()
  G = E - Fp2::one()

  return Fp4(F, G);

  Enforced with 2 Fp2_sqr_gadget's that ensure that:

  elt.c1 ^ 2 = Fp2(result.c0.c1 / 2, (result.c0.c0 - 1) / (2 * non_residue)) = A
  (elt.c1 + elt.c0) ^ 2 = A + result.c1 + Fp2(A.c1 * non_residue + 1, A.c0)

  (elt.c1 + elt.c0) ^ 2 = Fp2(result.c0.c1 / 2 + result.c1.c0 + (result.c0.c0 - 1) / 2 + 1,
                              (result.c0.c0 - 1) / (2 * non_residue) + result.c1.c1 + result.c0.c1 / 2)

  Corresponding test code:

    assert!(B.squared() == A + G + my_Fp2(A.c1 * non_residue + my_Fp::one(), A.c0));
    assert!(self.c1.squared().c0 == F.c1 * my_Fp(2).inverse());
    assert!(self.c1.squared().c1 == (F.c0 - my_Fp(1)) * (my_Fp(2) * non_residue).inverse());
*/
    c0_expr_c0.assign(pb, result.c0.c1 * FieldT(2).inverse());
    c0_expr_c1.assign(pb, (result.c0.c0 - FieldT(1)) * (FieldT(2) * Fp4T::non_residue).inverse());
    c0_expr.reset(Fp2_variable::<Fp2T>::new(pb, c0_expr_c0, c0_expr_c1, FMT(annotation_prefix, " c0_expr")));
    compute_c0_expr.reset(Fp2_sqr_gadget::<Fp2T>::new(pb, A.c1, *c0_expr, FMT(annotation_prefix, " compute_c0_expr")));

    A_c0_plus_A_c1_c0.assign(pb, A.c0.c0 + A.c1.c0);
    A_c0_plus_A_c1_c1.assign(pb, A.c0.c1 + A.c1.c1);
    A_c0_plus_A_c1.reset(Fp2_variable::<Fp2T>::new(pb, A_c0_plus_A_c1_c0, A_c0_plus_A_c1_c1, FMT(annotation_prefix, " A_c0_plus_A_c1")));

    c1_expr_c0.assign(pb, (result.c0.c1 + result.c0.c0 - FieldT(1)) * FieldT(2).inverse() + result.c1.c0 + FieldT(1));
    c1_expr_c1.assign(pb, (result.c0.c0 - FieldT(1)) * (FieldT(2) * Fp4T::non_residue).inverse() + result.c1.c1 + result.c0.c1 * FieldT(2).inverse());
    c1_expr.reset(Fp2_variable::<Fp2T>::new(pb, c1_expr_c0, c1_expr_c1, FMT(annotation_prefix, " c1_expr")));

    compute_c1_expr.reset(Fp2_sqr_gadget::<Fp2T>::new(pb, *A_c0_plus_A_c1, *c1_expr, FMT(annotation_prefix, " compute_c1_expr")));
    //  gadget<FieldT>(pb, annotation_prefix), 
    Self{A, result}
}


pub fn generate_r1cs_constraints()
{
    compute_c0_expr.generate_r1cs_constraints();
    compute_c1_expr.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_c0_expr.generate_r1cs_witness();

    A_c0_plus_A_c1_c0.evaluate(self.pb);
    A_c0_plus_A_c1_c1.evaluate(self.pb);
    compute_c1_expr.generate_r1cs_witness();

    let Aval= A.get_element();
    let Rval= Aval.squared();
    result.generate_r1cs_witness(Rval);
}

}

//#endif // FP4_GADGETS_TCC_
