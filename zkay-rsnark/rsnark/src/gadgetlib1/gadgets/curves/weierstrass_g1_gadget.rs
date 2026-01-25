// Declaration of interfaces for G1 gadgets.

// The gadgets verify curve arithmetic in G1 = E(F) where E/F: y^2 = x^3 + A * X + B
// is an elliptic curve over F in short Weierstrass form.

use crate::gadgetlib1::gadget::gadget;

use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, MulTConfig, SqrTConfig, VariableTConfig,
    other_curve, pairing_selector, ppTConfig,
};
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{protoboard,PBConfig,ProtoboardConfig};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};

use ff_curves::algebra::curves::public_params;
use ff_curves::{FpmConfig, Fr, G1, G2};
use ffec::{FieldTConfig, One, PpConfig, Zero};
use rccell::RcCell;
use std::marker::PhantomData;

// const coeff_a: i64 = 0; //ffec::G1::<other_curve<ppT,P>>::coeff_a;
// const coeff_b: i64 = 0; //ffec::G1::<other_curve<ppT,P>>::coeff_b;
// pub type G1<ppT> = ppT; //ffec::G1<other_curve<ppT,P>>;
/**
 * Gadget that represents a G1 variable.
 */
// pub trait ppTConfig<FieldT: FieldTConfig>: Clone + Default {
//     type Fr: FieldTConfig;
//     fn X(&self) -> FieldT;
//     fn Y(&self) -> FieldT;
//     fn to_affine_coordinates(&self);
// }

type FieldT<ppT> = Fr<ppT>;
#[derive(Clone, Default)]
pub struct G1_variable<ppT: ppTConfig> {
    //  : public gadget<ffec::Fr<ppT> >
    pub X: linear_combination<ppT::FieldT, pb_variable, pb_linear_combination>,
    pub Y: linear_combination<ppT::FieldT, pb_variable, pb_linear_combination>,
    pub all_vars: pb_linear_combination_array<ppT::FieldT, ppT::PB>,
}

/**
 * Gadget that creates constraints for the validity of a G1 variable.
 */

#[derive(Clone, Default)]
pub struct G1_checker_gadget<ppT: ppTConfig> {
    // : public gadget<ffec::Fr<ppT> >
    // type FieldT=ffec::Fr<ppT>;
    pub P: G1_variables<ppT>,
    pub P_X_squared: variable<ppT::FieldT, pb_variable>,
    pub P_Y_squared: variable<ppT::FieldT, pb_variable>,
}

/**
 * Gadget that creates constraints for G1 addition.
 */

#[derive(Clone, Default)]
pub struct G1_add_gadget<ppT: ppTConfig> {
    // : public gadget<ffec::Fr<ppT> >
    // type FieldT=ffec::Fr<ppT>;
    pub lambda: variable<ppT::FieldT, pb_variable>,
    pub inv: variable<ppT::FieldT, pb_variable>,
    pub A: G1_variables<ppT>,
    pub B: G1_variables<ppT>,
    pub C: G1_variables<ppT>,
}

/**
 * Gadget that creates constraints for G1 doubling.
 */
#[derive(Clone, Default)]
pub struct G1_dbl_gadget<ppT: ppTConfig> {
    // : public gadget<ffec::Fr<ppT> >
    // type FieldT=ffec::Fr<ppT>;
    pub Xsquared: variable<ppT::FieldT, pb_variable>,
    pub lambda: variable<ppT::FieldT, pb_variable>,
    pub A: G1_variables<ppT>,
    pub B: G1_variables<ppT>,
}

/**
 * Gadget that creates constraints for G1 multi-scalar multiplication.
 */
#[derive(Clone, Default)]
pub struct G1_multiscalar_mul_gadget<ppT: ppTConfig> {
    //  : public gadget<ffec::Fr<ppT> >
    //     type FieldT=ffec::Fr<ppT>;
    pub computed_results: Vec<G1_variables<ppT>>,
    pub chosen_results: Vec<G1_variables<ppT>>,
    pub adders: Vec<G1_add_gadgets<ppT>>,
    pub doublers: Vec<G1_dbl_gadgets<ppT>>,
    pub base: G1_variables<ppT>,
    pub scalars: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub points: Vec<G1_variables<ppT>>,
    pub points_and_powers: Vec<G1_variables<ppT>>,
    pub result: G1_variables<ppT>,
    pub elt_size: usize,
    pub num_points: usize,
    pub scalar_size: usize,
}

