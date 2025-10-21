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

use  <memory>

use crate::gadgetlib1::gadgets/fields/exponentiation_gadget;
use crate::gadgetlib1::gadgets/pairing/mnt_pairing_params;



/**
 * Gadget for final exponentiation with embedding degree 4.
 */
template<typename ppT>
class mnt4_final_exp_gadget : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fr<ppT> FieldT;

    Fqk_variable<ppT> el;
    std::shared_ptr<Fqk_variable<ppT> > one;
    std::shared_ptr<Fqk_variable<ppT> > el_inv;
    std::shared_ptr<Fqk_variable<ppT> > el_q_3;
    std::shared_ptr<Fqk_variable<ppT> > el_q_3_minus_1;
    std::shared_ptr<Fqk_variable<ppT> > alpha;
    std::shared_ptr<Fqk_variable<ppT> > beta;
    std::shared_ptr<Fqk_variable<ppT> > beta_q;
    std::shared_ptr<Fqk_variable<ppT> > el_inv_q_3;
    std::shared_ptr<Fqk_variable<ppT> > el_inv_q_3_minus_1;
    std::shared_ptr<Fqk_variable<ppT> > inv_alpha;
    std::shared_ptr<Fqk_variable<ppT> > inv_beta;
    std::shared_ptr<Fqk_variable<ppT> > w1;
    std::shared_ptr<Fqk_variable<ppT> > w0;
    std::shared_ptr<Fqk_variable<ppT> > result;

    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_el_inv;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_el_q_3_minus_1;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_beta;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_el_inv_q_3_minus_1;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_inv_beta;

    std::shared_ptr<exponentiation_gadget<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs> > compute_w1;
    std::shared_ptr<exponentiation_gadget<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs> > compute_w0;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_result;

    pb_variable<FieldT> result_is_one;

    mnt4_final_exp_gadget(protoboard<FieldT> &pb,
                          const Fqk_variable<ppT> &el,
                          const pb_variable<FieldT> &result_is_one,
                          const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Gadget for final exponentiation with embedding degree 6.
 */
template<typename ppT>
class mnt6_final_exp_gadget : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fr<ppT> FieldT;

    Fqk_variable<ppT> el;
    std::shared_ptr<Fqk_variable<ppT> > one;
    std::shared_ptr<Fqk_variable<ppT> > el_inv;
    std::shared_ptr<Fqk_variable<ppT> > el_q_2;
    std::shared_ptr<Fqk_variable<ppT> > el_q_2_minus_1;
    std::shared_ptr<Fqk_variable<ppT> > el_q_3_minus_q;
    std::shared_ptr<Fqk_variable<ppT> > el_inv_q_2;
    std::shared_ptr<Fqk_variable<ppT> > el_inv_q_2_minus_1;
    std::shared_ptr<Fqk_variable<ppT> > w1;
    std::shared_ptr<Fqk_variable<ppT> > w0;
    std::shared_ptr<Fqk_variable<ppT> > result;

    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_el_inv;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_el_q_2_minus_1;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_el_inv_q_2_minus_1;

    std::shared_ptr<exponentiation_gadget<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs> > compute_w1;
    std::shared_ptr<exponentiation_gadget<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs> > compute_w0;
    std::shared_ptr<Fqk_mul_gadget<ppT> > compute_result;

    pb_variable<FieldT> result_is_one;

    mnt6_final_exp_gadget(protoboard<FieldT> &pb,
                          const Fqk_variable<ppT> &el,
                          const pb_variable<FieldT> &result_is_one,
                          const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use crate::gadgetlib1::gadgets/pairing/weierstrass_final_exponentiation;

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
use crate::gadgetlib1::gadgets/pairing/mnt_pairing_params;



template<typename ppT>
mnt4_final_exp_gadget<ppT>::mnt4_final_exp_gadget(protoboard<FieldT> &pb,
                                                  const Fqk_variable<ppT> &el,
                                                  const pb_variable<FieldT> &result_is_one,
                                                  const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    el(el),
    result_is_one(result_is_one)
{
    one.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " one")));
    el_inv.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " el_inv")));
    el_q_3.reset(new Fqk_variable<ppT>(el.Frobenius_map(3)));
    el_q_3_minus_1.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " el_q_3_minus_1")));
    alpha.reset(new Fqk_variable<ppT>(el_q_3_minus_1->Frobenius_map(1)));
    beta.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " beta")));
    beta_q.reset(new Fqk_variable<ppT>(beta->Frobenius_map(1)));

    el_inv_q_3.reset(new Fqk_variable<ppT>(el_inv->Frobenius_map(3)));
    el_inv_q_3_minus_1.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " el_inv_q_3_minus_1")));
    inv_alpha.reset(new Fqk_variable<ppT>(el_inv_q_3_minus_1->Frobenius_map(1)));
    inv_beta.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " inv_beta")));
    w1.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " w1")));
    w0.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " w0")));
    result.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " result")));

    compute_el_inv.reset(new Fqk_mul_gadget<ppT>(pb, el, *el_inv, *one, FMT(annotation_prefix, " compute_el_inv")));
    compute_el_q_3_minus_1.reset(new Fqk_mul_gadget<ppT>(pb, *el_q_3, *el_inv, *el_q_3_minus_1, FMT(annotation_prefix, " compute_el_q_3_minus_1")));
    compute_beta.reset(new Fqk_mul_gadget<ppT>(pb, *alpha, *el_q_3_minus_1, *beta, FMT(annotation_prefix, " compute_beta")));

    compute_el_inv_q_3_minus_1.reset(new Fqk_mul_gadget<ppT>(pb, *el_inv_q_3, el, *el_inv_q_3_minus_1, FMT(annotation_prefix, " compute_el_inv__q_3_minus_1")));
    compute_inv_beta.reset(new Fqk_mul_gadget<ppT>(pb, *inv_alpha, *el_inv_q_3_minus_1, *inv_beta, FMT(annotation_prefix, " compute_inv_beta")));

    compute_w1.reset(new exponentiation_gadget<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs>(
        pb, *beta_q, ffec::mnt6_final_exponent_last_chunk_w1, *w1, FMT(annotation_prefix, " compute_w1")));

    compute_w0.reset(new exponentiation_gadget<FqkT<ppT>, Fp6_variable, Fp6_mul_gadget, Fp6_cyclotomic_sqr_gadget, ffec::mnt6_q_limbs>(
        pb,  (if ffec::mnt6_final_exponent_last_chunk_is_w0_neg {*inv_beta} else{*beta}), ffec::mnt6_final_exponent_last_chunk_abs_of_w0, *w0, FMT(annotation_prefix, " compute_w0")));

    compute_result.reset(new Fqk_mul_gadget<ppT>(pb, *w1, *w0, *result, FMT(annotation_prefix, " compute_result")));
}

