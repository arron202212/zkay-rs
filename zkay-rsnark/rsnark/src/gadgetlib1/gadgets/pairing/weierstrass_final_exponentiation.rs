/** @file
 *****************************************************************************

 Declaration of interfaces for final exponentiation gadgets.

 The gadgets verify final exponentiation for Weiersrass curves with embedding
 degrees 4 and 6.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_FINAL_EXPONENTIATION_HPP_
// #define WEIERSTRASS_FINAL_EXPONENTIATION_HPP_



use crate::gadgetlib1::gadgets::fields::exponentiation_gadget;
use crate::gadgetlib1::gadgets::pairing::mnt_pairing_params;



/**
 * Gadget for final exponentiation with embedding degree 4.
 */
  type FieldT=ffec::Fr<ppT>;
pub struct mnt4_final_exp_gadget<ppT> {//gadget<ffec::Fr<ppT> >

  

el:    Fqk_variable<ppT>,
one:    RcCell<Fqk_variable<ppT> >,
el_inv:    RcCell<Fqk_variable<ppT> >,
el_q_3:    RcCell<Fqk_variable<ppT> >,
el_q_3_minus_1:    RcCell<Fqk_variable<ppT> >,
alpha:    RcCell<Fqk_variable<ppT> >,
beta:    RcCell<Fqk_variable<ppT> >,
beta_q:    RcCell<Fqk_variable<ppT> >,
el_inv_q_3:    RcCell<Fqk_variable<ppT> >,
el_inv_q_3_minus_1:    RcCell<Fqk_variable<ppT> >,
inv_alpha:    RcCell<Fqk_variable<ppT> >,
inv_beta:    RcCell<Fqk_variable<ppT> >,
w1:    RcCell<Fqk_variable<ppT> >,
w0:    RcCell<Fqk_variable<ppT> >,
result:    RcCell<Fqk_variable<ppT> >,

compute_el_inv:    RcCell<Fqk_mul_gadget<ppT> >,
compute_el_q_3_minus_1:    RcCell<Fqk_mul_gadget<ppT> >,
compute_beta:    RcCell<Fqk_mul_gadget<ppT> >,
compute_el_inv_q_3_minus_1:    RcCell<Fqk_mul_gadget<ppT> >,
compute_inv_beta:    RcCell<Fqk_mul_gadget<ppT> >,

compute_w1:    RcCell<exponentiation_gadget<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs> >,
compute_w0:    RcCell<exponentiation_gadget<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs> >,
compute_result:    RcCell<Fqk_mul_gadget<ppT> >,

result_is_one:    pb_variable<FieldT>,

   
}

/**
 * Gadget for final exponentiation with embedding degree 6.
 */

pub struct mnt6_final_exp_gadget<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;

el:    Fqk_variable<ppT>,
one:    RcCell<Fqk_variable<ppT> >,
el_inv:    RcCell<Fqk_variable<ppT> >,
el_q_2:    RcCell<Fqk_variable<ppT> >,
el_q_2_minus_1:    RcCell<Fqk_variable<ppT> >,
el_q_3_minus_q:    RcCell<Fqk_variable<ppT> >,
el_inv_q_2:    RcCell<Fqk_variable<ppT> >,
el_inv_q_2_minus_1:    RcCell<Fqk_variable<ppT> >,
w1:    RcCell<Fqk_variable<ppT> >,
w0:    RcCell<Fqk_variable<ppT> >,
result:    RcCell<Fqk_variable<ppT> >,

compute_el_inv:    RcCell<Fqk_mul_gadget<ppT> >,
compute_el_q_2_minus_1:    RcCell<Fqk_mul_gadget<ppT> >,
compute_el_inv_q_2_minus_1:    RcCell<Fqk_mul_gadget<ppT> >,

compute_w1:    RcCell<exponentiation_gadget<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs> >,
compute_w0:    RcCell<exponentiation_gadget<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs> >,
compute_result:    RcCell<Fqk_mul_gadget<ppT> >,

result_is_one:    pb_variable<FieldT>,

}



use crate::gadgetlib1::gadgets::pairing::weierstrass_final_exponentiation;

//#endif // WEIERSTRASS_FINAL_EXPONENTIATION_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for final exponentiation gadgets.

 See weierstrass_final_exponentiation.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_FINAL_EXPONENTIATION_TCC_
// #define WEIERSTRASS_FINAL_EXPONENTIATION_TCC_

use crate::gadgetlib1::gadgets::basic_gadgets;
// use crate::gadgetlib1::gadgets::pairing::mnt_pairing_params;