pub type G1_variables<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G1_variable<ppT>>;
impl<ppT: ppTConfig> G1_variable<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        annotation_prefix: String,
    ) -> G1_variables<ppT> {
        let (mut X_var, mut Y_var) = (
            variable::<ppT::FieldT, pb_variable>::default(),
            variable::<ppT::FieldT, pb_variable>::default(),
        );

        X_var.allocate(&pb, prefix_format!(annotation_prefix, " X"));
        Y_var.allocate(&pb, prefix_format!(annotation_prefix, " Y"));

        let X: linear_combination<ppT::FieldT, pb_variable, pb_linear_combination> = X_var.into();
        let Y: linear_combination<ppT::FieldT, pb_variable, pb_linear_combination> = Y_var.into();
        let all_vars =
            pb_linear_combination_array::<ppT::FieldT, ppT::PB>::new(vec![X.clone(), Y.clone()]);
        gadget::<ppT::FieldT, ppT::PB, Self>::new(pb, annotation_prefix, Self { X, Y, all_vars })
    }

    pub fn new2(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        PP: G1<other_curve<ppT>>,
        annotation_prefix: String,
    ) -> G1_variables<ppT> {
        let mut Pcopy = PP.clone();
        Pcopy.to_affine_coordinates();
        let mut X =
            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::default();
        let mut Y =
            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::default();
        X.assign(
            &pb,
            &(Pcopy
                .X()
                .to_field::<linear_combination<ppT::FieldT, pb_variable, pb_linear_combination>>()),
        );
        Y.assign(
            &pb,
            &(Pcopy
                .Y()
                .to_field::<linear_combination<ppT::FieldT, pb_variable, pb_linear_combination>>()),
        );
        X.evaluate_pb(&pb);
        Y.evaluate_pb(&pb);
        let all_vars =
            pb_linear_combination_array::<ppT::FieldT, ppT::PB>::new(vec![X.clone(), Y.clone()]);
        gadget::<ppT::FieldT, ppT::PB, Self>::new(pb, annotation_prefix, Self { X, Y, all_vars })
    }
    pub fn size_in_bits() -> usize {
        2 * ppT::FieldT::size_in_bits()
    }

    pub fn num_variables() -> usize {
        2
    }
    pub fn num_field_elems() -> usize {
        2
    }
}

impl<ppT: ppTConfig> G1_variables<ppT> {
    pub fn generate_r1cs_witness(&self, el: &G1<other_curve<ppT>>) {
        let mut el_normalized = el.clone();
        el_normalized.to_affine_coordinates();

        // *self.pb.borrow_mut().lc_val_ref(&self.t.X) = el_normalized.X();
        // *self.pb.borrow_mut().lc_val_ref(&self.t.Y) = el_normalized.Y();
    }
}

pub type G1_checker_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G1_checker_gadget<ppT>>;
impl<ppT: ppTConfig> G1_checker_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        P: G1_variables<ppT>,
        annotation_prefix: String,
    ) -> G1_checker_gadgets<ppT> {
        let mut P_X_squared = variable::<ppT::FieldT, pb_variable>::default();
        let mut P_Y_squared = variable::<ppT::FieldT, pb_variable>::default();
        P_X_squared.allocate(&pb, prefix_format!(annotation_prefix, " P_X_squared"));
        P_Y_squared.allocate(&pb, prefix_format!(annotation_prefix, " P_Y_squared"));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                P,
                P_X_squared,
                P_Y_squared,
            },
        )
    }
}
impl<ppT: ppTConfig> G1_checker_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.P.t.X.clone().into()],
                vec![self.t.P.t.X.clone().into()],
                vec![self.t.P_X_squared.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " P_X_squared"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.P.t.Y.clone().into()],
                vec![self.t.P.t.Y.clone().into()],
                vec![self.t.P_Y_squared.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " P_Y_squared"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.P.t.X.clone().into()],
                vec![
                    self.t.P_X_squared.clone().into(),
                    linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::from(
                        variable::<ppT::FieldT, pb_variable>::from(ONE),
                    ) * ppT::FieldT::from(G1::<other_curve<ppT>>::coeff_a),
                ],
                vec![
                    self.t.P_Y_squared.clone().into(),
                    linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::from(
                        variable::<ppT::FieldT, pb_variable>::from(ONE),
                    ) * ppT::FieldT::from(-G1::<other_curve<ppT>>::coeff_b),
                ],
            ),
            prefix_format!(self.annotation_prefix, " curve_equation"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.P_X_squared) =
            self.pb.borrow().lc_val(&self.t.P.t.X).squared();
        *self.pb.borrow_mut().val_ref(&self.t.P_Y_squared) =
            self.pb.borrow().lc_val(&self.t.P.t.Y).squared();
    }
}

