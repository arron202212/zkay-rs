/** @file
 *****************************************************************************

 Declaration of interfaces for gadgets for Miller loops.

 The gadgets verify computations of (single or multiple simultaneous) Miller loops.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_MILLER_LOOP_HPP_
// #define WEIERSTRASS_MILLER_LOOP_HPP_



use crate::gadgetlib1::gadgets::pairing::pairing_params;
use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation;



/**
 * Gadget for doubling step in the Miller loop.
 *
 * Technical note:
 *
 * mnt_Fqk g_RR_at_P = mnt_Fqk(prec_P.PY_twist_squared,
 *                             -prec_P.PX * c.gamma_twist + c.gamma_X - c.old_RY);
 *
 *(later in Miller loop: f = f.squared() * g_RR_at_P)
 *
 * Note the slight interface change: this gadget allocates g_RR_at_P inside itself (!)
 */

pub struct mnt_miller_loop_dbl_line_eval<ppT> {//gadget<ppT::Fr >

    // type FieldT=ppT::Fr;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

prec_P:    G1_precomputation::<ppT>,
c:    precompute_G2_gadget_coeffs::<ppT>,
    g_RR_at_P:RcCell<Fqk_variable::<ppT> >, // reference from outside

gamma_twist:    RcCell<Fqe_variable::<ppT> >,
g_RR_at_P_c1:    RcCell<Fqe_variable::<ppT> >,
compute_g_RR_at_P_c1:    RcCell<Fqe_mul_by_lc_gadget::<ppT> >,


}

/**
 * Gadget for addition step in the Miller loop.
 *
 * Technical note:
 *
 * mnt_Fqk g_RQ_at_P = mnt_Fqk(prec_P.PY_twist_squared,
 *                            -prec_P.PX * c.gamma_twist + c.gamma_X - prec_Q.QY);
 *
 * (later in Miller loop: f = f * g_RQ_at_P)
 *
 * Note the slight interface change: this gadget will allocate g_RQ_at_P inside itself (!)
 */

pub struct mnt_miller_loop_add_line_eval<ppT> {//gadget<ppT::Fr >

    // type FieldT=ppT::Fr;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

invert_Q:    bool,
prec_P:    G1_precomputation::<ppT>,
c:    precompute_G2_gadget_coeffs::<ppT>,
Q:    G2_variable::<ppT>,
g_RQ_at_P:    RcCell<Fqk_variable::<ppT> >, // reference from outside

gamma_twist:    RcCell<Fqe_variable::<ppT> >,
g_RQ_at_P_c1:    RcCell<Fqe_variable::<ppT> >,
compute_g_RQ_at_P_c1:    RcCell<Fqe_mul_by_lc_gadget::<ppT> >,

 
}

/**
 * Gadget for verifying a single Miller loop.
 */

pub struct mnt_miller_loop_gadget<ppT> {//gadget<ppT::Fr >

    // type FieldT=ppT::Fr;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;

g_RR_at_Ps:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RQ_at_Ps:    Vec<RcCell<Fqk_variable::<ppT> > >,
fs:    Vec<RcCell<Fqk_variable::<ppT> > >,

addition_steps:    Vec<RcCell<mnt_miller_loop_add_line_eval::<ppT> > >,
doubling_steps:    Vec<RcCell<mnt_miller_loop_dbl_line_eval::<ppT> > >,

dbl_muls:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
dbl_sqrs:    Vec<RcCell<Fqk_sqr_gadget::<ppT> > >,
add_muls:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,

f_count:    usize,
add_count:    usize,
dbl_count:    usize,

prec_P:    G1_precomputation::<ppT>,
prec_Q:    G2_precomputation::<ppT>,
result:    Fqk_variable::<ppT>,

   
}


// pub fn  test_mnt_miller_loop(annotation:&String);

/**
 * Gadget for verifying a double Miller loop (where the second is inverted).
 */

// type FieldT=ppT::Fr;
//     type FqeT=ffec::Fqe<other_curve::<ppT> >;
//     type FqkT=ffec::Fqk<other_curve::<ppT> >;

pub struct mnt_e_over_e_miller_loop_gadget<ppT> {//gadget<ppT::Fr >

    

g_RR_at_P1s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RQ_at_P1s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RR_at_P2s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RQ_at_P2s:    Vec<RcCell<Fqk_variable::<ppT> > >,
fs:    Vec<RcCell<Fqk_variable::<ppT> > >,

addition_steps1:    Vec<RcCell<mnt_miller_loop_add_line_eval::<ppT> > >,
doubling_steps1:    Vec<RcCell<mnt_miller_loop_dbl_line_eval::<ppT> > >,
addition_steps2:    Vec<RcCell<mnt_miller_loop_add_line_eval::<ppT> > >,
doubling_steps2:    Vec<RcCell<mnt_miller_loop_dbl_line_eval::<ppT> > >,

dbl_sqrs:    Vec<RcCell<Fqk_sqr_gadget::<ppT> > >,
dbl_muls1:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
add_muls1:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
dbl_muls2:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
add_muls2:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,

f_count:    usize,
add_count:    usize,
dbl_count:    usize,

prec_P1:    G1_precomputation::<ppT>,
prec_Q1:    G2_precomputation::<ppT>,
prec_P2:    G1_precomputation::<ppT>,
prec_Q2:    G2_precomputation::<ppT>,
result:    Fqk_variable::<ppT>,

  
}


// pub fn  test_mnt_e_over_e_miller_loop(annotation:&String);

/**
 * Gadget for verifying a triple Miller loop (where the third is inverted).
 */

