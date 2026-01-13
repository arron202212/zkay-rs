// Declaration of interfaces for G2 gadgets.

// The gadgets verify curve arithmetic in G2 = E'(F) where E'/F^e: y^2 = x^3 + A' * X + B'
// is an elliptic curve over F^e in short Weierstrass form.
// use super::{ ppTConfig};
use crate::gadgetlib1::gadget::gadget;

use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, MulTConfig, SqrTConfig, VariableTConfig,
    other_curve, pairing_selector, ppTConfig,
};
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};

use ff_curves::algebra::curves::public_params;
use ff_curves::{FpmConfig, Fqe, Fqk, Fr, G1, G2};
use ffec::algebra::scalar_multiplication::wnaf;
use ffec::{FieldTConfig, One, PpConfig, Zero};
use rccell::RcCell;
use std::marker::PhantomData;
use std::ops::Add;
/**
 * Gadget that represents a G2 variable.
 */

type FieldT<ppT> = Fr<ppT>;
type FqeT<ppT> = Fqe<other_curve<ppT>>;
type FqkT<ppT> = Fqk<other_curve<ppT>>;
#[derive(Clone, Default)]
pub struct G2_variable<ppT: ppTConfig> {
    // : public gadget<ffec::Fr<ppT> >
    pub X: RcCell<Fqe_variable<ppT>>,
    pub Y: RcCell<Fqe_variable<ppT>>,
    pub all_vars: pb_linear_combination_array<ppT::FieldT, ppT::PB>,
}

/**
 * Gadget that creates constraints for the validity of a G2 variable.
 */
#[derive(Clone, Default)]
pub struct G2_checker_gadget<ppT: ppTConfig> {
    //  : public gadget<ffec::Fr<ppT> >
    //     type FieldT=ffec::Fr<ppT>;
    //     type FqeT=ffec::Fqe<other_curve<ppT> >;
    //     type FqkT=ffec::Fqk<other_curve<ppT> >;
    pub Q: G2_variables<ppT>,
    pub Xsquared: RcCell<Fqe_variable<ppT>>,
    pub Ysquared: RcCell<Fqe_variable<ppT>>,
    pub Xsquared_plus_a: RcCell<Fqe_variable<ppT>>,
    pub Ysquared_minus_b: RcCell<Fqe_variable<ppT>>,
    pub compute_Xsquared: RcCell<Fqe_sqr_gadget<ppT>>,
    pub compute_Ysquared: RcCell<Fqe_sqr_gadget<ppT>>,
    pub curve_equation: RcCell<Fqe_mul_gadget<ppT>>,
}

pub type G2_variables<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G2_variable<ppT>>;
impl<ppT: ppTConfig> G2_variable<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        annotation_prefix: String,
    ) -> G2_variables<ppT> {
        let X = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " X"),
        ));
        let Y = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " Y"),
        ));
        let all_vars = pb_linear_combination_array::<ppT::FieldT, ppT::PB>::new(
            X.borrow()
                .all_vars()
                .iter()
                .chain(Y.borrow().all_vars().iter())
                .cloned()
                .collect(),
        );
        gadget::<ppT::FieldT, ppT::PB, Self>::new(pb, annotation_prefix, Self { X, Y, all_vars })
    }

    pub fn new2(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        Q: G2<other_curve<ppT>>,
        annotation_prefix: String,
    ) -> G2_variables<ppT> {
        let mut Q_copy = Q.clone();
        Q_copy.to_affine_coordinates();

        let X = RcCell::new(Fqe_variable::<ppT>::new2(
            pb.clone(),
            Q_copy.X(),
            prefix_format!(annotation_prefix, " X"),
        ));
        let Y = RcCell::new(Fqe_variable::<ppT>::new2(
            pb.clone(),
            Q_copy.Y(),
            prefix_format!(annotation_prefix, " Y"),
        ));

        let all_vars = pb_linear_combination_array::<ppT::FieldT, ppT::PB>::new(
            X.borrow()
                .all_vars()
                .iter()
                .chain(Y.borrow().all_vars().iter())
                .cloned()
                .collect(),
        );
        gadget::<ppT::FieldT, ppT::PB, Self>::new(pb, annotation_prefix, Self { X, Y, all_vars })
    }
    pub fn size_in_bits() -> usize {
        return 2 * Fqe_variable::<ppT>::size_in_bits();
    }

    pub fn num_variables() -> usize {
        return 2 * Fqe_variable::<ppT>::num_variables();
    }
    pub fn num_field_elems() -> usize {
        return 2;
    }
}
impl<ppT: ppTConfig> G2_variables<ppT> {
    pub fn generate_r1cs_witness(&self, Q: &G2<other_curve<ppT>>) {
        let mut Qcopy = Q.clone();
        Qcopy.to_affine_coordinates();

        self.t.X.borrow().generate_r1cs_witness(&Qcopy.X());
        self.t.Y.borrow().generate_r1cs_witness(&Qcopy.Y());
    }
}

