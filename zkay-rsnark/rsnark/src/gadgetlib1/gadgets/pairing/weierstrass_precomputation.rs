// Declaration of interfaces for pairing precomputation gadgets.
// The gadgets verify correct precomputation of values for the G1 and G2 variables.

use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget::{G1_variable, G1_variables};
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget::{G2_variable, G2_variables};
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fpk_variableT, Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, MulTConfig, SqrTConfig,
    VariableTConfig, pairing_selector, ppTConfig,
};
use ff_curves::algebra::curves::mnt::mnt4::mnt4_init;
use ff_curves::algebra::curves::mnt::mnt6::mnt6_init;

use crate::gadgetlib1::constraint_profiling::PRINT_CONSTRAINT_PROFILING;
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
use ff_curves::{
    CoeffsConfig, FpmConfig, Fqe, Fqk, Fr, G1, G2, PublicParams, affine_ate_G_precomp_typeConfig,
};
use ffec::PpConfig;
use ffec::scalar_multiplication::multiexp::KCConfig;
use ffec::scalar_multiplication::wnaf::find_wnaf_u;
use std::ops::Mul;

use ffec::field_utils::bigint::bigint;
use ffec::{FieldTConfig, One, Zero};
use rccell::RcCell;
use std::marker::PhantomData;
use std::ops::Add;
// pub const pairing_loop_count: bigint<4> = bigint::<4>::one();
// pub fn affine_ate_precompute_G2<ppT: ppTConfig>(f: ppT) -> ppT {
//     f
// }
// pub fn affine_ate_precompute_G1<ppT: ppTConfig>(f: ppT) -> ppT {
//     f
// }
// pub fn affine_ate_miller_loop<ppT: ppTConfig>(f: ppT, f1: ppT) -> ppT {
//     f
// }

fn mnt4_twist<ppT: ppTConfig>() -> ppT {
    ppT::zero()
}
fn mnt6_twist<ppT: ppTConfig>() -> ppT {
    ppT::zero()
}
/**************************** G1 Precomputation ******************************/

type FieldT<ppT> = Fr<ppT>;
type FqeT<ppT> = Fqe<other_curve<ppT>>;
type FqkT<ppT> = Fqk<other_curve<ppT>>;

/**
 * Not a gadget. It only holds values.
 */
#[derive(Clone, Default)]
pub struct G1_precomputation<ppT: ppTConfig> {
    pub P: RcCell<G1_variables<ppT>>,
    pub PY_twist_squared: RcCell<Fqe_variable<ppT>>,
}

/**
 * Gadget that verifies correct precomputation of the G1 variable.
 */
#[derive(Clone, Default)]
pub struct precompute_G1_gadget<ppT: ppTConfig> {
    //gadget<Fr<ppT> >

    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub precomp: G1_precomputations<ppT>, // must be a reference.
}

/**************************** G2 Precomputation ******************************/

/**
 * Not a gadget. It only holds values.
 */
#[derive(Clone, Default)]
pub struct precompute_G2_gadget_coeffs<ppT: ppTConfig> {
    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub RX: RcCell<Fqe_variable<ppT>>,
    pub RY: RcCell<Fqe_variable<ppT>>,
    pub gamma: RcCell<Fqe_variable<ppT>>,
    pub gamma_X: RcCell<Fqe_variable<ppT>>,
}

/**
 * Not a gadget. It only holds values.
 */
#[derive(Clone, Default)]
pub struct G2_precomputation<ppT: ppTConfig> {
    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub Q: RcCell<G2_variables<ppT>>,

    pub coeffs: Vec<RcCell<precompute_G2_gadget_coeffss<ppT>>>,
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
#[derive(Clone, Default)]
pub struct precompute_G2_gadget_doubling_step<ppT: ppTConfig> {
    //gadget<Fr<ppT> >

    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub cur: precompute_G2_gadget_coeffss<ppT>,
    pub next: precompute_G2_gadget_coeffss<ppT>,

    pub RXsquared: RcCell<Fqe_variable<ppT>>,
    pub compute_RXsquared: RcCell<Fqe_sqr_gadget<ppT>>,
    pub three_RXsquared_plus_a: RcCell<Fqe_variable<ppT>>,
    pub two_RY: RcCell<Fqe_variable<ppT>>,
    pub compute_gamma: RcCell<Fqe_mul_gadget<ppT>>,
    pub compute_gamma_X: RcCell<Fqe_mul_gadget<ppT>>,