//   type FieldT=ppT::Fr;
//     type FqeT=ffec::Fqe<other_curve::<ppT> >;
//     type FqkT=ffec::Fqk<other_curve::<ppT> >;
pub struct mnt_e_times_e_over_e_miller_loop_gadget<ppT> {//gadget<ppT::Fr >

  

g_RR_at_P1s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RQ_at_P1s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RR_at_P2s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RQ_at_P2s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RR_at_P3s:    Vec<RcCell<Fqk_variable::<ppT> > >,
g_RQ_at_P3s:    Vec<RcCell<Fqk_variable::<ppT> > >,
fs:    Vec<RcCell<Fqk_variable::<ppT> > >,

addition_steps1:    Vec<RcCell<mnt_miller_loop_add_line_eval::<ppT> > >,
doubling_steps1:    Vec<RcCell<mnt_miller_loop_dbl_line_eval::<ppT> > >,
addition_steps2:    Vec<RcCell<mnt_miller_loop_add_line_eval::<ppT> > >,
doubling_steps2:    Vec<RcCell<mnt_miller_loop_dbl_line_eval::<ppT> > >,
addition_steps3:    Vec<RcCell<mnt_miller_loop_add_line_eval::<ppT> > >,
doubling_steps3:    Vec<RcCell<mnt_miller_loop_dbl_line_eval::<ppT> > >,

dbl_sqrs:    Vec<RcCell<Fqk_sqr_gadget::<ppT> > >,
dbl_muls1:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
add_muls1:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
dbl_muls2:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
add_muls2:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
dbl_muls3:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,
add_muls3:    Vec<RcCell<Fqk_special_mul_gadget::<ppT> > >,

f_count:    usize,
add_count:    usize,
dbl_count:    usize,

prec_P1:    G1_precomputation::<ppT>,
prec_Q1:    G2_precomputation::<ppT>,
prec_P2:    G1_precomputation::<ppT>,
prec_Q2:    G2_precomputation::<ppT>,
prec_P3:    G1_precomputation::<ppT>,
prec_Q3:    G2_precomputation::<ppT>,
result:    Fqk_variable::<ppT>,

}


// pub fn  test_mnt_e_times_e_over_e_miller_loop(annotation:&String);



// use crate::gadgetlib1::gadgets::pairing::weierstrass_miller_loop;

//#endif // WEIERSTRASS_MILLER_LOOP_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for gadgets for Miller loops.

 See weierstrass_miller_loop.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_MILLER_LOOP_TCC_
// #define WEIERSTRASS_MILLER_LOOP_TCC_

 use ffec::algebra::scalar_multiplication::wnaf;

use crate::gadgetlib1::constraint_profiling;
use crate::gadgetlib1::gadgets::basic_gadgets;



/*
  performs

  mnt_Fqk g_RR_at_P = mnt_Fqk(prec_P.PY_twist_squared,
  -prec_P.PX * c.gamma_twist + c.gamma_X - c.old_RY);

  (later in Miller loop: f = f.squared() * g_RR_at_P)
*/

/* Note the slight interface change: this gadget will allocate g_RR_at_P inside itself (!) */
impl mnt_miller_loop_dbl_line_eval::<ppT> {
pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                  prec_P:&G1_precomputation::<ppT>,
                                                                  c:&precompute_G2_gadget_coeffs::<ppT>,
g_RR_at_P:&                                                                  RcCell<Fqk_variable::<ppT> >,
                                                                  annotation_prefix:&String)->Self
    
