/** @file
 *****************************************************************************

 Declaration of interfaces for pairing precomputation gadgets.

 The gadgets verify correct precomputation of values for the G1 and G2 variables.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_PRECOMPUTATION_HPP_
// #define WEIERSTRASS_PRECOMPUTATION_HPP_



use ffec::algebra::curves::mnt::mnt4::mnt4_init;
use ffec::algebra::curves::mnt::mnt6::mnt6_init;

use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget;
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params;



/**************************** G1 Precomputation ******************************/

/**
 * Not a gadget. It only holds values.
 */

pub struct G1_precomputation<ppT> {

    // type FieldT=ffec::Fr<ppT>;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

P:    RcCell<G1_variable<ppT> >,
PY_twist_squared:    RcCell<Fqe_variable<ppT> >,

 
}

/**
 * Gadget that verifies correct precomputation of the G1 variable.
 */

pub struct precompute_G1_gadget<ppT> {//gadget<ffec::Fr<ppT> >

    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

precomp:    G1_precomputation<ppT>, // must be a reference.
}
impl precompute_G1_gadget<ppT> {
    /* two possible pre-computations one for mnt4 and one for mnt6 */
    
    pub fn new(pb:protoboard<FieldT>,
                         P:&G1_variable<ppT>,
precomp:                         G1_precomputation<ppT>, // will allocate this inside
                         annotation_prefix:&String,
                         )->Self
    {
// 4:std::enable_if<ffec::Fqk<other_curve::<ppT> >::extension_degree() ==, FieldT>::type& = FieldT()
       let  (c0, c1)=(pb_linear_combination::<FieldT>::new() ,pb_linear_combination::<FieldT>::new());
        c0.assign(pb, P.Y * ((ffec::mnt4_twist).squared().c0));
        c1.assign(pb, P.Y * ((ffec::mnt4_twist).squared().c1));

        precomp.P.reset(G1_variable::<ppT>::new(P));
        precomp.PY_twist_squared.reset(Fqe_variable::<ppT>::new(pb, c0, c1, FMT(annotation_prefix, " PY_twist_squared")));
        //  gadget<FieldT>(pb, annotation_prefix),
            Self{precomp}
    }

    
    pub fn new2(pb:protoboard<FieldT>,
                         P:&G1_variable<ppT>,
precomp:                         G1_precomputation<ppT>, // will allocate this inside
                         annotation_prefix:&String,
                         )->Self
        
    {
// 6:std::enable_if<ffec::Fqk<other_curve::<ppT> >::extension_degree() ==, FieldT>::type& = FieldT()
        let ( c0, c1, c2)=(pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new(),pb_linear_combination::<FieldT>::new());
        c0.assign(pb, P.Y * ((ffec::mnt6_twist).squared().c0));
        c1.assign(pb, P.Y * ((ffec::mnt6_twist).squared().c1));
        c2.assign(pb, P.Y * ((ffec::mnt6_twist).squared().c2));

        precomp.P.reset(G1_variable::<ppT>::new(P));
        precomp.PY_twist_squared.reset(Fqe_variable::<ppT>::new(pb, c0, c1, c2, FMT(annotation_prefix, " PY_twist_squared")));
        // gadget<FieldT>(pb, annotation_prefix),
           Self{ precomp}
    }

}





/**************************** G2 Precomputation ******************************/

/**
 * Not a gadget. It only holds values.
 */

pub struct precompute_G2_gadget_coeffs<ppT> {

    // type FieldT=ffec::Fr<ppT>;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

RX:    RcCell<Fqe_variable<ppT> >,
RY:    RcCell<Fqe_variable<ppT> >,
gamma:    RcCell<Fqe_variable<ppT> >,
gamma_X:    RcCell<Fqe_variable<ppT> >,


}

/**
 * Not a gadget. It only holds values.
 */

pub struct G2_precomputation<ppT> {

    // type FieldT=ffec::Fr<ppT>;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

Q:    RcCell<G2_variable<ppT> >,

coeffs:    Vec<RcCell<precompute_G2_gadget_coeffs<ppT> > >,

}