    pub next_RX_plus_two_RX: RcCell<Fqe_variable<ppT>>,
    pub compute_next_RX: RcCell<Fqe_sqr_gadget<ppT>>,

    pub RX_minus_next_RX: RcCell<Fqe_variable<ppT>>,
    pub RY_plus_next_RY: RcCell<Fqe_variable<ppT>>,
    pub compute_next_RY: RcCell<Fqe_mul_gadget<ppT>>,
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
#[derive(Clone, Default)]
pub struct precompute_G2_gadget_addition_step<ppT: ppTConfig> {
    //gadget<Fr<ppT> >

    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub invert_Q: bool,
    pub cur: precompute_G2_gadget_coeffss<ppT>,
    pub next: precompute_G2_gadget_coeffss<ppT>,
    pub Q: G2_variables<ppT>,

    pub RY_minus_QY: RcCell<Fqe_variable<ppT>>,
    pub RX_minus_QX: RcCell<Fqe_variable<ppT>>,
    pub compute_gamma: RcCell<Fqe_mul_gadget<ppT>>,
    pub compute_gamma_X: RcCell<Fqe_mul_gadget<ppT>>,

    pub next_RX_plus_RX_plus_QX: RcCell<Fqe_variable<ppT>>,
    pub compute_next_RX: RcCell<Fqe_sqr_gadget<ppT>>,

    pub RX_minus_next_RX: RcCell<Fqe_variable<ppT>>,
    pub RY_plus_next_RY: RcCell<Fqe_variable<ppT>>,
    pub compute_next_RY: RcCell<Fqe_mul_gadget<ppT>>,
}

/**
 * Gadget that verifies correct precomputation of the G2 variable.
 */
#[derive(Clone, Default)]
pub struct precompute_G2_gadget<ppT: ppTConfig> {
    //gadget<Fr<ppT> >

    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub addition_steps: Vec<RcCell<precompute_G2_gadget_addition_steps<ppT>>>,
    pub doubling_steps: Vec<RcCell<precompute_G2_gadget_doubling_steps<ppT>>>,

    pub add_count: usize,
    pub dbl_count: usize,

    pub precomp: G2_precomputations<ppT>, // important to have a reference here
}

// use  <type_traits>
// use crate::gadgetlib1::gadgets::pairing::mnt_pairing_params;

pub type G1_precomputations<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G1_precomputation<ppT>>;

impl<ppT: ppTConfig> G1_precomputation<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        P_val: G1<other_curve<ppT>>,
        annotation_prefix: String,
    ) -> G1_precomputations<ppT>
// where
    //     <ppT::other_curve_type as ff_curves::PublicParams>::Fr: Mul<
    //             <ppT as ff_curves::PublicParams>::Fr,
    //             Output = <ppT::other_curve_type as ff_curves::PublicParams>::Fr,
    //         >,
    {
        let mut P_val_copy = P_val.clone();
        P_val_copy.to_affine_coordinates();
        let P = RcCell::new(G1_variable::<ppT>::new2(
            pb.clone(),
            P_val_copy.clone(),
            prefix_format!(annotation_prefix, " P"),
        ));
        let PY_twist_squared = RcCell::new(Fqe_variable::<ppT>::new2(
            pb.clone(),
            P_val_copy.Y() * G2::<ppT>::twist().squared(),
            " PY_twist_squared".to_owned(),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                P,
                PY_twist_squared,
            },
        )
    }
}