impl mnt4_final_exp_gadget<ppT> {

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                  el:&Fqk_variable<ppT>,
                                                  result_is_one:&pb_variable<FieldT>,
                                                  annotation_prefix:&String)->Self
    
{
    one.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " one")));
    el_inv.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " el_inv")));
    el_q_3.reset(Fqk_variable::<ppT>::new(el.Frobenius_map(3)));
    el_q_3_minus_1.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " el_q_3_minus_1")));
    alpha.reset(Fqk_variable::<ppT>::new(el_q_3_minus_1.Frobenius_map(1)));
    beta.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " beta")));
    beta_q.reset(Fqk_variable::<ppT>::new(beta.Frobenius_map(1)));

    el_inv_q_3.reset(Fqk_variable::<ppT>::new(el_inv.Frobenius_map(3)));
    el_inv_q_3_minus_1.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " el_inv_q_3_minus_1")));
    inv_alpha.reset(Fqk_variable::<ppT>::new(el_inv_q_3_minus_1.Frobenius_map(1)));
    inv_beta.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " inv_beta")));
    w1.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " w1")));
    w0.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " w0")));
    result.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " result")));

    compute_el_inv.reset(Fqk_mul_gadget::<ppT>::new(pb, el, *el_inv, *one, FMT(annotation_prefix, " compute_el_inv")));
    compute_el_q_3_minus_1.reset(Fqk_mul_gadget::<ppT>::new(pb, *el_q_3, *el_inv, *el_q_3_minus_1, FMT(annotation_prefix, " compute_el_q_3_minus_1")));
    compute_beta.reset(Fqk_mul_gadget::<ppT>::new(pb, *alpha, *el_q_3_minus_1, *beta, FMT(annotation_prefix, " compute_beta")));

    compute_el_inv_q_3_minus_1.reset(Fqk_mul_gadget::<ppT>::new(pb, *el_inv_q_3, el, *el_inv_q_3_minus_1, FMT(annotation_prefix, " compute_el_inv__q_3_minus_1")));
    compute_inv_beta.reset(Fqk_mul_gadget::<ppT>::new(pb, *inv_alpha, *el_inv_q_3_minus_1, *inv_beta, FMT(annotation_prefix, " compute_inv_beta")));

    compute_w1.reset( exponentiation_gadget::<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs>::new(
        pb, *beta_q, ffec::mnt6_final_exponent_last_chunk_w1, *w1, FMT(annotation_prefix, " compute_w1")));

    compute_w0.reset( exponentiation_gadget::<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs>::new(
        pb,  (if ffec::mnt6_final_exponent_last_chunk_is_w0_neg {*inv_beta} else{*beta}), ffec::mnt6_final_exponent_last_chunk_abs_of_w0, *w0, FMT(annotation_prefix, " compute_w0")));

    compute_result.reset(Fqk_mul_gadget::<ppT>::new(pb, *w1, *w0, *result, FMT(annotation_prefix, " compute_result")));
    // gadget<FieldT>(&pb, annotation_prefix),
  Self {el,
    result_is_one}
}


pub fn generate_r1cs_constraints()
{
    one.generate_r1cs_equals_const_constraints(ffec::Fqk::<other_curve::<ppT> >::one());

    compute_el_inv.generate_r1cs_constraints();
    compute_el_q_3_minus_1.generate_r1cs_constraints();
    compute_beta.generate_r1cs_constraints();

    compute_el_inv_q_3_minus_1.generate_r1cs_constraints();
    compute_inv_beta.generate_r1cs_constraints();

    compute_w0.generate_r1cs_constraints();
    compute_w1.generate_r1cs_constraints();
    compute_result.generate_r1cs_constraints();

    generate_boolean_r1cs_constraint::<FieldT>(self.pb, result_is_one, FMT(self.annotation_prefix, " result_is_one"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, 1 - result.c0.c0, 0), " check c0.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c0.c1, 0), " check c0.c1");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c0.c2, 0), " check c0.c2");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c1.c0, 0), " check c1.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c1.c1, 0), " check c1.c1");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c1.c2, 0), " check c1.c2");
}


pub fn generate_r1cs_witness()
{
    one.generate_r1cs_witness(ffec::Fqk::<other_curve::<ppT> >::one());
    el_inv.generate_r1cs_witness(el.get_element().inverse());

    compute_el_inv.generate_r1cs_witness();
    el_q_3.evaluate();
    compute_el_q_3_minus_1.generate_r1cs_witness();
    alpha.evaluate();
    compute_beta.generate_r1cs_witness();
    beta_q.evaluate();

    el_inv_q_3.evaluate();
    compute_el_inv_q_3_minus_1.generate_r1cs_witness();
    inv_alpha.evaluate();
    compute_inv_beta.generate_r1cs_witness();

    compute_w0.generate_r1cs_witness();
    compute_w1.generate_r1cs_witness();
    compute_result.generate_r1cs_witness();

    self.pb.val(result_is_one) = if result.get_element() == one.get_element() {FieldT::one()} else{FieldT::zero()};
}
}

