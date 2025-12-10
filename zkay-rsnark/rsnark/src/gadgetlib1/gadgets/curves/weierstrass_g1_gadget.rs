// Declaration of interfaces for G1 gadgets.

// The gadgets verify curve arithmetic in G1 = E(F) where E/F: y^2 = x^3 + A * X + B
// is an elliptic curve over F in short Weierstrass form.

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
use ffec::Zero;
use rccell::RcCell;
use std::marker::PhantomData;

const coeff_a: i64 = 0; //ffec::G1::<other_curve<ppT>>::coeff_a;
const coeff_b: i64 = 0; //ffec::G1::<other_curve<ppT>>::coeff_b;
pub type G1<ppT> = ppT; //ffec::G1<other_curve<ppT>>;
/**
 * Gadget that represents a G1 variable.
 */
pub trait ppTConfig<FieldT: FieldTConfig>: Clone + Default {
    type Fr: FieldTConfig;
    fn X(&self) -> FieldT;
    fn Y(&self) -> FieldT;
    fn to_affine_coordinates(&self);
}

// type FieldT = ffec::Fr<ppT>;
#[derive(Clone, Default)]
pub struct G1_variable<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget<ffec::Fr<ppT> >
    pub X: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub Y: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub all_vars: pb_linear_combination_array<FieldT, PB>,
    _t: PhantomData<(ppT, PB)>,
}

/**
 * Gadget that creates constraints for the validity of a G1 variable.
 */

#[derive(Clone, Default)]
pub struct G1_checker_gadget<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<ffec::Fr<ppT> >
    // type FieldT=ffec::Fr<ppT>;
    pub P: G1_variables<ppT, FieldT, PB>,
    pub P_X_squared: variable<FieldT, pb_variable>,
    pub P_Y_squared: variable<FieldT, pb_variable>,
}

/**
 * Gadget that creates constraints for G1 addition.
 */

#[derive(Clone, Default)]
pub struct G1_add_gadget<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<ffec::Fr<ppT> >
    // type FieldT=ffec::Fr<ppT>;
    pub lambda: variable<FieldT, pb_variable>,
    pub inv: variable<FieldT, pb_variable>,
    pub A: G1_variables<ppT, FieldT, PB>,
    pub B: G1_variables<ppT, FieldT, PB>,
    pub C: G1_variables<ppT, FieldT, PB>,
}

/**
 * Gadget that creates constraints for G1 doubling.
 */
#[derive(Clone, Default)]
pub struct G1_dbl_gadget<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<ffec::Fr<ppT> >
    // type FieldT=ffec::Fr<ppT>;
    pub Xsquared: variable<FieldT, pb_variable>,
    pub lambda: variable<FieldT, pb_variable>,
    pub A: G1_variables<ppT, FieldT, PB>,
    pub B: G1_variables<ppT, FieldT, PB>,
}

/**
 * Gadget that creates constraints for G1 multi-scalar multiplication.
 */
#[derive(Clone, Default)]
pub struct G1_multiscalar_mul_gadget<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget<ffec::Fr<ppT> >
    //     type FieldT=ffec::Fr<ppT>;
    pub computed_results: Vec<G1_variables<ppT, FieldT, PB>>,
    pub chosen_results: Vec<G1_variables<ppT, FieldT, PB>>,
    pub adders: Vec<G1_add_gadgets<ppT, FieldT, PB>>,
    pub doublers: Vec<G1_dbl_gadgets<ppT, FieldT, PB>>,
    pub base: G1_variables<ppT, FieldT, PB>,
    pub scalars: pb_variable_array<FieldT, PB>,
    pub points: Vec<G1_variables<ppT, FieldT, PB>>,
    pub points_and_powers: Vec<G1_variables<ppT, FieldT, PB>>,
    pub result: G1_variables<ppT, FieldT, PB>,
    pub elt_size: usize,
    pub num_points: usize,
    pub scalar_size: usize,
}

pub type G1_variables<ppT, FieldT, PB> = gadget<FieldT, PB, G1_variable<ppT, FieldT, PB>>;
impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> G1_variable<ppT, FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        annotation_prefix: String,
    ) -> G1_variables<ppT, FieldT, PB> {
        let (mut X_var, mut Y_var) = (
            variable::<FieldT, pb_variable>::default(),
            variable::<FieldT, pb_variable>::default(),
        );

        X_var.allocate(&pb, prefix_format!(annotation_prefix, " X"));
        Y_var.allocate(&pb, prefix_format!(annotation_prefix, " Y"));

        let X = linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(X_var);
        let Y = linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(Y_var);
        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(vec![X.clone(), Y.clone()]);
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
        P: G1<ppT>,
        annotation_prefix: String,
    ) -> G1_variables<ppT, FieldT, PB> {
        let Pcopy = P.clone();
        Pcopy.to_affine_coordinates();
        let mut X = linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        let mut Y = linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        X.assign(&pb, &(Pcopy.X().into()));
        Y.assign(&pb, &(Pcopy.Y().into()));
        X.evaluate_pb(&pb);
        Y.evaluate_pb(&pb);
        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(vec![X.clone(), Y.clone()]);
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
}

impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> G1_variables<ppT, FieldT, PB> {
    pub fn generate_r1cs_witness(&self, el: &G1<ppT>) {
        let mut el_normalized = el.clone();
        el_normalized.to_affine_coordinates();

        *self.pb.borrow_mut().lc_val_ref(&self.t.X) = el_normalized.X();
        *self.pb.borrow_mut().lc_val_ref(&self.t.Y) = el_normalized.Y();
    }

    pub fn size_in_bits(&self) -> usize {
        return 2 * FieldT::size_in_bits();
    }

    pub fn num_variables(&self) -> usize {
        return 2;
    }
}

pub type G1_checker_gadgets<ppT, FieldT, PB> =
    gadget<FieldT, PB, G1_checker_gadget<ppT, FieldT, PB>>;
impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    G1_checker_gadget<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        P: G1_variables<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> G1_checker_gadgets<ppT, FieldT, PB> {
        let mut P_X_squared = variable::<FieldT, pb_variable>::default();
        let mut P_Y_squared = variable::<FieldT, pb_variable>::default();
        P_X_squared.allocate(&pb, prefix_format!(annotation_prefix, " P_X_squared"));
        P_Y_squared.allocate(&pb, prefix_format!(annotation_prefix, " P_Y_squared"));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    G1_checker_gadgets<ppT, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.P.t.X.clone().into()],
                vec![self.t.P.t.X.clone().into()],
                vec![self.t.P_X_squared.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " P_X_squared"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.P.t.Y.clone().into()],
                vec![self.t.P.t.Y.clone().into()],
                vec![self.t.P_Y_squared.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " P_Y_squared"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.P.t.X.clone().into()],
                vec![
                    self.t.P_X_squared.clone().into(),
                    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        variable::<FieldT, pb_variable>::from(ONE),
                    ) * coeff_a,
                ],
                vec![
                    self.t.P_Y_squared.clone().into(),
                    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        variable::<FieldT, pb_variable>::from(ONE),
                    ) * (-coeff_b),
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

pub type G1_add_gadgets<ppT, FieldT, PB> = gadget<FieldT, PB, G1_add_gadget<ppT, FieldT, PB>>;
impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> G1_add_gadget<ppT, FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: G1_variables<ppT, FieldT, PB>,
        B: G1_variables<ppT, FieldT, PB>,
        C: G1_variables<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> G1_add_gadgets<ppT, FieldT, PB> {
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
        let mut lambda = variable::<FieldT, pb_variable>::default();
        let mut inv = variable::<FieldT, pb_variable>::default();
        lambda.allocate(&pb, prefix_format!(annotation_prefix, " lambda"));
        inv.allocate(&pb, prefix_format!(annotation_prefix, " inv"));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> G1_add_gadgets<ppT, FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.B.t.X.clone().into(),
                    (self.t.A.t.X.clone() * (-1)).into(),
                ],
                vec![
                    self.t.B.t.Y.clone().into(),
                    (self.t.A.t.Y.clone() * (-1)).into(),
                ],
            ),
            prefix_format!(self.annotation_prefix, " calc_lambda"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
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
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.A.t.X.clone().into(),
                    (self.t.C.t.X.clone() * (-1)).into(),
                ],
                vec![self.t.C.t.Y.clone().into(), self.t.A.t.Y.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " calc_Y"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.inv.clone().into()],
                vec![
                    self.t.B.t.X.clone().into(),
                    (self.t.A.t.X.clone() * (-1)).into(),
                ],
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
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

