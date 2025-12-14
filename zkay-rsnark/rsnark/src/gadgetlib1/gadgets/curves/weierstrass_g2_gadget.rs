// Declaration of interfaces for G2 gadgets.

// The gadgets verify curve arithmetic in G2 = E'(F) where E'/F^e: y^2 = x^3 + A' * X + B'
// is an elliptic curve over F^e in short Weierstrass form.
use super::{
    Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, G1, G2, MulTConfig, SqrTConfig, VariableTConfig,
    coeff_a, coeff_b, ppTConfig,
};
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
use ffec::{One, Zero};
use rccell::RcCell;
use std::marker::PhantomData;
use std::ops::Add;
/**
 * Gadget that represents a G2 variable.
 */
#[derive(Clone, Default)]
pub struct G2_variable<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<ffec::Fr<ppT> >
    //     type FieldT=ffec::Fr<ppT>;
    //     type FqeT=ffec::Fqe<other_curve<ppT> >;
    //     type FqkT=ffec::Fqk<other_curve<ppT> >;
    pub X: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub Y: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub all_vars: pb_linear_combination_array<FieldT, PB>,
    _t: PhantomData<(ppT, PB)>,
}

/**
 * Gadget that creates constraints for the validity of a G2 variable.
 */
#[derive(Clone, Default)]
pub struct G2_checker_gadget<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget<ffec::Fr<ppT> >
    //     type FieldT=ffec::Fr<ppT>;
    //     type FqeT=ffec::Fqe<other_curve<ppT> >;
    //     type FqkT=ffec::Fqk<other_curve<ppT> >;
    pub Q: G2_variables<ppT, FieldT, PB>,
    pub Xsquared: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub Ysquared: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub Xsquared_plus_a: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub Ysquared_minus_b: RcCell<Fqe_variable<ppT, FieldT, PB>>,
    pub compute_Xsquared: RcCell<Fqe_sqr_gadget<ppT, FieldT, PB>>,
    pub compute_Ysquared: RcCell<Fqe_sqr_gadget<ppT, FieldT, PB>>,
    pub curve_equation: RcCell<Fqe_mul_gadget<ppT, FieldT, PB>>,
}

use ffec::algebra::scalar_multiplication::wnaf;

pub type G2_variables<ppT, FieldT, PB> = gadget<FieldT, PB, G2_variable<ppT, FieldT, PB>>;
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> G2_variable<ppT, FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        annotation_prefix: String,
    ) -> G2_variables<ppT, FieldT, PB> {
        let X = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " X"),
        ));
        let Y = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " Y"),
        ));
        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(
            X.borrow()
                .all_vars()
                .iter()
                .chain(Y.borrow().all_vars().iter())
                .cloned()
                .collect(),
        );
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                X,
                Y,
                all_vars,
                _t: PhantomData,
            },
        )
    }

    pub fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        Q: G2<ppT>,
        annotation_prefix: String,
    ) -> G2_variables<ppT, FieldT, PB> {
        let Q_copy = Q.clone();
        Q_copy.to_affine_coordinates();

        let X = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new2(
            pb.clone(),
            Q_copy.X(),
            prefix_format!(annotation_prefix, " X"),
        ));
        let Y = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new2(
            pb.clone(),
            Q_copy.Y(),
            prefix_format!(annotation_prefix, " Y"),
        ));

        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(
            X.borrow()
                .all_vars()
                .iter()
                .chain(Y.borrow().all_vars().iter())
                .cloned()
                .collect(),
        );
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                X,
                Y,
                all_vars,
                _t: PhantomData,
            },
        )
    }
    pub fn size_in_bits() -> usize {
        return 2 * Fqe_variable::<ppT, FieldT, PB>::size_in_bits();
    }

    pub fn num_variables() -> usize {
        return 2 * Fqe_variable::<ppT, FieldT, PB>::num_variables();
    }
    pub fn num_field_elems() -> usize {
        return 2;
    }
}
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> G2_variables<ppT, FieldT, PB> {
    pub fn generate_r1cs_witness(&self, Q: &ppT) {
        let mut Qcopy = Q.clone();
        Qcopy.to_affine_coordinates();

        self.t.X.borrow().generate_r1cs_witness(&Qcopy.X());
        self.t.Y.borrow().generate_r1cs_witness(&Qcopy.Y());
    }
}

pub type G2_checker_gadgets<ppT, FieldT, PB> =
    gadget<FieldT, PB, G2_checker_gadget<ppT, FieldT, PB>>;
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    G2_checker_gadget<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        Q: G2_variables<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> G2_checker_gadgets<ppT, FieldT, PB> {
        let Xsquared = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " Xsquared"),
        ));
        let Ysquared = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " Ysquared"),
        ));

        let compute_Xsquared = RcCell::new(Fqe_sqr_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            Q.t.X.clone(),
            Xsquared.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_Xsquared"),
        ));
        let compute_Ysquared = RcCell::new(Fqe_sqr_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            Q.t.Y.clone(),
            Ysquared.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_Ysquared"),
        ));

        let Xsquared_plus_a = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::from(
            Xsquared.borrow().clone() + coeff_a,
        ));
        let Ysquared_minus_b = RcCell::new(Fqe_variable::<ppT, FieldT, PB>::from(
            Ysquared.borrow().clone() + (-coeff_b),
        ));

        let curve_equation = RcCell::new(Fqe_mul_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            Q.t.X.borrow().clone(),
            Xsquared_plus_a.borrow().clone(),
            Ysquared_minus_b.borrow().clone(),
            prefix_format!(annotation_prefix, " curve_equation"),
        ));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    G2_checker_gadgets<ppT, FieldT, PB>
{
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

pub fn test_G2_checker_gadget<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>(
    annotation: String,
) {
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut g = G2_variable::<ppT, FieldT, PB>::new(pb.clone(), "g".to_owned());
    let mut g_check =
        G2_checker_gadget::<ppT, FieldT, PB>::new(pb.clone(), g.clone(), "g_check".to_owned());
    g_check.generate_r1cs_constraints();

    print!("positive test\n");
    g.generate_r1cs_witness(&ppT::one());
    g_check.generate_r1cs_witness();
    assert!(pb.borrow().is_satisfied());

    print!("negative test\n");
    g.generate_r1cs_witness(&ppT::zero());
    g_check.generate_r1cs_witness();
    assert!(!pb.borrow().is_satisfied());

    print!(
        "number of constraints for G2 checker (Fr is {})  = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
}
