// Declaration of interfaces for Fp3 gadgets.

// The gadgets verify field arithmetic in Fp3 = Fp[U]/(U^3-non_residue),
// where non_residue is in Fp.
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_variable,
};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::prefix_format;
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::One;
use rccell::RcCell;
use std::marker::PhantomData;

/**
 * Gadget that represents an Fp3 variable.
 */
trait Fp3TConfig: Default + Clone + std::ops::Mul<Output = Self> {
    type FieldT: FieldTConfig;
    fn c0(&self) -> Self::FieldT;
    fn c1(&self) -> Self::FieldT;
    fn c2(&self) -> Self::FieldT;
    fn c0_mut(&mut self) -> &mut Self::FieldT;
    fn c1_mut(&mut self) -> &mut Self::FieldT;
    fn c2_mut(&mut self) -> &mut Self::FieldT;
    const non_residue: Self::FieldT;
}
#[derive(Clone, Default)]
pub struct Fp3_variable<Fp3T: Fp3TConfig, PB: PBConfig> {
    //->Self public gadget<Fp3T::my_Fp>
    // type FieldT=Fp3T::my_Fp;
    pub c0: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
    pub c1: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
    pub c2: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
    pub all_vars: pb_linear_combination_array<Fp3T::FieldT, PB>,
    _t: PhantomData<PB>,
}

/**
 * Gadget that creates constraints for Fp3 by Fp3 multiplication.
 */
#[derive(Clone, Default)]
pub struct Fp3_mul_gadget<Fp3T: Fp3TConfig, PB: PBConfig> {
    // : public gadget<Fp3T::my_Fp>
    //     type FieldT=Fp3T::my_Fp;
    pub A: Fp3_variables<Fp3T, PB>,
    pub B: Fp3_variables<Fp3T, PB>,
    pub result: Fp3_variables<Fp3T, PB>,
    pub v0: variable<Fp3T::FieldT, pb_variable>,
    pub v4: variable<Fp3T::FieldT, pb_variable>,
}

/**
 * Gadget that creates constraints for Fp3 multiplication by a linear combination.
 */
#[derive(Clone, Default)]
pub struct Fp3_mul_by_lc_gadget<Fp3T: Fp3TConfig, PB: PBConfig> {
    // : public gadget<Fp3T::my_Fp>
    //     type FieldT=Fp3T::my_Fp;
    pub A: Fp3_variables<Fp3T, PB>,
    pub lc: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
    pub result: Fp3_variables<Fp3T, PB>,
}

/**
 * Gadget that creates constraints for Fp3 squaring.
 */
#[derive(Clone, Default)]
pub struct Fp3_sqr_gadget<Fp3T: Fp3TConfig, PB: PBConfig> {
    // : public gadget<Fp3T::my_Fp>
    //     type FieldT=Fp3T::my_Fp;
    pub A: Fp3_variables<Fp3T, PB>,
    pub result: Fp3_variables<Fp3T, PB>,
    pub mul: RcCell<Fp3_mul_gadgets<Fp3T, PB>>,
}

pub type Fp3_variables<Fp3T, PB> = gadget<<Fp3T as Fp3TConfig>::FieldT, PB, Fp3_variable<Fp3T, PB>>;

impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_variable<Fp3T, PB> {
    pub fn new(
        pb: RcCell<protoboard<Fp3T::FieldT, PB>>,
        annotation_prefix: String,
    ) -> Fp3_variables<Fp3T, PB> {
        let (mut c0_var, mut c1_var, mut c2_var) = (
            variable::<Fp3T::FieldT, pb_variable>::default(),
            variable::<Fp3T::FieldT, pb_variable>::default(),
            variable::<Fp3T::FieldT, pb_variable>::default(),
        );
        c0_var.allocate(&pb, prefix_format!(annotation_prefix, " c0"));
        c1_var.allocate(&pb, prefix_format!(annotation_prefix, " c1"));
        c2_var.allocate(&pb, prefix_format!(annotation_prefix, " c2"));
        let mut c0 = pb_linear_combination::new_with_var::<Fp3T::FieldT>(c0_var);
        let mut c1 = pb_linear_combination::new_with_var::<Fp3T::FieldT>(c1_var.clone());
        let mut c2 = pb_linear_combination::new_with_var::<Fp3T::FieldT>(c1_var);
        let all_vars = pb_linear_combination_array::<Fp3T::FieldT, PB>::new(vec![
            c0.clone(),
            c1.clone(),
            c2.clone(),
        ]);

        gadget::<Fp3T::FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                c2,
                all_vars,
                _t: PhantomData,
            },
        )
    }

    pub fn new2(
        pb: RcCell<protoboard<Fp3T::FieldT, PB>>,
        el: Fp3T,
        annotation_prefix: String,
    ) -> Fp3_variables<Fp3T, PB> {
        let mut c0 =
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default();
        let mut c1 =
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default();
        let mut c2 =
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default();

        c0.assign(&pb, &(el.c0().into()));
        c1.assign(&pb, &(el.c1().into()));
        c2.assign(&pb, &(el.c2().into()));

        c0.evaluate_pb(&pb);
        c1.evaluate_pb(&pb);
        c2.evaluate_pb(&pb);

        let all_vars = pb_linear_combination_array::<Fp3T::FieldT, PB>::new(vec![
            c0.clone(),
            c1.clone(),
            c2.clone(),
        ]);

        gadget::<Fp3T::FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                c2,
                all_vars,
                _t: PhantomData,
            },
        )
    }

    pub fn new3(
        pb: RcCell<protoboard<Fp3T::FieldT, PB>>,
        el: Fp3T,
        coeff: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Fp3_variables<Fp3T, PB> {
        let mut c0 =
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default();
        let mut c1 =
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default();
        let mut c2 =
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default();

        c0.assign(
            &pb,
            &(linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::from(
                el.c0(),
            ) * coeff.clone()),
        );
        c1.assign(
            &pb,
            &(linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::from(
                el.c1(),
            ) * coeff.clone()),
        );
        c2.assign(
            &pb,
            &(linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::from(
                el.c2(),
            ) * coeff),
        );

        let all_vars = pb_linear_combination_array::<Fp3T::FieldT, PB>::new(vec![
            c0.clone(),
            c1.clone(),
            c2.clone(),
        ]);

        gadget::<Fp3T::FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                c2,
                all_vars,
                _t: PhantomData,
            },
        )
    }

    pub fn new4(
        pb: RcCell<protoboard<Fp3T::FieldT, PB>>,
        c0: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
        c2: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Fp3_variables<Fp3T, PB> {
        let all_vars = pb_linear_combination_array::<Fp3T::FieldT, PB>::new(vec![
            c0.clone(),
            c1.clone(),
            c2.clone(),
        ]);

        gadget::<Fp3T::FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                c2,
                all_vars,
                _t: PhantomData,
            },
        )
    }
}
impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_variables<Fp3T, PB> {
    pub fn generate_r1cs_equals_const_constraints(&self, el: &Fp3T) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                1.into(),
                el.c0().into(),
                self.t.c0.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " c0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                1.into(),
                el.c1().into(),
                self.t.c1.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " c1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                1.into(),
                el.c2().into(),
                self.t.c2.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " c2"),
        );
    }

    pub fn generate_r1cs_witness(&self, el: &Fp3T) {
        *self.pb.borrow_mut().lc_val_ref(&self.t.c0) = el.c0();
        *self.pb.borrow_mut().lc_val_ref(&self.t.c1) = el.c1();
        *self.pb.borrow_mut().lc_val_ref(&self.t.c2) = el.c2();
    }

    pub fn get_element(&self) -> Fp3T {
        let mut el = Fp3T::default();
        *el.c0_mut() = self.pb.borrow().lc_val(&self.t.c0);
        *el.c1_mut() = self.pb.borrow().lc_val(&self.t.c1);
        *el.c2_mut() = self.pb.borrow().lc_val(&self.t.c2);
        return el;
    }

    pub fn mul_by_X(&self) -> Fp3_variables<Fp3T, PB> {
        let (mut new_c0, mut new_c1, mut new_c2) = (
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<Fp3T::FieldT, pb_variable, pb_linear_combination>::default(),
        );
        new_c0.assign(&self.pb, &(self.t.c2.clone() * Fp3T::non_residue));
        new_c1.assign(&self.pb, &self.t.c0);
        new_c2.assign(&self.pb, &self.t.c1);
        return Fp3_variable::<Fp3T, PB>::new4(
            self.pb.clone(),
            new_c0,
            new_c1,
            new_c2,
            prefix_format!(self.annotation_prefix, " mul_by_X"),
        );
    }

    pub fn evaluate(&self) {
        self.t.c0.evaluate_pb(&self.pb);
        self.t.c1.evaluate_pb(&self.pb);
        self.t.c2.evaluate_pb(&self.pb);
    }

    pub fn is_constant(&self) -> bool {
        return (self.t.c0.is_constant() && self.t.c1.is_constant() && self.t.c2.is_constant());
    }

    pub fn size_in_bits(&self) -> usize {
        return 3 * <Fp3T as Fp3TConfig>::FieldT::size_in_bits();
    }

    pub fn num_variables(&self) -> usize {
        return 3;
    }
}