pub type G1_dbl_gadgets<ppT, FieldT, PB> = gadget<FieldT, PB, G1_dbl_gadget<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> G1_dbl_gadget<ppT, FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: G1_variables<ppT, FieldT, PB>,
        B: G1_variables<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> G1_dbl_gadgets<ppT, FieldT, PB> {
        let mut Xsquared = variable::<FieldT, pb_variable>::default();
        let mut lambda = variable::<FieldT, pb_variable>::default();
        Xsquared.allocate(&pb, prefix_format!(annotation_prefix, " X_squared"));
        lambda.allocate(&pb, prefix_format!(annotation_prefix, " lambda"));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> G1_dbl_gadgets<ppT, FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.A.t.X.clone().into()],
                vec![self.t.A.t.X.clone().into()],
                vec![self.t.Xsquared.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " calc_Xsquared"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![(self.t.lambda.clone() * 2).into()],
                vec![self.t.A.t.Y.clone().into()],
                vec![
                    (self.t.Xsquared.clone() * 3).into(),
                    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        variable::<FieldT, pb_variable>::from(ONE),
                    ) * coeff_a,
                ],
            ),
            prefix_format!(self.annotation_prefix, " calc_lambda"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.B.t.X.clone().into(),
                    (self.t.A.t.X.clone() * 2).into(),
                ],
            ),
            prefix_format!(self.annotation_prefix, " calc_X"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.lambda.clone().into()],
                vec![
                    self.t.A.t.X.clone().into(),
                    (self.t.B.t.X.clone() * (-1)).into(),
                ],
                vec![self.t.B.t.Y.clone().into(), self.t.A.t.Y.clone().into()],
            ),
            prefix_format!(self.annotation_prefix, " calc_Y"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.Xsquared) =
            self.pb.borrow().lc_val(&self.t.A.t.X).squared();
        *self.pb.borrow_mut().val_ref(&self.t.lambda) =
            (self.pb.borrow().val(&self.t.Xsquared) * FieldT::from(3) + FieldT::from(coeff_a))
                * (FieldT::from(2) * self.pb.borrow().lc_val(&self.t.A.t.Y)).inverse();
        *self.pb.borrow_mut().lc_val_ref(&self.t.B.t.X) =
            self.pb.borrow().val(&self.t.lambda).squared()
                - FieldT::from(2) * self.pb.borrow().lc_val(&self.t.A.t.X);
        *self.pb.borrow_mut().lc_val_ref(&self.t.B.t.Y) = self.pb.borrow().val(&self.t.lambda)
            * (self.pb.borrow().lc_val(&self.t.A.t.X) - self.pb.borrow().lc_val(&self.t.B.t.X))
            - self.pb.borrow().lc_val(&self.t.A.t.Y);
    }
}

pub type G1_multiscalar_mul_gadgets<ppT, FieldT, PB> =
    gadget<FieldT, PB, G1_multiscalar_mul_gadget<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    G1_multiscalar_mul_gadget<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        base: G1_variables<ppT, FieldT, PB>,
        scalars: pb_variable_array<FieldT, PB>,
        elt_size: usize,
        points: Vec<G1_variables<ppT, FieldT, PB>>,
        result: G1_variables<ppT, FieldT, PB>,
        annotation_prefix: String,
    ) -> G1_multiscalar_mul_gadgets<ppT, FieldT, PB> {
        let num_points = points.len();
        let scalar_size = scalars.len();
        assert!(num_points >= 1);
        assert!(num_points * elt_size == scalar_size);
        let mut points_and_powers = vec![];
        let mut doublers = vec![];
        for i in 0..num_points {
            points_and_powers.push(points[i].clone());
            for j in 0..elt_size - 1 {
                points_and_powers.push(G1_variable::<ppT, FieldT, PB>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " points_{}_times_2_to_{}", i, j + 1),
                ));
                doublers.push(G1_dbl_gadget::<ppT, FieldT, PB>::new(
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
            computed_results.push(G1_variable::<ppT, FieldT, PB>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " computed_results_"),
            ));
            if i < scalar_size - 1 {
                chosen_results.push(G1_variable::<ppT, FieldT, PB>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " chosen_results_"),
                ));
            } else {
                chosen_results.push(result.clone());
            }

            adders.push(G1_add_gadget::<ppT, FieldT, PB>::new(
                pb.clone(),
                chosen_results[i].clone(),
                points_and_powers[i].clone(),
                computed_results[i].clone(),
                prefix_format!(annotation_prefix, " adders_"),
            ));
        }
        gadget::<FieldT, PB, Self>::new(
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

impl<ppT: ppTConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    G1_multiscalar_mul_gadgets<ppT, FieldT, PB>
{
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
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
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
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
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
                if self.pb.borrow().val(&self.t.scalars[i]) == FieldT::zero() {
                    self.pb.borrow().lc_val(&self.t.chosen_results[i].t.X)
                } else {
                    self.pb.borrow().lc_val(&self.t.computed_results[i].t.X)
                };
            *self
                .pb
                .borrow_mut()
                .lc_val_ref(&self.t.chosen_results[i + 1].t.Y) =
                if self.pb.borrow().val(&self.t.scalars[i]) == FieldT::zero() {
                    self.pb.borrow().lc_val(&self.t.chosen_results[i].t.Y)
                } else {
                    self.pb.borrow().lc_val(&self.t.computed_results[i].t.Y)
                };
        }
    }
}