{
    gamma_twist=RcCell::new(Fqe_variable::<ppT>::new(c.gamma.mul_by_X()));
    // prec_P.PX * c.gamma_twist = c.gamma_X - c.old_RY - g_RR_at_P_c1
    if gamma_twist.is_constant()
    {
        gamma_twist.evaluate();
        let gamma_twist_const= gamma_twist.get_element();
        g_RR_at_P_c1=RcCell::new(Fqe_variable::<ppT>::new(Fqe_variable::<ppT>(self.pb, -gamma_twist_const, prec_P.P.X, FMT(annotation_prefix, " tmp")) +
                                                 *(c.gamma_X) + *(c.RY) * (-FieldT::one())));
    }
    else if prec_P.P.X.is_constant()
    {
        prec_P.P.X.evaluate(pb);
        let P_X_const= prec_P.P.X.constant_term();
        g_RR_at_P_c1=RcCell::new(Fqe_variable::<ppT>::new(*gamma_twist * (-P_X_const) + *(c.gamma_X) + *(c.RY) * (-FieldT::one())));
    }
    else
    {
        g_RR_at_P_c1=RcCell::new(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_RR_at_P_c1")));
        compute_g_RR_at_P_c1=RcCell::new(Fqe_mul_by_lc_gadget::<ppT>::new(pb, *gamma_twist, prec_P.P.X,
                                                                 *(c.gamma_X) + *(c.RY) * (-FieldT::one()) + (*g_RR_at_P_c1) * (-FieldT::one()),
                                                               FMT(annotation_prefix, " compute_g_RR_at_P_c1")));
    }
    g_RR_at_P=RcCell::new(Fqk_variable::<ppT>::new(pb, *(prec_P.PY_twist_squared), *g_RR_at_P_c1, FMT(annotation_prefix, " g_RR_at_P")));
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{prec_P,c,g_RR_at_P}
}


pub fn generate_r1cs_constraints()
{
    if !gamma_twist.is_constant() && !prec_P.P.X.is_constant()
    {
        compute_g_RR_at_P_c1.generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    gamma_twist.evaluate();
    let gamma_twist_val= gamma_twist.get_element();
    let PX_val= self.pb.lc_val(prec_P.P.X);
    let gamma_X_val= c.gamma_X.get_element();
    let RY_val= c.RY.get_element();
    let g_RR_at_P_c1_val= -PX_val * gamma_twist_val + gamma_X_val - RY_val;
    g_RR_at_P_c1.generate_r1cs_witness(g_RR_at_P_c1_val);

    if !gamma_twist.is_constant() && !prec_P.P.X.is_constant()
    {
        compute_g_RR_at_P_c1.generate_r1cs_witness();
    }
    g_RR_at_P.evaluate();
}}

/*
  performs
  mnt_Fqk g_RQ_at_P = mnt_Fqk(prec_P.PY_twist_squared,
  -prec_P.PX * c.gamma_twist + c.gamma_X - prec_Q.QY);

  (later in Miller loop: f = f * g_RQ_at_P)

  If invert_Q is set to true: use -QY in place of QY everywhere above.
*/

/* Note the slight interface change: this gadget will allocate g_RQ_at_P inside itself (!) */
impl mnt_miller_loop_add_line_eval::<ppT>{
pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                  invert_Q:bool,
                                                                  prec_P:&G1_precomputation::<ppT>,
                                                                  c:&precompute_G2_gadget_coeffs::<ppT>,
                                                                  Q:&G2_variable::<ppT>,
g_RQ_at_P:&                                                                  RcCell<Fqk_variable::<ppT> >,
                                                                  annotation_prefix:&String)->Self

{
    gamma_twist=RcCell::new(Fqe_variable::<ppT>::new(c.gamma.mul_by_X()));
    // prec_P.PX * c.gamma_twist = c.gamma_X - prec_Q.QY - g_RQ_at_P_c1
    if gamma_twist.is_constant()
    {
        gamma_twist.evaluate();
        let gamma_twist_const= gamma_twist.get_element();
        g_RQ_at_P_c1=RcCell::new(Fqe_variable::<ppT>::new(Fqe_variable::<ppT>(self.pb, -gamma_twist_const, prec_P.P.X, FMT(annotation_prefix, " tmp")) +
                                                 *(c.gamma_X) + *(Q.Y) * (if !invert_Q  {-FieldT::one()} else {FieldT::one()})));
    }
    else if prec_P.P.X.is_constant()
    {
        prec_P.P.X.evaluate(pb);
        let P_X_const= prec_P.P.X.constant_term();
        g_RQ_at_P_c1=RcCell::new(Fqe_variable::<ppT>::new(*gamma_twist * (-P_X_const) + *(c.gamma_X) + *(Q.Y) * (if !invert_Q {-FieldT::one() }else {FieldT::one()})));
    }
    else
    {
        g_RQ_at_P_c1=RcCell::new(Fqe_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_RQ_at_Q_c1")));
        compute_g_RQ_at_P_c1=RcCell::new(Fqe_mul_by_lc_gadget::<ppT>::new(pb, *gamma_twist, prec_P.P.X,
                                                                 *(c.gamma_X) + *(Q.Y) * (if !invert_Q { -FieldT::one()} else {FieldT::one()}) + (*g_RQ_at_P_c1) * (-FieldT::one()),
                                                               FMT(annotation_prefix, " compute_g_RQ_at_P_c1")));
    }
    g_RQ_at_P=RcCell::new(Fqk_variable::<ppT>::new(pb, *(prec_P.PY_twist_squared), *g_RQ_at_P_c1, FMT(annotation_prefix, " g_RQ_at_P")));
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{invert_Q,prec_P,c,Q,g_RQ_at_P}
}


pub fn generate_r1cs_constraints()
{
    if !gamma_twist.is_constant() && !prec_P.P.X.is_constant()
    {
        compute_g_RQ_at_P_c1.generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    gamma_twist.evaluate();
    let gamma_twist_val= gamma_twist.get_element();
    let PX_val= self.pb.lc_val(prec_P.P.X);
    let gamma_X_val= c.gamma_X.get_element();
    let QY_val= Q.Y.get_element();
    let g_RQ_at_P_c1_val= -PX_val * gamma_twist_val + gamma_X_val +  (if !invert_Q {-QY_val} else{QY_val});
    g_RQ_at_P_c1.generate_r1cs_witness(g_RQ_at_P_c1_val);

    if !gamma_twist.is_constant() && !prec_P.P.X.is_constant()
    {
        compute_g_RQ_at_P_c1.generate_r1cs_witness();
    }
    g_RQ_at_P.evaluate();
}
}

impl mnt_miller_loop_gadget::<ppT> {

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                    prec_P:&G1_precomputation::<ppT>,
                                                    prec_Q:&G2_precomputation::<ppT>,
                                                    result:&Fqk_variable::<ppT>,
                                                    annotation_prefix:&String)->Self
    
{
    let loop_count= pairing_selector::<ppT>::pairing_loop_count;

    f_count = add_count = dbl_count = 0;

    let mut  found_nonzero = false;
    let mut  NAF = find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        dbl_count+=1;
        f_count += 2;

        if NAF[i] != 0
        {
            add_count+=1;
            f_count += 1;
        }
    }

    fs.resize(f_count);
    doubling_steps.resize(dbl_count);
    addition_steps.resize(add_count);
    g_RR_at_Ps.resize(dbl_count);
    g_RQ_at_Ps.resize(add_count);

    for i in 0..f_count
    {
        fs[i]=RcCell::new(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " fs_{}", i)));
    }

    dbl_sqrs.resize(dbl_count);
    dbl_muls.resize(dbl_count);
    add_muls.resize(add_count);

    let mut  add_id = 0;
    let mut  dbl_id = 0;
    let mut  f_id = 0;
    let mut  prec_id = 0;

    found_nonzero = false;
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps[dbl_id]=RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(pb,
                                                                            prec_P, *prec_Q.coeffs[prec_id],
                                                                            g_RR_at_Ps[dbl_id],
                                                                          FMT(annotation_prefix, " doubling_steps_{}", dbl_id)));
        prec_id+=1;
        dbl_sqrs[dbl_id]=RcCell::new(Fqk_sqr_gadget::<ppT>::new(pb, *fs[f_id], *fs[f_id+1], FMT(annotation_prefix, " dbl_sqrs_{}", dbl_id)));
        f_id+=1;
        dbl_muls[dbl_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RR_at_Ps[dbl_id],  (if f_id + 1 == f_count {result} else{*fs[f_id+1]}), FMT(annotation_prefix, " dbl_muls_{}", dbl_id)));
        f_id+=1;
        dbl_id+=1;

        if NAF[i] != 0
        {
            addition_steps[add_id]=RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(pb,
                                                                                NAF[i] < 0,
                                                                                prec_P, *prec_Q.coeffs[prec_id], *prec_Q.Q,
                                                                                g_RQ_at_Ps[add_id],
                                                                              FMT(annotation_prefix, " addition_steps_{}", add_id)));
            prec_id+=1;
            add_muls[add_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RQ_at_Ps[add_id],  (if f_id + 1 == f_count {result} else{*fs[f_id+1]}), FMT(annotation_prefix, " add_muls_{}", add_id)));
            f_id+=1;
            add_id+=1;
        }
    }
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{prec_P,prec_Q,result}
}


