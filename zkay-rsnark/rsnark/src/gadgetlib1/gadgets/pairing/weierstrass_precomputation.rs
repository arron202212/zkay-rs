// Declaration of interfaces for pairing precomputation gadgets.
// The gadgets verify correct precomputation of values for the G1 and G2 variables.

use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget::{G1_variable, G1_variables};
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget::{G2_variable, G2_variables};
use ff_curves::algebra::curves::mnt::mnt4::mnt4_init;
use ff_curves::algebra::curves::mnt::mnt6::mnt6_init;
use ffec::scalar_multiplication::wnaf::find_wnaf;
// use crate::gadgetlib1::gadgets::pairing::pairing_params::{Fqe_variable,Fqe_mul_gadget,Fqe_sqr_gadget};
use crate::gadgetlib1::gadgets::curves::{
    Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, G1, G2, MulTConfig, SqrTConfig, VariableTConfig,
    ppTConfig,
};

use crate::gadgetlib1::constraint_profiling::PRINT_CONSTRAINT_PROFILING;
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params::other_curve;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::prefix_format;
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ff_curves::algebra::curves::public_params;
use ffec::field_utils::bigint::bigint;
use ffec::{One, Zero};
use rccell::RcCell;
use std::marker::PhantomData;
use std::ops::Add;
pub const pairing_loop_count: bigint<4> = bigint::<4>::one();
pub fn affine_ate_precompute_G2<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>(
    f: ppT,
) -> ppT {
    f
}
pub fn affine_ate_precompute_G1<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>(
    f: ppT,
) -> ppT {
    f
}
pub fn affine_ate_miller_loop<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>(
    f: ppT,
    f1: ppT,
) -> ppT {
    f
}

fn mnt4_twist<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>() -> ppT {
    ppT::zero()
}
fn mnt6_twist<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>() -> ppT {
    ppT::zero()
}
/**************************** G1 Precomputation ******************************/

/**
 * Not a gadget. It only holds values.
 */
#[derive(Clone, Default)]
pub struct G1_precomputation<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub P: RcCell<G1_variables<ppT, FieldT, PB>>,
    pub PY_twist_squared: RcCell<Fqe_variable<ppT, FieldT, PB>>,
}

/**
 * Gadget that verifies correct precomputation of the G1 variable.
 */
#[derive(Clone, Default)]
pub struct precompute_G1_gadget<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<Fr<ppT> >

    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub precomp: G1_precomputations<ppT, FieldT, PB>, // must be a reference.
}

/**************************** G2 Precomputation ******************************/

/**
 * Not a gadget. It only holds values.
 */
#[derive(Clone, Default)]
pub struct precompute_G2_gadget_coeffs<
    ppT: ppTConfig<FieldT, PB>,
    FieldT: FieldTConfig,
    PB: PBConfig,
> {
    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub RX: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub RY: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub gamma: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub gamma_X: RcCell<Fqe_variable<ppT, FieldT, PB>>,
}

/**
 * Not a gadget. It only holds values.
 */
#[derive(Clone, Default)]
pub struct G2_precomputation<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub Q: RcCell<G2_variables<ppT, FieldT, PB>>,

    pub coeffs: Vec<RcCell<precompute_G2_gadget_coeffss<ppT, FieldT, PB>>>,
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
pub struct precompute_G2_gadget_doubling_step<
    ppT: ppTConfig<FieldT, PB>,
    FieldT: FieldTConfig,
    PB: PBConfig,