/**
 * Technical note:
 *
 * QX and QY -- X and Y coordinates of Q
 *
 * initialization:
 * coeffs[0].RX = QX
 * coeffs[0].RY = QY
 *
 * G2_precompute_doubling_step relates coeffs[i] and coeffs[i+1] as follows
 *
 * coeffs[i]
 * gamma = (3 * RX^2 + twist_coeff_a) * (2*RY).inverse()
 * gamma_X = gamma * RX
 *
 * coeffs[i+1]
 * RX = prev_gamma^2 - (2*prev_RX)
 * RY = prev_gamma * (prev_RX - RX) - prev_RY
 */

pub struct precompute_G2_gadget_doubling_step<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

cur:    precompute_G2_gadget_coeffs<ppT>,
next:    precompute_G2_gadget_coeffs<ppT>,

RXsquared:    RcCell<Fqe_variable<ppT> >,
compute_RXsquared:    RcCell<Fqe_sqr_gadget<ppT> >,
three_RXsquared_plus_a:    RcCell<Fqe_variable<ppT> >,
two_RY:    RcCell<Fqe_variable<ppT> >,
compute_gamma:    RcCell<Fqe_mul_gadget<ppT> >,
compute_gamma_X:    RcCell<Fqe_mul_gadget<ppT> >,

next_RX_plus_two_RX:    RcCell<Fqe_variable<ppT> >,
compute_next_RX:    RcCell<Fqe_sqr_gadget<ppT> >,

RX_minus_next_RX:    RcCell<Fqe_variable<ppT> >,
RY_plus_next_RY:    RcCell<Fqe_variable<ppT> >,
compute_next_RY:    RcCell<Fqe_mul_gadget<ppT> >,

  
}

/**
 * Technical note:
 *
 * G2_precompute_addition_step relates coeffs[i] and coeffs[i+1] as follows
 *
 * coeffs[i]
 * gamma = (RY - QY) * (RX - QX).inverse()
 * gamma_X = gamma * QX
 *
 * coeffs[i+1]
 * RX = prev_gamma^2 + (prev_RX + QX)
 * RY = prev_gamma * (prev_RX - RX) - prev_RY
 *
 * (where prev_ in [i+1] refer to things from [i])
 *
 * If invert_Q is set to true: use -QY in place of QY everywhere above.
 */

pub struct precompute_G2_gadget_addition_step<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

invert_Q:    bool,
cur:    precompute_G2_gadget_coeffs<ppT>,
next:    precompute_G2_gadget_coeffs<ppT>,
Q:    G2_variable<ppT>,

RY_minus_QY:    RcCell<Fqe_variable<ppT> >,
RX_minus_QX:    RcCell<Fqe_variable<ppT> >,
compute_gamma:    RcCell<Fqe_mul_gadget<ppT> >,
compute_gamma_X:    RcCell<Fqe_mul_gadget<ppT> >,

next_RX_plus_RX_plus_QX:    RcCell<Fqe_variable<ppT> >,
compute_next_RX:    RcCell<Fqe_sqr_gadget<ppT> >,

RX_minus_next_RX:    RcCell<Fqe_variable<ppT> >,
RY_plus_next_RY:    RcCell<Fqe_variable<ppT> >,
compute_next_RY:    RcCell<Fqe_mul_gadget<ppT> >,


}

/**
 * Gadget that verifies correct precomputation of the G2 variable.
 */

pub struct precompute_G2_gadget<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

addition_steps:    Vec<RcCell<precompute_G2_gadget_addition_step<ppT> > >,
doubling_steps:    Vec<RcCell<precompute_G2_gadget_doubling_step<ppT> > >,

add_count:    usize,
dbl_count:    usize,

precomp:    G2_precomputation<ppT>, // important to have a reference here

}






// use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation;

//#endif // WEIERSTRASS_PRECOMPUTATION_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for pairing precomputation gadgets.

 See weierstrass_precomputation.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_PRECOMPUTATION_TCC_
// #define WEIERSTRASS_PRECOMPUTATION_TCC_

// use  <type_traits>

use crate::gadgetlib1::gadgets::pairing::mnt_pairing_params;