impl mnt6_final_exp_gadget<ppT>{
pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                  el:&Fqk_variable<ppT>,
                                                  result_is_one:&pb_variable<FieldT>,
                                                  annotation_prefix:&String)->Self
   
{
    one.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " one")));
    el_inv.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " el_inv")));
    el_q_2.reset(Fqk_variable::<ppT>::new(el.Frobenius_map(2)));
    el_q_2_minus_1.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " el_q_2_minus_1")));
    el_q_3_minus_q.reset(Fqk_variable::<ppT>::new(el_q_2_minus_1.Frobenius_map(1)));
    el_inv_q_2.reset(Fqk_variable::<ppT>::new(el_inv.Frobenius_map(2)));
    el_inv_q_2_minus_1.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " el_inv_q_2_minus_1")));
    w1.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " w1")));
    w0.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " w0")));
    result.reset(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " result")));

    compute_el_inv.reset(Fqk_mul_gadget::<ppT>::new(pb, el, *el_inv, *one, FMT(annotation_prefix, " compute_el_inv")));
    compute_el_q_2_minus_1.reset(Fqk_mul_gadget::<ppT>::new(pb, *el_q_2, *el_inv, *el_q_2_minus_1, FMT(annotation_prefix, " compute_el_q_2_minus_1")));
    compute_el_inv_q_2_minus_1.reset(Fqk_mul_gadget::<ppT>::new(pb, *el_inv_q_2, el, *el_inv_q_2_minus_1, FMT(annotation_prefix, " compute_el_inv_q_2_minus_1")));

    compute_w1.reset( exponentiation_gadget::<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs>::new(
        pb, *el_q_3_minus_q, ffec::mnt4_final_exponent_last_chunk_w1, *w1, FMT(annotation_prefix, " compute_w1")));
    compute_w0.reset( exponentiation_gadget::<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs>::new(
        pb,  (if ffec::mnt4_final_exponent_last_chunk_is_w0_neg {*el_inv_q_2_minus_1} else{*el_q_2_minus_1}), ffec::mnt4_final_exponent_last_chunk_abs_of_w0, *w0, FMT(annotation_prefix, " compute_w0")));
    compute_result.reset(Fqk_mul_gadget::<ppT>::new(pb, *w1, *w0, *result, FMT(annotation_prefix, " compute_result")));
    //  gadget<FieldT>(&pb, annotation_prefix),
   Self{el,
    result_is_one}
}


pub fn generate_r1cs_constraints()
{
    one.generate_r1cs_equals_const_constraints(ffec::Fqk::<other_curve::<ppT> >::one());

    compute_el_inv.generate_r1cs_constraints();
    compute_el_q_2_minus_1.generate_r1cs_constraints();
    compute_el_inv_q_2_minus_1.generate_r1cs_constraints();
    compute_w1.generate_r1cs_constraints();
    compute_w0.generate_r1cs_constraints();
    compute_result.generate_r1cs_constraints();

    generate_boolean_r1cs_constraint::<FieldT>(self.pb, result_is_one, FMT(self.annotation_prefix, " result_is_one"));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, 1 - result.c0.c0, 0), " check c0.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c0.c1, 0), " check c0.c1");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c1.c0, 0), " check c1.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(result_is_one, result.c1.c1, 0), " check c1.c0");
}


pub fn generate_r1cs_witness()
{
    one.generate_r1cs_witness(ffec::Fqk::<other_curve::<ppT> >::one());
    el_inv.generate_r1cs_witness(el.get_element().inverse());

    compute_el_inv.generate_r1cs_witness();
    el_q_2.evaluate();
    compute_el_q_2_minus_1.generate_r1cs_witness();
    el_q_3_minus_q.evaluate();
    el_inv_q_2.evaluate();
    compute_el_inv_q_2_minus_1.generate_r1cs_witness();
    compute_w1.generate_r1cs_witness();
    compute_w0.generate_r1cs_witness();
    compute_result.generate_r1cs_witness();

    self.pb.val(result_is_one) = if result.get_element() == one.get_element() {FieldT::one()} else{FieldT::zero()};
}

}

//#endif // WEIERSTRASS_FINAL_EXPONENTIATION_TCC_
