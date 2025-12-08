// Declaration of interfaces for Fp2 gadgets.

// The gadgets verify field arithmetic in Fp2 = Fp[U]/(U^2-non_residue),
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
 * Gadget that represents an Fp2 variable.
 */
pub trait Fp2TConfig<FieldT: FieldTConfig>: Default + Clone {
    // type FieldT: FieldTConfig;
    fn c0(&self) -> FieldT;
    fn c1(&self) -> FieldT;
    fn c0_mut(&mut self) -> &mut FieldT;
    fn c1_mut(&mut self) -> &mut FieldT;
    const non_residue: FieldT;
    const Frobenius_coeffs_c1: [FieldT; 2];
}

#[derive(Clone, Default)]
pub struct Fp2_variable<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp2T::my_Fp>
    //     type FieldT=Fp2T::my_Fp;
    pub c0: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub c1: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub all_vars: pb_linear_combination_array<FieldT, PB>,
    _t: PhantomData<(Fp2T, PB)>,
}

/**
 * Gadget that creates constraints for Fp2 by Fp2 multiplication.
 */
#[derive(Clone, Default)]
pub struct Fp2_mul_gadget<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget<Fp2T::my_Fp>
    //     type FieldT=Fp2T::my_Fp;
    pub A: Fp2_variables<Fp2T, FieldT, PB>,
    pub B: Fp2_variables<Fp2T, FieldT, PB>,
    pub result: Fp2_variables<Fp2T, FieldT, PB>,
    pub v1: variable<FieldT, pb_variable>,
}

/**
 * Gadget that creates constraints for Fp2 multiplication by a linear combination.
 */
#[derive(Clone, Default)]
pub struct Fp2_mul_by_lc_gadget<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget<Fp2T::my_Fp>
    // type FieldT=Fp2T::my_Fp;
    pub A: Fp2_variables<Fp2T, FieldT, PB>,
    pub lc: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub result: Fp2_variables<Fp2T, FieldT, PB>,
}

/**
 * Gadget that creates constraints for Fp2 squaring.
 */
#[derive(Clone, Default)]
pub struct Fp2_sqr_gadget<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp2T::my_Fp>
    // type FieldT=Fp2T::my_Fp;
    pub A: Fp2_variables<Fp2T, FieldT, PB>,
    pub result: Fp2_variables<Fp2T, FieldT, PB>,
}

// use crate::gadgetlib1::gadgets::fields::fp2_gadgets;

//#endif // FP2_GADGETS_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for Fp2 gadgets.

See fp2_gadgets.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef FP2_GADGETS_TCC_
// #define FP2_GADGETS_TCC_

pub type Fp2_variables<Fp2T, FieldT, PB> = gadget<FieldT, PB, Fp2_variable<Fp2T, FieldT, PB>>;
impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> Fp2_variable<Fp2T, FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        annotation_prefix: String,
    ) -> Fp2_variables<Fp2T, FieldT, PB> {
        let (mut c0_var, mut c1_var) = (
            variable::<FieldT, pb_variable>::default(),
            variable::<FieldT, pb_variable>::default(),
        );
        c0_var.allocate(&pb, prefix_format!(annotation_prefix, " c0"));
        c1_var.allocate(&pb, prefix_format!(annotation_prefix, " c1"));

        let mut c0 = pb_linear_combination::new_with_var::<FieldT>(c0_var);
        let mut c1 = pb_linear_combination::new_with_var::<FieldT>(c1_var);
        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(vec![c0.clone(), c1.clone()]);
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                all_vars,
                _t: PhantomData,
            },
        )
    }

    pub fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        el: Fp2T,
        annotation_prefix: String,
    ) -> Fp2_variables<Fp2T, FieldT, PB> {
        let mut c0 = linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        let mut c1 = linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        c0.assign(&pb, &(el.c0().into()));
        c1.assign(&pb, &(el.c1().into()));

        c0.evaluate_pb(&pb);
        c1.evaluate_pb(&pb);
        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(vec![c0.clone(), c1.clone()]);
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                all_vars,
                _t: PhantomData,
            },
        )
    }

    pub fn new3(
        pb: RcCell<protoboard<FieldT, PB>>,
        el: Fp2T,
        coeff: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Fp2_variables<Fp2T, FieldT, PB> {
        let mut c0 = linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        let mut c1 = linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        c0.assign(
            &pb,
            &(linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(el.c0())
                * coeff.clone()),
        );
        c1.assign(
            &pb,
            &(linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(el.c1())
                * coeff),
        );
        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(vec![c0.clone(), c1.clone()]);
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                all_vars,
                _t: PhantomData,
            },
        )
    }

    pub fn new4(
        pb: RcCell<protoboard<FieldT, PB>>,
        c0: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Fp2_variables<Fp2T, FieldT, PB> {
        let all_vars = pb_linear_combination_array::<FieldT, PB>::new(vec![c0.clone(), c1.clone()]);
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                c0,
                c1,
                all_vars,
                _t: PhantomData,
            },
        )
    }
}
impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> Fp2_variables<Fp2T, FieldT, PB> {
    pub fn generate_r1cs_equals_const_constraints(&self, el: &Fp2T) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(1),
                el.c0().into(),
                self.t.c0.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " c0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(1),
                el.c1().into(),
                self.t.c1.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " c1"),
        );
    }

    pub fn generate_r1cs_witness(&self, el: &Fp2T) {
        *self.pb.borrow_mut().lc_val_ref(&self.t.c0) = el.c0();
        *self.pb.borrow_mut().lc_val_ref(&self.t.c1) = el.c1();
    }

    pub fn get_element(&self) -> Fp2T {
        let mut el = Fp2T::default();
        *el.c0_mut() = self.pb.borrow().lc_val(&self.t.c0);
        *el.c1_mut() = self.pb.borrow().lc_val(&self.t.c1);
        return el;
    }

    pub fn mul_by_X(&self) -> Fp2_variables<Fp2T, FieldT, PB> {
        let (mut new_c0, mut new_c1) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        new_c0.assign(&self.pb, &(self.t.c1.clone() * Fp2T::non_residue));
        new_c1.assign(&self.pb, &self.t.c0);
        return Fp2_variable::<Fp2T, FieldT, PB>::new4(
            self.pb.clone(),
            new_c0,
            new_c1,
            prefix_format!(self.annotation_prefix, " mul_by_X"),
        );
    }

    pub fn evaluate(&self) {
        self.t.c0.evaluate_pb(&self.pb);
        self.t.c1.evaluate_pb(&self.pb);
    }

    pub fn is_constant(&self) -> bool {
        return (self.t.c0.is_constant() && self.t.c1.is_constant());
    }

    pub fn size_in_bits(&self) -> usize {
        return 2 * FieldT::size_in_bits();
    }

    pub fn num_variables(&self) -> usize {
        return 2;
    }
}