pub fn generate_r1cs_constraints()
{
    fs[0].generate_r1cs_equals_const_constraints(FqkT::one());

    for i in 0..dbl_count
    {
        doubling_steps[i].generate_r1cs_constraints();
        dbl_sqrs[i].generate_r1cs_constraints();
        dbl_muls[i].generate_r1cs_constraints();
    }

    for i in 0..add_count
    {
        addition_steps[i].generate_r1cs_constraints();
        add_muls[i].generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    fs[0].generate_r1cs_witness(FqkT::one());

    let mut  add_id = 0;
    let mut  dbl_id = 0;

    let loop_count = pairing_selector::<ppT>::pairing_loop_count;

    let mut  found_nonzero = false;
    let mut  NAF = find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps[dbl_id].generate_r1cs_witness();
        dbl_sqrs[dbl_id].generate_r1cs_witness();
        dbl_muls[dbl_id].generate_r1cs_witness();
        dbl_id+=1;

        if NAF[i] != 0
        {
            addition_steps[add_id].generate_r1cs_witness();
            add_muls[add_id].generate_r1cs_witness();
            add_id+=1;
        }
    }
}

}

pub fn  test_mnt_miller_loop(annotation:&String)
{
    let mut  pb=protoboard::<ppT::Fr >::new();
    let mut  P_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
    let mut  Q_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

   let mut  P= G1_variable::<ppT>::new(pb, "P");
    let mut  Q=G2_variable::<ppT>::new(pb, "Q");

    let mut  prec_P=G1_precomputation::<ppT>::new();
    let mut  prec_Q=G2_precomputation::<ppT>::new();

     let mut compute_prec_P=precompute_G1_gadget::<ppT>::new(pb, P, prec_P, "prec_P");
     let mut compute_prec_Q=precompute_G2_gadget::<ppT>::new(pb, Q, prec_Q, "prec_Q");

     let mut result=Fqk_variable::<ppT>::new(pb, "result");
     let mut miller=mnt_miller_loop_gadget::<ppT>::new(pb, prec_P, prec_Q, result, "miller");

    PROFILE_CONSTRAINTS(&pb, "precompute P");
    {
        compute_prec_P.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "precompute Q");
    {
        compute_prec_Q.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "Miller loop");
    {
        miller.generate_r1cs_constraints();
    }
    PRINT_CONSTRAINT_PROFILING();

    P.generate_r1cs_witness(P_val);
    compute_prec_P.generate_r1cs_witness();
    Q.generate_r1cs_witness(Q_val);
    compute_prec_Q.generate_r1cs_witness();
    miller.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    let native_prec_P = other_curve::<ppT>::affine_ate_precompute_G1(P_val);
    let native_prec_Q = other_curve::<ppT>::affine_ate_precompute_G2(Q_val);
    let native_result = other_curve::<ppT>::affine_ate_miller_loop(native_prec_P, native_prec_Q);

    assert!(result.get_element() == native_result);
    print!("number of constraints for Miller loop (Fr is {})  = {}\n", annotation, pb.num_constraints());
}

impl mnt_e_over_e_miller_loop_gadget::<ppT>{
pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                      prec_P1:&G1_precomputation::<ppT>,
                                                                      prec_Q1:&G2_precomputation::<ppT>,
                                                                      prec_P2:&G1_precomputation::<ppT>,
                                                                      prec_Q2:&G2_precomputation::<ppT>,
                                                                      result:&Fqk_variable::<ppT>,
                                                                      annotation_prefix:&String)->Self

