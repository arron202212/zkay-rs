//  Declaration of interfaces for gadgets for Miller loops.

//  The gadgets verify computations of (single or multiple simultaneous) Miller loops.

use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget::{G1_variable, G1_variables};
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget::{G2_variable, G2_variables};
use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation::{
    G1_precomputation, G1_precomputations, G2_precomputation, G2_precomputations,
    precompute_G1_gadget, precompute_G2_gadget, precompute_G2_gadget_coeffss,
};
// affine_ate_miller_loop, affine_ate_precompute_G1, affine_ate_precompute_G2, ppT::P::pairing_loop_count,
use ff_curves::{Fqe, G1, G2};
use ffec::PpConfig;
use ffec::scalar_multiplication::multiexp::KCConfig;
use std::ops::{Add, Mul};
// use ff_curves::algebra::curves::mnt::mnt4::mnt4_init;
// use ff_curves::algebra::curves::mnt::mnt6::mnt6_init;
use ffec::scalar_multiplication::wnaf::find_wnaf_u;
// use crate::gadgetlib1::gadgets::pairing::pairing_params::{Fqe_variable,Fqe_mul_gadget,Fqe_sqr_gadget};
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqe_mul_by_lc_gadget, Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, Fqk_special_mul_gadget,
    Fqk_sqr_gadget, Fqk_variable, MulTConfig, SqrTConfig, VariableTConfig, pairing_selector,
    ppTConfig,
};

use crate::gadgetlib1::constraint_profiling::{PRINT_CONSTRAINT_PROFILING, PROFILE_CONSTRAINTS};
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params::other_curve;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ff_curves::algebra::curves::public_params;
use ff_curves::{Fqk, Fr};
use ffec::FieldTConfig;
use ffec::field_utils::bigint::bigint;
use ffec::{One, Zero};
use rccell::RcCell;
use std::marker::PhantomData;

/**
 * Gadget for doubling step in the Miller loop.
 *
 * Technical note:
 *
 * mnt_Fqk g_RR_at_P = mnt_Fqk(prec_P.t.PY_twist_squared,
 *                             -prec_P.PX * c.gamma_twist + c.t.gamma_X - c.old_RY);
 *
 *(later in Miller loop: f = f.squared() * g_RR_at_P)
 *
 * Note the slight interface change: this gadget allocates g_RR_at_P inside itself (!)
 */
type FieldT<ppT> = Fr<ppT>;
type FqeT<ppT> = Fqe<other_curve<ppT>>;
type FqkT<ppT> = Fqk<other_curve<ppT>>;
#[derive(Clone, Default)]
pub struct mnt_miller_loop_dbl_line_eval<ppT: ppTConfig> {
    //gadget<ppT::Fr >
    prec_P: G1_precomputations<ppT>,
    c: precompute_G2_gadget_coeffss<ppT>,
    g_RR_at_P: RcCell<Fqk_variable<ppT>>, // reference from outside

    gamma_twist: RcCell<Fqe_variable<ppT>>,
    g_RR_at_P_c1: RcCell<Fqe_variable<ppT>>,
    compute_g_RR_at_P_c1: RcCell<Fqe_mul_by_lc_gadget<ppT>>,
}

/**
 * Gadget for addition step in the Miller loop.
 *
 * Technical note:
 *
 * mnt_Fqk g_RQ_at_P = mnt_Fqk(prec_P.t.PY_twist_squared,
 *                            -prec_P.PX * c.gamma_twist + c.t.gamma_X - prec_Q.QY);
 *
 * (later in Miller loop: f = f * g_RQ_at_P)
 *
 * Note the slight interface change: this gadget will allocate g_RQ_at_P inside itself (!)
 */
#[derive(Clone, Default)]
pub struct mnt_miller_loop_add_line_eval<ppT: ppTConfig> {
    //gadget<ppT::Fr >

    // type FieldT=ppT::Fr;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;
    invert_Q: bool,
    prec_P: G1_precomputations<ppT>,
    c: precompute_G2_gadget_coeffss<ppT>,
    Q: G2_variables<ppT>,
    g_RQ_at_P: RcCell<Fqk_variable<ppT>>, // reference from outside

    gamma_twist: RcCell<Fqe_variable<ppT>>,
    g_RQ_at_P_c1: RcCell<Fqe_variable<ppT>>,
    compute_g_RQ_at_P_c1: RcCell<Fqe_mul_by_lc_gadget<ppT>>,
}

/**
 * Gadget for verifying a single Miller loop.
 */
#[derive(Clone, Default)]
pub struct mnt_miller_loop_gadget<ppT: ppTConfig> {
    //gadget<ppT::Fr >

    // type FieldT=ppT::Fr;
    // type FqeT=ffec::Fqe<other_curve::<ppT> >;
    // type FqkT=ffec::Fqk<other_curve::<ppT> >;
    g_RR_at_Ps: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RQ_at_Ps: Vec<RcCell<Fqk_variable<ppT>>>,
    fs: Vec<RcCell<Fqk_variable<ppT>>>,

    addition_steps: Vec<RcCell<mnt_miller_loop_add_line_evals<ppT>>>,
    doubling_steps: Vec<RcCell<mnt_miller_loop_dbl_line_evals<ppT>>>,

    dbl_muls: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    dbl_sqrs: Vec<RcCell<Fqk_sqr_gadget<ppT>>>,
    add_muls: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,

    f_count: usize,
    add_count: usize,
    dbl_count: usize,

    prec_P: G1_precomputations<ppT>,
    prec_Q: G2_precomputations<ppT>,
    result: Fqk_variable<ppT>,
}

/**
 * Gadget for verifying a double Miller loop (where the second is inverted).
 */

// type FieldT=ppT::Fr;
//     type FqeT=ffec::Fqe<other_curve::<ppT> >;
//     type FqkT=ffec::Fqk<other_curve::<ppT> >;
#[derive(Clone, Default)]
pub struct mnt_e_over_e_miller_loop_gadget<ppT: ppTConfig> {
    //gadget<ppT::Fr >
    g_RR_at_P1s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RQ_at_P1s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RR_at_P2s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RQ_at_P2s: Vec<RcCell<Fqk_variable<ppT>>>,
    fs: Vec<RcCell<Fqk_variable<ppT>>>,

    addition_steps1: Vec<RcCell<mnt_miller_loop_add_line_evals<ppT>>>,
    doubling_steps1: Vec<RcCell<mnt_miller_loop_dbl_line_evals<ppT>>>,
    addition_steps2: Vec<RcCell<mnt_miller_loop_add_line_evals<ppT>>>,
    doubling_steps2: Vec<RcCell<mnt_miller_loop_dbl_line_evals<ppT>>>,

    dbl_sqrs: Vec<RcCell<Fqk_sqr_gadget<ppT>>>,
    dbl_muls1: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    add_muls1: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    dbl_muls2: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    add_muls2: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,

    f_count: usize,
    add_count: usize,
    dbl_count: usize,

    prec_P1: G1_precomputations<ppT>,
    prec_Q1: G2_precomputations<ppT>,
    prec_P2: G1_precomputations<ppT>,
    prec_Q2: G2_precomputations<ppT>,
    result: Fqk_variable<ppT>,
}

/**
 * Gadget for verifying a triple Miller loop (where the third is inverted).
 */