impl G1_precomputation<ppT>{

// pub fn new2()
// {
//     // will be filled in precompute_G1_gadget, so do nothing here
// }


pub fn new(pb:protoboard<FieldT>,
                                          P_val:&ffec::G1<other_curve<ppT> >,
                                          annotation_prefix:&String)->Self
{
let P_val_copy= P_val;
    P_val_copy.to_affine_coordinates();
    P.reset(G1_variable::<ppT>::new(pb, P_val_copy, FMT(annotation_prefix, " P")));
    PY_twist_squared.reset(Fqe_variable::<ppT>::new(pb, P_val_copy.Y() * ffec::G2::<other_curve::<ppT> >::twist.squared(), " PY_twist_squared"));
}


pub fn generate_r1cs_constraints()
{
    /* the same for neither ppT = mnt4 nor ppT = mnt6 */
}


pub fn generate_r1cs_witness()
{
    precomp.PY_twist_squared.evaluate(); /* the same for both ppT = mnt4 and ppT = mnt6 */
}
}

pub fn  test_G1_variable_precomp(annotation:&String)
{
    let mut pb=protoboard::<ffec::Fr::<ppT> >:: new();
    let mut  g_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();

    let mut g=G1_variable::<ppT>::new(pb, "g");
let mut precomp=    G1_precomputation::<ppT>::new();
    let mut do_precomp=precompute_G1_gadget::<ppT>::new(pb, g, precomp, "do_precomp");
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(g_val);
    do_precomp.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    let mut const_precomp=G1_precomputation::<ppT>::new(pb, g_val, "const_precomp");

let native_precomp= other_curve::<ppT>::affine_ate_precompute_G1(g_val);
    assert!(precomp.PY_twist_squared.get_element() == native_precomp.PY_twist_squared);
    assert!(const_precomp.PY_twist_squared.get_element() == native_precomp.PY_twist_squared);

    print!("number of constraints for G1 precomp (Fr is {})  = {}\n", annotation, pb.num_constraints());
}

impl precompute_G1_gadget<ppT> {
// pub fn new()
// {
// }


pub fn new(pb:protoboard<FieldT>,
                                          Q_val:&ffec::G2::<other_curve::<ppT> >,
                                          annotation_prefix:&String)->Self
{
    Q.reset(G2_variable::<ppT>::new(pb, Q_val, FMT(annotation_prefix, " Q")));
    let  native_precomp = other_curve::<ppT>::affine_ate_precompute_G2(Q_val);

    coeffs.resize(native_precomp.coeffs.len() + 1); // the last precomp remains for convenient programming
    for i in 0..native_precomp.coeffs.len()
    {
        coeffs[i].reset(precompute_G2_gadget_coeffs::<ppT>::new());
        coeffs[i].RX.reset(Fqe_variable::<ppT>::new(pb, native_precomp.coeffs[i].old_RX, FMT(annotation_prefix, " RX")));
        coeffs[i].RY.reset(Fqe_variable::<ppT>::new(pb, native_precomp.coeffs[i].old_RY, FMT(annotation_prefix, " RY")));
        coeffs[i].gamma.reset(Fqe_variable::<ppT>::new(pb, native_precomp.coeffs[i].gamma, FMT(annotation_prefix, " gamma")));
        coeffs[i].gamma_X.reset(Fqe_variable::<ppT>::new(pb, native_precomp.coeffs[i].gamma_X, FMT(annotation_prefix, " gamma_X")));
    }
}

}
impl precompute_G2_gadget_coeffs<ppT>{
// pub fn new2()
// {
//     // we will be filled in precomputed case of precompute_G2_gadget, so do nothing here
// }


pub fn new(pb:protoboard<FieldT>,
                                                              annotation_prefix:&String)->Self
{
    RX.reset(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " RX")));
    RY.reset(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " RY")));
    gamma.reset(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " gamma")));
    gamma_X.reset(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " gamma_X")));
    Self{}
}



pub fn new2(pb:protoboard<FieldT>,
                                                              Q:&G2_variable<ppT>,
                                                              annotation_prefix:&String)->Self
{
    RX.reset(Fqe_variable::<ppT>::new(*(Q.X)));
    RY.reset(Fqe_variable::<ppT>::new(*(Q.Y)));
    gamma.reset(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " gamma")));
    gamma_X.reset(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " gamma_X")));
}
}

