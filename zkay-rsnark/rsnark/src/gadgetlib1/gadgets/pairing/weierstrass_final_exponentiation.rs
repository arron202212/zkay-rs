//  Declaration of interfaces for final exponentiation gadgets.

//  The gadgets verify final exponentiation for Weiersrass curves with embedding
//  degrees 4 and 6.
use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget::{G1_variable, G1_variables};
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget::{G2_variable, G2_variables};
use crate::gadgetlib1::gadgets::fields::exponentiation_gadget::{
    exponentiation_gadget, exponentiation_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqk_mul_gadget, Fqk_sqr_gadget, Fqk_variable, MulTConfig, VariableTConfig, pairing_selector,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation::{
    G1_precomputation, G1_precomputations, G2_precomputation, G2_precomputations,
    precompute_G1_gadget, precompute_G2_gadget, precompute_G2_gadget_coeffss,
};
// affine_ate_miller_loop, affine_ate_precompute_G1, affine_ate_precompute_G2, pairing_loop_count,
// use ff_curves::algebra::curves::mnt::mnt4::mnt4_init;
// use ff_curves::algebra::curves::mnt::mnt6::mnt6_init;
// use scalar_multiplication::wnaf::find_wnaf;
// use crate::gadgetlib1::gadgets::pairing::pairing_params::{Fqe_variable,Fqe_mul_gadget,Fqe_sqr_gadget};
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqe_mul_by_lc_gadget, Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, Fqk_special_mul_gadget,
    ppTConfig,
};

use crate::gadgetlib1::constraint_profiling::{PRINT_CONSTRAINT_PROFILING, PROFILE_CONSTRAINTS};
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::generate_boolean_r1cs_constraint;
use crate::gadgetlib1::gadgets::pairing::pairing_params::other_curve;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ff_curves::Fr;
use ff_curves::algebra::curves::public_params;
use ffec::FieldTConfig;
use ffec::field_utils::bigint::bigint;
use ffec::{One, Zero};
use rccell::RcCell;
use std::marker::PhantomData;
use std::ops::Add;
const mnt6_final_exponent_last_chunk_w1: bigint<4> = bigint::<4>::one();
const mnt4_final_exponent_last_chunk_w1: bigint<4> = bigint::<4>::one();
const mnt4_final_exponent_last_chunk_abs_of_w0: bigint<4> = bigint::<4>::one();
const mnt4_final_exponent_last_chunk_is_w0_neg: bigint<4> = bigint::<4>::one();
const mnt6_final_exponent_last_chunk_is_w0_neg: bigint<4> = bigint::<4>::one();
const mnt6_final_exponent_last_chunk_abs_of_w0: bigint<4> = bigint::<4>::one();

/**
 * Gadget for final exponentiation with embedding degree 4.
 */
type FieldT<ppT> = Fr<ppT>;
#[derive(Clone, Default)]
pub struct mnt4_final_exp_gadget<ppT: ppTConfig> {
    //gadget<Fr<ppT> >
    el: Fqk_variable<ppT>,
    one: RcCell<Fqk_variable<ppT>>,
    el_inv: RcCell<Fqk_variable<ppT>>,
    el_q_3: RcCell<Fqk_variable<ppT>>,
    el_q_3_minus_1: RcCell<Fqk_variable<ppT>>,
    alpha: RcCell<Fqk_variable<ppT>>,
    beta: RcCell<Fqk_variable<ppT>>,
    beta_q: RcCell<Fqk_variable<ppT>>,
    el_inv_q_3: RcCell<Fqk_variable<ppT>>,
    el_inv_q_3_minus_1: RcCell<Fqk_variable<ppT>>,
    inv_alpha: RcCell<Fqk_variable<ppT>>,
    inv_beta: RcCell<Fqk_variable<ppT>>,
    w1: RcCell<Fqk_variable<ppT>>,
    w0: RcCell<Fqk_variable<ppT>>,
    result: RcCell<Fqk_variable<ppT>>,

    compute_el_inv: RcCell<Fqk_mul_gadget<ppT>>,
    compute_el_q_3_minus_1: RcCell<Fqk_mul_gadget<ppT>>,
    compute_beta: RcCell<Fqk_mul_gadget<ppT>>,
    compute_el_inv_q_3_minus_1: RcCell<Fqk_mul_gadget<ppT>>,
    compute_inv_beta: RcCell<Fqk_mul_gadget<ppT>>,