// type FieldT<ppT> = Fr<ppT>;
// type FqeT<ppT,P> = Fqe<other_curve<ppT,P>>;
// type FqkT<ppT,P> = Fqk<other_curve<ppT,P>>;
#[derive(Clone, Default)]
pub struct mnt_e_times_e_over_e_miller_loop_gadget<ppT: ppTConfig> {
    //gadget<ppT::Fr >
    g_RR_at_P1s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RQ_at_P1s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RR_at_P2s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RQ_at_P2s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RR_at_P3s: Vec<RcCell<Fqk_variable<ppT>>>,
    g_RQ_at_P3s: Vec<RcCell<Fqk_variable<ppT>>>,
    fs: Vec<RcCell<Fqk_variable<ppT>>>,

    addition_steps1: Vec<RcCell<mnt_miller_loop_add_line_evals<ppT>>>,
    doubling_steps1: Vec<RcCell<mnt_miller_loop_dbl_line_evals<ppT>>>,
    addition_steps2: Vec<RcCell<mnt_miller_loop_add_line_evals<ppT>>>,
    doubling_steps2: Vec<RcCell<mnt_miller_loop_dbl_line_evals<ppT>>>,
    addition_steps3: Vec<RcCell<mnt_miller_loop_add_line_evals<ppT>>>,
    doubling_steps3: Vec<RcCell<mnt_miller_loop_dbl_line_evals<ppT>>>,

    dbl_sqrs: Vec<RcCell<Fqk_sqr_gadget<ppT>>>,
    dbl_muls1: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    add_muls1: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    dbl_muls2: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    add_muls2: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    dbl_muls3: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,
    add_muls3: Vec<RcCell<Fqk_special_mul_gadget<ppT>>>,

    f_count: usize,
    add_count: usize,
    dbl_count: usize,

    prec_P1: G1_precomputations<ppT>,
    prec_Q1: G2_precomputations<ppT>,
    prec_P2: G1_precomputations<ppT>,
    prec_Q2: G2_precomputations<ppT>,
    prec_P3: G1_precomputations<ppT>,
    prec_Q3: G2_precomputations<ppT>,
    result: Fqk_variable<ppT>,
}

/*
  performs

  mnt_Fqk g_RR_at_P = mnt_Fqk(prec_P.t.PY_twist_squared,
  -prec_P.PX * c.gamma_twist + c.t.gamma_X - c.old_RY);

  (later in Miller loop: f = f.squared() * g_RR_at_P)
*/

/* Note the slight interface change: this gadget will allocate g_RR_at_P inside itself (!) */
pub type mnt_miller_loop_dbl_line_evals<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, mnt_miller_loop_dbl_line_eval<ppT>>;
impl<ppT: ppTConfig> mnt_miller_loop_dbl_line_eval<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        prec_P: G1_precomputations<ppT>,
        c: precompute_G2_gadget_coeffss<ppT>,
        g_RR_at_P: RcCell<Fqk_variable<ppT>>,
        annotation_prefix: String,
    ) -> mnt_miller_loop_dbl_line_evals<ppT> {
        let gamma_twist = RcCell::new(c.t.gamma.borrow().mul_by_X());
        // prec_P.PX * c.gamma_twist = c.t.gamma_X - c.old_RY - g_RR_at_P_c1
        let mut g_RR_at_P_c1;
        let mut compute_g_RR_at_P_c1 = RcCell::new(Fqe_mul_by_lc_gadget::<ppT>::default());
        if gamma_twist.borrow().is_constant() {
            gamma_twist.borrow().evaluate();
            let gamma_twist_const = gamma_twist.borrow().get_element();
            g_RR_at_P_c1 = RcCell::new(
                Fqe_variable::<ppT>::newvv(
                    pb.clone(),
                    -gamma_twist_const.to_field::<ppT::FieldT>(),
                    prec_P.t.P.borrow().t.X.clone(),
                    prefix_format!(annotation_prefix, " tmp"),
                ) + c.t.gamma_X.borrow().clone()
                    + c.t.RY.borrow().clone() * (-ppT::FieldT::one()),
            );
        } else if prec_P.t.P.borrow().t.X.is_constant() {
            prec_P.t.P.borrow().t.X.evaluate_pb(&pb);
            let P_X_const = prec_P.t.P.borrow().t.X.constant_term();
            g_RR_at_P_c1 = RcCell::new(
                gamma_twist.borrow().clone() * (-P_X_const.clone())
                    + c.t.gamma_X.borrow().clone()
                    + c.t.RY.borrow().clone() * (-ppT::FieldT::one()),
            );
        } else {
            g_RR_at_P_c1 = RcCell::new(Fqe_variable::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " g_RR_at_P_c1"),
            ));
            compute_g_RR_at_P_c1 = RcCell::new(Fqe_mul_by_lc_gadget::<ppT>::new2(
                pb.clone(),
                gamma_twist.borrow().clone(),
                prec_P.t.P.borrow().t.X.clone(),
                c.t.gamma_X.borrow().clone()
                    + c.t.RY.borrow().clone() * (-ppT::FieldT::one())
                    + g_RR_at_P_c1.borrow().clone() * (-ppT::FieldT::one()),
                prefix_format!(annotation_prefix, " compute_g_RR_at_P_c1"),
            ));
        }
        let g_RR_at_P = RcCell::new(Fqk_variable::<ppT>::newe(
            pb.clone(),
            prec_P.t.PY_twist_squared.clone(),
            g_RR_at_P_c1.clone(),
            prefix_format!(annotation_prefix, " g_RR_at_P"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb.clone(),
            annotation_prefix,
            Self {
                prec_P,
                c,
                g_RR_at_P,
                gamma_twist,
                g_RR_at_P_c1,
                compute_g_RR_at_P_c1,
            },
        )
    }
}
impl<ppT: ppTConfig> mnt_miller_loop_dbl_line_evals<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        if !self.t.gamma_twist.borrow().is_constant()
            && !self.t.prec_P.t.P.borrow().t.X.is_constant()
        {
            self.t
                .compute_g_RR_at_P_c1
                .borrow()
                .generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.gamma_twist.borrow().evaluate();
        let gamma_twist_val = self.t.gamma_twist.borrow().get_element();
        let PX_val = self.pb.borrow().lc_val(&self.t.prec_P.t.P.borrow().t.X);
        let gamma_X_val = self.t.c.t.gamma_X.borrow().get_element();
        let RY_val = self.t.c.t.RY.borrow().get_element();
        let g_RR_at_P_c1_val = gamma_twist_val * (-PX_val) + gamma_X_val - RY_val;
        self.t
            .g_RR_at_P_c1
            .borrow()
            .generate_r1cs_witness(&g_RR_at_P_c1_val.to_field::<ppT::FieldT>());

        if !self.t.gamma_twist.borrow().is_constant()
            && !self.t.prec_P.t.P.borrow().t.X.is_constant()
        {
            self.t.compute_g_RR_at_P_c1.borrow().generate_r1cs_witness();
        }
        self.t.g_RR_at_P.borrow().evaluate();
    }
}

/*
  performs
  mnt_Fqk g_RQ_at_P = mnt_Fqk(prec_P.t.PY_twist_squared,
  -prec_P.PX * c.gamma_twist + c.t.gamma_X - prec_Q.QY);

  (later in Miller loop: f = f * g_RQ_at_P)

  If invert_Q is set to true: use -QY in place of QY everywhere above.
*/