{
    let loop_count = pairing_selector::<ppT>::pairing_loop_count;

    let (f_count , add_count , dbl_count) = (0,0,0);

    let mut  found_nonzero = false;
    let mut  NAF = find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        dbl_count+=1;
        f_count += 3;

        if NAF[i] != 0
        {
            add_count+=1;
            f_count += 2;
        }
    }

    fs.resize(f_count);
    doubling_steps1.resize(dbl_count);
    addition_steps1.resize(add_count);
    doubling_steps2.resize(dbl_count);
    addition_steps2.resize(add_count);
    g_RR_at_P1s.resize(dbl_count);
    g_RQ_at_P1s.resize(add_count);
    g_RR_at_P2s.resize(dbl_count);
    g_RQ_at_P2s.resize(add_count);

    for i in 0..f_count
    {
        fs[i]=RcCell::new(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " fs_{}", i)));
    }

    dbl_sqrs.resize(dbl_count);
    dbl_muls1.resize(dbl_count);
    add_muls1.resize(add_count);
    dbl_muls2.resize(dbl_count);
    add_muls2.resize(add_count);

    let mut  add_id = 0;
    let mut  dbl_id = 0;
    let mut  f_id = 0;
    let mut  prec_id = 0;

    found_nonzero = false;
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps1[dbl_id]=RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(pb,
                                                                             prec_P1, *prec_Q1.coeffs[prec_id],
                                                                             g_RR_at_P1s[dbl_id],
                                                                           FMT(annotation_prefix, " doubling_steps1_{}", dbl_id)));
        doubling_steps2[dbl_id]=RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(pb,
                                                                             prec_P2, *prec_Q2.coeffs[prec_id],
                                                                             g_RR_at_P2s[dbl_id],
                                                                           FMT(annotation_prefix, " doubling_steps2_{}", dbl_id)));
        prec_id+=1;

        dbl_sqrs[dbl_id]=RcCell::new(Fqk_sqr_gadget::<ppT>::new(pb, *fs[f_id], *fs[f_id+1], FMT(annotation_prefix, " dbl_sqrs_{}", dbl_id)));
        f_id+=1;
        dbl_muls1[dbl_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RR_at_P1s[dbl_id], *fs[f_id+1], FMT(annotation_prefix, " dbl_mul1s_{}", dbl_id)));
        f_id+=1;
        dbl_muls2[dbl_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb,  (if f_id + 1 == f_count {result} else{*fs[f_id+1]}), *g_RR_at_P2s[dbl_id], *fs[f_id], FMT(annotation_prefix, " dbl_mul2s_{}", dbl_id)));
        f_id+=1;
        dbl_id+=1;

        if NAF[i] != 0
        {
            addition_steps1[add_id]=RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(pb,
                                                                                 NAF[i] < 0,
                                                                                 prec_P1, *prec_Q1.coeffs[prec_id], *prec_Q1.Q,
                                                                                 g_RQ_at_P1s[add_id],
                                                                               FMT(annotation_prefix, " addition_steps1_{}", add_id)));
            addition_steps2[add_id]=RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(pb,
                                                                                 NAF[i] < 0,
                                                                                 prec_P2, *prec_Q2.coeffs[prec_id], *prec_Q2.Q,
                                                                                 g_RQ_at_P2s[add_id],
                                                                               FMT(annotation_prefix, " addition_steps2_{}", add_id)));
            prec_id+=1;
            add_muls1[add_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RQ_at_P1s[add_id], *fs[f_id+1], FMT(annotation_prefix, " add_mul1s_{}", add_id)));
            f_id+=1;
            add_muls2[add_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb,  (if f_id + 1 == f_count {result} else{*fs[f_id+1]}), *g_RQ_at_P2s[add_id], *fs[f_id], FMT(annotation_prefix, " add_mul2s_{}", add_id)));
            f_id+=1;
            add_id+=1;
        }
    }
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{prec_P1,prec_Q1,prec_P2,prec_Q2,result}
}


pub fn generate_r1cs_constraints()
{
    fs[0].generate_r1cs_equals_const_constraints(FqkT::one());

    for i in 0..dbl_count
    {
        doubling_steps1[i].generate_r1cs_constraints();
        doubling_steps2[i].generate_r1cs_constraints();
        dbl_sqrs[i].generate_r1cs_constraints();
        dbl_muls1[i].generate_r1cs_constraints();
        dbl_muls2[i].generate_r1cs_constraints();
    }

    for i in 0..add_count
    {
        addition_steps1[i].generate_r1cs_constraints();
        addition_steps2[i].generate_r1cs_constraints();
        add_muls1[i].generate_r1cs_constraints();
        add_muls2[i].generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    fs[0].generate_r1cs_witness(FqkT::one());

    let mut  add_id = 0;
    let mut  dbl_id = 0;
    let mut  f_id = 0;

    let loop_count = pairing_selector::<ppT>::pairing_loop_count;

    let mut  found_nonzero = false;
    let mut  NAF = find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps1[dbl_id].generate_r1cs_witness();
        doubling_steps2[dbl_id].generate_r1cs_witness();
        dbl_sqrs[dbl_id].generate_r1cs_witness();
        f_id+=1;
        dbl_muls1[dbl_id].generate_r1cs_witness();
        f_id+=1;
         (if f_id+1 == f_count {result} else{*fs[f_id+1]}).generate_r1cs_witness(fs[f_id].get_element() * g_RR_at_P2s[dbl_id].get_element().inverse());
        dbl_muls2[dbl_id].generate_r1cs_witness();
        f_id+=1;
        dbl_id+=1;

        if NAF[i] != 0
        {
            addition_steps1[add_id].generate_r1cs_witness();
            addition_steps2[add_id].generate_r1cs_witness();
            add_muls1[add_id].generate_r1cs_witness();
            f_id+=1;
             (if f_id+1 == f_count {result} else{*fs[f_id+1]}).generate_r1cs_witness(fs[f_id].get_element() * g_RQ_at_P2s[add_id].get_element().inverse());
            add_muls2[add_id].generate_r1cs_witness();
            f_id+=1;
            add_id+=1;
        }
    }
}
}