> {
    //gadget<Fr<ppT> >

    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub cur: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,
    pub next: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,

    pub RXsquared: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_RXsquared: RcCell<Fqe_sqr_gadget<ppT, FieldT, PB>>,
    pub three_RXsquared_plus_a: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub two_RY: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_gamma: RcCell<Fqe_mul_gadget<ppT, FieldT, PB>>,
    pub compute_gamma_X: RcCell<Fqe_mul_gadget<ppT, FieldT, PB>>,

    pub next_RX_plus_two_RX: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_next_RX: RcCell<Fqe_sqr_gadget<ppT, FieldT, PB>>,

    pub RX_minus_next_RX: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub RY_plus_next_RY: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_next_RY: RcCell<Fqe_mul_gadget<ppT, FieldT, PB>>,
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
pub struct precompute_G2_gadget_addition_step<
    ppT: ppTConfig<FieldT, PB>,
    FieldT: FieldTConfig,
    PB: PBConfig,
> {
    //gadget<Fr<ppT> >

    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub invert_Q: bool,
    pub cur: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,
    pub next: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,
    pub Q: G2_variables<ppT, FieldT, PB>,

    pub RY_minus_QY: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub RX_minus_QX: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_gamma: RcCell<Fqe_mul_gadget<ppT, FieldT, PB>>,
    pub compute_gamma_X: RcCell<Fqe_mul_gadget<ppT, FieldT, PB>>,

    pub next_RX_plus_RX_plus_QX: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_next_RX: RcCell<Fqe_sqr_gadget<ppT, FieldT, PB>>,

    pub RX_minus_next_RX: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub RY_plus_next_RY: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_next_RY: RcCell<Fqe_mul_gadget<ppT, FieldT, PB>>,
}

/**
 * Gadget that verifies correct precomputation of the G2 variable.
 */
#[derive(Clone, Default)]
pub struct precompute_G2_gadget<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<Fr<ppT> >

    // type FieldT=Fr<ppT>;
    // type FqeT=Fqe<other_curve::<ppT> >;
    // type FqkT=Fqk<other_curve::<ppT> >;
    pub addition_steps: Vec<RcCell<precompute_G2_gadget_addition_steps<ppT, FieldT, PB>>>,
    pub doubling_steps: Vec<RcCell<precompute_G2_gadget_doubling_steps<ppT, FieldT, PB>>>,

    pub add_count: usize,
    pub dbl_count: usize,

    pub precomp: G2_precomputations<ppT, FieldT, PB>, // important to have a reference here
}

// use  <type_traits>
// use crate::gadgetlib1::gadgets::pairing::mnt_pairing_params;

pub type G1_precomputations<ppT, FieldT, PB> =
    gadget<FieldT, PB, G1_precomputation<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    G1_precomputation<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        P_val: G1<ppT>,
        annotation_prefix: String,
    ) -> G1_precomputations<ppT, FieldT, PB> {
        let mut P_val_copy = P_val.clone();
        P_val_copy.to_affine_coordinates();
        let P = RcCell::new(G1_variable::<ppT, FieldT, PB>::new2(
            pb.clone(),
            P_val_copy.clone(),
            prefix_format!(annotation_prefix, " P"),
        ));
        let PY_twist_squared = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new2(
            pb.clone(),
            P_val_copy.Y() * G2::<ppT>::twist().squared(),
            " PY_twist_squared".to_owned(),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                P,
                PY_twist_squared,
            },
        )
    }
}

pub type precompute_G1_gadgets<ppT, FieldT, PB> =
    gadget<FieldT, PB, precompute_G1_gadget<ppT, FieldT, PB>>;
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G1_gadget<ppT, FieldT, PB>
{
    /* two possible pre-computations one for mnt4 and one for mnt6 */
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        P: G1_variables<ppT, FieldT, PB>,
        mut precomp: G1_precomputations<ppT, FieldT, PB>, // will allocate this inside
        annotation_prefix: String,
    ) -> precompute_G1_gadgets<ppT, FieldT, PB> {
        // 4:std::enable_if<Fqk<other_curve::<ppT> >::extension_degree() ==, FieldT>::type& = FieldT()
        let (mut c0, mut c1) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        c0.assign(
            &pb,
            &(P.t.Y.clone() * (mnt4_twist::<ppT, FieldT, PB>().squared().c0()).to_field()),
        );
        c1.assign(
            &pb,
            &(P.t.Y.clone() * (mnt4_twist::<ppT, FieldT, PB>().squared().c1()).to_field()),
        );

        precomp.t.P = RcCell::new(P.clone());
        precomp.t.PY_twist_squared = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new22(
            pb.clone(),
            c0,
            c1,
            prefix_format!(annotation_prefix, " PY_twist_squared"),
        ));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { precomp })
    }

    pub fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        P: G1_variables<ppT, FieldT, PB>,
        mut precomp: G1_precomputations<ppT, FieldT, PB>, // will allocate this inside
        annotation_prefix: String,
    ) -> precompute_G1_gadgets<ppT, FieldT, PB> {
        // 6:std::enable_if<Fqk<other_curve::<ppT> >::extension_degree() ==, FieldT>::type& = FieldT()
        let (mut c0, mut c1, mut c2) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        c0.assign(
            &pb,
            &(P.t.Y.clone() * (mnt6_twist::<ppT, FieldT, PB>().squared().c0()).to_field()),
        );
        c1.assign(
            &pb,
            &(P.t.Y.clone() * (mnt6_twist::<ppT, FieldT, PB>().squared().c1()).to_field()),
        );
        c2.assign(
            &pb,
            &(P.t.Y.clone() * (mnt6_twist::<ppT, FieldT, PB>().squared().c2()).to_field()),
        );

        precomp.t.P = RcCell::new(P.clone());
        precomp.t.PY_twist_squared = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new3(
            pb.clone(),
            c0,
            c1,
            c2,
            prefix_format!(annotation_prefix, " PY_twist_squared"),
        ));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { precomp })
    }
}

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G1_gadgets<ppT, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        /* the same for neither ppT = mnt4 nor ppT = mnt6 */
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.precomp.t.PY_twist_squared.borrow().evaluate(); /* the same for both ppT = mnt4 and ppT = mnt6 */
    }
}