pub type precompute_G1_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, precompute_G1_gadget<ppT>>;
impl<ppT: ppTConfig> precompute_G1_gadget<ppT> {
    /* two possible pre-computations one for mnt4 and one for mnt6 */
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        P: G1_variables<ppT>,
        mut precomp: G1_precomputations<ppT>, // will allocate this inside
        annotation_prefix: String,
    ) -> precompute_G1_gadgets<ppT> {
        // 4:std::enable_if<Fqk<other_curve::<ppT> >::extension_degree() ==, FieldT>::type& = FieldT()
        let (mut c0, mut c1) = (
            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::default(),
        );
        c0.assign(
            &pb,
            &(P.t.Y.clone() * (mnt4_twist::<ppT>().squared().c0()).to_field::<ppT::FieldT>()),
        );
        c1.assign(
            &pb,
            &(P.t.Y.clone() * (mnt4_twist::<ppT>().squared().c1()).to_field::<ppT::FieldT>()),
        );

        precomp.t.P = RcCell::new(P.clone());
        precomp.t.PY_twist_squared = RcCell::new(Fqe_variable::<ppT>::new22(
            pb.clone(),
            c0,
            c1,
            prefix_format!(annotation_prefix, " PY_twist_squared"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(pb, annotation_prefix, Self { precomp })
    }

    pub fn new2(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        P: G1_variables<ppT>,
        mut precomp: G1_precomputations<ppT>, // will allocate this inside
        annotation_prefix: String,
    ) -> precompute_G1_gadgets<ppT> {
        // 6:std::enable_if<Fqk<other_curve::<ppT> >::extension_degree() ==, FieldT>::type& = FieldT()
        let (mut c0, mut c1, mut c2) = (
            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::default(),
        );
        c0.assign(
            &pb,
            &(P.t.Y.clone() * (mnt6_twist::<ppT>().squared().c0()).to_field::<ppT::FieldT>()),
        );
        c1.assign(
            &pb,
            &(P.t.Y.clone() * (mnt6_twist::<ppT>().squared().c1()).to_field::<ppT::FieldT>()),
        );
        c2.assign(
            &pb,
            &(P.t.Y.clone() * (mnt6_twist::<ppT>().squared().c2()).to_field::<ppT::FieldT>()),
        );

        precomp.t.P = RcCell::new(P.clone());
        precomp.t.PY_twist_squared = RcCell::new(Fqe_variable::<ppT>::new3(
            pb.clone(),
            c0,
            c1,
            c2,
            prefix_format!(annotation_prefix, " PY_twist_squared"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(pb, annotation_prefix, Self { precomp })
    }
}

impl<ppT: ppTConfig> precompute_G1_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        /* the same for neither ppT = mnt4 nor ppT = mnt6 */
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.precomp.t.PY_twist_squared.borrow().evaluate(); /* the same for both ppT = mnt4 and ppT = mnt6 */
    }
}

pub fn test_G1_variable_precomp<ppT: ppTConfig + std::cmp::PartialEq<Fpk_variableT<ppT>>>(
    annotation: &String,
)
// where
// <ppT as ff_curves::PublicParams>::G1:
//     Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G1>,
// <ppT as ff_curves::PublicParams>::G2:
//     Mul<ppT::FieldT, Output = <ppT as ff_curves::PublicParams>::G2>,
{
    let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
    let mut g_val = ppT::FieldT::random_element() * G1::<ppT>::one();

    let mut g = G1_variable::<ppT>::new(pb.clone(), "g".to_owned());
    let mut precomp = G1_precomputations::<ppT>::default();
    let mut do_precomp = precompute_G1_gadget::<ppT>::new(
        pb.clone(),
        g.clone(),
        precomp.clone(),
        "do_precomp".to_owned(),
    );
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(&g_val);
    do_precomp.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    let mut const_precomp =
        G1_precomputation::<ppT>::new(pb.clone(), g_val.clone(), "const_precomp".to_owned());

    let native_precomp = ppT::affine_ate_precompute_G1(&g_val);
    assert!(
        precomp.t.PY_twist_squared.borrow().get_element()
            == native_precomp.PY_twist_squared::<ppT::FieldT>()
    );
    assert!(
        const_precomp.t.PY_twist_squared.borrow().get_element()
            == native_precomp.PY_twist_squared::<ppT::FieldT>()
    );

    print!(
        "number of constraints for G1 precomp (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}

pub type G2_precomputations<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G2_precomputation<ppT>>;

impl<ppT: ppTConfig> G2_precomputation<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        Q_val: G2<other_curve<ppT>>,
        annotation_prefix: String,
    ) -> G2_precomputations<ppT> {
        let Q = RcCell::new(G2_variable::<ppT>::new2(
            pb.clone(),
            Q_val.clone(),
            prefix_format!(annotation_prefix, " Q"),
        ));
        let native_precomp = other_curve::<ppT>::affine_ate_precompute_G2(&Q_val);

        let coeffs = vec![
            RcCell::new(precompute_G2_gadget_coeffss::<ppT>::default());
            native_precomp.coeffs().len() + 1
        ]; // the last precomp remains for convenient programming
        for i in 0..native_precomp.coeffs().len() {
            coeffs[i].borrow_mut().t.RX = RcCell::new(Fqe_variable::<ppT>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].old_RX::<ppT::FieldT>().clone(),
                prefix_format!(annotation_prefix, " RX"),
            ));
            coeffs[i].borrow_mut().t.RY = RcCell::new(Fqe_variable::<ppT>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].old_RY::<ppT::FieldT>().clone(),
                prefix_format!(annotation_prefix, " RY"),
            ));
            coeffs[i].borrow_mut().t.gamma = RcCell::new(Fqe_variable::<ppT>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].gamma::<ppT::FieldT>().clone(),
                prefix_format!(annotation_prefix, " gamma"),
            ));
            coeffs[i].borrow_mut().t.gamma_X = RcCell::new(Fqe_variable::<ppT>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].gamma_X::<ppT::FieldT>().clone(),
                prefix_format!(annotation_prefix, " gamma_X"),
            ));
        }

        gadget::<ppT::FieldT, ppT::PB, Self>::new(pb, annotation_prefix, Self { Q, coeffs })
    }
}