/*
 QX and QY -- X and Y coordinates of Q

 initialization:
 coeffs[0].RX = QX
 coeffs[0].RY = QY

 G2_precompute_doubling_step relates coeffs[i] and coeffs[i+1] as follows

 coeffs[i]
 gamma = (3 * RX^2 + twist_coeff_a) * (2*RY).inverse()
 gamma_X = gamma * RX

 coeffs[i+1]
 RX = prev_gamma^2 - (2*prev_RX)
 RY = prev_gamma * (prev_RX - RX) - prev_RY
 */

impl G2_precomputation<ppT>{
pub fn new(pb:protoboard<FieldT>,
                                                                            cur:&precompute_G2_gadget_coeffs<ppT>,
                                                                            next:&precompute_G2_gadget_coeffs<ppT>,
                                                                            annotation_prefix:&String)->Self
    
{
    RXsquared.reset(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " RXsquared")));
    compute_RXsquared.reset(Fqe_sqr_gadget::<ppT>::new(pb, *(cur.RX), *RXsquared, FMT(annotation_prefix, " compute_RXsquared")));
    three_RXsquared_plus_a.reset(Fqe_variable::<ppT>::new((*RXsquared) * FieldT(3) + ffec::G2::<other_curve::<ppT> >::coeff_a));
    two_RY.reset(Fqe_variable::<ppT>::new(*(cur.RY) * FieldT(2)));

    compute_gamma.reset(Fqe_mul_gadget::<ppT>::new(pb, *(cur.gamma), *two_RY, *three_RXsquared_plus_a, FMT(annotation_prefix, " compute_gamma")));
    compute_gamma_X.reset(Fqe_mul_gadget::<ppT>::new(pb, *(cur.gamma), *(cur.RX), *(cur.gamma_X), FMT(annotation_prefix, " compute_gamma_X")));

    next_RX_plus_two_RX.reset(Fqe_variable::<ppT>::new(*(next.RX) + *(cur.RX) * FieldT(2)));
    compute_next_RX.reset(Fqe_sqr_gadget::<ppT>::new(pb, *(cur.gamma), *next_RX_plus_two_RX, FMT(annotation_prefix, " compute_next_RX")));

    RX_minus_next_RX.reset(Fqe_variable::<ppT>::new(*(cur.RX) + *(next.RX) * (-FieldT::one())));
    RY_plus_next_RY.reset(Fqe_variable::<ppT>::new(*(cur.RY) + *(next.RY)));
    compute_next_RY.reset(Fqe_mul_gadget::<ppT>::new(pb, *(cur.gamma), *RX_minus_next_RX, *RY_plus_next_RY, FMT(annotation_prefix, " compute_next_RY")));
    // gadget<FieldT>(pb, annotation_prefix),
   Self{cur,
    next}
}


pub fn generate_r1cs_constraints()
{
    compute_RXsquared.generate_r1cs_constraints();
    compute_gamma.generate_r1cs_constraints();
    compute_gamma_X.generate_r1cs_constraints();
    compute_next_RX.generate_r1cs_constraints();
    compute_next_RY.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_RXsquared.generate_r1cs_witness();
    two_RY.evaluate();
    three_RXsquared_plus_a.evaluate();

    let three_RXsquared_plus_a_val= three_RXsquared_plus_a.get_element();
    let two_RY_val= two_RY.get_element();
    let gamma_val= three_RXsquared_plus_a_val * two_RY_val.inverse();
    cur.gamma.generate_r1cs_witness(gamma_val);

    compute_gamma.generate_r1cs_witness();
    compute_gamma_X.generate_r1cs_witness();

    let RX_val= cur.RX.get_element();
    let RY_val= cur.RY.get_element();
    let next_RX_val= gamma_val.squared() - RX_val - RX_val;
    let next_RY_val= gamma_val * (RX_val - next_RX_val) - RY_val;

    next.RX.generate_r1cs_witness(next_RX_val);
    next.RY.generate_r1cs_witness(next_RY_val);

    RX_minus_next_RX.evaluate();
    RY_plus_next_RY.evaluate();

    compute_next_RX.generate_r1cs_witness();
    compute_next_RY.generate_r1cs_witness();
}
}
/*
 G2_precompute_addition_step relates coeffs[i] and coeffs[i+1] as follows

 coeffs[i]
 gamma = (RY - QY) * (RX - QX).inverse()
 gamma_X = gamma * QX

 coeffs[i+1]
 RX = prev_gamma^2 - (prev_RX + QX)
 RY = prev_gamma * (prev_RX - RX) - prev_RY

 (where prev_ in [i+1] refer to things from [i])

 If invert_Q is set to true: use -QY in place of QY everywhere above.
 */