pub fn  test_mnt_e_over_e_miller_loop(annotation:&String)
{
    let mut pb=protoboard::<ppT::Fr >::new();
    let mut  P1_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
    let mut  Q1_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

    let mut  P2_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
    let mut  Q2_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

let mut P1=     G1_variable::<ppT>::new(pb, "P1");
let mut Q1=     G2_variable::<ppT>::new(pb, "Q1");
let mut P2=     G1_variable::<ppT>::new(pb, "P2");
let mut Q2=     G2_variable::<ppT>::new(pb, "Q2");

let mut prec_P1=     G1_precomputation::<ppT>::new();
let mut compute_prec_P1=     precompute_G1_gadget::<ppT>::new(pb, P1, prec_P1, "compute_prec_P1");
let mut prec_P2=     G1_precomputation::<ppT>::new();
let mut compute_prec_P2=     precompute_G1_gadget::<ppT>::new(pb, P2, prec_P2, "compute_prec_P2");
let mut prec_Q1=     G2_precomputation::<ppT>::new();
let mut compute_prec_Q1=     precompute_G2_gadget::<ppT>::new(pb, Q1, prec_Q1, "compute_prec_Q1");
let mut prec_Q2=     G2_precomputation::<ppT>::new();
let mut compute_prec_Q2=     precompute_G2_gadget::<ppT>::new(pb, Q2, prec_Q2, "compute_prec_Q2");

let mut result=     Fqk_variable::<ppT>::new(pb, "result");
let mut miller=     mnt_e_over_e_miller_loop_gadget::<ppT>::new(pb, prec_P1, prec_Q1, prec_P2, prec_Q2, result, "miller");

    PROFILE_CONSTRAINTS(&pb, "precompute P");
    {
        compute_prec_P1.generate_r1cs_constraints();
        compute_prec_P2.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "precompute Q");
    {
        compute_prec_Q1.generate_r1cs_constraints();
        compute_prec_Q2.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "Miller loop");
    {
        miller.generate_r1cs_constraints();
    }
    PRINT_CONSTRAINT_PROFILING();

    P1.generate_r1cs_witness(P1_val);
    compute_prec_P1.generate_r1cs_witness();
    Q1.generate_r1cs_witness(Q1_val);
    compute_prec_Q1.generate_r1cs_witness();
    P2.generate_r1cs_witness(P2_val);
    compute_prec_P2.generate_r1cs_witness();
    Q2.generate_r1cs_witness(Q2_val);
    compute_prec_Q2.generate_r1cs_witness();
    miller.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    let mut native_prec_P1 = other_curve::<ppT>::affine_ate_precompute_G1(P1_val);
    let mut  native_prec_Q1 = other_curve::<ppT>::affine_ate_precompute_G2(Q1_val);
    let mut  native_prec_P2 = other_curve::<ppT>::affine_ate_precompute_G1(P2_val);
    let mut  native_prec_Q2 = other_curve::<ppT>::affine_ate_precompute_G2(Q2_val);
    let mut  native_result = (other_curve::<ppT>::affine_ate_miller_loop(native_prec_P1, native_prec_Q1) *
                                            other_curve::<ppT>::affine_ate_miller_loop(native_prec_P2, native_prec_Q2).inverse());

    assert!(result.get_element() == native_result);
    print!("number of constraints for e over e Miller loop (Fr is {})  = {}\n", annotation, pb.num_constraints());
}
impl mnt_e_times_e_over_e_miller_loop_gadget::<ppT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                                      prec_P1:&G1_precomputation::<ppT>,
                                                                                      prec_Q1:&G2_precomputation::<ppT>,
                                                                                      prec_P2:&G1_precomputation::<ppT>,
                                                                                      prec_Q2:&G2_precomputation::<ppT>,
                                                                                      prec_P3:&G1_precomputation::<ppT>,
                                                                                      prec_Q3:&G2_precomputation::<ppT>,
                                                                                      result:&Fqk_variable::<ppT>,
                                                                                      annotation_prefix:&String)->Self