pub type G1_add_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G1_add_gadget<ppT>>;
impl<ppT: ppTConfig> G1_add_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        A: G1_variables<ppT>,
        B: G1_variables<ppT>,
        C: G1_variables<ppT>,
        annotation_prefix: String,
    ) -> G1_add_gadgets<ppT> {
        /*
          lambda = (B.y - A.y)/(B.x - A.x)
          C.x = lambda^2 - A.x - B.x
          C.y = lambda(A.x - C.x) - A.y

          Special cases:

          doubling: if B.y = A.y and B.x = A.x then lambda is unbound and
          C = (lambda^2, lambda^3)

          addition of negative point: if B.y = -A.y and B.x = A.x then no
          lambda can satisfy the first equation unless B.y - A.y = 0. But
          then this reduces to doubling.

          So we need to check that A.x - B.x != 0, which can be done by
          enforcing I * (B.x - A.x) = 1
        */
        let mut lambda = variable::<ppT::FieldT, pb_variable>::default();
        let mut inv = variable::<ppT::FieldT, pb_variable>::default();
        lambda.allocate(&pb, prefix_format!(annotation_prefix, " lambda"));
        inv.allocate(&pb, prefix_format!(annotation_prefix, " inv"));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                lambda,
                inv,
                A,
                B,
                C,
            },
        )
    }
}
impl<ppT: ppTConfig> G1_add_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.B.t.X.clone().into(),
                    (self.t.A.t.X.clone() * ppT::FieldT::from(-1)).into(),
                ],
                vec![
                    self.t.B.t.Y.clone().into(),
                    (self.t.A.t.Y.clone() * ppT::FieldT::from(-1)).into(),
                ],
            ),
            prefix_format!(self.annotation_prefix, " calc_lambda"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.C.t.X.clone().into(),
                    self.t.A.t.X.clone().into(),
                    self.t.B.t.X.clone().into(),
                ],
            ),
            prefix_format!(self.annotation_prefix, " calc_X"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.A.t.X.clone().into(),
                    (self.t.C.t.X.clone() * ppT::FieldT::from(-1)).into(),
                ],
                vec![self.t.C.t.Y.clone().into(), self.t.A.t.Y.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " calc_Y"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.inv.clone().into()],
                vec![
                    self.t.B.t.X.clone().into(),
                    (self.t.A.t.X.clone() * ppT::FieldT::from(-1)).into(),
                ],
                vec![variable::<ppT::FieldT, pb_variable>::from(ONE).into()],
            ),
            prefix_format!(self.annotation_prefix, " no_special_cases"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.inv) = (self.pb.borrow().lc_val(&self.t.B.t.X)
            - self.pb.borrow().lc_val(&self.t.A.t.X))
        .inverse();
        *self.pb.borrow_mut().val_ref(&self.t.lambda) = (self.pb.borrow().lc_val(&self.t.B.t.Y)
            - self.pb.borrow().lc_val(&self.t.A.t.Y))
            * self.pb.borrow().val(&self.t.inv);
        *self.pb.borrow_mut().lc_val_ref(&self.t.C.t.X) =
            self.pb.borrow().val(&self.t.lambda).squared()
                - self.pb.borrow().lc_val(&self.t.A.t.X)
                - self.pb.borrow().lc_val(&self.t.B.t.X);
        *self.pb.borrow_mut().lc_val_ref(&self.t.C.t.Y) = self.pb.borrow().val(&self.t.lambda)
            * (self.pb.borrow().lc_val(&self.t.A.t.X) - self.pb.borrow().lc_val(&self.t.C.t.X))
            - self.pb.borrow().lc_val(&self.t.A.t.Y);
    }
}