pub type Fp3_mul_gadgets<Fp3T, PB> =
    gadget<<Fp3T as Fp3TConfig>::FieldT, PB, Fp3_mul_gadget<Fp3T, PB>>;

impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_mul_gadget<Fp3T, PB> {
    pub fn new(
        pb: RcCell<protoboard<Fp3T::FieldT, PB>>,
        A: Fp3_variables<Fp3T, PB>,
        B: Fp3_variables<Fp3T, PB>,
        result: Fp3_variables<Fp3T, PB>,
        annotation_prefix: String,
    ) -> Fp3_mul_gadgets<Fp3T, PB> {
        let mut v0 = variable::<Fp3T::FieldT, pb_variable>::default();
        v0.allocate(&pb, prefix_format!(annotation_prefix, " v0"));
        let mut v4 = variable::<Fp3T::FieldT, pb_variable>::default();
        v4.allocate(&pb, prefix_format!(annotation_prefix, " v4"));

        gadget::<Fp3T::FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                A,
                B,
                v0,
                v4,
                result,
            },
        )
    }
}
impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_mul_gadgets<Fp3T, PB> {
    pub fn generate_r1cs_constraints(&self) {
        /*
            Tom-Cook-3x for Fp3:
                v0 = A.c0 * B.c0
                v1 = (A.c0 + A.c1 + A.c2) * (B.c0 + B.c1 + B.c2)
                v2 = (A.c0 - A.c1 + A.c2) * (B.c0 - B.c1 + B.c2)
                v3 = (A.c0 + 2*A.c1 + 4*A.c2) * (B.c0 + 2*B.c1 + 4*B.c2)
                v4 = A.c2 * B.c2
                result.c0 = v0 + non_residue * (v0/2 - v1/2 - v2/6 + v3/6 - 2*v4)
                result.c1 = -(1/2) v0 +  v1 - (1/3) v2 - (1/6) v3 + 2 v4 + non_residue*v4
                result.c2 = -v0 + (1/2) v1 + (1/2) v2 - v4

            Enforced with 5 constraints. Doing so requires some care, as we first
            compute two of the v_i explicitly, and then "inline" result.c1/c2/c3
            in computations of teh remaining three v_i.

            Concretely, we first compute v0 and v4 explicitly, via 2 constraints:
                A.c0 * B.c0 = v0
                A.c2 * B.c2 = v4
            Then we use the following 3 additional constraints:
                v1 = result.c1 + result.c2 + (result.c0 - v0)/non_residue + v0 + v4 - non_residue v4
                v2 = -result.c1 + result.c2 + v0 + (-result.c0 + v0)/non_residue + v4 + non_residue v4
                v3 = 2 * result.c1 + 4 result.c2 + (8*(result.c0 - v0))/non_residue + v0 + 16 * v4 - 2 * non_residue * v4

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab

            NOTE: the expressions above were cherry-picked from the Mathematica result
            of the following command:

            (# -> Solve[{c0 == v0 + non_residue*(v0/2 - v1/2 - v2/6 + v3/6 - 2 v4),
                        c1 == -(1/2) v0 + v1 - (1/3) v2 - (1/6) v3 + 2 v4 + non_residue*v4,
                        c2 == -v0 + (1/2) v1 + (1/2) v2 - v4}, #] // FullSimplify) & /@
            Subsets[{v0, v1, v2, v3, v4}, {3}]
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone().into(),
                self.t.B.t.c0.clone().into(),
                self.t.v0.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " v0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c2.clone().into(),
                self.t.B.t.c2.clone().into(),
                self.t.v4.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " v4"),
        );

        let beta = Fp3T::non_residue;

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone() + self.t.A.t.c1.clone() + self.t.A.t.c2.clone(),
                self.t.B.t.c0.clone() + self.t.B.t.c1.clone() + self.t.B.t.c2.clone(),
                self.t.result.t.c1.clone()
                    + self.t.result.t.c2.clone()
                    + self.t.result.t.c0.clone() * beta.inverse()
                    + self.t.v0.clone() * (Fp3T::FieldT::from(1) - beta.inverse())
                    + self.t.v4.clone() * (Fp3T::FieldT::from(1) - beta.clone()),
            ),
            prefix_format!(self.annotation_prefix, " v1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone() - self.t.A.t.c1.clone() + self.t.A.t.c2.clone(),
                self.t.B.t.c0.clone() - self.t.B.t.c1.clone() + self.t.B.t.c2.clone(),
                -self.t.result.t.c1.clone()
                    + self.t.result.t.c2.clone()
                    + self.t.v0.clone() * (Fp3T::FieldT::from(1) + beta.inverse())
                    - self.t.result.t.c0.clone() * beta.inverse()
                    + self.t.v4.clone() * (Fp3T::FieldT::from(1) + beta.clone()),
            ),
            prefix_format!(self.annotation_prefix, " v2"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone() + self.t.A.t.c1.clone() * 2 + self.t.A.t.c2.clone() * 4,
                self.t.B.t.c0.clone() + self.t.B.t.c1.clone() * 2 + self.t.B.t.c2.clone() * 4,
                self.t.result.t.c1.clone() * 2
                    + self.t.result.t.c2.clone() * 2
                    + self.t.result.t.c0.clone() * (Fp3T::FieldT::from(8) * beta.inverse())
                    + self.t.v0.clone()
                        * (Fp3T::FieldT::from(1) - Fp3T::FieldT::from(8) * beta.inverse())
                    + self.t.v4.clone() * (Fp3T::FieldT::from(16) - Fp3T::FieldT::from(2) * beta),
            ),
            prefix_format!(self.annotation_prefix, " v3"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.v0) =
            self.pb.borrow().lc_val(&self.t.A.t.c0) * self.pb.borrow().lc_val(&self.t.B.t.c0);
        *self.pb.borrow_mut().val_ref(&self.t.v4) =
            self.pb.borrow().lc_val(&self.t.A.t.c2) * self.pb.borrow().lc_val(&self.t.B.t.c2);

        let Aval = self.t.A.get_element();
        let Bval = self.t.B.get_element();
        let Rval = Aval * Bval;
        self.t.result.generate_r1cs_witness(&Rval);
    }
}
pub type Fp3_mul_by_lc_gadgets<Fp3T, PB> =
    gadget<<Fp3T as Fp3TConfig>::FieldT, PB, Fp3_mul_by_lc_gadget<Fp3T, PB>>;

impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_mul_by_lc_gadget<Fp3T, PB> {
    pub fn new(
        pb: RcCell<protoboard<Fp3T::FieldT, PB>>,
        A: Fp3_variables<Fp3T, PB>,
        lc: linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination>,
        result: Fp3_variables<Fp3T, PB>,
        annotation_prefix: String,
    ) -> Fp3_mul_by_lc_gadgets<Fp3T, PB> {
        gadget::<Fp3T::FieldT, PB, Self>::new(pb, annotation_prefix, Self { A, lc, result })
    }
}
impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_mul_by_lc_gadgets<Fp3T, PB> {
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone(),
                self.t.lc.clone(),
                self.t.result.t.c0.clone(),
            ),
            prefix_format!(self.annotation_prefix, " result.c0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c1.clone(),
                self.t.lc.clone(),
                self.t.result.t.c1.clone(),
            ),
            prefix_format!(self.annotation_prefix, " result.c1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<Fp3T::FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c2.clone(),
                self.t.lc.clone(),
                self.t.result.t.c2.clone(),
            ),
            prefix_format!(self.annotation_prefix, " result.c2"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c0) =
            self.pb.borrow().lc_val(&self.t.A.t.c0) * self.pb.borrow().lc_val(&self.t.lc);
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c1) =
            self.pb.borrow().lc_val(&self.t.A.t.c1) * self.pb.borrow().lc_val(&self.t.lc);
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c2) =
            self.pb.borrow().lc_val(&self.t.A.t.c2) * self.pb.borrow().lc_val(&self.t.lc);
    }
}
pub type Fp3_sqr_gadgets<Fp3T, PB> =
    gadget<<Fp3T as Fp3TConfig>::FieldT, PB, Fp3_sqr_gadget<Fp3T, PB>>;

impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_sqr_gadget<Fp3T, PB> {
    pub fn new(
        pb: RcCell<protoboard<Fp3T::FieldT, PB>>,
        A: Fp3_variables<Fp3T, PB>,
        result: Fp3_variables<Fp3T, PB>,
        annotation_prefix: String,
    ) -> Fp3_sqr_gadgets<Fp3T, PB> {
        let mul = RcCell::new(Fp3_mul_gadget::<Fp3T, PB>::new(
            pb.clone(),
            A.clone(),
            A.clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " mul"),
        ));
        gadget::<Fp3T::FieldT, PB, Self>::new(pb, annotation_prefix, Self { A, mul, result })
    }
}
impl<Fp3T: Fp3TConfig, PB: PBConfig> Fp3_sqr_gadgets<Fp3T, PB> {
    pub fn generate_r1cs_constraints(&self) {
        // We can't do better than 5 constraints for squaring, so we just use multiplication.
        self.t.mul.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.mul.borrow().generate_r1cs_witness();
    }
}

//
// Fp3_variable<Fp3T, PB> pub fn operator*(coeff:&FieldT) const
// {
//     linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination> new_c0, new_c1, new_c2;
//     new_c0.assign(self.pb, self.c0 * coeff);
//     new_c1.assign(self.pb, self.c1 * coeff);
//     new_c2.assign(self.pb, self.c2 * coeff);
//     return Fp3_variable<Fp3T, PB> (self.pb, new_c0, new_c1, new_c2, prefix_format!(self.annotation_prefix, " operator*"));
// }

//
// Fp3_variable<Fp3T, PB> pub fn operator+(other:&Fp3_variable<Fp3T, PB> ) const
// {
//     linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination> new_c0, new_c1, new_c2;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     new_c2.assign(self.pb, self.c2 + other.c2);
//     return Fp3_variable<Fp3T, PB> (self.pb, new_c0, new_c1, new_c2, prefix_format!(self.annotation_prefix, " operator+"));
// }

//
// Fp3_variable<Fp3T, PB> pub fn operator+(other:&Fp3T) const
// {
//     linear_combination<Fp3T::FieldT, pb_variable, pb_linear_combination> new_c0, new_c1, new_c2;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     new_c2.assign(self.pb, self.c2 + other.c2);
//     return Fp3_variable<Fp3T, PB> (self.pb, new_c0, new_c1, new_c2, prefix_format!(self.annotation_prefix, " operator+"));
// }