    compute_w1: RcCell<exponentiation_gadgets<ppT>>,
    compute_w0: RcCell<exponentiation_gadgets<ppT>>,
    compute_result: RcCell<Fqk_mul_gadget<ppT>>,

    result_is_one: variable<ppT::FieldT, pb_variable>,
}

/**
 * Gadget for final exponentiation with embedding degree 6.
 */
#[derive(Clone, Default)]
pub struct mnt6_final_exp_gadget<ppT: ppTConfig> {
    //gadget<Fr<ppT> >

    // type FieldT=Fr<ppT>;
    el: Fqk_variable<ppT>,
    one: RcCell<Fqk_variable<ppT>>,
    el_inv: RcCell<Fqk_variable<ppT>>,
    el_q_2: RcCell<Fqk_variable<ppT>>,
    el_q_2_minus_1: RcCell<Fqk_variable<ppT>>,
    el_q_3_minus_q: RcCell<Fqk_variable<ppT>>,
    el_inv_q_2: RcCell<Fqk_variable<ppT>>,
    el_inv_q_2_minus_1: RcCell<Fqk_variable<ppT>>,
    w1: RcCell<Fqk_variable<ppT>>,
    w0: RcCell<Fqk_variable<ppT>>,
    result: RcCell<Fqk_variable<ppT>>,

    compute_el_inv: RcCell<Fqk_mul_gadget<ppT>>,
    compute_el_q_2_minus_1: RcCell<Fqk_mul_gadget<ppT>>,
    compute_el_inv_q_2_minus_1: RcCell<Fqk_mul_gadget<ppT>>,

    compute_w1: RcCell<exponentiation_gadgets<ppT>>,
    compute_w0: RcCell<exponentiation_gadgets<ppT>>,
    compute_result: RcCell<Fqk_mul_gadget<ppT>>,

    result_is_one: variable<ppT::FieldT, pb_variable>,
}

pub type mnt4_final_exp_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, mnt4_final_exp_gadget<ppT>>;