pub type Fp2_mul_gadgets<Fp2T, FieldT, PB> = gadget<FieldT, PB, Fp2_mul_gadget<Fp2T, FieldT, PB>>;
impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp2_mul_gadget<Fp2T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: Fp2_variables<Fp2T, FieldT, PB>,
        B: Fp2_variables<Fp2T, FieldT, PB>,
        result: Fp2_variables<Fp2T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp2_mul_gadgets<Fp2T, FieldT, PB> {
        let mut v1 = variable::<FieldT, pb_variable>::default();
        v1.allocate(&pb, prefix_format!(annotation_prefix, " v1"));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { A, B, v1, result })
    }
}
impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp2_mul_gadgets<Fp2T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        /*
            Karatsuba multiplication for Fp2:
                v0 = A.t.c0 * B.t.c0
                v1 = A.t.c1 * B.t.c1
                result.t.c0 = v0 + non_residue * v1
                result.t.c1 = (A.t.c0 + A.t.c1) * (B.t.c0 + B.t.c1) - v0 - v1

            Enforced with 3 constraints:
                A.t.c1 * B.t.c1 = v1
                A.t.c0 * B.t.c0 = result.t.c0 - non_residue * v1
                (A.t.c0+A.t.c1)*(B.t.c0+B.t.c1) = result.t.c1 + result.t.c0 + (1 - non_residue) * v1

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c1.clone().into(),
                self.t.B.t.c1.clone().into(),
                self.t.v1.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " v1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone().into(),
                self.t.B.t.c0.clone().into(),
                self.t.result.t.c0.clone() + self.t.v1.clone() * (-Fp2T::non_residue),
            ),
            prefix_format!(self.annotation_prefix, " result.t.c0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone() + self.t.A.t.c1.clone(),
                self.t.B.t.c0.clone() + self.t.B.t.c1.clone(),
                self.t.result.t.c1.clone()
                    + self.t.result.t.c0.clone()
                    + self.t.v1.clone() * (FieldT::one() - Fp2T::non_residue),
            ),
            prefix_format!(self.annotation_prefix, " result.t.c1"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        let aA = self.pb.borrow().lc_val(&self.t.A.t.c0) * self.pb.borrow().lc_val(&self.t.B.t.c0);
        *self.pb.borrow_mut().val_ref(&self.t.v1) =
            self.pb.borrow().lc_val(&self.t.A.t.c1) * self.pb.borrow().lc_val(&self.t.B.t.c1);
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c0) =
            aA.clone() + Fp2T::non_residue * self.pb.borrow().val(&self.t.v1);
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c1) =
            (self.pb.borrow().lc_val(&self.t.A.t.c0) + self.pb.borrow().lc_val(&self.t.A.t.c1))
                * (self.pb.borrow().lc_val(&self.t.B.t.c0)
                    + self.pb.borrow().lc_val(&self.t.B.t.c1))
                - aA
                - self.pb.borrow().lc_val(&(self.t.v1.clone().into()));
    }
}

pub type Fp2_mul_by_lc_gadgets<Fp2T, FieldT, PB> =
    gadget<FieldT, PB, Fp2_mul_by_lc_gadget<Fp2T, FieldT, PB>>;

impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp2_mul_by_lc_gadget<Fp2T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: Fp2_variables<Fp2T, FieldT, PB>,
        lc: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        result: Fp2_variables<Fp2T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp2_mul_by_lc_gadgets<Fp2T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { A, lc, result })
    }
}

impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp2_mul_by_lc_gadgets<Fp2T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone(),
                self.t.lc.clone(),
                self.t.result.t.c0.clone(),
            ),
            prefix_format!(self.annotation_prefix, " result.t.c0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c1.clone(),
                self.t.lc.clone(),
                self.t.result.t.c1.clone(),
            ),
            prefix_format!(self.annotation_prefix, " result.t.c1"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c0) =
            self.pb.borrow().lc_val(&self.t.A.t.c0) * self.pb.borrow().lc_val(&self.t.lc);
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c1) =
            self.pb.borrow().lc_val(&self.t.A.t.c1) * self.pb.borrow().lc_val(&self.t.lc);
    }
}

pub type Fp2_sqr_gadgets<Fp2T, FieldT, PB> = gadget<FieldT, PB, Fp2_sqr_gadget<Fp2T, FieldT, PB>>;

impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp2_sqr_gadget<Fp2T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: Fp2_variables<Fp2T, FieldT, PB>,
        result: Fp2_variables<Fp2T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp2_sqr_gadgets<Fp2T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { A, result })
    }
}
impl<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp2_sqr_gadgets<Fp2T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        /*
            Complex multiplication for Fp2:
                v0 = A.t.c0 * A.t.c1
                result.t.c0 = (A.t.c0 + A.t.c1) * (A.t.c0 + non_residue * A.t.c1) - (1 + non_residue) * v0
                result.t.c1 = 2 * v0

            Enforced with 2 constraints:
                (2*A.t.c0) * A.t.c1 = result.t.c1
                (A.t.c0 + A.t.c1) * (A.t.c0 + non_residue * A.t.c1) = result.t.c0 + result.t.c1 * (1 + non_residue)/2

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(2)
                    * self.t.A.t.c0.clone(),
                self.t.A.t.c1.clone(),
                self.t.result.t.c1.clone(),
            ),
            prefix_format!(self.annotation_prefix, " result.t.c1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.clone() + self.t.A.t.c1.clone(),
                self.t.A.t.c0.clone()
                    + linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        Fp2T::non_residue,
                    ) * self.t.A.t.c1.clone(),
                self.t.result.t.c0.clone()
                    + self.t.result.t.c1.clone()
                        * (FieldT::one() + Fp2T::non_residue)
                        * FieldT::from(2).inverse(),
            ),
            prefix_format!(self.annotation_prefix, " result.t.c0"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        let a = self.pb.borrow().lc_val(&self.t.A.t.c0);
        let b = self.pb.borrow().lc_val(&self.t.A.t.c1);
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c1) =
            FieldT::from(2) * a.clone() * b.clone();
        *self.pb.borrow_mut().lc_val_ref(&self.t.result.t.c0) = (a.clone() + b.clone())
            * (a.clone() + Fp2T::non_residue * b.clone())
            - a.clone() * b.clone()
            - Fp2T::non_residue * a.clone() * b.clone();
    }
}

//#endif // FP2_GADGETS_TCC_

// pub fn operator*(coeff:&FieldT) ->Fp2_variable<Fp2T,PB >
// {
//     linear_combination<FieldT,pb_variable,pb_linear_combination> new_c0, new_c1;
//     new_c0.assign(self.pb, self.c0 * coeff);
//     new_c1.assign(self.pb, self.c1 * coeff);
//     return Fp2_variable<Fp2T,PB >(self.pb, new_c0, new_c1, prefix_format!(self.annotation_prefix, " operator*"));
// }

// pub fn operator+(other:&Fp2_variable<Fp2T,PB >) ->Fp2_variable<Fp2T,PB >
// {
//     linear_combination<FieldT,pb_variable,pb_linear_combination> new_c0, new_c1;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     return Fp2_variable<Fp2T,PB >(self.pb, new_c0, new_c1, prefix_format!(self.annotation_prefix, " operator+"));
// }

// pub fn operator+(other:&Fp2T) ->Fp2_variable<Fp2T,PB >
// {
//     linear_combination<FieldT,pb_variable,pb_linear_combination> new_c0, new_c1;
//     new_c0.assign(self.pb, self.c0 + other.c0);
//     new_c1.assign(self.pb, self.c1 + other.c1);
//     return Fp2_variable<Fp2T,PB >(self.pb, new_c0, new_c1, prefix_format!(self.annotation_prefix, " operator+"));
// }
