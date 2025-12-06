/** @file
 *****************************************************************************

 Declaration of interfaces for G1 gadgets.

 The gadgets verify curve arithmetic in G1 = E(F) where E/F: y^2 = x^3 + A * X + B
 is an elliptic curve over F in short Weierstrass form.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G1_GADGET_HPP_
// #define WEIERSTRASS_G1_GADGET_HPP_

use ff_curves::algebra::curves::public_params;

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params;



/**
 * Gadget that represents a G1 variable.
 */
// 
type FieldT=ffec::Fr<ppT> ;
pub struct  G1_variable{
//  : public gadget<ffec::Fr<ppT> > 
    

X:    pb_linear_combination<FieldT>,
Y:    pb_linear_combination<FieldT>,

all_vars:    pb_linear_combination_array<FieldT>,

   
}

/**
 * Gadget that creates constraints for the validity of a G1 variable.
 */
// 
pub struct  G1_checker_gadget<ppT>  {
// : public gadget<ffec::Fr<ppT> > 
    // type FieldT=ffec::Fr<ppT>;

P:    G1_variable<ppT>,
P_X_squared:    pb_variable<FieldT>,
P_Y_squared:    pb_variable<FieldT>,

   
}

/**
 * Gadget that creates constraints for G1 addition.
 */
// 
pub struct  G1_add_gadget {
// : public gadget<ffec::Fr<ppT> > 
    // type FieldT=ffec::Fr<ppT>;

lambda:    pb_variable<FieldT>,
inv:    pb_variable<FieldT>,

A:    G1_variable<ppT>,
B:    G1_variable<ppT>,
C:    G1_variable<ppT>,

}

/**
 * Gadget that creates constraints for G1 doubling.
 */
// 
pub struct  G1_dbl_gadget<ppT> {
// : public gadget<ffec::Fr<ppT> >
    // type FieldT=ffec::Fr<ppT>;

Xsquared:    pb_variable<FieldT>,
lambda:    pb_variable<FieldT>,

A:    G1_variable<ppT>,
B:    G1_variable<ppT>,

    
}

/**
 * Gadget that creates constraints for G1 multi-scalar multiplication.
 */
// 
pub struct  G1_multiscalar_mul_gadget{
//  : public gadget<ffec::Fr<ppT> > 
//     type FieldT=ffec::Fr<ppT>;

computed_results:    Vec<G1_variable<ppT> >,
chosen_results:    Vec<G1_variable<ppT> >,
adders:    Vec<G1_add_gadget<ppT> >,
doublers:    Vec<G1_dbl_gadget<ppT> >,

base:    G1_variable<ppT>,
scalars:    pb_variable_array<FieldT>,
points:    Vec<G1_variable<ppT> >,
points_and_powers:    Vec<G1_variable<ppT> >,
result:    G1_variable<ppT>,

elt_size:     usize,
num_points:     usize,
scalar_size:     usize,

}



// use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget;