pub type precompute_G2_gadget_coeffss<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, precompute_G2_gadget_coeffs<ppT>>;

impl<ppT: ppTConfig> precompute_G2_gadget_coeffs<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_coeffss<ppT> {
        let RX = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " RX"),
        ));
        let RY = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " RY"),
        ));
        let gamma = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma"),
        ));
        let gamma_X = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma_X"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                RX,
                RY,
                gamma,
                gamma_X,
            },
        )
    }

    pub fn new2(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        Q: G2_variables<ppT>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_coeffss<ppT> {
        let RX = Q.t.X.clone();
        let RY = Q.t.Y.clone();
        let gamma = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma"),
        ));
        let gamma_X = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma_X"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                RX,
                RY,
                gamma,
                gamma_X,
            },
        )
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
pub type precompute_G2_gadget_doubling_steps<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    precompute_G2_gadget_doubling_step<ppT>,
>;

impl<ppT: ppTConfig> precompute_G2_gadget_doubling_step<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        cur: precompute_G2_gadget_coeffss<ppT>,
        next: precompute_G2_gadget_coeffss<ppT>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_doubling_steps<ppT> {
        let RXsquared = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " RXsquared"),
        ));
        let compute_RXsquared = RcCell::new(Fqe_sqr_gadget::<ppT>::new(
            pb.clone(),
            cur.t.RX.clone(),
            RXsquared.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_RXsquared"),
        ));

        let three_RXsquared_plus_a = RcCell::new(
            RXsquared.borrow().clone() * ppT::FieldT::from(3) + G2::<other_curve<ppT>>::coeff_a,
        );
        let two_RY = RcCell::new(cur.t.RY.borrow().clone() * ppT::FieldT::from(2));

        let compute_gamma = RcCell::new(Fqe_mul_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            two_RY.borrow().clone(),
            three_RXsquared_plus_a.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma"),
        ));
        let compute_gamma_X = RcCell::new(Fqe_mul_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            cur.t.RX.borrow().clone(),
            cur.t.gamma_X.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma_X"),
        ));

        let next_RX_plus_two_RX = RcCell::new(
            next.t.RX.borrow().clone() + cur.t.RX.borrow().clone() * ppT::FieldT::from(2),
        );
        let compute_next_RX = RcCell::new(Fqe_sqr_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.clone(),
            next_RX_plus_two_RX.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RX"),
        ));

        let RX_minus_next_RX = RcCell::new(
            cur.t.RX.borrow().clone() + next.t.RX.borrow().clone() * (-ppT::FieldT::one()),
        );
        let RY_plus_next_RY = RcCell::new(cur.t.RY.borrow().clone() + next.t.RY.borrow().clone());
        let compute_next_RY = RcCell::new(Fqe_mul_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            RX_minus_next_RX.borrow().clone(),
            RY_plus_next_RY.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RY"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb.clone(),
            annotation_prefix,
            Self {
                cur,
                next,
                RXsquared,
                compute_RXsquared,
                three_RXsquared_plus_a,
                two_RY,
                compute_gamma,
                compute_gamma_X,
                next_RX_plus_two_RX,
                compute_next_RX,
                RX_minus_next_RX,
                RY_plus_next_RY,
                compute_next_RY,
            },
        )
    }
}
impl<ppT: ppTConfig> precompute_G2_gadget_doubling_steps<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .compute_RXsquared
            .borrow()
            .generate_r1cs_constraints();
        self.t.compute_gamma.borrow().generate_r1cs_constraints();
        self.t.compute_gamma_X.borrow().generate_r1cs_constraints();
        self.t.compute_next_RX.borrow().generate_r1cs_constraints();
        self.t.compute_next_RY.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_RXsquared.borrow().generate_r1cs_witness();
        self.t.two_RY.borrow().evaluate();
        self.t.three_RXsquared_plus_a.borrow().evaluate();

        let three_RXsquared_plus_a_val = self.t.three_RXsquared_plus_a.borrow().get_element();
        let two_RY_val = self.t.two_RY.borrow().get_element();
        let gamma_val = three_RXsquared_plus_a_val * two_RY_val.inverse();
        self.t
            .cur
            .t
            .gamma
            .borrow()
            .generate_r1cs_witness(&gamma_val.to_field::<ppT::FieldT>());

        self.t.compute_gamma.borrow().generate_r1cs_witness();
        self.t.compute_gamma_X.borrow().generate_r1cs_witness();

        let RX_val = self.t.cur.t.RX.borrow().get_element();
        let RY_val = self.t.cur.t.RY.borrow().get_element();
        let next_RX_val = gamma_val.squared() - RX_val.clone() - RX_val.clone();
        let next_RY_val = gamma_val * (RX_val.clone() - next_RX_val.clone()) - RY_val.clone();

        self.t
            .next
            .t
            .RX
            .borrow()
            .generate_r1cs_witness(&next_RX_val.to_field::<ppT::FieldT>());
        self.t
            .next
            .t
            .RY
            .borrow()
            .generate_r1cs_witness(&next_RY_val.to_field::<ppT::FieldT>());

        self.t.RX_minus_next_RX.borrow().evaluate();
        self.t.RY_plus_next_RY.borrow().evaluate();

        self.t.compute_next_RX.borrow().generate_r1cs_witness();
        self.t.compute_next_RY.borrow().generate_r1cs_witness();
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