impl<ppT: ppTConfig> mnt4_final_exp_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        el: Fqk_variable<ppT>,
        result_is_one: variable<ppT::FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> mnt4_final_exp_gadgets<ppT> {
        let one = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " one"),
        ));
        let el_inv = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " el_inv"),
        ));
        let el_q_3 = RcCell::new(el.Frobenius_map(3));
        let el_q_3_minus_1 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " el_q_3_minus_1"),
        ));
        let alpha = RcCell::new(el_q_3_minus_1.borrow().Frobenius_map(1));
        let beta = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " beta"),
        ));
        let beta_q = RcCell::new(beta.borrow().Frobenius_map(1));

        let el_inv_q_3 = RcCell::new(el_inv.borrow().Frobenius_map(3));
        let el_inv_q_3_minus_1 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " el_inv_q_3_minus_1"),
        ));
        let inv_alpha = RcCell::new(el_inv_q_3_minus_1.borrow().Frobenius_map(1));
        let inv_beta = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " inv_beta"),
        ));
        let w1 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " w1"),
        ));
        let w0 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " w0"),
        ));
        let result = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " result"),
        ));

        let compute_el_inv = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            el.clone(),
            el_inv.borrow().clone(),
            one.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_el_inv"),
        ));
        let compute_el_q_3_minus_1 = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            el_q_3.borrow().clone(),
            el_inv.borrow().clone(),
            el_q_3_minus_1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_el_q_3_minus_1"),
        ));
        let compute_beta = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            alpha.borrow().clone(),
            el_q_3_minus_1.borrow().clone(),
            beta.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_beta"),
        ));

        let compute_el_inv_q_3_minus_1 = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            el_inv_q_3.borrow().clone(),
            el.clone(),
            el_inv_q_3_minus_1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_el_inv__q_3_minus_1"),
        ));
        let compute_inv_beta = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            inv_alpha.borrow().clone(),
            el_inv_q_3_minus_1.borrow().clone(),
            inv_beta.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_inv_beta"),
        ));

        let compute_w1 = RcCell::new(exponentiation_gadget::<ppT>::new(
            pb.clone(),
            beta_q.borrow().clone(),
            mnt6_final_exponent_last_chunk_w1.clone(),
            w1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_w1"),
        ));

        let compute_w0 = RcCell::new(exponentiation_gadget::<ppT>::new(
            pb.clone(),
            (if mnt6_final_exponent_last_chunk_is_w0_neg == bigint::<4>::default() {
                inv_beta.borrow().clone()
            } else {
                beta.borrow().clone()
            }),
            mnt6_final_exponent_last_chunk_abs_of_w0,
            w0.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_w0"),
        ));

        let compute_result = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            w1.borrow().clone(),
            w0.borrow().clone(),
            result.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_result"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                el,
                one,
                el_inv,
                el_q_3,
                el_q_3_minus_1,
                alpha,
                beta,
                beta_q,
                el_inv_q_3,
                el_inv_q_3_minus_1,
                inv_alpha,
                inv_beta,
                w1,
                w0,
                result,

                compute_el_inv,
                compute_el_q_3_minus_1,
                compute_beta,
                compute_el_inv_q_3_minus_1,
                compute_inv_beta,

                compute_w1,
                compute_w0,
                compute_result,

                result_is_one,
            },
        )
    }
}
impl<ppT: ppTConfig> mnt4_final_exp_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .one
            .borrow()
            .generate_r1cs_equals_const_constraints(&ppT::FieldT::one());

        self.t.compute_el_inv.borrow().generate_r1cs_constraints();
        self.t
            .compute_el_q_3_minus_1
            .borrow()
            .generate_r1cs_constraints();
        self.t.compute_beta.borrow().generate_r1cs_constraints();

        self.t
            .compute_el_inv_q_3_minus_1
            .borrow()
            .generate_r1cs_constraints();
        self.t.compute_inv_beta.borrow().generate_r1cs_constraints();

        self.t.compute_w0.borrow().generate_r1cs_constraints();
        self.t.compute_w1.borrow().generate_r1cs_constraints();
        self.t.compute_result.borrow().generate_r1cs_constraints();

        generate_boolean_r1cs_constraint::<ppT::FieldT, ppT::PB>(
            &self.pb,
            &(self.t.result_is_one.clone().into()),
            prefix_format!(self.annotation_prefix, " result_is_one"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                (-self.t.result.borrow().c0().c0().clone() + 1)
                    .to_field()
                    .into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c0.c0".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c0().c1().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c0.c1".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c0().c2().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c0.c2".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c1().c0().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c1.c0".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c1().c1().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c1.c1".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c1().c2().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c1.c2".to_owned(),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        self.t
            .one
            .borrow()
            .generate_r1cs_witness(&ppT::FieldT::one());
        self.t
            .el_inv
            .borrow()
            .generate_r1cs_witness(&self.t.el.get_element().inverse().to_field::<ppT::FieldT>());

        self.t.compute_el_inv.borrow().generate_r1cs_witness();
        self.t.el_q_3.borrow().evaluate();
        self.t
            .compute_el_q_3_minus_1
            .borrow()
            .generate_r1cs_witness();
        self.t.alpha.borrow().evaluate();
        self.t.compute_beta.borrow().generate_r1cs_witness();
        self.t.beta_q.borrow().evaluate();

        self.t.el_inv_q_3.borrow().evaluate();
        self.t
            .compute_el_inv_q_3_minus_1
            .borrow()
            .generate_r1cs_witness();
        self.t.inv_alpha.borrow().evaluate();
        self.t.compute_inv_beta.borrow().generate_r1cs_witness();

        self.t.compute_w0.borrow().generate_r1cs_witness();
        self.t.compute_w1.borrow().generate_r1cs_witness();
        self.t.compute_result.borrow().generate_r1cs_witness();

        *self.pb.borrow_mut().val_ref(&self.t.result_is_one) =
            if self.t.result.borrow().get_element() == self.t.one.borrow().get_element() {
                ppT::FieldT::one()
            } else {
                ppT::FieldT::zero()
            };
    }
}