//#endif // WEIERSTRASS_G1_GADGET_TCC_
/** @file
 *****************************************************************************

 Implementation of interfaces for G1 gadgets.

 See weierstrass_g1_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G1_GADGET_TCC_
// #define WEIERSTRASS_G1_GADGET_TCC_


impl G1_variable<ppT>{

pub fn new(
pb:&RcCell<protoboard<FieldT>>,
                              annotation_prefix:&String) ->Self
   
{
    let (X_var, Y_var)=( variable::<FieldT,pb_variable>::new(),variable::<FieldT,pb_variable>::new());

    X_var.allocate(&pb, FMT(annotation_prefix, " X"));
    Y_var.allocate(&pb, FMT(annotation_prefix, " Y"));

    X = pb_linear_combination::<FieldT>(X_var);
    Y = pb_linear_combination::<FieldT>(Y_var);

    all_vars.push(X);
    all_vars.push(Y);
    //  gadget<FieldT>(&pb, annotation_prefix)
}


pub fn new2(
pb:&RcCell<protoboard<FieldT>>,
                              P:&ffec::G1::<other_curve::<ppT> >,
                              annotation_prefix:&String)->Self
    
{
    let  Pcopy = P.clone();
    Pcopy.to_affine_coordinates();

    X.assign(&pb, Pcopy.X());
    Y.assign(&pb, Pcopy.Y());
    X.evaluate(pb);
    Y.evaluate(pb);
    all_vars.push(X);
    all_vars.push(Y);
    // gadget<FieldT>(&pb, annotation_prefix)
}


pub fn  generate_r1cs_witness(el:&ffec::G1::<other_curve::<ppT> >)
{
   let mut el_normalized = el.clone();
    el_normalized.to_affine_coordinates();

    self.pb.borrow().lc_val(X) = el_normalized.X();
    self.pb.borrow().lc_val(Y) = el_normalized.Y();
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
impl G1_checker_gadget<ppT>{

pub fn new(
pb:&RcCell<protoboard<FieldT>>, P:&G1_variable<ppT>, annotation_prefix:&String)->Self
    
{
    P_X_squared.allocate(&pb, FMT(annotation_prefix, " P_X_squared"));
    P_Y_squared.allocate(&pb, FMT(annotation_prefix, " P_Y_squared"));
    // gadget<FieldT>(&pb, annotation_prefix),
    Self {P}
}


pub fn  generate_r1cs_constraints()
{
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![P.X],
        vec![P.X],
        vec![P_X_squared]),
      FMT(self.annotation_prefix, " P_X_squared"));
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![P.Y ],
        vec![P.Y ],
        vec![P_Y_squared ]),
      FMT(self.annotation_prefix, " P_Y_squared"));
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![P.X ],
        vec![P_X_squared, ONE * ffec::G1::<other_curve::<ppT> >::coeff_a],
        vec![P_Y_squared, ONE * (-ffec::G1::<other_curve::<ppT> >::coeff_b)]),
      FMT(self.annotation_prefix, " curve_equation"));
}


pub fn  generate_r1cs_witness()
{
    self.pb.borrow().val(&P_X_squared) = self.pb.borrow().lc_val(P.X).squared();
    self.pb.borrow().val(&P_Y_squared) = self.pb.borrow().lc_val(P.Y).squared();
}
}
impl G1_add_gadget<ppT>{

pub fn new(
pb:&RcCell<protoboard<FieldT>>,
                                  A:&G1_variable<ppT>,
                                  B:&G1_variable<ppT>,
                                  C:&G1_variable<ppT>,
                                  annotation_prefix:&String)->Self
    
{
    /*
      lambda = (B.y - A.y)/(B.x - A.x)
      C.x = lambda^2 - A.x - B.x
      C.y = lambda(A.x - C.x) - A.y

      Special cases:

      doubling: if B.y = A.y and B.x = A.x then lambda is unbound and
      C = (lambda^2, lambda^3)

      addition of negative point: if B.y = -A.y and B.x = A.x then no
      lambda can satisfy the first equation unless B.y - A.y = 0. But
      then this reduces to doubling.

      So we need to check that A.x - B.x != 0, which can be done by
      enforcing I * (B.x - A.x) = 1
    */
    lambda.allocate(&pb, FMT(annotation_prefix, " lambda"));
    inv.allocate(&pb, FMT(annotation_prefix, " inv"));
    Self{
    // gadget<FieldT>(&pb, annotation_prefix),
    A,
    B,
    C
    }
}


pub fn  generate_r1cs_constraints()
{
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![lambda],
        vec![B.X, A.X * (-1)],
        vec![B.Y, A.Y * (-1)]),
      FMT(self.annotation_prefix, " calc_lambda"));

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![lambda ],
        vec![lambda],
        vec![C.X, A.X, B.X]),
      FMT(self.annotation_prefix, " calc_X"));

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![lambda],
        vec![A.X, C.X * (-1)],
        vec![C.Y, A.Y ]),
      FMT(self.annotation_prefix, " calc_Y"));

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![inv],
        vec![B.X, A.X * (-1)],
        vec![ONE]),
      FMT(self.annotation_prefix, " no_special_cases"));
}


pub fn  generate_r1cs_witness()
{
    self.pb.borrow().val(&inv) = (self.pb.borrow().lc_val(B.X) - self.pb.borrow().lc_val(A.X)).inverse();
    self.pb.borrow().val(&lambda) = (self.pb.borrow().lc_val(B.Y) - self.pb.borrow().lc_val(A.Y)) * self.pb.borrow().val(&inv);
    self.pb.borrow().lc_val(C.X) = self.pb.borrow().val(&lambda).squared() - self.pb.borrow().lc_val(A.X) - self.pb.borrow().lc_val(B.X);
    self.pb.borrow().lc_val(C.Y) = self.pb.borrow().val(&lambda) * (self.pb.borrow().lc_val(A.X) - self.pb.borrow().lc_val(C.X)) - self.pb.borrow().lc_val(A.Y);
}
}
impl G1_dbl_gadget<ppT>{

pub fn new(
pb:&RcCell<protoboard<FieldT>>,
                                  A:&G1_variable<ppT>,
                                  B:&G1_variable<ppT>,
                                  annotation_prefix:&String)->Self
  
{
    Xsquared.allocate(&pb, FMT(annotation_prefix, " X_squared"));
    lambda.allocate(&pb, FMT(annotation_prefix, " lambda"));
    Self{
    //   gadget<FieldT>(&pb, annotation_prefix),
    A,
    B
    }
}


pub fn  generate_r1cs_constraints()
{
    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![A.X ],
        vec![A.X ],
        vec![Xsquared ]),
       FMT(self.annotation_prefix, " calc_Xsquared"));

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![lambda * 2 ],
        vec![A.Y ],
        vec![Xsquared * 3, ONE * ffec::G1::<other_curve::<ppT> >::coeff_a]),
      FMT(self.annotation_prefix, " calc_lambda"));

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![lambda],
        vec![lambda],
        vec![B.X, A.X * 2]),
      FMT(self.annotation_prefix, " calc_X"));

    self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(
        vec![lambda],
        vec![A.X, B.X * (-1) ],
        vec![B.Y, A.Y ]),
      FMT(self.annotation_prefix, " calc_Y"));
}