pub type G2_checker_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G2_checker_gadget<ppT>>;
impl<ppT: ppTConfig> G2_checker_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        Q: G2_variables<ppT>,
        annotation_prefix: String,
    ) -> G2_checker_gadgets<ppT> {
        let Xsquared = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " Xsquared"),
        ));
        let Ysquared = RcCell::new(Fqe_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " Ysquared"),
        ));

        let compute_Xsquared = RcCell::new(Fqe_sqr_gadget::<ppT>::new(
            pb.clone(),
            Q.t.X.clone(),
            Xsquared.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_Xsquared"),
        ));
        let compute_Ysquared = RcCell::new(Fqe_sqr_gadget::<ppT>::new(
            pb.clone(),
            Q.t.Y.clone(),
            Ysquared.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_Ysquared"),
        ));

        let Xsquared_plus_a = RcCell::new(Fqe_variable::<ppT>::from(
            Xsquared.borrow().clone() + G2::<other_curve<ppT>>::coeff_a,
        ));
        let Ysquared_minus_b = RcCell::new(Fqe_variable::<ppT>::from(
            Ysquared.borrow().clone() + (-G2::<other_curve<ppT>>::coeff_b),
        ));

        let curve_equation = RcCell::new(Fqe_mul_gadget::<ppT>::new(
            pb.clone(),
            Q.t.X.borrow().clone(),
            Xsquared_plus_a.borrow().clone(),
            Ysquared_minus_b.borrow().clone(),
            prefix_format!(annotation_prefix, " curve_equation"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                Q,
                Xsquared,
                Ysquared,
                Xsquared_plus_a,
                Ysquared_minus_b,
                compute_Xsquared,
                compute_Ysquared,
                curve_equation,
            },
        )
    }
}
impl<ppT: ppTConfig> G2_checker_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_Xsquared.borrow().generate_r1cs_constraints();
        self.t.compute_Ysquared.borrow().generate_r1cs_constraints();
        self.t.curve_equation.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_Xsquared.borrow().generate_r1cs_witness();
        self.t.compute_Ysquared.borrow().generate_r1cs_witness();
        self.t.Xsquared_plus_a.borrow().evaluate();
        self.t.curve_equation.borrow().generate_r1cs_witness();
    }
}

pub fn test_G2_checker_gadget<ppT: ppTConfig>(annotation: String) {
    let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
    let mut g = G2_variable::<ppT>::new(pb.clone(), "g".to_owned());
    let mut g_check = G2_checker_gadget::<ppT>::new(pb.clone(), g.clone(), "g_check".to_owned());
    g_check.generate_r1cs_constraints();

    print!("positive test\n");
    g.generate_r1cs_witness(&G2::<other_curve<ppT>>::one());
    g_check.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    print!("negative test\n");
    g.generate_r1cs_witness(&G2::<other_curve<ppT>>::zero());
    g_check.generate_r1cs_witness();
    assert!(!pb.borrow().is_satisfied());

    print!(
        "number of constraints for G2 checker (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}