pub fn test_G1_variable_precomp<
    ppT: ppTConfig<FieldT, PB> + std::cmp::PartialEq<<ppT as ppTConfig<FieldT, PB>>::Fpk_variableT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
>(
    annotation: &String,
) {
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut g_val = G1::<ppT>::one() * FieldT::random_element();

    let mut g = G1_variable::<ppT, FieldT, PB>::new(pb.clone(), "g".to_owned());
    let mut precomp = G1_precomputations::<ppT, FieldT, PB>::default();
    let mut do_precomp = precompute_G1_gadget::<ppT, FieldT, PB>::new(
        pb.clone(),
        g.clone(),
        precomp.clone(),
        "do_precomp".to_owned(),
    );
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(&g_val);
    do_precomp.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    let mut const_precomp = G1_precomputation::<ppT, FieldT, PB>::new(
        pb.clone(),
        g_val.clone(),
        "const_precomp".to_owned(),
    );

    let native_precomp = affine_ate_precompute_G1(g_val);
    assert!(precomp.t.PY_twist_squared.borrow().get_element() == native_precomp.PY_twist_squared());
    assert!(
        const_precomp.t.PY_twist_squared.borrow().get_element()
            == native_precomp.PY_twist_squared()
    );

    print!(
        "number of constraints for G1 precomp (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}

pub type G2_precomputations<ppT, FieldT, PB> =
    gadget<FieldT, PB, G2_precomputation<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    G2_precomputation<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        Q_val: G2<ppT>,
        annotation_prefix: String,
    ) -> G2_precomputations<ppT, FieldT, PB> {
        let Q = RcCell::new(G2_variable::<ppT, FieldT, PB>::new2(
            pb.clone(),
            Q_val.clone(),
            prefix_format!(annotation_prefix, " Q"),
        ));
        let native_precomp = affine_ate_precompute_G2(Q_val);

        let coeffs = vec![
            RcCell::new(precompute_G2_gadget_coeffss::<ppT, FieldT, PB>::default());
            native_precomp.coeffs().len() + 1
        ]; // the last precomp remains for convenient programming
        for i in 0..native_precomp.coeffs().len() {
            coeffs[i].borrow_mut().t.RX = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].old_RX().clone(),
                prefix_format!(annotation_prefix, " RX"),
            ));
            coeffs[i].borrow_mut().t.RY = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].old_RY().clone(),
                prefix_format!(annotation_prefix, " RY"),
            ));
            coeffs[i].borrow_mut().t.gamma = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].gamma().clone(),
                prefix_format!(annotation_prefix, " gamma"),
            ));
            coeffs[i].borrow_mut().t.gamma_X = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new2(
                pb.clone(),
                native_precomp.coeffs()[i].gamma_X().clone(),
                prefix_format!(annotation_prefix, " gamma_X"),
            ));
        }

        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { Q, coeffs })
    }
}