{
    let loop_count = pairing_selector::<ppT>::pairing_loop_count;

    f_count = add_count = dbl_count = 0;

    let mut  found_nonzero = false;
    let mut  NAF = find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        dbl_count+=1;
        f_count += 4;

        if NAF[i] != 0
        {
            add_count+=1;
            f_count += 3;
        }
    }

    fs.resize(f_count);
    doubling_steps1.resize(dbl_count);
    addition_steps1.resize(add_count);
    doubling_steps2.resize(dbl_count);
    addition_steps2.resize(add_count);
    doubling_steps3.resize(dbl_count);
    addition_steps3.resize(add_count);
    g_RR_at_P1s.resize(dbl_count);
    g_RQ_at_P1s.resize(add_count);
    g_RR_at_P2s.resize(dbl_count);
    g_RQ_at_P2s.resize(add_count);
    g_RR_at_P3s.resize(dbl_count);
    g_RQ_at_P3s.resize(add_count);

    for i in 0..f_count
    {
        fs[i]=RcCell::new(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " fs_{}", i)));
    }

    dbl_sqrs.resize(dbl_count);
    dbl_muls1.resize(dbl_count);
    add_muls1.resize(add_count);
    dbl_muls2.resize(dbl_count);
    add_muls2.resize(add_count);
    dbl_muls3.resize(dbl_count);
    add_muls3.resize(add_count);

    let mut  add_id = 0;
    let mut  dbl_id = 0;
    let mut  f_id = 0;
    let mut  prec_id = 0;

    found_nonzero = false;
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps1[dbl_id]=RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(pb,
                                                                             prec_P1, *prec_Q1.coeffs[prec_id],
                                                                             g_RR_at_P1s[dbl_id],
                                                                           FMT(annotation_prefix, " doubling_steps1_{}", dbl_id)));
        doubling_steps2[dbl_id]=RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(pb,
                                                                             prec_P2, *prec_Q2.coeffs[prec_id],
                                                                             g_RR_at_P2s[dbl_id],
                                                                           FMT(annotation_prefix, " doubling_steps2_{}", dbl_id)));
        doubling_steps3[dbl_id]=RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(pb,
                                                                             prec_P3, *prec_Q3.coeffs[prec_id],
                                                                             g_RR_at_P3s[dbl_id],
                                                                           FMT(annotation_prefix, " doubling_steps3_{}", dbl_id)));
        prec_id+=1;

        dbl_sqrs[dbl_id]=RcCell::new(Fqk_sqr_gadget::<ppT>::new(pb, *fs[f_id], *fs[f_id+1], FMT(annotation_prefix, " dbl_sqrs_{}", dbl_id)));
        f_id+=1;
        dbl_muls1[dbl_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RR_at_P1s[dbl_id], *fs[f_id+1], FMT(annotation_prefix, " dbl_muls1_{}", dbl_id)));
        f_id+=1;
        dbl_muls2[dbl_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RR_at_P2s[dbl_id], *fs[f_id+1], FMT(annotation_prefix, " dbl_muls2_{}", dbl_id)));
        f_id+=1;
        dbl_muls3[dbl_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb,  (if f_id + 1 == f_count {result} else{*fs[f_id+1]}), *g_RR_at_P3s[dbl_id], *fs[f_id], FMT(annotation_prefix, " dbl_muls3_{}", dbl_id)));
        f_id+=1;
        dbl_id+=1;

        if NAF[i] != 0
        {
            addition_steps1[add_id]=RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(pb,
                                                                                 NAF[i] < 0,
                                                                                 prec_P1, *prec_Q1.coeffs[prec_id], *prec_Q1.Q,
                                                                                 g_RQ_at_P1s[add_id],
                                                                               FMT(annotation_prefix, " addition_steps1_{}", add_id)));
            addition_steps2[add_id]=RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(pb,
                                                                                 NAF[i] < 0,
                                                                                 prec_P2, *prec_Q2.coeffs[prec_id], *prec_Q2.Q,
                                                                                 g_RQ_at_P2s[add_id],
                                                                               FMT(annotation_prefix, " addition_steps2_{}", add_id)));
            addition_steps3[add_id]=RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(pb,
                                                                                 NAF[i] < 0,
                                                                                 prec_P3, *prec_Q3.coeffs[prec_id], *prec_Q3.Q,
                                                                                 g_RQ_at_P3s[add_id],
                                                                               FMT(annotation_prefix, " addition_steps3_{}", add_id)));
            prec_id+=1;
            add_muls1[add_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RQ_at_P1s[add_id], *fs[f_id+1], FMT(annotation_prefix, " add_muls1_{}", add_id)));
            f_id+=1;
            add_muls2[add_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb, *fs[f_id], *g_RQ_at_P2s[add_id], *fs[f_id+1], FMT(annotation_prefix, " add_muls2_{}", add_id)));
            f_id+=1;
            add_muls3[add_id]=RcCell::new(Fqk_special_mul_gadget::<ppT>::new(pb,  (if f_id + 1 == f_count {result} else{*fs[f_id+1]}), *g_RQ_at_P3s[add_id], *fs[f_id], FMT(annotation_prefix, " add_muls3_{}", add_id)));
            f_id+=1;
            add_id+=1;
        }
    }
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{prec_P1,prec_Q1,prec_P2,prec_Q2,prec_P3,prec_Q3,result}
}