pub type G1_dbl_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G1_dbl_gadget<ppT>>;

impl<ppT: ppTConfig> G1_dbl_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        A: G1_variables<ppT>,
        B: G1_variables<ppT>,
        annotation_prefix: String,
    ) -> G1_dbl_gadgets<ppT> {
        let mut Xsquared = variable::<ppT::FieldT, pb_variable>::default();
        let mut lambda = variable::<ppT::FieldT, pb_variable>::default();
        Xsquared.allocate(&pb, prefix_format!(annotation_prefix, " X_squared"));
        lambda.allocate(&pb, prefix_format!(annotation_prefix, " lambda"));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                Xsquared,
                lambda,
                A,
                B,
            },
        )
    }
}
impl<ppT: ppTConfig> G1_dbl_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.A.t.X.clone().into()],
                vec![self.t.A.t.X.clone().into()],
                vec![self.t.Xsquared.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " calc_Xsquared"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![(self.t.lambda.clone() * ppT::FieldT::from(2)).into()],
                vec![self.t.A.t.Y.clone().into()],
                vec![
                    (self.t.Xsquared.clone() * ppT::FieldT::from(3)).into(),
                    linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::from(
                        variable::<ppT::FieldT, pb_variable>::from(ONE),
                    ) * ppT::FieldT::from(G1::<other_curve<ppT>>::coeff_a),
                ],
            ),
            prefix_format!(self.annotation_prefix, " calc_lambda"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.B.t.X.clone().into(),
                    (self.t.A.t.X.clone() * ppT::FieldT::from(2)).into(),
                ],
            ),
            prefix_format!(self.annotation_prefix, " calc_X"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.A.t.X.clone().into(),
                    (self.t.B.t.X.clone() * ppT::FieldT::from(-1)).into(),
                ],
                vec![self.t.B.t.Y.clone().into(), self.t.A.t.Y.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " calc_Y"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.Xsquared) =
            self.pb.borrow().lc_val(&self.t.A.t.X).squared();
        *self.pb.borrow_mut().val_ref(&self.t.lambda) = (self.pb.borrow().val(&self.t.Xsquared)
            * ppT::FieldT::from(3)
            + ppT::FieldT::from(G1::<other_curve<ppT>>::coeff_a))
            * (ppT::FieldT::from(2) * self.pb.borrow().lc_val(&self.t.A.t.Y)).inverse();
        *self.pb.borrow_mut().lc_val_ref(&self.t.B.t.X) =
            self.pb.borrow().val(&self.t.lambda).squared()
                - ppT::FieldT::from(2) * self.pb.borrow().lc_val(&self.t.A.t.X);
        *self.pb.borrow_mut().lc_val_ref(&self.t.B.t.Y) = self.pb.borrow().val(&self.t.lambda)
            * (self.pb.borrow().lc_val(&self.t.A.t.X) - self.pb.borrow().lc_val(&self.t.B.t.X))
            - self.pb.borrow().lc_val(&self.t.A.t.Y);
    }
}

pub type G1_multiscalar_mul_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, G1_multiscalar_mul_gadget<ppT>>;