/* Note the slight interface change: this gadget will allocate g_RQ_at_P inside itself (!) */
pub type mnt_miller_loop_add_line_evals<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, mnt_miller_loop_add_line_eval<ppT>>;
impl<ppT: ppTConfig> mnt_miller_loop_add_line_eval<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        invert_Q: bool,
        prec_P: G1_precomputations<ppT>,
        c: precompute_G2_gadget_coeffss<ppT>,
        Q: G2_variables<ppT>,
        g_RQ_at_P: RcCell<Fqk_variable<ppT>>,
        annotation_prefix: String,
    ) -> mnt_miller_loop_add_line_evals<ppT> {
        let gamma_twist = RcCell::new(c.t.gamma.borrow().mul_by_X());
        let mut g_RQ_at_P_c1;
        let mut compute_g_RQ_at_P_c1 = RcCell::new(Fqe_mul_by_lc_gadget::<ppT>::default());
        // prec_P.PX * c.gamma_twist = c.t.gamma_X - prec_Q.QY - g_RQ_at_P_c1
        if gamma_twist.borrow().is_constant() {
            gamma_twist.borrow().evaluate();
            let gamma_twist_const = gamma_twist.borrow().get_element();
            g_RQ_at_P_c1 = RcCell::new(
                Fqe_variable::<ppT>::newvv(
                    pb.clone(),
                    -gamma_twist_const.to_field::<ppT::FieldT>(),
                    prec_P.t.P.borrow().t.X.clone(),
                    prefix_format!(annotation_prefix, " tmp"),
                ) + c.t.gamma_X.borrow().clone()
                    + Q.t.Y.borrow().clone()
                        * (if !invert_Q {
                            -ppT::FieldT::one()
                        } else {
                            ppT::FieldT::one()
                        }),
            );
        } else if prec_P.t.P.borrow().t.X.is_constant() {
            prec_P.t.P.borrow().t.X.evaluate_pb(&pb);
            let P_X_const = prec_P.t.P.borrow().t.X.constant_term();
            g_RQ_at_P_c1 = RcCell::new(
                gamma_twist.borrow().clone() * (-P_X_const)
                    + c.t.gamma_X.borrow().clone()
                    + Q.t.Y.borrow().clone()
                        * (if !invert_Q {
                            -ppT::FieldT::one()
                        } else {
                            ppT::FieldT::one()
                        }),
            );
        } else {
            g_RQ_at_P_c1 = RcCell::new(Fqe_variable::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " g_RQ_at_Q_c1"),
            ));
            compute_g_RQ_at_P_c1 = RcCell::new(Fqe_mul_by_lc_gadget::<ppT>::new2(
                pb.clone(),
                gamma_twist.borrow().clone(),
                prec_P.t.P.borrow().t.X.clone(),
                c.t.gamma_X.borrow().clone()
                    + Q.t.Y.borrow().clone()
                        * (if !invert_Q {
                            -ppT::FieldT::one()
                        } else {
                            ppT::FieldT::one()
                        })
                    + (g_RQ_at_P_c1.borrow().clone()) * (-ppT::FieldT::one()),
                prefix_format!(annotation_prefix, " compute_g_RQ_at_P_c1"),
            ));
        }
        let g_RQ_at_P = RcCell::new(Fqk_variable::<ppT>::newe(
            pb.clone(),
            prec_P.t.PY_twist_squared.clone(),
            g_RQ_at_P_c1.clone(),
            prefix_format!(annotation_prefix, " g_RQ_at_P"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                invert_Q,
                prec_P,
                c,
                Q,
                g_RQ_at_P,
                gamma_twist,
                g_RQ_at_P_c1,
                compute_g_RQ_at_P_c1,
            },
        )
    }
}
impl<ppT: ppTConfig> mnt_miller_loop_add_line_evals<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        if !self.t.gamma_twist.borrow().is_constant()
            && !self.t.prec_P.t.P.borrow().t.X.is_constant()
        {
            self.t
                .compute_g_RQ_at_P_c1
                .borrow()
                .generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.gamma_twist.borrow().evaluate();
        let gamma_twist_val = self.t.gamma_twist.borrow().get_element();
        let PX_val = self.pb.borrow().lc_val(&self.t.prec_P.t.P.borrow().t.X);
        let gamma_X_val = self.t.c.t.gamma_X.borrow().get_element();
        let QY_val = self.t.Q.t.Y.borrow().get_element();
        let g_RQ_at_P_c1_val = gamma_twist_val.clone() * (-PX_val.clone())
            + gamma_X_val.clone()
            + (if !self.t.invert_Q {
                -QY_val.clone()
            } else {
                QY_val.clone()
            });
        self.t
            .g_RQ_at_P_c1
            .borrow()
            .generate_r1cs_witness(&g_RQ_at_P_c1_val.to_field::<ppT::FieldT>());

        if !self.t.gamma_twist.borrow().is_constant()
            && !self.t.prec_P.t.P.borrow().t.X.is_constant()
        {
            self.t.compute_g_RQ_at_P_c1.borrow().generate_r1cs_witness();
        }
        self.t.g_RQ_at_P.borrow().evaluate();
    }
}
pub type mnt_miller_loop_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, mnt_miller_loop_gadget<ppT>>;
impl<ppT: ppTConfig> mnt_miller_loop_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        prec_P: G1_precomputations<ppT>,
        prec_Q: G2_precomputations<ppT>,
        result: Fqk_variable<ppT>,
        annotation_prefix: String,
    ) -> mnt_miller_loop_gadgets<ppT> {
        let loop_count = ppT::P::pairing_loop_count;
        let (mut f_count, mut add_count, mut dbl_count) = (0, 0, 0);

        let mut found_nonzero = false;
        let mut NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            dbl_count += 1;
            f_count += 2;

            if NAF[i] != 0 {
                add_count += 1;
                f_count += 1;
            }
        }

        let mut fs = vec![RcCell::new(Fqk_variable::<ppT>::default()); f_count];

        let mut doubling_steps =
            vec![RcCell::new(mnt_miller_loop_dbl_line_evals::<ppT>::default()); dbl_count];
        let mut addition_steps =
            vec![RcCell::new(mnt_miller_loop_add_line_evals::<ppT>::default()); add_count];

        let mut g_RR_at_Ps = vec![RcCell::new(Fqk_variable::<ppT>::default()); dbl_count];
        let mut g_RQ_at_Ps = vec![RcCell::new(Fqk_variable::<ppT>::default()); add_count];

        for i in 0..f_count {
            fs[i] = RcCell::new(Fqk_variable::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " fs_{}", i),
            ));
        }

        let mut dbl_sqrs = vec![RcCell::new(Fqk_sqr_gadget::<ppT>::default()); dbl_count];
        let mut dbl_muls = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); dbl_count];
        let mut add_muls = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); add_count];

        let mut add_id = 0;
        let mut dbl_id = 0;
        let mut f_id = 0;
        let mut prec_id = 0;

        found_nonzero = false;
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            doubling_steps[dbl_id] = RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(
                pb.clone(),
                prec_P.clone(),
                prec_Q.t.coeffs[prec_id].borrow().clone(),
                g_RR_at_Ps[dbl_id].clone(),
                prefix_format!(annotation_prefix, " doubling_steps_{}", dbl_id),
            ));
            prec_id += 1;
            dbl_sqrs[dbl_id] = RcCell::new(Fqk_sqr_gadget::<ppT>::new(
                pb.clone(),
                fs[f_id].clone(),
                fs[f_id + 1].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_sqrs_{}", dbl_id),
            ));
            f_id += 1;
            dbl_muls[dbl_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                pb.clone(),
                fs[f_id].borrow().clone(),
                g_RR_at_Ps[dbl_id].borrow().clone(),
                (if f_id + 1 == f_count {
                    result.clone()
                } else {
                    fs[f_id + 1].borrow().clone()
                }),
                prefix_format!(annotation_prefix, " dbl_muls_{}", dbl_id),
            ));
            f_id += 1;
            dbl_id += 1;

            if NAF[i] != 0 {
                addition_steps[add_id] = RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(
                    pb.clone(),
                    NAF[i] < 0,
                    prec_P.clone(),
                    prec_Q.t.coeffs[prec_id].borrow().clone(),
                    prec_Q.t.Q.borrow().clone(),
                    g_RQ_at_Ps[add_id].clone(),
                    prefix_format!(annotation_prefix, " addition_steps_{}", add_id),
                ));
                prec_id += 1;
                add_muls[add_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                    pb.clone(),
                    fs[f_id].borrow().clone(),
                    g_RQ_at_Ps[add_id].borrow().clone(),
                    (if f_id + 1 == f_count {
                        result.clone()
                    } else {
                        fs[f_id + 1].borrow().clone()
                    }),
                    prefix_format!(annotation_prefix, " add_muls_{}", add_id),
                ));
                f_id += 1;
                add_id += 1;
            }
        }
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                g_RR_at_Ps,
                g_RQ_at_Ps,
                fs,

                addition_steps,
                doubling_steps,

                dbl_muls,
                dbl_sqrs,
                add_muls,

                f_count,
                add_count,
                dbl_count,
                prec_P,
                prec_Q,
                result,
            },
        )
    }
}
impl<ppT: ppTConfig> mnt_miller_loop_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.fs[0]
            .borrow()
            .generate_r1cs_equals_const_constraints(&ppT::FieldT::one());

        for i in 0..self.t.dbl_count {
            self.t.doubling_steps[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.dbl_sqrs[i].borrow().generate_r1cs_constraints();
            self.t.dbl_muls[i].borrow().generate_r1cs_constraints();
        }

        for i in 0..self.t.add_count {
            self.t.addition_steps[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.add_muls[i].borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.fs[0]
            .borrow()
            .generate_r1cs_witness(&ppT::FieldT::one());

        let mut add_id = 0;
        let mut dbl_id = 0;

        let loop_count = ppT::P::pairing_loop_count;

        let mut found_nonzero = false;
        let mut NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            self.t.doubling_steps[dbl_id]
                .borrow()
                .generate_r1cs_witness();
            self.t.dbl_sqrs[dbl_id].borrow().generate_r1cs_witness();
            self.t.dbl_muls[dbl_id].borrow().generate_r1cs_witness();
            dbl_id += 1;

            if NAF[i] != 0 {
                self.t.addition_steps[add_id]
                    .borrow()
                    .generate_r1cs_witness();
                self.t.add_muls[add_id].borrow().generate_r1cs_witness();
                add_id += 1;
            }
        }
    }
}