pub fn generate_r1cs_constraints()
{
    fs[0].generate_r1cs_equals_const_constraints(FqkT::one());

    for i in 0..dbl_count
    {
        doubling_steps1[i].generate_r1cs_constraints();
        doubling_steps2[i].generate_r1cs_constraints();
        doubling_steps3[i].generate_r1cs_constraints();
        dbl_sqrs[i].generate_r1cs_constraints();
        dbl_muls1[i].generate_r1cs_constraints();
        dbl_muls2[i].generate_r1cs_constraints();
        dbl_muls3[i].generate_r1cs_constraints();
    }

    for i in 0..add_count
    {
        addition_steps1[i].generate_r1cs_constraints();
        addition_steps2[i].generate_r1cs_constraints();
        addition_steps3[i].generate_r1cs_constraints();
        add_muls1[i].generate_r1cs_constraints();
        add_muls2[i].generate_r1cs_constraints();
        add_muls3[i].generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    fs[0].generate_r1cs_witness(FqkT::one());

    let mut  add_id = 0;
    let mut  dbl_id = 0;
    let mut  f_id = 0;

    let loop_count = pairing_selector::<ppT>::pairing_loop_count;

    let mut  found_nonzero = false;
    let mut  NAF = find_wnaf(1, loop_count);
    for i in ( 0..=NAF.len()-1).rev()
    {
        if !found_nonzero
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps1[dbl_id].generate_r1cs_witness();
        doubling_steps2[dbl_id].generate_r1cs_witness();
        doubling_steps3[dbl_id].generate_r1cs_witness();
        dbl_sqrs[dbl_id].generate_r1cs_witness();
        f_id+=1;
        dbl_muls1[dbl_id].generate_r1cs_witness();
        f_id+=1;
        dbl_muls2[dbl_id].generate_r1cs_witness();
        f_id+=1;
         (if f_id+1 == f_count {result} else{*fs[f_id+1]}).generate_r1cs_witness(fs[f_id].get_element() * g_RR_at_P3s[dbl_id].get_element().inverse());
        dbl_muls3[dbl_id].generate_r1cs_witness();
        f_id+=1;
        dbl_id+=1;

        if NAF[i] != 0
        {
            addition_steps1[add_id].generate_r1cs_witness();
            addition_steps2[add_id].generate_r1cs_witness();
            addition_steps3[add_id].generate_r1cs_witness();
            add_muls1[add_id].generate_r1cs_witness();
            f_id+=1;
            add_muls2[add_id].generate_r1cs_witness();
            f_id+=1;
             (if f_id+1 == f_count {result} else{*fs[f_id+1]}).generate_r1cs_witness(fs[f_id].get_element() * g_RQ_at_P3s[add_id].get_element().inverse());
            add_muls3[add_id].generate_r1cs_witness();
            f_id+=1;
            add_id+=1;
        }
    }
}

}
pub fn  test_mnt_e_times_e_over_e_miller_loop(annotation:&String)
{
    let mut pb=protoboard::<ppT::Fr >::new();
let mut P1_val=  ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
let mut Q1_val=  ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

let mut P2_val=  ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
let mut Q2_val=  ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

let mut P3_val=  ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
let mut Q3_val=  ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

let mut P1=     G1_variable::<ppT>::new(pb, "P1");
let mut Q1=     G2_variable::<ppT>::new(pb, "Q1");
let mut P2=     G1_variable::<ppT>::new(pb, "P2");
let mut Q2=     G2_variable::<ppT>::new(pb, "Q2");
let mut P3=     G1_variable::<ppT>::new(pb, "P3");
let mut Q3=     G2_variable::<ppT>::new(pb, "Q3");

let mut prec_P1=     G1_precomputation::<ppT>::new();
let mut compute_prec_P1=     precompute_G1_gadget::<ppT>::new(pb, P1, prec_P1, "compute_prec_P1");
let mut prec_P2=     G1_precomputation::<ppT>::new();
let mut compute_prec_P2=     precompute_G1_gadget::<ppT>::new(pb, P2, prec_P2, "compute_prec_P2");
let mut prec_P3=     G1_precomputation::<ppT>::new();
let mut compute_prec_P3=     precompute_G1_gadget::<ppT>::new(pb, P3, prec_P3, "compute_prec_P3");
let mut prec_Q1=     G2_precomputation::<ppT>::new();
let mut compute_prec_Q1=     precompute_G2_gadget::<ppT>::new(pb, Q1, prec_Q1, "compute_prec_Q1");
let mut prec_Q2=     G2_precomputation::<ppT>::new();
let mut compute_prec_Q2=     precompute_G2_gadget::<ppT>::new(pb, Q2, prec_Q2, "compute_prec_Q2");
let mut prec_Q3=     G2_precomputation::<ppT>::new();
let mut compute_prec_Q3=     precompute_G2_gadget::<ppT>::new(pb, Q3, prec_Q3, "compute_prec_Q3");

    let mut result=Fqk_variable::<ppT> ::new(pb, "result");
    let mut  miller=mnt_e_times_e_over_e_miller_loop_gadget::<ppT>::new(pb, prec_P1, prec_Q1, prec_P2, prec_Q2, prec_P3, prec_Q3, result, "miller");

    PROFILE_CONSTRAINTS(&pb, "precompute P");
    {
        compute_prec_P1.generate_r1cs_constraints();
        compute_prec_P2.generate_r1cs_constraints();
        compute_prec_P3.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "precompute Q");
    {
        compute_prec_Q1.generate_r1cs_constraints();
        compute_prec_Q2.generate_r1cs_constraints();
        compute_prec_Q3.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "Miller loop");
    {
        miller.generate_r1cs_constraints();
    }
    PRINT_CONSTRAINT_PROFILING();

    P1.generate_r1cs_witness(P1_val);
    compute_prec_P1.generate_r1cs_witness();
    Q1.generate_r1cs_witness(Q1_val);
    compute_prec_Q1.generate_r1cs_witness();
    P2.generate_r1cs_witness(P2_val);
    compute_prec_P2.generate_r1cs_witness();
    Q2.generate_r1cs_witness(Q2_val);
    compute_prec_Q2.generate_r1cs_witness();
    P3.generate_r1cs_witness(P3_val);
    compute_prec_P3.generate_r1cs_witness();
    Q3.generate_r1cs_witness(Q3_val);
    compute_prec_Q3.generate_r1cs_witness();
    miller.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    let mut  native_prec_P1 = other_curve::<ppT>::affine_ate_precompute_G1(P1_val);
    let mut  native_prec_Q1 = other_curve::<ppT>::affine_ate_precompute_G2(Q1_val);
    let mut  native_prec_P2 = other_curve::<ppT>::affine_ate_precompute_G1(P2_val);
    let mut  native_prec_Q2 = other_curve::<ppT>::affine_ate_precompute_G2(Q2_val);
    let mut  native_prec_P3 = other_curve::<ppT>::affine_ate_precompute_G1(P3_val);
    let mut  native_prec_Q3 = other_curve::<ppT>::affine_ate_precompute_G2(Q3_val);
    let mut  native_result = (other_curve::<ppT>::affine_ate_miller_loop(native_prec_P1, native_prec_Q1) *
                                            other_curve::<ppT>::affine_ate_miller_loop(native_prec_P2, native_prec_Q2) *
                                            other_curve::<ppT>::affine_ate_miller_loop(native_prec_P3, native_prec_Q3).inverse());

    assert!(result.get_element() == native_result);
    print!("number of constraints for e times e over e Miller loop (Fr is {})  = {}\n", annotation, pb.num_constraints());
}



//#endif // WEIERSTRASS_MILLER_LOOP_TCC_