template<typename ppT>
void mnt4_final_exp_gadget<ppT>::generate_r1cs_constraints()
{
    one->generate_r1cs_equals_const_constraints(ffec::Fqk<other_curve<ppT> >::one());

    compute_el_inv->generate_r1cs_constraints();
    compute_el_q_3_minus_1->generate_r1cs_constraints();
    compute_beta->generate_r1cs_constraints();

    compute_el_inv_q_3_minus_1->generate_r1cs_constraints();
    compute_inv_beta->generate_r1cs_constraints();

    compute_w0->generate_r1cs_constraints();
    compute_w1->generate_r1cs_constraints();
    compute_result->generate_r1cs_constraints();

    generate_boolean_r1cs_constraint<FieldT>(self.pb, result_is_one, FMT(self.annotation_prefix, " result_is_one"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, 1 - result->c0.c0, 0), " check c0.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c0.c1, 0), " check c0.c1");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c0.c2, 0), " check c0.c2");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c1.c0, 0), " check c1.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c1.c1, 0), " check c1.c1");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c1.c2, 0), " check c1.c2");
}

template<typename ppT>
void mnt4_final_exp_gadget<ppT>::generate_r1cs_witness()
{
    one->generate_r1cs_witness(ffec::Fqk<other_curve<ppT> >::one());
    el_inv->generate_r1cs_witness(el.get_element().inverse());

    compute_el_inv->generate_r1cs_witness();
    el_q_3->evaluate();
    compute_el_q_3_minus_1->generate_r1cs_witness();
    alpha->evaluate();
    compute_beta->generate_r1cs_witness();
    beta_q->evaluate();

    el_inv_q_3->evaluate();
    compute_el_inv_q_3_minus_1->generate_r1cs_witness();
    inv_alpha->evaluate();
    compute_inv_beta->generate_r1cs_witness();

    compute_w0->generate_r1cs_witness();
    compute_w1->generate_r1cs_witness();
    compute_result->generate_r1cs_witness();

    self.pb.val(result_is_one) = if result->get_element() == one->get_element() {FieldT::one()} else{FieldT::zero()};
}