impl precompute_G2_gadget_doubling_step<ppT> {
pub fn new(pb:protoboard<FieldT>,
                                                                            invert_Q:bool,
                                                                            cur:&precompute_G2_gadget_coeffs<ppT>,
                                                                            next:&precompute_G2_gadget_coeffs<ppT>,
                                                                            Q:&G2_variable<ppT>,
                                                                            annotation_prefix:&String)->Self
   
{
    RY_minus_QY.reset(Fqe_variable::<ppT>::new(*(cur.RY) + *(Q.Y) * (if !invert_Q { -FieldT::one()} else {FieldT::one()})));

    RX_minus_QX.reset(Fqe_variable::<ppT>::new(*(cur.RX) + *(Q.X) * (-FieldT::one())));
    compute_gamma.reset(Fqe_mul_gadget::<ppT>::new(pb, *(cur.gamma), *RX_minus_QX, *RY_minus_QY, FMT(annotation_prefix, " compute_gamma")));
    compute_gamma_X.reset(Fqe_mul_gadget::<ppT>::new(pb, *(cur.gamma), *(Q.X), *(cur.gamma_X), FMT(annotation_prefix, " compute_gamma_X")));

    next_RX_plus_RX_plus_QX.reset(Fqe_variable::<ppT>::new(*(next.RX) + *(cur.RX) + *(Q.X)));
    compute_next_RX.reset(Fqe_sqr_gadget::<ppT>::new(pb, *(cur.gamma), *next_RX_plus_RX_plus_QX, FMT(annotation_prefix, " compute_next_RX")));

    RX_minus_next_RX.reset(Fqe_variable::<ppT>::new(*(cur.RX) + *(next.RX) * (-FieldT::one())));
    RY_plus_next_RY.reset(Fqe_variable::<ppT>::new(*(cur.RY) + *(next.RY)));
    compute_next_RY.reset(Fqe_mul_gadget::<ppT>::new(pb, *(cur.gamma), *RX_minus_next_RX, *RY_plus_next_RY, FMT(annotation_prefix, " compute_next_RY")));
    //  gadget<FieldT>(pb, annotation_prefix),
   Self{invert_Q,
   cur,
   next,
    Q}
}


pub fn generate_r1cs_constraints()
{
    compute_gamma.generate_r1cs_constraints();
    compute_gamma_X.generate_r1cs_constraints();
    compute_next_RX.generate_r1cs_constraints();
    compute_next_RY.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    RY_minus_QY.evaluate();
    RX_minus_QX.evaluate();

    let RY_minus_QY_val= RY_minus_QY.get_element();
    let RX_minus_QX_val= RX_minus_QX.get_element();
    let gamma_val= RY_minus_QY_val * RX_minus_QX_val.inverse();
    cur.gamma.generate_r1cs_witness(gamma_val);

    compute_gamma.generate_r1cs_witness();
    compute_gamma_X.generate_r1cs_witness();

    let RX_val= cur.RX.get_element();
    let RY_val= cur.RY.get_element();
    let QX_val= Q.X.get_element();
    let next_RX_val= gamma_val.squared() - RX_val - QX_val;
    let next_RY_val= gamma_val * (RX_val - next_RX_val) - RY_val;

    next.RX.generate_r1cs_witness(next_RX_val);
    next.RY.generate_r1cs_witness(next_RY_val);

    next_RX_plus_RX_plus_QX.evaluate();
    RX_minus_next_RX.evaluate();
    RY_plus_next_RY.evaluate();

    compute_next_RX.generate_r1cs_witness();
    compute_next_RY.generate_r1cs_witness();
}

}