pub type mnt6_final_exp_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, mnt6_final_exp_gadget<ppT>>;
impl<ppT: ppTConfig> mnt6_final_exp_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        el: Fqk_variable<ppT>,
        result_is_one: variable<ppT::FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> mnt6_final_exp_gadgets<ppT> {
        let one = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " one"),
        ));
        let el_inv = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " el_inv"),
        ));
        let el_q_2 = RcCell::new(el.Frobenius_map(2));
        let el_q_2_minus_1 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " el_q_2_minus_1"),
        ));
        let el_q_3_minus_q = RcCell::new(el_q_2_minus_1.borrow().Frobenius_map(1));
        let el_inv_q_2 = RcCell::new(el_inv.borrow().Frobenius_map(2));
        let el_inv_q_2_minus_1 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " el_inv_q_2_minus_1"),
        ));
        let w1 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " w1"),
        ));
        let w0 = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " w0"),
        ));
        let result = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " result"),
        ));

        let compute_el_inv = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            el.clone(),
            el_inv.borrow().clone(),
            one.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_el_inv"),
        ));
        let compute_el_q_2_minus_1 = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            el_q_2.borrow().clone(),
            el_inv.borrow().clone(),
            el_q_2_minus_1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_el_q_2_minus_1"),
        ));
        let compute_el_inv_q_2_minus_1 = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            el_inv_q_2.borrow().clone(),
            el.clone(),
            el_inv_q_2_minus_1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_el_inv_q_2_minus_1"),
        ));

        let compute_w1 = RcCell::new(exponentiation_gadget::<ppT>::new(
            pb.clone(),
            el_q_3_minus_q.borrow().clone(),
            mnt4_final_exponent_last_chunk_w1.clone(),
            w1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_w1"),
        ));
        let compute_w0 = RcCell::new(exponentiation_gadget::<ppT>::new(
            pb.clone(),
            (if mnt4_final_exponent_last_chunk_is_w0_neg == bigint::<4>::default() {
                el_inv_q_2_minus_1.borrow().clone()
            } else {
                el_q_2_minus_1.borrow().clone()
            }),
            mnt4_final_exponent_last_chunk_abs_of_w0.clone(),
            w0.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_w0"),
        ));
        let compute_result = RcCell::new(Fqk_mul_gadget::<ppT>::new(
            pb.clone(),
            w1.borrow().clone(),
            w0.borrow().clone(),
            result.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_result"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                el,
                one,
                el_inv,
                el_q_2,
                el_q_2_minus_1,
                el_q_3_minus_q,
                el_inv_q_2,
                el_inv_q_2_minus_1,
                w1,
                w0,
                result,

                compute_el_inv,
                compute_el_q_2_minus_1,
                compute_el_inv_q_2_minus_1,

                compute_w1,
                compute_w0,
                compute_result,

                result_is_one,
            },
        )
    }
}
impl<ppT: ppTConfig> mnt6_final_exp_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .one
            .borrow()
            .generate_r1cs_equals_const_constraints(&ppT::FieldT::one());

        self.t.compute_el_inv.borrow().generate_r1cs_constraints();
        self.t
            .compute_el_q_2_minus_1
            .borrow()
            .generate_r1cs_constraints();
        self.t
            .compute_el_inv_q_2_minus_1
            .borrow()
            .generate_r1cs_constraints();
        self.t.compute_w1.borrow().generate_r1cs_constraints();
        self.t.compute_w0.borrow().generate_r1cs_constraints();
        self.t.compute_result.borrow().generate_r1cs_constraints();

        generate_boolean_r1cs_constraint::<ppT::FieldT, ppT::PB>(
            &self.pb,
            &(self.t.result_is_one.clone().into()),
            prefix_format!(self.annotation_prefix, " result_is_one"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                (-self.t.result.borrow().c0().c0().clone() + 1)
                    .to_field()
                    .into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c0.c0".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c0().c1().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c0.c1".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c1().c0().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c1.c0".to_owned(),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.result_is_one.clone().into(),
                self.t.result.borrow().c1().c1().clone().to_field().into(),
                ppT::FieldT::from(0).into(),
            ),
            " check c1.c0".to_owned(),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        self.t
            .one
            .borrow()
            .generate_r1cs_witness(&ppT::FieldT::one());
        self.t
            .el_inv
            .borrow()
            .generate_r1cs_witness(&self.t.el.get_element().inverse().to_field::<ppT::FieldT>());

        self.t.compute_el_inv.borrow().generate_r1cs_witness();
        self.t.el_q_2.borrow().evaluate();
        self.t
            .compute_el_q_2_minus_1
            .borrow()
            .generate_r1cs_witness();
        self.t.el_q_3_minus_q.borrow().evaluate();
        self.t.el_inv_q_2.borrow().evaluate();
        self.t
            .compute_el_inv_q_2_minus_1
            .borrow()
            .generate_r1cs_witness();
        self.t.compute_w1.borrow().generate_r1cs_witness();
        self.t.compute_w0.borrow().generate_r1cs_witness();
        self.t.compute_result.borrow().generate_r1cs_witness();

        *self.pb.borrow_mut().val_ref(&self.t.result_is_one) =
            if self.t.result.borrow().get_element() == self.t.one.borrow().get_element() {
                ppT::FieldT::one()
            } else {
                ppT::FieldT::zero()
            };
    }
}