impl<ppT: ppTConfig> G1_multiscalar_mul_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        base: G1_variables<ppT>,
        scalars: pb_variable_array<ppT::FieldT, ppT::PB>,
        elt_size: usize,
        points: Vec<G1_variables<ppT>>,
        result: G1_variables<ppT>,
        annotation_prefix: String,
    ) -> G1_multiscalar_mul_gadgets<ppT> {
        let num_points = points.len();
        let scalar_size = scalars.len();
        assert!(num_points >= 1);
        assert!(num_points * elt_size == scalar_size);
        let mut points_and_powers = vec![];
        let mut doublers = vec![];
        for i in 0..num_points {
            points_and_powers.push(points[i].clone());
            for j in 0..elt_size - 1 {
                points_and_powers.push(G1_variable::<ppT>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " points_{}_times_2_to_{}", i, j + 1),
                ));
                doublers.push(G1_dbl_gadget::<ppT>::new(
                    pb.clone(),
                    points_and_powers[i * elt_size + j].clone(),
                    points_and_powers[i * elt_size + j + 1].clone(),
                    prefix_format!(annotation_prefix, " double_{}_to_2_to_{}", i, j + 1),
                ));
            }
        }
        let mut computed_results = vec![];
        let mut chosen_results = vec![];
        let mut adders = vec![];
        chosen_results.push(base.clone());
        for i in 0..scalar_size {
            computed_results.push(G1_variable::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " computed_results_"),
            ));
            if i < scalar_size - 1 {
                chosen_results.push(G1_variable::<ppT>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " chosen_results_"),
                ));
            } else {
                chosen_results.push(result.clone());
            }

            adders.push(G1_add_gadget::<ppT>::new(
                pb.clone(),
                chosen_results[i].clone(),
                points_and_powers[i].clone(),
                computed_results[i].clone(),
                prefix_format!(annotation_prefix, " adders_"),
            ));
        }
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                computed_results,
                chosen_results,
                adders,
                doublers,
                base,
                scalars,
                points,
                points_and_powers,
                result,
                elt_size,
                num_points,
                scalar_size,
            },
        )
    }
}

impl<ppT: ppTConfig> G1_multiscalar_mul_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        let num_constraints_before = self.pb.borrow().num_constraints();

        for i in 0..self.t.scalar_size - self.t.num_points {
            self.t.doublers[i].generate_r1cs_constraints();
        }

        for i in 0..self.t.scalar_size {
            self.t.adders[i].generate_r1cs_constraints();

            /*
              chosen_results[i+1].X = scalars[i] * computed_results[i].X + (1-scalars[i]) *  chosen_results[i].X
              chosen_results[i+1].X - chosen_results[i].X = scalars[i] * (computed_results[i].X - chosen_results[i].X)
            */
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.scalars[i].clone().into(),
                    (self.t.computed_results[i].t.X.clone() - self.t.chosen_results[i].t.X.clone())
                        .into(),
                    (self.t.chosen_results[i + 1].t.X.clone()
                        - self.t.chosen_results[i].t.X.clone())
                    .into(),
                ),
                prefix_format!(self.annotation_prefix, " chosen_results_{}_X", i + 1),
            );
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.scalars[i].clone().into(),
                    (self.t.computed_results[i].t.Y.clone() - self.t.chosen_results[i].t.Y.clone())
                        .into(),
                    (self.t.chosen_results[i + 1].t.Y.clone()
                        - self.t.chosen_results[i].t.Y.clone())
                    .into(),
                ),
                prefix_format!(self.annotation_prefix, " chosen_results_{}_Y", i + 1),
            );
        }

        let num_constraints_after = self.pb.borrow().num_constraints();
        assert!(
            num_constraints_after - num_constraints_before
                == 4 * (self.t.scalar_size - self.t.num_points) + (4 + 2) * self.t.scalar_size
        );
    }

    pub fn generate_r1cs_witness(&self) {
        for i in 0..self.t.scalar_size - self.t.num_points {
            self.t.doublers[i].generate_r1cs_witness();
        }

        for i in 0..self.t.scalar_size {
            self.t.adders[i].generate_r1cs_witness();
            *self
                .pb
                .borrow_mut()
                .lc_val_ref(&self.t.chosen_results[i + 1].t.X) =
                if self.pb.borrow().val(&self.t.scalars[i]) == ppT::FieldT::zero() {
                    self.pb.borrow().lc_val(&self.t.chosen_results[i].t.X)
                } else {
                    self.pb.borrow().lc_val(&self.t.computed_results[i].t.X)
                };
            *self
                .pb
                .borrow_mut()
                .lc_val_ref(&self.t.chosen_results[i + 1].t.Y) =
                if self.pb.borrow().val(&self.t.scalars[i]) == ppT::FieldT::zero() {
                    self.pb.borrow().lc_val(&self.t.chosen_results[i].t.Y)
                } else {
                    self.pb.borrow().lc_val(&self.t.computed_results[i].t.Y)
                };
        }
    }
}