pub fn test_mnt_miller_loop<ppT: ppTConfig>(annotation: &String)
// where
// <ppT as ff_curves::PublicParams>::G1:
//     Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G1>,
// <ppT as ff_curves::PublicParams>::G2:
//     Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G2>,
{
    let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
    let mut P_val = ppT::FieldT::random_element() * G1::<other_curve<ppT>>::one();
    let mut Q_val = ppT::FieldT::random_element() * G2::<other_curve<ppT>>::one();

    let mut P = G1_variable::<ppT>::new(pb.clone(), "P".to_owned());
    let mut Q = G2_variable::<ppT>::new(pb.clone(), "Q".to_owned());

    let mut prec_P = G1_precomputations::<ppT>::default();
    let mut prec_Q = G2_precomputations::<ppT>::default();

    let mut compute_prec_P = precompute_G1_gadget::<ppT>::new(
        pb.clone(),
        P.clone(),
        prec_P.clone(),
        "prec_P".to_owned(),
    );
    let mut compute_prec_Q = precompute_G2_gadget::<ppT>::new(
        pb.clone(),
        Q.clone(),
        prec_Q.clone(),
        "prec_Q".to_owned(),
    );

    let mut result = Fqk_variable::<ppT>::new(pb.clone(), "result".to_owned());
    let mut miller = mnt_miller_loop_gadget::<ppT>::new(
        pb.clone(),
        prec_P.clone(),
        prec_Q.clone(),
        result.clone(),
        "miller".to_owned(),
    );

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

    P.generate_r1cs_witness(&P_val);
    compute_prec_P.generate_r1cs_witness();
    Q.generate_r1cs_witness(&Q_val);
    compute_prec_Q.generate_r1cs_witness();
    miller.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    let native_prec_P = ppT::affine_ate_precompute_G1(&P_val);
    let native_prec_Q = ppT::affine_ate_precompute_G2(&Q_val);
    let native_result = ppT::affine_ate_miller_loop(&native_prec_P, &native_prec_Q);

    assert!(result.get_element() == native_result);
    print!(
        "number of constraints for Miller loop (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}
pub type mnt_e_over_e_miller_loop_gadgets<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    mnt_e_over_e_miller_loop_gadget<ppT>,
>;
impl<ppT: ppTConfig> mnt_e_over_e_miller_loop_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        prec_P1: G1_precomputations<ppT>,
        prec_Q1: G2_precomputations<ppT>,
        prec_P2: G1_precomputations<ppT>,
        prec_Q2: G2_precomputations<ppT>,
        result: Fqk_variable<ppT>,
        annotation_prefix: String,
    ) -> mnt_e_over_e_miller_loop_gadgets<ppT> {
        let loop_count = ppT::P::pairing_loop_count;

        let (mut f_count, mut add_count, mut dbl_count) = (0, 0, 0);

        let mut found_nonzero = false;
        let mut NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            dbl_count += 1;
            f_count += 3;

            if NAF[i] != 0 {
                add_count += 1;
                f_count += 2;
            }
        }

        let mut fs = vec![RcCell::new(Fqk_variable::<ppT>::default()); f_count];
        let mut doubling_steps1 =
            vec![RcCell::new(mnt_miller_loop_dbl_line_evals::<ppT>::default()); dbl_count];
        let mut addition_steps1 =
            vec![RcCell::new(mnt_miller_loop_add_line_evals::<ppT>::default()); add_count];
        let mut doubling_steps2 =
            vec![RcCell::new(mnt_miller_loop_dbl_line_evals::<ppT>::default()); dbl_count];
        let mut addition_steps2 =
            vec![RcCell::new(mnt_miller_loop_add_line_evals::<ppT>::default()); add_count];

        let mut g_RR_at_P1s = vec![RcCell::new(Fqk_variable::<ppT>::default()); dbl_count];
        let mut g_RQ_at_P1s = vec![RcCell::new(Fqk_variable::<ppT>::default()); add_count];
        let mut g_RR_at_P2s = vec![RcCell::new(Fqk_variable::<ppT>::default()); dbl_count];
        let mut g_RQ_at_P2s = vec![RcCell::new(Fqk_variable::<ppT>::default()); add_count];

        for i in 0..f_count {
            fs[i] = RcCell::new(Fqk_variable::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " fs_{}", i),
            ));
        }

        let mut dbl_sqrs = vec![RcCell::new(Fqk_sqr_gadget::<ppT>::default()); dbl_count];
        let mut dbl_muls1 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); dbl_count];
        let mut add_muls1 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); add_count];
        let mut dbl_muls2 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); dbl_count];
        let mut add_muls2 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); add_count];

        let mut add_id = 0;
        let mut dbl_id = 0;
        let mut f_id = 0;
        let mut prec_id = 0;

        found_nonzero = false;
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            doubling_steps1[dbl_id] = RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(
                pb.clone(),
                prec_P1.clone(),
                prec_Q1.t.coeffs[prec_id].borrow().clone(),
                g_RR_at_P1s[dbl_id].clone(),
                prefix_format!(annotation_prefix, " doubling_steps1_{}", dbl_id),
            ));
            doubling_steps2[dbl_id] = RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(
                pb.clone(),
                prec_P2.clone(),
                prec_Q2.t.coeffs[prec_id].borrow().clone(),
                g_RR_at_P2s[dbl_id].clone(),
                prefix_format!(annotation_prefix, " doubling_steps2_{}", dbl_id),
            ));
            prec_id += 1;

            dbl_sqrs[dbl_id] = RcCell::new(Fqk_sqr_gadget::<ppT>::new(
                pb.clone(),
                fs[f_id].clone(),
                fs[f_id + 1].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_sqrs_{}", dbl_id),
            ));
            f_id += 1;
            dbl_muls1[dbl_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                pb.clone(),
                fs[f_id].borrow().clone(),
                g_RR_at_P1s[dbl_id].borrow().clone(),
                fs[f_id + 1].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_mul1s_{}", dbl_id),
            ));
            f_id += 1;
            dbl_muls2[dbl_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                pb.clone(),
                (if f_id + 1 == f_count {
                    result.clone()
                } else {
                    fs[f_id + 1].borrow().clone()
                }),
                g_RR_at_P2s[dbl_id].borrow().clone(),
                fs[f_id].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_mul2s_{}", dbl_id),
            ));
            f_id += 1;
            dbl_id += 1;

            if NAF[i] != 0 {
                addition_steps1[add_id] = RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(
                    pb.clone(),
                    NAF[i] < 0,
                    prec_P1.clone(),
                    prec_Q1.t.coeffs[prec_id].borrow().clone(),
                    prec_Q1.t.Q.borrow().clone(),
                    g_RQ_at_P1s[add_id].clone(),
                    prefix_format!(annotation_prefix, " addition_steps1_{}", add_id),
                ));
                addition_steps2[add_id] = RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(
                    pb.clone(),
                    NAF[i] < 0,
                    prec_P2.clone(),
                    prec_Q2.t.coeffs[prec_id].borrow().clone(),
                    prec_Q2.t.Q.borrow().clone(),
                    g_RQ_at_P2s[add_id].clone(),
                    prefix_format!(annotation_prefix, " addition_steps2_{}", add_id),
                ));
                prec_id += 1;
                add_muls1[add_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                    pb.clone(),
                    fs[f_id].borrow().clone(),
                    g_RQ_at_P1s[add_id].borrow().clone(),
                    fs[f_id + 1].borrow().clone(),
                    prefix_format!(annotation_prefix, " add_mul1s_{}", add_id),
                ));
                f_id += 1;
                add_muls2[add_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                    pb.clone(),
                    (if f_id + 1 == f_count {
                        result.clone()
                    } else {
                        fs[f_id + 1].borrow().clone()
                    }),
                    g_RQ_at_P2s[add_id].borrow().clone(),
                    fs[f_id].borrow().clone(),
                    prefix_format!(annotation_prefix, " add_mul2s_{}", add_id),
                ));
                f_id += 1;
                add_id += 1;
            }
        }
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                g_RR_at_P1s,
                g_RQ_at_P1s,
                g_RR_at_P2s,
                g_RQ_at_P2s,
                fs,

                addition_steps1,
                doubling_steps1,
                addition_steps2,
                doubling_steps2,

                dbl_sqrs,
                dbl_muls1,
                add_muls1,
                dbl_muls2,
                add_muls2,

                f_count,
                add_count,
                dbl_count,
                prec_P1,
                prec_Q1,
                prec_P2,
                prec_Q2,
                result,
            },
        )
    }
}
impl<ppT: ppTConfig> mnt_e_over_e_miller_loop_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.fs[0]
            .borrow()
            .generate_r1cs_equals_const_constraints(&ppT::FieldT::one());

        for i in 0..self.t.dbl_count {
            self.t.doubling_steps1[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.doubling_steps2[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.dbl_sqrs[i].borrow().generate_r1cs_constraints();
            self.t.dbl_muls1[i].borrow().generate_r1cs_constraints();
            self.t.dbl_muls2[i].borrow().generate_r1cs_constraints();
        }

        for i in 0..self.t.add_count {
            self.t.addition_steps1[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.addition_steps2[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.add_muls1[i].borrow().generate_r1cs_constraints();
            self.t.add_muls2[i].borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.fs[0]
            .borrow()
            .generate_r1cs_witness(&ppT::FieldT::one());

        let mut add_id = 0;
        let mut dbl_id = 0;
        let mut f_id = 0;

        let loop_count = ppT::P::pairing_loop_count;

        let mut found_nonzero = false;
        let mut NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            self.t.doubling_steps1[dbl_id]
                .borrow()
                .generate_r1cs_witness();
            self.t.doubling_steps2[dbl_id]
                .borrow()
                .generate_r1cs_witness();
            self.t.dbl_sqrs[dbl_id].borrow().generate_r1cs_witness();
            f_id += 1;
            self.t.dbl_muls1[dbl_id].borrow().generate_r1cs_witness();
            f_id += 1;
            (if f_id + 1 == self.t.f_count {
                self.t.result.clone()
            } else {
                self.t.fs[f_id + 1].borrow().clone()
            })
            .generate_r1cs_witness(
                &((self.t.fs[f_id].borrow().get_element()
                    * self.t.g_RR_at_P2s[dbl_id].borrow().get_element().inverse())
                .to_field::<ppT::FieldT>()),
            );
            self.t.dbl_muls2[dbl_id].borrow().generate_r1cs_witness();
            f_id += 1;
            dbl_id += 1;

            if NAF[i] != 0 {
                self.t.addition_steps1[add_id]
                    .borrow()
                    .generate_r1cs_witness();
                self.t.addition_steps2[add_id]
                    .borrow()
                    .generate_r1cs_witness();
                self.t.add_muls1[add_id].borrow().generate_r1cs_witness();
                f_id += 1;
                (if f_id + 1 == self.t.f_count {
                    self.t.result.clone()
                } else {
                    self.t.fs[f_id + 1].borrow().clone()
                })
                .generate_r1cs_witness(
                    &((self.t.fs[f_id].borrow().get_element()
                        * self.t.g_RQ_at_P2s[add_id].borrow().get_element().inverse())
                    .to_field::<ppT::FieldT>()),
                );
                self.t.add_muls2[add_id].borrow().generate_r1cs_witness();
                f_id += 1;
                add_id += 1;
            }
        }
    }
}

pub fn test_mnt_e_over_e_miller_loop<ppT: ppTConfig>(annotation: &String)
// where
// <ppT as ff_curves::PublicParams>::G1:
//     Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G1>,
// <ppT as ff_curves::PublicParams>::G2:
//     Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G2>,
{
    let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
    let mut P1_val = ppT::FieldT::random_element() * G1::<other_curve<ppT>>::one();
    let mut Q1_val = ppT::FieldT::random_element() * G2::<other_curve<ppT>>::one();

    let mut P2_val = ppT::FieldT::random_element() * G1::<other_curve<ppT>>::one();
    let mut Q2_val = ppT::FieldT::random_element() * G2::<other_curve<ppT>>::one();

    let mut P1 = G1_variable::<ppT>::new(pb.clone(), "P1".to_owned());
    let mut Q1 = G2_variable::<ppT>::new(pb.clone(), "Q1".to_owned());
    let mut P2 = G1_variable::<ppT>::new(pb.clone(), "P2".to_owned());
    let mut Q2 = G2_variable::<ppT>::new(pb.clone(), "Q2".to_owned());

    let mut prec_P1 = G1_precomputations::<ppT>::default();
    let mut compute_prec_P1 = precompute_G1_gadget::<ppT>::new(
        pb.clone(),
        P1.clone(),
        prec_P1.clone(),
        "compute_prec_P1".to_owned(),
    );
    let mut prec_P2 = G1_precomputations::<ppT>::default();
    let mut compute_prec_P2 = precompute_G1_gadget::<ppT>::new(
        pb.clone(),
        P2.clone(),
        prec_P2.clone(),
        "compute_prec_P2".to_owned(),
    );
    let mut prec_Q1 = G2_precomputations::<ppT>::default();
    let mut compute_prec_Q1 = precompute_G2_gadget::<ppT>::new(
        pb.clone(),
        Q1.clone(),
        prec_Q1.clone(),
        "compute_prec_Q1".to_owned(),
    );
    let mut prec_Q2 = G2_precomputations::<ppT>::default();
    let mut compute_prec_Q2 = precompute_G2_gadget::<ppT>::new(
        pb.clone(),
        Q2.clone(),
        prec_Q2.clone(),
        "compute_prec_Q2".to_owned(),
    );

    let mut result = Fqk_variable::<ppT>::new(pb.clone(), "result".to_owned());
    let mut miller = mnt_e_over_e_miller_loop_gadget::<ppT>::new(
        pb.clone(),
        prec_P1.clone(),
        prec_Q1.clone(),
        prec_P2.clone(),
        prec_Q2.clone(),
        result.clone(),
        "miller".to_owned(),
    );

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

    P1.generate_r1cs_witness(&P1_val);
    compute_prec_P1.generate_r1cs_witness();
    Q1.generate_r1cs_witness(&Q1_val);
    compute_prec_Q1.generate_r1cs_witness();
    P2.generate_r1cs_witness(&P2_val);
    compute_prec_P2.generate_r1cs_witness();
    Q2.generate_r1cs_witness(&Q2_val);
    compute_prec_Q2.generate_r1cs_witness();
    miller.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    let mut native_prec_P1 = ppT::affine_ate_precompute_G1(&P1_val);
    let mut native_prec_Q1 = ppT::affine_ate_precompute_G2(&Q1_val);
    let mut native_prec_P2 = ppT::affine_ate_precompute_G1(&P2_val);
    let mut native_prec_Q2 = ppT::affine_ate_precompute_G2(&Q2_val);
    let mut native_result = (ppT::affine_ate_miller_loop(&native_prec_P1, &native_prec_Q1)
        * ppT::affine_ate_miller_loop(&native_prec_P2, &native_prec_Q2).inverse());

    assert!(result.get_element() == native_result);
    print!(
        "number of constraints for e over e Miller loop (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}

pub type mnt_e_times_e_over_e_miller_loop_gadgets<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    mnt_e_times_e_over_e_miller_loop_gadget<ppT>,
>;
impl<ppT: ppTConfig> mnt_e_times_e_over_e_miller_loop_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        prec_P1: G1_precomputations<ppT>,
        prec_Q1: G2_precomputations<ppT>,
        prec_P2: G1_precomputations<ppT>,
        prec_Q2: G2_precomputations<ppT>,
        prec_P3: G1_precomputations<ppT>,
        prec_Q3: G2_precomputations<ppT>,
        result: Fqk_variable<ppT>,
        annotation_prefix: String,
    ) -> mnt_e_times_e_over_e_miller_loop_gadgets<ppT> {
        let mut loop_count = ppT::P::pairing_loop_count;

        let (mut f_count, mut add_count, mut dbl_count) = (0, 0, 0);

        let mut found_nonzero = false;
        let mut NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            dbl_count += 1;
            f_count += 4;

            if NAF[i] != 0 {
                add_count += 1;
                f_count += 3;
            }
        }

        let mut fs = vec![RcCell::new(Fqk_variable::<ppT>::default()); f_count];
        let mut doubling_steps1 =
            vec![RcCell::new(mnt_miller_loop_dbl_line_evals::<ppT>::default()); dbl_count];
        let mut addition_steps1 =
            vec![RcCell::new(mnt_miller_loop_add_line_evals::<ppT>::default()); add_count];
        let mut doubling_steps2 =
            vec![RcCell::new(mnt_miller_loop_dbl_line_evals::<ppT>::default()); dbl_count];
        let mut addition_steps2 =
            vec![RcCell::new(mnt_miller_loop_add_line_evals::<ppT>::default()); add_count];
        let mut doubling_steps3 =
            vec![RcCell::new(mnt_miller_loop_dbl_line_evals::<ppT>::default()); dbl_count];
        let mut addition_steps3 =
            vec![RcCell::new(mnt_miller_loop_add_line_evals::<ppT>::default()); add_count];
        let mut g_RR_at_P1s = vec![RcCell::new(Fqk_variable::<ppT>::default()); dbl_count];
        let mut g_RQ_at_P1s = vec![RcCell::new(Fqk_variable::<ppT>::default()); add_count];
        let mut g_RR_at_P2s = vec![RcCell::new(Fqk_variable::<ppT>::default()); dbl_count];
        let mut g_RQ_at_P2s = vec![RcCell::new(Fqk_variable::<ppT>::default()); add_count];
        let mut g_RR_at_P3s = vec![RcCell::new(Fqk_variable::<ppT>::default()); dbl_count];
        let mut g_RQ_at_P3s = vec![RcCell::new(Fqk_variable::<ppT>::default()); add_count];

        for i in 0..f_count {
            fs[i] = RcCell::new(Fqk_variable::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " fs_{}", i),
            ));
        }

        let mut dbl_sqrs = vec![RcCell::new(Fqk_sqr_gadget::<ppT>::default()); dbl_count];
        let mut dbl_muls1 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); dbl_count];
        let mut add_muls1 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); add_count];
        let mut dbl_muls2 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); dbl_count];
        let mut add_muls2 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); add_count];
        let mut dbl_muls3 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); dbl_count];
        let mut add_muls3 = vec![RcCell::new(Fqk_special_mul_gadget::<ppT>::default()); add_count];

        let mut add_id = 0;
        let mut dbl_id = 0;
        let mut f_id = 0;
        let mut prec_id = 0;

        found_nonzero = false;
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            doubling_steps1[dbl_id] = RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(
                pb.clone(),
                prec_P1.clone(),
                prec_Q1.t.coeffs[prec_id].borrow().clone(),
                g_RR_at_P1s[dbl_id].clone(),
                prefix_format!(annotation_prefix, " doubling_steps1_{}", dbl_id),
            ));
            doubling_steps2[dbl_id] = RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(
                pb.clone(),
                prec_P2.clone(),
                prec_Q2.t.coeffs[prec_id].borrow().clone(),
                g_RR_at_P2s[dbl_id].clone(),
                prefix_format!(annotation_prefix, " doubling_steps2_{}", dbl_id),
            ));
            doubling_steps3[dbl_id] = RcCell::new(mnt_miller_loop_dbl_line_eval::<ppT>::new(
                pb.clone(),
                prec_P3.clone(),
                prec_Q3.t.coeffs[prec_id].borrow().clone(),
                g_RR_at_P3s[dbl_id].clone(),
                prefix_format!(annotation_prefix, " doubling_steps3_{}", dbl_id),
            ));
            prec_id += 1;

            dbl_sqrs[dbl_id] = RcCell::new(Fqk_sqr_gadget::<ppT>::new(
                pb.clone(),
                fs[f_id].clone(),
                fs[f_id + 1].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_sqrs_{}", dbl_id),
            ));
            f_id += 1;
            dbl_muls1[dbl_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                pb.clone(),
                fs[f_id].borrow().clone(),
                g_RR_at_P1s[dbl_id].borrow().clone(),
                fs[f_id + 1].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_muls1_{}", dbl_id),
            ));
            f_id += 1;
            dbl_muls2[dbl_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                pb.clone(),
                fs[f_id].borrow().clone(),
                g_RR_at_P2s[dbl_id].borrow().clone(),
                fs[f_id + 1].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_muls2_{}", dbl_id),
            ));
            f_id += 1;
            dbl_muls3[dbl_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                pb.clone(),
                (if f_id + 1 == f_count {
                    result.clone()
                } else {
                    fs[f_id + 1].borrow().clone()
                }),
                g_RR_at_P3s[dbl_id].borrow().clone(),
                fs[f_id].borrow().clone(),
                prefix_format!(annotation_prefix, " dbl_muls3_{}", dbl_id),
            ));
            f_id += 1;
            dbl_id += 1;

            if NAF[i] != 0 {
                addition_steps1[add_id] = RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(
                    pb.clone(),
                    NAF[i] < 0,
                    prec_P1.clone(),
                    prec_Q1.t.coeffs[prec_id].borrow().clone(),
                    prec_Q1.t.Q.borrow().clone(),
                    g_RQ_at_P1s[add_id].clone(),
                    prefix_format!(annotation_prefix, " addition_steps1_{}", add_id),
                ));
                addition_steps2[add_id] = RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(
                    pb.clone(),
                    NAF[i] < 0,
                    prec_P2.clone(),
                    prec_Q2.t.coeffs[prec_id].borrow().clone(),
                    prec_Q2.t.Q.borrow().clone(),
                    g_RQ_at_P2s[add_id].clone(),
                    prefix_format!(annotation_prefix, " addition_steps2_{}", add_id),
                ));
                addition_steps3[add_id] = RcCell::new(mnt_miller_loop_add_line_eval::<ppT>::new(
                    pb.clone(),
                    NAF[i] < 0,
                    prec_P3.clone(),
                    prec_Q3.t.coeffs[prec_id].borrow().clone(),
                    prec_Q3.t.Q.borrow().clone(),
                    g_RQ_at_P3s[add_id].clone(),
                    prefix_format!(annotation_prefix, " addition_steps3_{}", add_id),
                ));
                prec_id += 1;
                add_muls1[add_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                    pb.clone(),
                    fs[f_id].borrow().clone(),
                    g_RQ_at_P1s[add_id].borrow().clone(),
                    fs[f_id + 1].borrow().clone(),
                    prefix_format!(annotation_prefix, " add_muls1_{}", add_id),
                ));
                f_id += 1;
                add_muls2[add_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                    pb.clone(),
                    fs[f_id].borrow().clone(),
                    g_RQ_at_P2s[add_id].borrow().clone(),
                    fs[f_id + 1].borrow().clone(),
                    prefix_format!(annotation_prefix, " add_muls2_{}", add_id),
                ));
                f_id += 1;
                add_muls3[add_id] = RcCell::new(Fqk_special_mul_gadget::<ppT>::new(
                    pb.clone(),
                    (if f_id + 1 == f_count {
                        result.clone()
                    } else {
                        fs[f_id + 1].borrow().clone()
                    }),
                    g_RQ_at_P3s[add_id].borrow().clone(),
                    fs[f_id].borrow().clone(),
                    prefix_format!(annotation_prefix, " add_muls3_{}", add_id),
                ));
                f_id += 1;
                add_id += 1;
            }
        }
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                g_RR_at_P1s,
                g_RQ_at_P1s,
                g_RR_at_P2s,
                g_RQ_at_P2s,
                g_RR_at_P3s,
                g_RQ_at_P3s,
                fs,

                addition_steps1,
                doubling_steps1,
                addition_steps2,
                doubling_steps2,
                addition_steps3,
                doubling_steps3,

                dbl_sqrs,
                dbl_muls1,
                add_muls1,
                dbl_muls2,
                add_muls2,
                dbl_muls3,
                add_muls3,

                f_count,
                add_count,
                dbl_count,

                prec_P1,
                prec_Q1,
                prec_P2,
                prec_Q2,
                prec_P3,
                prec_Q3,
                result,
            },
        )
    }
}
impl<ppT: ppTConfig> mnt_e_times_e_over_e_miller_loop_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.fs[0]
            .borrow()
            .generate_r1cs_equals_const_constraints(&ppT::FieldT::one());

        for i in 0..self.t.dbl_count {
            self.t.doubling_steps1[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.doubling_steps2[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.doubling_steps3[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.dbl_sqrs[i].borrow().generate_r1cs_constraints();
            self.t.dbl_muls1[i].borrow().generate_r1cs_constraints();
            self.t.dbl_muls2[i].borrow().generate_r1cs_constraints();
            self.t.dbl_muls3[i].borrow().generate_r1cs_constraints();
        }

        for i in 0..self.t.add_count {
            self.t.addition_steps1[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.addition_steps2[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.addition_steps3[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.add_muls1[i].borrow().generate_r1cs_constraints();
            self.t.add_muls2[i].borrow().generate_r1cs_constraints();
            self.t.add_muls3[i].borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.fs[0]
            .borrow()
            .generate_r1cs_witness(&ppT::FieldT::one());

        let mut add_id = 0;
        let mut dbl_id = 0;
        let mut f_id = 0;

        let loop_count = ppT::P::pairing_loop_count;

        let mut found_nonzero = false;
        let mut NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            self.t.doubling_steps1[dbl_id]
                .borrow()
                .generate_r1cs_witness();
            self.t.doubling_steps2[dbl_id]
                .borrow()
                .generate_r1cs_witness();
            self.t.doubling_steps3[dbl_id]
                .borrow()
                .generate_r1cs_witness();
            self.t.dbl_sqrs[dbl_id].borrow().generate_r1cs_witness();
            f_id += 1;
            self.t.dbl_muls1[dbl_id].borrow().generate_r1cs_witness();
            f_id += 1;
            self.t.dbl_muls2[dbl_id].borrow().generate_r1cs_witness();
            f_id += 1;
            (if f_id + 1 == self.t.f_count {
                self.t.result.clone()
            } else {
                self.t.fs[f_id + 1].borrow().clone()
            })
            .generate_r1cs_witness(
                &(self.t.fs[f_id].borrow().get_element()
                    * self.t.g_RR_at_P3s[dbl_id].borrow().get_element().inverse())
                .to_field::<ppT::FieldT>(),
            );
            self.t.dbl_muls3[dbl_id].borrow().generate_r1cs_witness();
            f_id += 1;
            dbl_id += 1;

            if NAF[i] != 0 {
                self.t.addition_steps1[add_id]
                    .borrow()
                    .generate_r1cs_witness();
                self.t.addition_steps2[add_id]
                    .borrow()
                    .generate_r1cs_witness();
                self.t.addition_steps3[add_id]
                    .borrow()
                    .generate_r1cs_witness();
                self.t.add_muls1[add_id].borrow().generate_r1cs_witness();
                f_id += 1;
                self.t.add_muls2[add_id].borrow().generate_r1cs_witness();
                f_id += 1;
                (if f_id + 1 == self.t.f_count {
                    self.t.result.clone()
                } else {
                    self.t.fs[f_id + 1].borrow().clone()
                })
                .generate_r1cs_witness(
                    &((self.t.fs[f_id].borrow().get_element()
                        * self.t.g_RQ_at_P3s[add_id].borrow().get_element().inverse())
                    .to_field::<ppT::FieldT>()),
                );
                self.t.add_muls3[add_id].borrow().generate_r1cs_witness();
                f_id += 1;
                add_id += 1;
            }
        }
    }
}
pub fn test_mnt_e_times_e_over_e_miller_loop<ppT: ppTConfig>(annotation: &String)
where
    <ppT as ff_curves::PublicParams>::G1:
        Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G1>,
    <ppT as ff_curves::PublicParams>::G2:
        Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G2>,
{
    let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
    let mut P1_val = G1::<ppT>::one() * ppT::FieldT::random_element();
    let mut Q1_val = G2::<ppT>::one() * ppT::FieldT::random_element();

    let mut P2_val = G1::<ppT>::one() * ppT::FieldT::random_element();
    let mut Q2_val = G2::<ppT>::one() * ppT::FieldT::random_element();

    let mut P3_val = G1::<ppT>::one() * ppT::FieldT::random_element();
    let mut Q3_val = G2::<ppT>::one() * ppT::FieldT::random_element();

    let mut P1 = G1_variable::<ppT>::new(pb.clone(), "P1".to_owned());
    let mut Q1 = G2_variable::<ppT>::new(pb.clone(), "Q1".to_owned());
    let mut P2 = G1_variable::<ppT>::new(pb.clone(), "P2".to_owned());
    let mut Q2 = G2_variable::<ppT>::new(pb.clone(), "Q2".to_owned());
    let mut P3 = G1_variable::<ppT>::new(pb.clone(), "P3".to_owned());
    let mut Q3 = G2_variable::<ppT>::new(pb.clone(), "Q3".to_owned());

    let mut prec_P1 = G1_precomputations::<ppT>::default();
    let mut compute_prec_P1 = precompute_G1_gadget::<ppT>::new(
        pb.clone(),
        P1.clone(),
        prec_P1.clone(),
        "compute_prec_P1".to_owned(),
    );
    let mut prec_P2 = G1_precomputations::<ppT>::default();
    let mut compute_prec_P2 = precompute_G1_gadget::<ppT>::new(
        pb.clone(),
        P2.clone(),
        prec_P2.clone(),
        "compute_prec_P2".to_owned(),
    );
    let mut prec_P3 = G1_precomputations::<ppT>::default();
    let mut compute_prec_P3 = precompute_G1_gadget::<ppT>::new(
        pb.clone(),
        P3.clone(),
        prec_P3.clone(),
        "compute_prec_P3".to_owned(),
    );
    let mut prec_Q1 = G2_precomputations::<ppT>::default();
    let mut compute_prec_Q1 = precompute_G2_gadget::<ppT>::new(
        pb.clone(),
        Q1.clone(),
        prec_Q1.clone(),
        "compute_prec_Q1".to_owned(),
    );
    let mut prec_Q2 = G2_precomputations::<ppT>::default();
    let mut compute_prec_Q2 = precompute_G2_gadget::<ppT>::new(
        pb.clone(),
        Q2.clone(),
        prec_Q2.clone(),
        "compute_prec_Q2".to_owned(),
    );
    let mut prec_Q3 = G2_precomputations::<ppT>::default();
    let mut compute_prec_Q3 = precompute_G2_gadget::<ppT>::new(
        pb.clone(),
        Q3.clone(),
        prec_Q3.clone(),
        "compute_prec_Q3".to_owned(),
    );

    let mut result = Fqk_variable::<ppT>::new(pb.clone(), "result".to_owned());
    let mut miller = mnt_e_times_e_over_e_miller_loop_gadget::<ppT>::new(
        pb.clone(),
        prec_P1.clone(),
        prec_Q1.clone(),
        prec_P2.clone(),
        prec_Q2.clone(),
        prec_P3.clone(),
        prec_Q3.clone(),
        result.clone(),
        "miller".to_owned(),
    );

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

    P1.generate_r1cs_witness(&P1_val);
    compute_prec_P1.generate_r1cs_witness();
    Q1.generate_r1cs_witness(&Q1_val);
    compute_prec_Q1.generate_r1cs_witness();
    P2.generate_r1cs_witness(&P2_val);
    compute_prec_P2.generate_r1cs_witness();
    Q2.generate_r1cs_witness(&Q2_val);
    compute_prec_Q2.generate_r1cs_witness();
    P3.generate_r1cs_witness(&P3_val);
    compute_prec_P3.generate_r1cs_witness();
    Q3.generate_r1cs_witness(&Q3_val);
    compute_prec_Q3.generate_r1cs_witness();
    miller.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    let mut native_prec_P1 = ppT::affine_ate_precompute_G1(&P1_val);
    let mut native_prec_Q1 = ppT::affine_ate_precompute_G2(&Q1_val);
    let mut native_prec_P2 = ppT::affine_ate_precompute_G1(&P2_val);
    let mut native_prec_Q2 = ppT::affine_ate_precompute_G2(&Q2_val);
    let mut native_prec_P3 = ppT::affine_ate_precompute_G1(&P3_val);
    let mut native_prec_Q3 = ppT::affine_ate_precompute_G2(&Q3_val);
    let mut native_result = ppT::affine_ate_miller_loop(&native_prec_P1, &native_prec_Q1)
        * ppT::affine_ate_miller_loop(&native_prec_P2, &native_prec_Q2)
        * ppT::affine_ate_miller_loop(&native_prec_P3, &native_prec_Q3).inverse();

    assert!(result.get_element() == native_result);
    print!(
        "number of constraints for e times e over e Miller loop (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}