pub type precompute_G2_gadget_coeffss<ppT, FieldT, PB> =
    gadget<FieldT, PB, precompute_G2_gadget_coeffs<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G2_gadget_coeffs<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_coeffss<ppT, FieldT, PB> {
        let RX = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " RX"),
        ));
        let RY = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " RY"),
        ));
        let gamma = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma"),
        ));
        let gamma_X = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma_X"),
        ));
        gadget::<FieldT, PB, Self>::new(
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
        pb: RcCell<protoboard<FieldT, PB>>,
        Q: G2_variables<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_coeffss<ppT, FieldT, PB> {
        let RX = Q.t.X.clone();
        let RY = Q.t.Y.clone();
        let gamma = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma"),
        ));
        let gamma_X = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma_X"),
        ));
        gadget::<FieldT, PB, Self>::new(
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
pub type precompute_G2_gadget_doubling_steps<ppT, FieldT, PB> =
    gadget<FieldT, PB, precompute_G2_gadget_doubling_step<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G2_gadget_doubling_step<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        cur: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,
        next: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_doubling_steps<ppT, FieldT, PB> {
        let RXsquared = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " RXsquared"),
        ));
        let compute_RXsquared = RcCell::new(Fqe_sqr_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.RX.clone(),
            RXsquared.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_RXsquared"),
        ));
        let three_RXsquared_plus_a =
            RcCell::new((RXsquared.borrow().clone()) * FieldT::from(3) + G2::<ppT>::coeff_a());
        let two_RY = RcCell::new(cur.t.RY.borrow().clone() * FieldT::from(2));

        let compute_gamma = RcCell::new(Fqe_mul_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            two_RY.borrow().clone(),
            three_RXsquared_plus_a.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma"),
        ));
        let compute_gamma_X = RcCell::new(Fqe_mul_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            cur.t.RX.borrow().clone(),
            cur.t.gamma_X.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma_X"),
        ));

        let next_RX_plus_two_RX =
            RcCell::new(next.t.RX.borrow().clone() + cur.t.RX.borrow().clone() * FieldT::from(2));
        let compute_next_RX = RcCell::new(Fqe_sqr_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.clone(),
            next_RX_plus_two_RX.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RX"),
        ));

        let RX_minus_next_RX =
            RcCell::new(cur.t.RX.borrow().clone() + next.t.RX.borrow().clone() * (-FieldT::one()));
        let RY_plus_next_RY = RcCell::new(cur.t.RY.borrow().clone() + next.t.RY.borrow().clone());
        let compute_next_RY = RcCell::new(Fqe_mul_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            RX_minus_next_RX.borrow().clone(),
            RY_plus_next_RY.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RY"),
        ));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G2_gadget_doubling_steps<ppT, FieldT, PB>
{
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
            .generate_r1cs_witness(&gamma_val.to_field());

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
            .generate_r1cs_witness(&next_RX_val.to_field());
        self.t
            .next
            .t
            .RY
            .borrow()
            .generate_r1cs_witness(&next_RY_val.to_field());

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

pub type precompute_G2_gadget_addition_steps<ppT, FieldT, PB> =
    gadget<FieldT, PB, precompute_G2_gadget_addition_step<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G2_gadget_addition_step<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        invert_Q: bool,
        cur: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,
        next: precompute_G2_gadget_coeffss<ppT, FieldT, PB>,
        Q: G2_variables<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> precompute_G2_gadget_addition_steps<ppT, FieldT, PB> {
        let RY_minus_QY = RcCell::new(
            cur.t.RY.borrow().clone()
                + Q.t.Y.borrow().clone()
                    * (if !invert_Q {
                        -FieldT::one()
                    } else {
                        FieldT::one()
                    }),
        );

        let RX_minus_QX =
            RcCell::new(cur.t.RX.borrow().clone() + Q.t.X.borrow().clone() * (-FieldT::one()));
        let compute_gamma = RcCell::new(Fqe_mul_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            RX_minus_QX.borrow().clone(),
            RY_minus_QY.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma"),
        ));
        let compute_gamma_X = RcCell::new(Fqe_mul_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            Q.t.X.borrow().clone(),
            cur.t.gamma_X.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_gamma_X"),
        ));

        let next_RX_plus_RX_plus_QX = RcCell::new(
            next.t.RX.borrow().clone() + cur.t.RX.borrow().clone() + Q.t.X.borrow().clone(),
        );
        let compute_next_RX = RcCell::new(Fqe_sqr_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.clone(),
            next_RX_plus_RX_plus_QX.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RX"),
        ));

        let RX_minus_next_RX =
            RcCell::new(cur.t.RX.borrow().clone() + next.t.RX.borrow().clone() * (-FieldT::one()));
        let RY_plus_next_RY = RcCell::new(cur.t.RY.borrow().clone() + next.t.RY.borrow().clone());
        let compute_next_RY = RcCell::new(Fqe_mul_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            cur.t.gamma.borrow().clone(),
            RX_minus_next_RX.borrow().clone(),
            RY_plus_next_RY.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_next_RY"),
        ));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G2_gadget_addition_steps<ppT, FieldT, PB>
{
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
            .generate_r1cs_witness(&gamma_val.to_field());

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
            .generate_r1cs_witness(&next_RX_val.to_field());
        self.t
            .next
            .t
            .RY
            .borrow()
            .generate_r1cs_witness(&next_RY_val.to_field());

        self.t.next_RX_plus_RX_plus_QX.borrow().evaluate();
        self.t.RX_minus_next_RX.borrow().evaluate();
        self.t.RY_plus_next_RY.borrow().evaluate();

        self.t.compute_next_RX.borrow().generate_r1cs_witness();
        self.t.compute_next_RY.borrow().generate_r1cs_witness();
    }
}