pub type precompute_G2_gadget_addition_steps<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    precompute_G2_gadget_addition_step<ppT>,
>;

impl<ppT: ppTConfig> precompute_G2_gadget_addition_step<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        invert_Q: bool,
        cur: precompute_G2_gadget_coeffss<ppT>,
        next: precompute_G2_gadget_coeffss<ppT>,
        Q: G2_variables<ppT>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_addition_steps<ppT> {
        let RY_minus_QY = RcCell::new(
            cur.t.RY.borrow().clone()
                + Q.t.Y.borrow().clone()
                    * (if !invert_Q {
                        -ppT::FieldT::one()
                    } else {
                        ppT::FieldT::one()
                    }),
        );

        let RX_minus_QX =
            RcCell::new(cur.t.RX.borrow().clone() + Q.t.X.borrow().clone() * (-ppT::FieldT::one()));
        let compute_gamma = RcCell::new(Fqe_mul_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            RX_minus_QX.borrow().clone(),
            RY_minus_QY.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma"),
        ));
        let compute_gamma_X = RcCell::new(Fqe_mul_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            Q.t.X.borrow().clone(),
            cur.t.gamma_X.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma_X"),
        ));

        let next_RX_plus_RX_plus_QX = RcCell::new(
            next.t.RX.borrow().clone() + cur.t.RX.borrow().clone() + Q.t.X.borrow().clone(),
        );
        let compute_next_RX = RcCell::new(Fqe_sqr_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.clone(),
            next_RX_plus_RX_plus_QX.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RX"),
        ));

        let RX_minus_next_RX = RcCell::new(
            cur.t.RX.borrow().clone() + next.t.RX.borrow().clone() * (-ppT::FieldT::one()),
        );
        let RY_plus_next_RY = RcCell::new(cur.t.RY.borrow().clone() + next.t.RY.borrow().clone());
        let compute_next_RY = RcCell::new(Fqe_mul_gadget::<ppT>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            RX_minus_next_RX.borrow().clone(),
            RY_plus_next_RY.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RY"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                invert_Q,
                cur,
                next,
                Q,
                RY_minus_QY,
                RX_minus_QX,
                compute_gamma,
                compute_gamma_X,

                next_RX_plus_RX_plus_QX,
                compute_next_RX,

                RX_minus_next_RX,
                RY_plus_next_RY,
                compute_next_RY,
            },
        )
    }
}
impl<ppT: ppTConfig> precompute_G2_gadget_addition_steps<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_gamma.borrow().generate_r1cs_constraints();
        self.t.compute_gamma_X.borrow().generate_r1cs_constraints();
        self.t.compute_next_RX.borrow().generate_r1cs_constraints();
        self.t.compute_next_RY.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.RY_minus_QY.borrow().evaluate();
        self.t.RX_minus_QX.borrow().evaluate();

        let RY_minus_QY_val = self.t.RY_minus_QY.borrow().get_element();
        let RX_minus_QX_val = self.t.RX_minus_QX.borrow().get_element();
        let gamma_val = RY_minus_QY_val * RX_minus_QX_val.inverse();
        self.t
            .cur
            .t
            .gamma
            .borrow()
            .generate_r1cs_witness(&gamma_val.to_field::<ppT::FieldT>());

        self.t.compute_gamma.borrow().generate_r1cs_witness();
        self.t.compute_gamma_X.borrow().generate_r1cs_witness();

        let RX_val = self.t.cur.t.RX.borrow().get_element();
        let RY_val = self.t.cur.t.RY.borrow().get_element();
        let QX_val = self.t.Q.t.X.borrow().get_element();
        let next_RX_val = gamma_val.squared() - RX_val.clone() - QX_val.clone();
        let next_RY_val = gamma_val * (RX_val.clone() - next_RX_val.clone()) - RY_val.clone();

        self.t
            .next
            .t
            .RX
            .borrow()
            .generate_r1cs_witness(&next_RX_val.to_field::<ppT::FieldT>());
        self.t
            .next
            .t
            .RY
            .borrow()
            .generate_r1cs_witness(&next_RY_val.to_field::<ppT::FieldT>());

        self.t.next_RX_plus_RX_plus_QX.borrow().evaluate();
        self.t.RX_minus_next_RX.borrow().evaluate();
        self.t.RY_plus_next_RY.borrow().evaluate();

        self.t.compute_next_RX.borrow().generate_r1cs_witness();
        self.t.compute_next_RY.borrow().generate_r1cs_witness();
    }
}