pub fn  generate_r1cs_witness()
{
    self.pb.borrow().val(&Xsquared) = self.pb.borrow().lc_val(A.X).squared();
    self.pb.borrow().val(&lambda) = (FieldT(3) * self.pb.borrow().val(&Xsquared) + ffec::G1::<other_curve::<ppT> >::coeff_a) * (FieldT(2) * self.pb.borrow().lc_val(A.Y)).inverse();
    self.pb.borrow().lc_val(B.X) = self.pb.borrow().val(&lambda).squared() - FieldT(2) * self.pb.borrow().lc_val(A.X);
    self.pb.borrow().lc_val(B.Y) = self.pb.borrow().val(&lambda) * (self.pb.borrow().lc_val(A.X) - self.pb.borrow().lc_val(B.X)) - self.pb.borrow().lc_val(A.Y);
}
}

impl G1_multiscalar_mul_gadget<ppT>{

pub fn new(
    pb:&RcCell<protoboard<FieldT>>,
                                                          base:&G1_variable<ppT>,
                                                          scalars:&pb_variable_array<FieldT>,
                                                          elt_size:usize,
                                                          points:&Vec<G1_variable<ppT> >,
                                                          result: &G1_variable<ppT> ,
                                                          annotation_prefix:&String)->Self
    
{
    assert!(num_points >= 1);
    assert!(num_points * elt_size == scalar_size);

    for i in 0..num_points
    {
        points_and_powers.push(points[i]);
        for j in 0..elt_size - 1
        {
            points_and_powers.push(G1_variable::<ppT>(&pb, FMT(annotation_prefix, " points_{}_times_2_to_{}", i, j+1)));
            doublers.push(G1_dbl_gadget::<ppT>(&pb, points_and_powers[i*elt_size + j], points_and_powers[i*elt_size + j + 1], FMT(annotation_prefix, " double_{}_to_2_to_{}", i, j+1)));
        }
    }

    chosen_results.push(base);
    for i in 0..scalar_size
    {
        computed_results.push(G1_variable::<ppT>(&pb, FMT(annotation_prefix, " computed_results_{}")));
        if i < scalar_size-1
        {
            chosen_results.push(G1_variable::<ppT>(&pb, FMT(annotation_prefix, " chosen_results_{}")));
        }
        else
        {
            chosen_results.push(result);
        }

        adders.push(G1_add_gadget::<ppT>(&pb, chosen_results[i], points_and_powers[i], computed_results[i], FMT(annotation_prefix, " adders_{}")));
    }
    Self{
    // gadget<FieldT>(&pb, annotation_prefix),
    base,
    scalars,
    points,
    result,
    elt_size,
    num_points,
    scalar_size
    }
}


pub fn  generate_r1cs_constraints()
{
    let  num_constraints_before = self.pb.num_constraints();

    for i in 0..scalar_size - num_points
    {
        doublers[i].generate_r1cs_constraints();
    }

    for i in 0..scalar_size
    {
        adders[i].generate_r1cs_constraints();

        /*
          chosen_results[i+1].X = scalars[i] * computed_results[i].X + (1-scalars[i]) *  chosen_results[i].X
          chosen_results[i+1].X - chosen_results[i].X = scalars[i] * (computed_results[i].X - chosen_results[i].X)
        */
        self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(scalars[i],
                                                             computed_results[i].X - chosen_results[i].X,
                                                             chosen_results[i+1].X - chosen_results[i].X),
                                   FMT(self.annotation_prefix, " chosen_results_{}_X", i+1));
        self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(scalars[i],
                                                             computed_results[i].Y - chosen_results[i].Y,
                                                             chosen_results[i+1].Y - chosen_results[i].Y),
                                   FMT(self.annotation_prefix, " chosen_results_{}_Y", i+1));
    }

    let  num_constraints_after = self.pb.num_constraints();
    assert!(num_constraints_after - num_constraints_before == 4 * (scalar_size-num_points) + (4 + 2) * scalar_size);
}


pub fn  generate_r1cs_witness()
{
    for i in 0..scalar_size - num_points
    {
        doublers[i].generate_r1cs_witness();
    }

    for i in 0..scalar_size
    {
        adders[i].generate_r1cs_witness();
        self.pb.borrow().lc_val(chosen_results[i+1].X) = if self.pb.borrow().val(&scalars[i]) == ppT::Fr::zero() {self.pb.borrow().lc_val(chosen_results[i].X)} else{self.pb.borrow().lc_val(computed_results[i].X)};
        self.pb.borrow().lc_val(chosen_results[i+1].Y) = if self.pb.borrow().val(&scalars[i]) == ppT::Fr::zero() {self.pb.borrow().lc_val(chosen_results[i].Y)} else{self.pb.borrow().lc_val(computed_results[i].Y)};
    }
}
}


//#endif // WEIERSTRASS_G1_GADGET_TCC_