template<typename ppT>
mnt6_final_exp_gadget<ppT>::mnt6_final_exp_gadget(protoboard<FieldT> &pb,
                                                  const Fqk_variable<ppT> &el,
                                                  const pb_variable<FieldT> &result_is_one,
                                                  const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    el(el),
    result_is_one(result_is_one)
{
    one.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " one")));
    el_inv.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " el_inv")));
    el_q_2.reset(new Fqk_variable<ppT>(el.Frobenius_map(2)));
    el_q_2_minus_1.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " el_q_2_minus_1")));
    el_q_3_minus_q.reset(new Fqk_variable<ppT>(el_q_2_minus_1->Frobenius_map(1)));
    el_inv_q_2.reset(new Fqk_variable<ppT>(el_inv->Frobenius_map(2)));
    el_inv_q_2_minus_1.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " el_inv_q_2_minus_1")));
    w1.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " w1")));
    w0.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " w0")));
    result.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " result")));

    compute_el_inv.reset(new Fqk_mul_gadget<ppT>(pb, el, *el_inv, *one, FMT(annotation_prefix, " compute_el_inv")));
    compute_el_q_2_minus_1.reset(new Fqk_mul_gadget<ppT>(pb, *el_q_2, *el_inv, *el_q_2_minus_1, FMT(annotation_prefix, " compute_el_q_2_minus_1")));
    compute_el_inv_q_2_minus_1.reset(new Fqk_mul_gadget<ppT>(pb, *el_inv_q_2, el, *el_inv_q_2_minus_1, FMT(annotation_prefix, " compute_el_inv_q_2_minus_1")));

    compute_w1.reset(new exponentiation_gadget<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs>(
        pb, *el_q_3_minus_q, ffec::mnt4_final_exponent_last_chunk_w1, *w1, FMT(annotation_prefix, " compute_w1")));
    compute_w0.reset(new exponentiation_gadget<FqkT<ppT>, Fp4_variable, Fp4_mul_gadget, Fp4_cyclotomic_sqr_gadget, ffec::mnt4_q_limbs>(
        pb,  (if ffec::mnt4_final_exponent_last_chunk_is_w0_neg {*el_inv_q_2_minus_1} else{*el_q_2_minus_1}), ffec::mnt4_final_exponent_last_chunk_abs_of_w0, *w0, FMT(annotation_prefix, " compute_w0")));
    compute_result.reset(new Fqk_mul_gadget<ppT>(pb, *w1, *w0, *result, FMT(annotation_prefix, " compute_result")));
}

template<typename ppT>
void mnt6_final_exp_gadget<ppT>::generate_r1cs_constraints()
{
    one->generate_r1cs_equals_const_constraints(ffec::Fqk<other_curve<ppT> >::one());

    compute_el_inv->generate_r1cs_constraints();
    compute_el_q_2_minus_1->generate_r1cs_constraints();
    compute_el_inv_q_2_minus_1->generate_r1cs_constraints();
    compute_w1->generate_r1cs_constraints();
    compute_w0->generate_r1cs_constraints();
    compute_result->generate_r1cs_constraints();

    generate_boolean_r1cs_constraint<FieldT>(self.pb, result_is_one, FMT(self.annotation_prefix, " result_is_one"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, 1 - result->c0.c0, 0), " check c0.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c0.c1, 0), " check c0.c1");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c1.c0, 0), " check c1.c0");
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(result_is_one, result->c1.c1, 0), " check c1.c0");
}

template<typename ppT>
void mnt6_final_exp_gadget<ppT>::generate_r1cs_witness()
{
    one->generate_r1cs_witness(ffec::Fqk<other_curve<ppT> >::one());
    el_inv->generate_r1cs_witness(el.get_element().inverse());

    compute_el_inv->generate_r1cs_witness();
    el_q_2->evaluate();
    compute_el_q_2_minus_1->generate_r1cs_witness();
    el_q_3_minus_q->evaluate();
    el_inv_q_2->evaluate();
    compute_el_inv_q_2_minus_1->generate_r1cs_witness();
    compute_w1->generate_r1cs_witness();
    compute_w0->generate_r1cs_witness();
    compute_result->generate_r1cs_witness();

    self.pb.val(result_is_one) = if result->get_element() == one->get_element() {FieldT::one()} else{FieldT::zero()};
}



//#endif // WEIERSTRASS_FINAL_EXPONENTIATION_TCC_
