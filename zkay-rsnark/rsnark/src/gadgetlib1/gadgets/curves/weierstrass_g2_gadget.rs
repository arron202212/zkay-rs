/** @file
 *****************************************************************************

 Declaration of interfaces for G2 gadgets.

 The gadgets verify curve arithmetic in G2 = E'(F) where E'/F^e: y^2 = x^3 + A' * X + B'
 is an elliptic curve over F^e in short Weierstrass form.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G2_GADGET_HPP_
// #define WEIERSTRASS_G2_GADGET_HPP_

// 

use ff_curves::algebra::curves::public_params;

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params;



/**
 * Gadget that represents a G2 variable.
 */

pub struct G2_variable  {
// : public gadget<ffec::Fr<ppT> >
//     type FieldT=ffec::Fr<ppT>;
//     type FqeT=ffec::Fqe<other_curve<ppT> >;
//     type FqkT=ffec::Fqk<other_curve<ppT> >;

X:    RcCell<Fqe_variable<ppT> >,
Y:    RcCell<Fqe_variable<ppT> >,

all_vars:    pb_linear_combination_array<FieldT>,



}

/**
 * Gadget that creates constraints for the validity of a G2 variable.
 */

pub struct G2_checker_gadget{
//  : public gadget<ffec::Fr<ppT> > 
//     type FieldT=ffec::Fr<ppT>;
//     type FqeT=ffec::Fqe<other_curve<ppT> >;
//     type FqkT=ffec::Fqk<other_curve<ppT> >;

Q:    G2_variable<ppT>,

Xsquared:    RcCell<Fqe_variable<ppT> >,
Ysquared:    RcCell<Fqe_variable<ppT> >,
Xsquared_plus_a:    RcCell<Fqe_variable<ppT> >,
Ysquared_minus_b:    RcCell<Fqe_variable<ppT> >,

compute_Xsquared:    RcCell<Fqe_sqr_gadget<ppT> >,
compute_Ysquared:    RcCell<Fqe_sqr_gadget<ppT> >,
curve_equation:    RcCell<Fqe_mul_gadget<ppT> >,


}



// use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget;

//#endif // WEIERSTRASS_G2_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for G2 gadgets.

 See weierstrass_g2_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G2_GADGET_TCC_
// #define WEIERSTRASS_G2_GADGET_TCC_

 use ffec::algebra::scalar_multiplication::wnaf;

impl G2_variable<ppT>{


pub fn new(pb:&RcCell<protoboard<FieldT>>,
                              annotation_prefix:&String)->Self
    
{
    X=RcCell::new(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " X")));
    Y=RcCell::new(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " Y")));

    all_vars.insert(all_vars.end(), X.all_vars.begin(), X.all_vars.end());
    all_vars.insert(all_vars.end(), Y.all_vars.begin(), Y.all_vars.end());
    // gadget<FieldT>(&pb, annotation_prefix)
}


pub fn new2(pb:&RcCell<protoboard<FieldT>>,
                              Q:&ffec::G2<other_curve<ppT> >,
                              annotation_prefix:&String)->Self
    
{
    let  Q_copy = Q.clone();
    Q_copy.to_affine_coordinates();

    X=RcCell::new(Fqe_variable::<ppT>::new(pb, Q_copy.X(), FMT(annotation_prefix, " X")));
    Y=RcCell::new(Fqe_variable::<ppT>::new(pb, Q_copy.Y(), FMT(annotation_prefix, " Y")));

    all_vars.insert(all_vars.end(), X.all_vars.begin(), X.all_vars.end());
    all_vars.insert(all_vars.end(), Y.all_vars.begin(), Y.all_vars.end());
    // gadget<FieldT>(&pb, annotation_prefix)
}


pub fn generate_r1cs_witness(Q:&ffec::G2<other_curve<ppT> >)
{
    let mut  Qcopy = Q.clone();
    Qcopy.to_affine_coordinates();

    X.generate_r1cs_witness(Qcopy.X());
    Y.generate_r1cs_witness(Qcopy.Y());
}


pub fn size_in_bits()->usize
{
    return 2 * Fqe_variable::<ppT>::size_in_bits();
}


pub fn num_variables(&self)->usize
{
    return 2 * Fqe_variable::<ppT>::num_variables();
}
}
impl G2_checker_gadget<ppT>{

pub fn new(pb:&RcCell<protoboard<FieldT>>,
                                          Q:&G2_variable<ppT>,
                                          annotation_prefix:&String)->Self
  
{
    Xsquared=RcCell::new(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " Xsquared")));
    Ysquared=RcCell::new(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " Ysquared")));

    compute_Xsquared=RcCell::new(Fqe_sqr_gadget::<ppT>::new(pb, *(Q.X), *Xsquared, FMT(annotation_prefix, " compute_Xsquared")));
    compute_Ysquared=RcCell::new(Fqe_sqr_gadget::<ppT>::new(pb, *(Q.Y), *Ysquared, FMT(annotation_prefix, " compute_Ysquared")));

    Xsquared_plus_a=RcCell::new(Fqe_variable::<ppT>::new((*Xsquared) + ffec::G2::<other_curve::<ppT> >::coeff_a));
    Ysquared_minus_b=RcCell::new(Fqe_variable::<ppT>::new((*Ysquared) + (-ffec::G2::<other_curve::<ppT> >::coeff_b)));

    curve_equation=RcCell::new(Fqe_mul_gadget::<ppT>::new(pb, *(Q.X), *Xsquared_plus_a, *Ysquared_minus_b, FMT(annotation_prefix, " curve_equation")));
    Self{
    //   gadget<FieldT>(&pb, annotation_prefix),
    Q
    }
}


pub fn generate_r1cs_constraints()
{
    compute_Xsquared.generate_r1cs_constraints();
    compute_Ysquared.generate_r1cs_constraints();
    curve_equation.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_Xsquared.generate_r1cs_witness();
    compute_Ysquared.generate_r1cs_witness();
    Xsquared_plus_a.evaluate();
    curve_equation.generate_r1cs_witness();
}
}


pub fn  test_G2_checker_gadget(annotation:&String)
{
    let mut  pb=protoboard::<ppT::Fr >::new();
    let mut  g=G2_variable::<ppT>::new(pb, "g");
    let mut g_check= G2_checker_gadget::<ppT>::new(pb, g, "g_check");
    g_check.generate_r1cs_constraints();

    print!("positive test\n");
    g.generate_r1cs_witness(ffec::G2::<other_curve::<ppT> >::one());
    g_check.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    print!("negative test\n");
    g.generate_r1cs_witness(ffec::G2::<other_curve::<ppT> >::zero());
    g_check.generate_r1cs_witness();
    assert!(!pb.is_satisfied());

    print!("number of constraints for G2 checker (Fr is {})  = {}\n", annotation, pb.num_constraints());
}



//#endif // WEIERSTRASS_G2_GADGET_TCC_