pub type precompute_G2_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, precompute_G2_gadget<ppT>>;

impl<ppT: ppTConfig> precompute_G2_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        Q: G2_variables<ppT>,
        mut precomp: G2_precomputations<ppT>, // will allocate this inside
        annotation_prefix: String,
    ) -> precompute_G2_gadgets<ppT> {
        precomp.t.Q = RcCell::new(Q.clone());

        let mut loop_count = ppT::P::pairing_loop_count;
        let mut coeff_count = 1; // the last RX/RY are unused in Miller loop, but will need to get allocated somehow
        let mut add_count = 0;
        let mut dbl_count = 0;

        let mut found_nonzero = false;
        let NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            dbl_count += 1;
            coeff_count += 1;

            if NAF[i] != 0 {
                add_count += 1;
                coeff_count += 1;
            }
        }

        precomp.t.coeffs.resize(
            coeff_count,
            RcCell::new(precompute_G2_gadget_coeffss::<ppT>::default()),
        );
        let mut addition_steps =
            vec![RcCell::new(precompute_G2_gadget_addition_steps::<ppT>::default()); add_count];
        let mut doubling_steps =
            vec![RcCell::new(precompute_G2_gadget_doubling_steps::<ppT>::default()); dbl_count];

        precomp.t.coeffs[0] = RcCell::new(precompute_G2_gadget_coeffs::<ppT>::new2(
            pb.clone(),
            Q.clone(),
            prefix_format!(annotation_prefix, " coeffs_0"),
        ));
        for i in 1..coeff_count {
            precomp.t.coeffs[i] = RcCell::new(precompute_G2_gadget_coeffs::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " coeffs_{}", i),
            ));
        }

        let mut add_id = 0;
        let mut dbl_id = 0;
        let mut coeff_id = 0;

        found_nonzero = false;
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            doubling_steps[dbl_id] = RcCell::new(precompute_G2_gadget_doubling_step::<ppT>::new(
                pb.clone(),
                precomp.t.coeffs[coeff_id].borrow().clone(),
                precomp.t.coeffs[coeff_id + 1].borrow().clone(),
                prefix_format!(annotation_prefix, " doubling_steps_{}", dbl_id),
            ));
            dbl_id += 1;
            coeff_id += 1;

            if NAF[i] != 0 {
                addition_steps[add_id] =
                    RcCell::new(precompute_G2_gadget_addition_step::<ppT>::new(
                        pb.clone(),
                        NAF[i] < 0,
                        precomp.t.coeffs[coeff_id].borrow().clone(),
                        precomp.t.coeffs[coeff_id + 1].borrow().clone(),
                        Q.clone(),
                        prefix_format!(annotation_prefix, " addition_steps_{}", add_id),
                    ));
                add_id += 1;
                coeff_id += 1;
            }
        }
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                addition_steps,
                doubling_steps,
                add_count,
                dbl_count,
                precomp,
            },
        )
    }
}
impl<ppT: ppTConfig> precompute_G2_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..self.t.dbl_count {
            self.t.doubling_steps[i]
                .borrow()
                .generate_r1cs_constraints();
        }

        for i in 0..self.t.add_count {
            self.t.addition_steps[i]
                .borrow()
                .generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.precomp.t.coeffs[0]
            .borrow()
            .t
            .RX
            .borrow()
            .generate_r1cs_witness(
                &self
                    .t
                    .precomp
                    .t
                    .Q
                    .borrow()
                    .t
                    .X
                    .borrow()
                    .get_element()
                    .to_field::<ppT::FieldT>(),
            );
        self.t.precomp.t.coeffs[0]
            .borrow()
            .t
            .RY
            .borrow()
            .generate_r1cs_witness(
                &self
                    .t
                    .precomp
                    .t
                    .Q
                    .borrow()
                    .t
                    .Y
                    .borrow()
                    .get_element()
                    .to_field::<ppT::FieldT>(),
            );

        let mut loop_count = ppT::P::pairing_loop_count;

        let mut add_id = 0;
        let mut dbl_id = 0;

        let mut found_nonzero = false;
        let NAF = find_wnaf_u(1, loop_count);
        for i in (0..=NAF.len() - 1).rev() {
            if !found_nonzero {
                /* this skips the MSB itself */
                found_nonzero |= (NAF[i] != 0);
                continue;
            }

            self.t.doubling_steps[dbl_id]
                .borrow()
                .generate_r1cs_witness();
            dbl_id += 1;

            if NAF[i] != 0 {
                self.t.addition_steps[add_id]
                    .borrow()
                    .generate_r1cs_witness();
                add_id += 1;
            }
        }
    }
}
pub fn test_G2_variable_precomp<ppT: ppTConfig + std::cmp::PartialEq<FieldT<ppT>>>(
    annotation: &String,
)
// where
// <ppT as ff_curves::PublicParams>::G2:
//     Mul<<ppT as ff_curves::PublicParams>::Fr, Output = <ppT as ff_curves::PublicParams>::G2>,
// <ppT::other_curve_type as ff_curves::PublicParams>::G2: Mul<
//         <ppT::other_curve_type as ff_curves::PublicParams>::Fr,
//         Output = <ppT::other_curve_type as ff_curves::PublicParams>::G2,
//     >,
{
    let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
    let g_val = G2::<other_curve<ppT>>::one() * FieldT::<other_curve<ppT>>::random_element();

    let mut g = G2_variable::<ppT>::new(pb.clone(), "g".to_owned());
    let mut precomp = G2_precomputations::<ppT>::default();
    let mut do_precomp = precompute_G2_gadget::<ppT>::new(
        pb.clone(),
        g.clone(),
        precomp.clone(),
        "do_precomp".to_owned(),
    );
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(&g_val);
    do_precomp.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    let native_precomp = other_curve::<ppT>::affine_ate_precompute_G2(&g_val);

    assert!(precomp.t.coeffs.len() - 1 == native_precomp.coeffs().len()); // the last precomp is unused, but remains for convenient programming
    for i in 0..native_precomp.coeffs().len() {
        assert!(
            precomp.t.coeffs[i].borrow().t.RX.borrow().get_element()
                == native_precomp.coeffs()[i].old_RX::<ppT::FieldT>()
        );
        assert!(
            precomp.t.coeffs[i].borrow().t.RY.borrow().get_element()
                == native_precomp.coeffs()[i].old_RY::<ppT::FieldT>()
        );
        assert!(
            precomp.t.coeffs[i].borrow().t.gamma.borrow().get_element()
                == native_precomp.coeffs()[i].gamma::<ppT::FieldT>()
        );
        assert!(
            precomp.t.coeffs[i]
                .borrow()
                .t
                .gamma_X
                .borrow()
                .get_element()
                == native_precomp.coeffs()[i].gamma_X::<ppT::FieldT>()
        );
    }

    print!(
        "number of constraints for G2 precomp (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}