impl precompute_G2_gadget_addition_step<ppT> {
pub fn new(pb:protoboard<FieldT>,
                                                Q:&G2_variable<ppT>,
precomp:                                                G2_precomputation<ppT>,  // will allocate this inside
                                                annotation_prefix:&String)->Self
  
{
    precomp.Q.reset(G2_variable::<ppT>::new(Q));

    let loop_count = pairing_selector::<ppT>::pairing_loop_count;
    let coeff_count= 1; // the last RX/RY are unused in Miller loop, but will need to get allocated somehow
    self.add_count = 0;
    self.dbl_count = 0;

let found_nonzero= false;
let NAF= find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        dbl_count+=1;
        coeff_count+=1;

        if NAF[i] != 0
        {
            add_count+=1;
            coeff_count+=1;
        }
    }

    precomp.coeffs.resize(coeff_count);
    addition_steps.resize(add_count);
    doubling_steps.resize(dbl_count);

    precomp.coeffs[0].reset(precompute_G2_gadget_coeffs::<ppT>::new(pb, Q, FMT(annotation_prefix, " coeffs_0")));
    for i in 1..coeff_count
    {
        precomp.coeffs[i].reset(precompute_G2_gadget_coeffs::<ppT>::new(pb, FMT(annotation_prefix, " coeffs_{}", i)));
    }

let add_id= 0;
let dbl_id= 0;
let coeff_id= 0;

    found_nonzero = false;
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps[dbl_id].reset(precompute_G2_gadget_doubling_step::<ppT>::new(pb, *(precomp.coeffs[coeff_id]), *(precomp.coeffs[coeff_id+1]),
                                                                                 FMT(annotation_prefix, " doubling_steps_{}", dbl_id)));
        dbl_id+=1;
        coeff_id+=1;

        if NAF[i] != 0
        {
            addition_steps[add_id].reset(precompute_G2_gadget_addition_step::<ppT>::new(pb, NAF[i] < 0, *(precomp.coeffs[coeff_id]), *(precomp.coeffs[coeff_id+1]), Q,
                                                                                     FMT(annotation_prefix, " addition_steps_{}", add_id)));
            add_id+=1;
            coeff_id+=1;
        }
    }
    //   gadget<FieldT>(pb, annotation_prefix),
    Self{precomp}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..dbl_count
    {
        doubling_steps[i].generate_r1cs_constraints();
    }

    for i in 0..add_count
    {
        addition_steps[i].generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    precomp.coeffs[0].RX.generate_r1cs_witness(precomp.Q.X.get_element());
    precomp.coeffs[0].RY.generate_r1cs_witness(precomp.Q.Y.get_element());

    let loop_count = pairing_selector::<ppT>::pairing_loop_count;

let add_id= 0;
let dbl_id= 0;

let found_nonzero= false;
let NAF= find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps[dbl_id].generate_r1cs_witness();
        dbl_id+=1;

        if NAF[i] != 0
        {
            addition_steps[add_id].generate_r1cs_witness();
            add_id+=1;
        }
    }
}

}
pub fn  test_G2_variable_precomp(annotation:&String)
{
    let mut pb=protoboard::<ffec::Fr::<ppT> >:: new();
let g_val= ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

   let mut  g=G2_variable::<ppT>::new(pb, "g");
    let mut precomp=G2_precomputation::<ppT>::new();
    let mut  do_precomp=precompute_G2_gadget::<ppT>::new(pb, g, precomp, "do_precomp");
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(g_val);
    do_precomp.generate_r1cs_witness();
    assert!(pb.is_satisfied());

let native_precomp= other_curve::<ppT>::affine_ate_precompute_G2(g_val);

    assert!(precomp.coeffs.len() - 1 == native_precomp.coeffs.len()); // the last precomp is unused, but remains for convenient programming
    for i in 0..native_precomp.coeffs.len()
    {
        assert!(precomp.coeffs[i].RX.get_element() == native_precomp.coeffs[i].old_RX);
        assert!(precomp.coeffs[i].RY.get_element() == native_precomp.coeffs[i].old_RY);
        assert!(precomp.coeffs[i].gamma.get_element() == native_precomp.coeffs[i].gamma);
        assert!(precomp.coeffs[i].gamma_X.get_element() == native_precomp.coeffs[i].gamma_X);
    }

    print!("number of constraints for G2 precomp (Fr is {})  = {}\n", annotation, pb.num_constraints());
}



//#endif // WEIERSTRASS_PRECOMPUTATION_TCC_