pub type precompute_G2_gadgets<ppT, FieldT, PB> =
    gadget<FieldT, PB, precompute_G2_gadget<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G2_gadget<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        Q: G2_variables<ppT, FieldT, PB>,
        mut precomp: G2_precomputations<ppT, FieldT, PB>, // will allocate this inside
        annotation_prefix: String,
    ) -> precompute_G2_gadgets<ppT, FieldT, PB> {
        precomp.t.Q = RcCell::new(Q.clone());

        let mut loop_count = pairing_loop_count;
        let mut coeff_count = 1; // the last RX/RY are unused in Miller loop, but will need to get allocated somehow
        let mut add_count = 0;
        let mut dbl_count = 0;

        let mut found_nonzero = false;
        let NAF = find_wnaf(1, &loop_count);
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
            RcCell::new(precompute_G2_gadget_coeffss::<ppT, FieldT, PB>::default()),
        );
        let mut addition_steps =
            vec![
                RcCell::new(precompute_G2_gadget_addition_steps::<ppT, FieldT, PB>::default());
                add_count
            ];
        let mut doubling_steps =
            vec![
                RcCell::new(precompute_G2_gadget_doubling_steps::<ppT, FieldT, PB>::default());
                dbl_count
            ];

        precomp.t.coeffs[0] = RcCell::new(precompute_G2_gadget_coeffs::<ppT, FieldT, PB>::new2(
            pb.clone(),
            Q.clone(),
            prefix_format!(annotation_prefix, " coeffs_0"),
        ));
        for i in 1..coeff_count {
            precomp.t.coeffs[i] = RcCell::new(precompute_G2_gadget_coeffs::<ppT, FieldT, PB>::new(
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

            doubling_steps[dbl_id] =
                RcCell::new(precompute_G2_gadget_doubling_step::<ppT, FieldT, PB>::new(
                    pb.clone(),
                    precomp.t.coeffs[coeff_id].borrow().clone(),
                    precomp.t.coeffs[coeff_id + 1].borrow().clone(),
                    prefix_format!(annotation_prefix, " doubling_steps_{}", dbl_id),
                ));
            dbl_id += 1;
            coeff_id += 1;

            if NAF[i] != 0 {
                addition_steps[add_id] =
                    RcCell::new(precompute_G2_gadget_addition_step::<ppT, FieldT, PB>::new(
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
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    precompute_G2_gadgets<ppT, FieldT, PB>
{
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
                    .to_field(),
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
                    .to_field(),
            );

        let mut loop_count = pairing_loop_count;

        let mut add_id = 0;
        let mut dbl_id = 0;

        let mut found_nonzero = false;
        let NAF = find_wnaf(1, &loop_count);
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
pub fn test_G2_variable_precomp<
    ppT: ppTConfig<FieldT, PB> + std::cmp::PartialEq<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
>(
    annotation: &String,
) {
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let g_val = FieldT::random_element() * FieldT::one();

    let mut g = G2_variable::<ppT, FieldT, PB>::new(pb.clone(), "g".to_owned());
    let mut precomp = G2_precomputations::<ppT, FieldT, PB>::default();
    let mut do_precomp = precompute_G2_gadget::<ppT, FieldT, PB>::new(
        pb.clone(),
        g.clone(),
        precomp.clone(),
        "do_precomp".to_owned(),
    );
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(&ppT::from_field(&g_val));
    do_precomp.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    let native_precomp = affine_ate_precompute_G2(ppT::from_field(&g_val));

    assert!(precomp.t.coeffs.len() - 1 == native_precomp.coeffs().len()); // the last precomp is unused, but remains for convenient programming
    for i in 0..native_precomp.coeffs().len() {
        assert!(
            precomp.t.coeffs[i].borrow().t.RX.borrow().get_element()
                == native_precomp.coeffs()[i].old_RX()
        );
        assert!(
            precomp.t.coeffs[i].borrow().t.RY.borrow().get_element()
                == native_precomp.coeffs()[i].old_RY()
        );
        assert!(
            precomp.t.coeffs[i].borrow().t.gamma.borrow().get_element()
                == native_precomp.coeffs()[i].gamma()
        );
        assert!(
            precomp.t.coeffs[i]
                .borrow()
                .t
                .gamma_X
                .borrow()
                .get_element()
                == native_precomp.coeffs()[i].gamma_X()
        );
    }

    print!(
        "number of constraints for G2 precomp (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}
