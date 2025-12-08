use crate::gadgetlib1::gadget::gadget;
/** @file
*****************************************************************************

Declaration of interfaces for Fp4 gadgets.

The gadgets verify field arithmetic in Fp4 = Fp2[V]/(V^2-U) where
Fp2 = Fp[U]/(U^2-non_residue) and non_residue is in Fp.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef FP4_GADGETS_HPP_
// #define FP4_GADGETS_HPP_
use crate::gadgetlib1::gadgets::fields::fp2_gadgets::{
    Fp2_mul_gadget, Fp2_mul_gadgets, Fp2_sqr_gadget, Fp2_sqr_gadgets, Fp2_variable, Fp2_variables,
    Fp2TConfig,
};
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

pub type pb_linear_combinations<FieldT> =
    linear_combination<FieldT, pb_variable, pb_linear_combination>;

#[inline]
pub fn default_pb_lc<FieldT: FieldTConfig>()
-> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    linear_combination::<FieldT, pb_variable, pb_linear_combination>::default()
}

#[inline]
pub fn i64_to_pb_lc<FieldT: FieldTConfig>(
    d: i64,
) -> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(d)
}

#[inline]
pub fn field_to_pb_lc<FieldT: FieldTConfig>(
    f: FieldT,
) -> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(f)
}

#[inline]
pub fn inverse<FieldT: FieldTConfig>(d: i32) -> FieldT {
    FieldT::from(d).inverse()
}
/**
 * Gadget that represents an Fp4 variable.
 */
pub trait Fp4TConfig<FieldT: FieldTConfig>:
    FieldTConfig + Default + Clone + std::ops::Mul<Output = Self>
{
    // type FieldT: FieldTConfig;
    type Fp2T: Fp2TConfig<FieldT>;
    fn c0(&self) -> Self::Fp2T;
    fn c1(&self) -> Self::Fp2T;
    fn c0_mut(&mut self) -> &mut Self::Fp2T;
    fn c1_mut(&mut self) -> &mut Self::Fp2T;
    const non_residue: FieldT;
    const Frobenius_coeffs_c1: [FieldT; 2];
}

#[derive(Clone, Default)]
pub struct Fp4_variable<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp4T::my_Fp>
    //     type FieldT=Fp4T::my_Fp;
    //     type Fp2T=Fp4T::my_Fpe;
    pub c0: Fp2_variables<Fp4T::Fp2T, FieldT, PB>,
    pub c1: Fp2_variables<Fp4T::Fp2T, FieldT, PB>,
}

/**
 * Gadget that creates constraints for Fp4 multiplication (towering formulas).
 */
#[derive(Clone, Default)]
pub struct Fp4_tower_mul_gadget<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp4T::my_Fp>
    // type FieldT=Fp4T::my_Fp;
    // type Fp2T=Fp4T::my_Fpe;
    pub A: Fp4_variables<Fp4T, FieldT, PB>,
    pub B: Fp4_variables<Fp4T, FieldT, PB>,
    pub result: Fp4_variables<Fp4T, FieldT, PB>,
    pub v0_c0: pb_linear_combinations<FieldT>,
    pub v0_c1: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c0: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c1: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub v0: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub v1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub Bc0_plus_Bc1_c0: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1_c1: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub result_c1_plus_v0_plus_v1_c0: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1_c1: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_v0: RcCell<Fp2_mul_gadgets<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_v1: RcCell<Fp2_mul_gadgets<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_result_c1: RcCell<Fp2_mul_gadgets<Fp4T::Fp2T, FieldT, PB>>,
}

/**
 * Gadget that creates constraints for Fp4 multiplication (direct formulas).
 */
#[derive(Clone, Default)]
pub struct Fp4_direct_mul_gadget<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp4T::my_Fp>
    //     type FieldT=Fp4T::my_Fp;
    //     type Fp2T=Fp4T::my_Fpe;
    pub A: Fp4_variables<Fp4T, FieldT, PB>,
    pub B: Fp4_variables<Fp4T, FieldT, PB>,
    pub result: Fp4_variables<Fp4T, FieldT, PB>,
    pub v1: variable<FieldT, pb_variable>,
    pub v2: variable<FieldT, pb_variable>,
    pub v6: variable<FieldT, pb_variable>,
}

/**
 * Alias default multiplication gadget
 */
//
pub type Fp4_mul_gadget<Fp4T, FieldT, PB> = Fp4_direct_mul_gadget<Fp4T, FieldT, PB>;

/**
 * Gadget that creates constraints for Fp4 squaring.
 */
#[derive(Clone, Default)]
pub struct Fp4_sqr_gadget<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp4T::my_Fp>
    //     type FieldT=Fp4T::my_Fp;
    //     type Fp2T=Fp4T::my_Fpe;
    pub A: Fp4_variables<Fp4T, FieldT, PB>,
    pub result: Fp4_variables<Fp4T, FieldT, PB>,
    pub v1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub v0_c0: pb_linear_combinations<FieldT>,
    pub v0_c1: pb_linear_combinations<FieldT>,
    pub v0: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_v0: RcCell<Fp2_sqr_gadgets<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_v1: RcCell<Fp2_sqr_gadgets<Fp4T::Fp2T, FieldT, PB>>,
    pub Ac0_plus_Ac1_c0: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c1: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub result_c1_plus_v0_plus_v1_c0: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1_c1: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_result_c1: RcCell<Fp2_sqr_gadgets<Fp4T::Fp2T, FieldT, PB>>,
}

/**
 * Gadget that creates constraints for Fp4 cyclotomic squaring
 */
#[derive(Clone, Default)]
pub struct Fp4_cyclotomic_sqr_gadget<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp4T::my_Fp>
    //     type FieldT=Fp4T::my_Fp;
    //     type Fp2T=Fp4T::my_Fpe;
    pub A: Fp4_variables<Fp4T, FieldT, PB>,
    pub result: Fp4_variables<Fp4T, FieldT, PB>,
    pub c0_expr_c0: pb_linear_combinations<FieldT>,
    pub c0_expr_c1: pb_linear_combinations<FieldT>,
    pub c0_expr: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_c0_expr: RcCell<Fp2_sqr_gadgets<Fp4T::Fp2T, FieldT, PB>>,
    pub A_c0_plus_A_c1_c0: pb_linear_combinations<FieldT>,
    pub A_c0_plus_A_c1_c1: pb_linear_combinations<FieldT>,
    pub A_c0_plus_A_c1: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub c1_expr_c0: pb_linear_combinations<FieldT>,
    pub c1_expr_c1: pb_linear_combinations<FieldT>,
    pub c1_expr: RcCell<Fp2_variables<Fp4T::Fp2T, FieldT, PB>>,
    pub compute_c1_expr: RcCell<Fp2_sqr_gadgets<Fp4T::Fp2T, FieldT, PB>>,
}

pub type Fp4_variables<Fp4T, FieldT, PB> = gadget<FieldT, PB, Fp4_variable<Fp4T, FieldT, PB>>;
impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> Fp4_variable<Fp4T, FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        annotation_prefix: String,
    ) -> Fp4_variables<Fp4T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(
            pb.clone(),
            annotation_prefix.clone(),
            Self {
                c0: Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " c0"),
                ),
                c1: Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " c1"),
                ),
            },
        )
    }

    pub fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        el: Fp4T,
        annotation_prefix: String,
    ) -> Fp4_variables<Fp4T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(
            pb.clone(),
            annotation_prefix.clone(),
            Self {
                c0: Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new2(
                    pb.clone(),
                    el.c0(),
                    prefix_format!(annotation_prefix, " c0"),
                ),
                c1: Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new2(
                    pb.clone(),
                    el.c1(),
                    prefix_format!(annotation_prefix, " c1"),
                ),
            },
        )
    }

    pub fn new3(
        pb: RcCell<protoboard<FieldT, PB>>,
        c0: Fp2_variables<Fp4T::Fp2T, FieldT, PB>,
        c1: Fp2_variables<Fp4T::Fp2T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp4_variables<Fp4T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { c0, c1 })
    }
}
impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> Fp4_variables<Fp4T, FieldT, PB> {
    pub fn generate_r1cs_equals_const_constraints(&self, el: &Fp4T) {
        self.t.c0.generate_r1cs_equals_const_constraints(&el.c0());
        self.t.c1.generate_r1cs_equals_const_constraints(&el.c1());
    }

    pub fn generate_r1cs_witness(&self, el: &Fp4T) {
        self.t.c0.generate_r1cs_witness(&el.c0());
        self.t.c1.generate_r1cs_witness(&el.c1());
    }

    pub fn get_element(&self) -> Fp4T {
        let mut el = Fp4T::default();
        *el.c0_mut() = self.t.c0.get_element();
        *el.c1_mut() = self.t.c1.get_element();
        return el;
    }

    pub fn Frobenius_map(&self, power: usize) -> Fp4_variables<Fp4T, FieldT, PB> {
        let (mut new_c0c0, mut new_c0c1, mut new_c1c0, mut new_c1c1) = (
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
        );
        new_c0c0.assign(&self.pb, &self.t.c0.t.c0);
        new_c0c1.assign(
            &self.pb,
            &(self.t.c0.t.c1.clone() * Fp4T::Fp2T::Frobenius_coeffs_c1[power % 2].clone()),
        );
        new_c1c0.assign(
            &self.pb,
            &(self.t.c1.t.c0.clone() * Fp4T::Fp2T::Frobenius_coeffs_c1[power % 4].clone()),
        );
        new_c1c1.assign(
            &self.pb,
            &(self.t.c1.t.c1.clone()
                * Fp4T::Fp2T::Frobenius_coeffs_c1[power % 4].clone()
                * Fp4T::Fp2T::Frobenius_coeffs_c1[power % 2].clone()),
        );

        return Fp4_variable::<Fp4T, FieldT, PB>::new3(
            self.pb.clone(),
            Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
                self.pb.clone(),
                new_c0c0.clone(),
                new_c0c1.clone(),
                prefix_format!(self.annotation_prefix, " Frobenius_map_c0"),
            ),
            Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
                self.pb.clone(),
                new_c1c0,
                new_c1c1,
                prefix_format!(self.annotation_prefix, " Frobenius_map_c1"),
            ),
            prefix_format!(self.annotation_prefix, " Frobenius_map"),
        );
    }

    pub fn evaluate(&self) {
        self.t.c0.evaluate();
        self.t.c1.evaluate();
    }
}

pub type Fp4_tower_mul_gadgets<Fp4T, FieldT, PB> =
    gadget<FieldT, PB, Fp4_tower_mul_gadget<Fp4T, FieldT, PB>>;
impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_tower_mul_gadget<Fp4T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: Fp4_variables<Fp4T, FieldT, PB>,
        B: Fp4_variables<Fp4T, FieldT, PB>,
        result: Fp4_variables<Fp4T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp4_tower_mul_gadgets<Fp4T, FieldT, PB> {
        /*
          Karatsuba multiplication for Fp4 as a quadratic extension of Fp2:
          v0 = A.t.c0 * B.t.c0
          v1 = A.t.c1 * B.t.c1
          self.t.result.t.c0 = v0 + non_residue * v1
          self.t.result.t.c1 = (A.t.c0 + A.t.c1) * (B.t.c0 + B.t.c1) - v0 - v1
          where "non_residue * elem" := (non_residue * elt.c1, elt.c0)

          Enforced with 3 Fp2_mul_gadget's that ensure that:
          A.t.c1 * B.t.c1 = v1
          A.t.c0 * B.t.c0 = v0
          (A.t.c0+A.t.c1)*(B.t.c0+B.t.c1) = self.t.result.t.c1 + v0 + v1

          Reference:
          "Multiplication and Squaring on Pairing-Friendly Fields"
          Devegili, OhEigeartaigh, Scott, Dahab
        */
        let v1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " v1"),
        ));

        let compute_v1 = RcCell::new(Fp2_mul_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            A.t.c1.clone(),
            B.t.c1.clone(),
            v1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_v1"),
        ));
        let mut v0_c0 = default_pb_lc::<FieldT>();

        let mut v0_c0 = default_pb_lc::<FieldT>();
        let mut v0_c1 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c0 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c1 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c0 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c1 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c0 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c1 = default_pb_lc::<FieldT>();

        v0_c0.assign(
            &pb,
            &(result.t.c0.t.c0.clone() - v1.borrow().t.c1.clone() * Fp4T::Fp2T::non_residue),
        );
        v0_c1.assign(&pb, &(result.t.c0.t.c1.clone() - v1.borrow().t.c0.clone()));
        let v0 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            v0_c0.clone(),
            v0_c1.clone(),
            prefix_format!(annotation_prefix, " v0"),
        ));

        let compute_v0 = RcCell::new(Fp2_mul_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            A.t.c0.clone(),
            B.t.c0.clone(),
            v0.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_v0"),
        ));

        Ac0_plus_Ac1_c0.assign(&pb, &(A.t.c0.t.c0.clone() + A.t.c1.t.c0.clone()));
        Ac0_plus_Ac1_c1.assign(&pb, &(A.t.c0.t.c1.clone() + A.t.c1.t.c1.clone()));
        let Ac0_plus_Ac1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            Ac0_plus_Ac1_c0.clone(),
            Ac0_plus_Ac1_c1.clone(),
            prefix_format!(annotation_prefix, " Ac0_plus_Ac1"),
        ));

        Bc0_plus_Bc1_c0.assign(&pb, &(B.t.c0.t.c0.clone() + B.t.c1.t.c0.clone()));
        Bc0_plus_Bc1_c1.assign(&pb, &(B.t.c0.t.c1.clone() + B.t.c1.t.c1.clone()));
        let Bc0_plus_Bc1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            Bc0_plus_Bc1_c0.clone(),
            Bc0_plus_Bc1_c1.clone(),
            prefix_format!(annotation_prefix, " Bc0_plus_Bc1"),
        ));

        result_c1_plus_v0_plus_v1_c0.assign(
            &pb,
            &(result.t.c1.t.c0.clone() + v0.borrow().t.c0.clone() + v1.borrow().t.c0.clone()),
        );
        result_c1_plus_v0_plus_v1_c1.assign(
            &pb,
            &(result.t.c1.t.c1.clone() + v0.borrow().t.c1.clone() + v1.borrow().t.c1.clone()),
        );
        let result_c1_plus_v0_plus_v1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            result_c1_plus_v0_plus_v1_c0.clone(),
            result_c1_plus_v0_plus_v1_c1.clone(),
            prefix_format!(annotation_prefix, " result_c1_plus_v0_plus_v1"),
        ));

        let compute_result_c1 = RcCell::new(Fp2_mul_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            Ac0_plus_Ac1.borrow().clone(),
            Bc0_plus_Bc1.borrow().clone(),
            result_c1_plus_v0_plus_v1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_result_c1"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                A,
                B,
                result,
                v0_c0,
                v0_c1,
                Ac0_plus_Ac1_c0,
                Ac0_plus_Ac1_c1,
                Ac0_plus_Ac1,
                v0,
                v1,
                Bc0_plus_Bc1_c0,
                Bc0_plus_Bc1_c1,
                Bc0_plus_Bc1,
                result_c1_plus_v0_plus_v1_c0,
                result_c1_plus_v0_plus_v1_c1,
                result_c1_plus_v0_plus_v1,
                compute_v0,
                compute_v1,
                compute_result_c1,
            },
        )
    }
}

impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_tower_mul_gadgets<Fp4T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_v0.borrow().generate_r1cs_constraints();
        self.t.compute_v1.borrow().generate_r1cs_constraints();
        self.t
            .compute_result_c1
            .borrow()
            .generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_v0.borrow().generate_r1cs_witness();
        self.t.compute_v1.borrow().generate_r1cs_witness();

        self.t.Ac0_plus_Ac1_c0.evaluate_pb(&self.pb);
        self.t.Ac0_plus_Ac1_c1.evaluate_pb(&self.pb);

        self.t.Bc0_plus_Bc1_c0.evaluate_pb(&self.pb);
        self.t.Bc0_plus_Bc1_c1.evaluate_pb(&self.pb);

        self.t.compute_result_c1.borrow().generate_r1cs_witness();

        let Aval = self.t.A.get_element();
        let Bval = self.t.B.get_element();
        let Rval = Aval * Bval;

        self.t.result.generate_r1cs_witness(&Rval);
    }
}

pub type Fp4_direct_mul_gadgets<Fp4T, FieldT, PB> =
    gadget<FieldT, PB, Fp4_direct_mul_gadget<Fp4T, FieldT, PB>>;

impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_direct_mul_gadget<Fp4T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: Fp4_variables<Fp4T, FieldT, PB>,
        B: Fp4_variables<Fp4T, FieldT, PB>,
        result: Fp4_variables<Fp4T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp4_direct_mul_gadgets<Fp4T, FieldT, PB> {
        /*
            Tom-Cook-4x for Fp4 (beta is the quartic non-residue):
                v0 = a0*b0,
                v1 = (a0+a1+a2+a3)*(b0+b1+b2+b3),
                v2 = (a0-a1+a2-a3)*(b0-b1+b2-b3),
                v3 = (a0+2a1+4a2+8a3)*(b0+2b1+4b2+8b3),
                v4 = (a0-2a1+4a2-8a3)*(b0-2b1+4b2-8b3),
                v5 = (a0+3a1+9a2+27a3)*(b0+3b1+9b2+27b3),
                v6 = a3*b3

                self.t.result.t.c0 = v0+beta((1/4)v0-(1/6)(v1+v2)+(1/24)(v3+v4)-5v6),
                self.t.result.t.c1 = -(1/3)v0+v1-(1/2)v2-(1/4)v3+(1/20)v4+(1/30)v5-12v6+beta(-(1/12)(v0-v1)+(1/24)(v2-v3)-(1/120)(v4-v5)-3v6),
                self.t.result.c2 = -(5/4)v0+(2/3)(v1+v2)-(1/24)(v3+v4)+4v6+beta v6,
                self.t.result.c3 = (1/12)(5v0-7v1)-(1/24)(v2-7v3+v4+v5)+15v6

            Enforced with 7 constraints. Doing so requires some care, as we first
            compute three of the v_i explicitly, and then "inline" self.t.result.t.c0/c1/c2/c3
            in computations of the remaining four v_i.

            Concretely, we first compute v1, v2 and v6 explicitly, via 3 constraints as above.
                v1 = (a0+a1+a2+a3)*(b0+b1+b2+b3),
                v2 = (a0-a1+a2-a3)*(b0-b1+b2-b3),
                v6 = a3*b3

            Then we use the following 4 additional constraints:
                (1-beta) v0 = c0 + beta c2 - (beta v1)/2 - (beta v2)/ 2 - (-1 + beta) beta v6
                (1-beta) v3 = -15 c0 - 30 c1 - 3 (4 + beta) c2 - 6 (4 + beta) c3 + (24 - (3 beta)/2) v1 + (-8 + beta/2) v2 + 3 (-16 + beta) (-1 + beta) v6
                (1-beta) v4 = -15 c0 + 30 c1 - 3 (4 + beta) c2 + 6 (4 + beta) c3 + (-8 + beta/2) v1 + (24 - (3 beta)/2) v2 + 3 (-16 + beta) (-1 + beta) v6
                (1-beta) v5 = -80 c0 - 240 c1 - 8 (9 + beta) c2 - 24 (9 + beta) c3 - 2 (-81 + beta) v1 + (-81 + beta) v2 + 8 (-81 + beta) (-1 + beta) v6

            The isomorphism between the representation above and towering is:
                (a0, a1, a2, a3) <-> (a.c0.t.c0, a.c1.t.c0, a.c0.t.c1, a.c1.t.c1)

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab

            NOTE: the expressions above were cherry-picked from the Mathematica result
            of the following command:

            (# -> Solve[{c0 == v0+beta((1/4)v0-(1/6)(v1+v2)+(1/24)(v3+v4)-5v6),
            c1 == -(1/3)v0+v1-(1/2)v2-(1/4)v3+(1/20)v4+(1/30)v5-12v6+beta(-(1/12)(v0-v1)+(1/24)(v2-v3)-(1/120)(v4-v5)-3v6),
            c2 == -(5/4)v0+(2/3)(v1+v2)-(1/24)(v3+v4)+4v6+beta v6,
            c3 == (1/12)(5v0-7v1)-(1/24)(v2-7v3+v4+v5)+15v6}, #] // FullSimplify) & /@ Subsets[{v0, v1, v2, v3, v4, v5}, {4}]

            and simplified by multiplying the selected result by (1-beta)
        */
        let mut v1 = variable::<FieldT, pb_variable>::default();
        let mut v2 = variable::<FieldT, pb_variable>::default();
        let mut v6 = variable::<FieldT, pb_variable>::default();
        v1.allocate(&pb, prefix_format!(annotation_prefix, " v1"));
        v2.allocate(&pb, prefix_format!(annotation_prefix, " v2"));
        v6.allocate(&pb, prefix_format!(annotation_prefix, " v6"));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                A,
                B,
                result,
                v1,
                v2,
                v6,
            },
        )
    }
}
impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_direct_mul_gadgets<Fp4T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        let beta = Fp4T::Fp2T::non_residue;
        let u = (FieldT::one() - beta.clone()).inverse();

        // const pb_linear_combinations<FieldT>
        let a0 = &self.t.A.t.c0.t.c0;
        let a1 = &self.t.A.t.c1.t.c0;
        let a2 = &self.t.A.t.c0.t.c1;
        let a3 = &self.t.A.t.c1.t.c1;
        let b0 = &self.t.B.t.c0.t.c0;
        let b1 = &self.t.B.t.c1.t.c0;
        let b2 = &self.t.B.t.c0.t.c1;
        let b3 = &self.t.B.t.c1.t.c1;
        let c0 = &self.t.result.t.c0.t.c0;
        let c1 = &self.t.result.t.c1.t.c0;
        let c2 = &self.t.result.t.c0.t.c1;
        let c3 = &self.t.result.t.c1.t.c1;

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                a0.clone() + a1.clone() + a2.clone() + a3.clone(),
                b0.clone() + b1.clone() + b2.clone() + b3.clone(),
                self.t.v1.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " v1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                a0.clone() - a1.clone() + a2.clone() - a3.clone(),
                b0.clone() - b1.clone() + b2.clone() - b3.clone(),
                self.t.v2.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " v2"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                a3.clone(),
                b3.clone(),
                self.t.v6.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " v6"),
        );
        let beta_lc =
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(beta.clone());

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                a0.clone(),
                b0.clone(),
                c0.clone() * u.clone() + c2.clone() * beta.clone() * u.clone()
                    - beta_lc.clone() * u.clone() * inverse::<FieldT>(2) * self.t.v1.clone()
                    - beta_lc.clone() * u.clone() * inverse::<FieldT>(2) * self.t.v2.clone()
                    + self.t.v6.clone() * beta.clone(),
            ),
            prefix_format!(self.annotation_prefix, " v0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                a0.clone() + a1.clone() * 2 + a2.clone() * 4 + a3.clone() * 8,
                b0.clone() + b1.clone() * 2 + b2.clone() * 4 + b3.clone() * 8,
                c0.clone() * u.clone() * (-15)
                    - c1.clone() * 30 * u.clone()
                    - c2.clone() * 3 * (beta.clone() + 4) * u.clone()
                    - c3.clone() * 6 * (beta.clone() + 4) * u.clone()
                    + self.t.v1.clone()
                        * (-beta.clone() * 3 * inverse::<FieldT>(2) + 24)
                        * u.clone()
                    + self.t.v2.clone() * (beta.clone() * inverse::<FieldT>(2) - 8) * u.clone()
                    - (self.t.v6.clone() * ((beta.clone() - 16) * 3)),
            ),
            prefix_format!(self.annotation_prefix, " v3"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                a0.clone() - a1.clone() * 2 + a2.clone() * 4 - a3.clone() * 8,
                b0.clone() - b1.clone() * 2 + b2.clone() * 4 - b3.clone() * 8,
                -c0.clone() * 15 * u.clone() + c1.clone() * 30 * u.clone()
                    - c2.clone() * 3 * (beta.clone() + 4) * u.clone()
                    + c3.clone() * 6 * (beta.clone() + 4) * u.clone()
                    + linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        -(beta.clone() * 3 * inverse::<FieldT>(2)) + 24,
                    ) * u.clone()
                        * self.t.v2.clone()
                    + self.t.v1.clone() * (beta.clone() * inverse::<FieldT>(2) - 8) * u.clone()
                    - self.t.v6.clone() * 3 * (beta.clone() - 16),
            ),
            prefix_format!(self.annotation_prefix, " v4"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                a0.clone() + a1.clone() * 3 + a2.clone() * 9 + a3.clone() * 27,
                b0.clone() + b1.clone() * 3 + b2.clone() * 9 + b3.clone() * 27,
                -c0.clone() * 80 * u.clone()
                    - c1.clone() * 240 * u.clone()
                    - c2.clone() * 8 * (beta.clone() + 9) * u.clone()
                    - c3.clone() * 24 * (beta.clone() + 9) * u.clone()
                    - self.t.v1.clone() * 2 * (beta.clone() - 81) * u.clone()
                    + self.t.v2.clone() * (beta.clone() - 81) * u.clone()
                    - self.t.v6.clone() * 8 * (beta.clone() - 81),
            ),
            prefix_format!(self.annotation_prefix, " v5"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        // const pb_linear_combinations<FieldT>
        let a0 = &self.t.A.t.c0.t.c0;
        let a1 = &self.t.A.t.c1.t.c0;
        let a2 = &self.t.A.t.c0.t.c1;
        let a3 = &self.t.A.t.c1.t.c1;
        let b0 = &self.t.B.t.c0.t.c0;
        let b1 = &self.t.B.t.c1.t.c0;
        let b2 = &self.t.B.t.c0.t.c1;
        let b3 = &self.t.B.t.c1.t.c1;

        *self.pb.borrow_mut().val_ref(&self.t.v1) = ((self.pb.borrow().lc_val(a0)
            + self.pb.borrow().lc_val(a1)
            + self.pb.borrow().lc_val(a2)
            + self.pb.borrow().lc_val(a3))
            * (self.pb.borrow().lc_val(b0)
                + self.pb.borrow().lc_val(b1)
                + self.pb.borrow().lc_val(b2)
                + self.pb.borrow().lc_val(b3)));
        *self.pb.borrow_mut().val_ref(&self.t.v2) = ((self.pb.borrow().lc_val(a0)
            - self.pb.borrow().lc_val(a1)
            + self.pb.borrow().lc_val(a2)
            - self.pb.borrow().lc_val(a3))
            * (self.pb.borrow().lc_val(b0) - self.pb.borrow().lc_val(b1)
                + self.pb.borrow().lc_val(b2)
                - self.pb.borrow().lc_val(b3)));
        *self.pb.borrow_mut().val_ref(&self.t.v6) =
            self.pb.borrow().lc_val(a3) * self.pb.borrow().lc_val(b3);

        let Aval = self.t.A.get_element();
        let Bval = self.t.B.get_element();
        let Rval = Aval * Bval;

        self.t.result.generate_r1cs_witness(&Rval);
    }
}

pub type Fp4_sqr_gadgets<Fp4T, FieldT, PB> = gadget<FieldT, PB, Fp4_sqr_gadget<Fp4T, FieldT, PB>>;
impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_sqr_gadget<Fp4T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: Fp4_variables<Fp4T, FieldT, PB>,
        result: Fp4_variables<Fp4T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp4_sqr_gadgets<Fp4T, FieldT, PB> {
        /*
          Karatsuba squaring for Fp4 as a quadratic extension of Fp2:
          v0 = A.t.c0^2
          v1 = A.t.c1^2
          self.t.result.t.c0 = v0 + non_residue * v1
          self.t.result.t.c1 = (A.t.c0 + A.t.c1)^2 - v0 - v1
          where "non_residue * elem" := (non_residue * elt.c1, elt.c0)

          Enforced with 3 Fp2_sqr_gadget's that ensure that:
          A.t.c1^2 = v1
          A.t.c0^2 = v0
          (A.t.c0+A.t.c1)^2 = self.t.result.t.c1 + v0 + v1

          Reference:
          "Multiplication and Squaring on Pairing-Friendly Fields"
          Devegili, OhEigeartaigh, Scott, Dahab
        */
        let mut v0_c0 = default_pb_lc::<FieldT>();
        let mut v0_c1 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c0 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c1 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c0 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c1 = default_pb_lc::<FieldT>();

        let v1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " v1"),
        ));
        let compute_v1 = RcCell::new(Fp2_sqr_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            A.t.c1.clone(),
            v1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_v1"),
        ));

        v0_c0.assign(
            &pb,
            &(result.t.c0.t.c0.clone() - v1.borrow().t.c1.clone() * Fp4T::Fp2T::non_residue),
        );
        v0_c1.assign(&pb, &(result.t.c0.t.c1.clone() - v1.borrow().t.c0.clone()));
        let v0 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            v0_c0.clone(),
            v0_c1.clone(),
            prefix_format!(annotation_prefix, " v0"),
        ));

        let compute_v0 = RcCell::new(Fp2_sqr_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            A.t.c0.clone(),
            v0.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_v0"),
        ));

        Ac0_plus_Ac1_c0.assign(&pb, &(A.t.c0.t.c0.clone() + A.t.c1.t.c0.clone()));
        Ac0_plus_Ac1_c1.assign(&pb, &(A.t.c0.t.c1.clone() + A.t.c1.t.c1.clone()));
        let Ac0_plus_Ac1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            Ac0_plus_Ac1_c0.clone(),
            Ac0_plus_Ac1_c1.clone(),
            prefix_format!(annotation_prefix, " Ac0_plus_Ac1"),
        ));

        result_c1_plus_v0_plus_v1_c0.assign(
            &pb,
            &(result.t.c1.t.c0.clone() + v0.borrow().t.c0.clone() + v1.borrow().t.c0.clone()),
        );
        result_c1_plus_v0_plus_v1_c1.assign(
            &pb,
            &(result.t.c1.t.c1.clone() + v0.borrow().t.c1.clone() + v1.borrow().t.c1.clone()),
        );
        let result_c1_plus_v0_plus_v1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            result_c1_plus_v0_plus_v1_c0.clone(),
            result_c1_plus_v0_plus_v1_c1.clone(),
            prefix_format!(annotation_prefix, " result_c1_plus_v0_plus_v1"),
        ));

        let compute_result_c1 = RcCell::new(Fp2_sqr_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            Ac0_plus_Ac1.borrow().clone(),
            result_c1_plus_v0_plus_v1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_result_c1"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                A,
                result,
                v1,
                v0_c0,
                v0_c1,
                v0,
                compute_v0,
                compute_v1,
                Ac0_plus_Ac1_c0,
                Ac0_plus_Ac1_c1,
                Ac0_plus_Ac1,
                result_c1_plus_v0_plus_v1_c0,
                result_c1_plus_v0_plus_v1_c1,
                result_c1_plus_v0_plus_v1,
                compute_result_c1,
            },
        )
    }
}

impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_sqr_gadgets<Fp4T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_v1.borrow().generate_r1cs_constraints();
        self.t.compute_v0.borrow().generate_r1cs_constraints();
        self.t
            .compute_result_c1
            .borrow()
            .generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_v1.borrow().generate_r1cs_witness();

        self.t.v0_c0.evaluate_pb(&self.pb);
        self.t.v0_c1.evaluate_pb(&self.pb);
        self.t.compute_v0.borrow().generate_r1cs_witness();

        self.t.Ac0_plus_Ac1_c0.evaluate_pb(&self.pb);
        self.t.Ac0_plus_Ac1_c1.evaluate_pb(&self.pb);
        self.t.compute_result_c1.borrow().generate_r1cs_witness();

        let Aval = self.t.A.get_element();
        let Rval = Aval.squared();
        self.t.result.generate_r1cs_witness(&Rval);
    }
}

pub type Fp4_cyclotomic_sqr_gadgets<Fp4T, FieldT, PB> =
    gadget<FieldT, PB, Fp4_cyclotomic_sqr_gadget<Fp4T, FieldT, PB>>;
impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_cyclotomic_sqr_gadget<Fp4T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: Fp4_variables<Fp4T, FieldT, PB>,
        result: Fp4_variables<Fp4T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp4_cyclotomic_sqr_gadgets<Fp4T, FieldT, PB> {
        /*
          A = elt.c1 ^ 2
          B = elt.c1 + elt.c0;
          C = B ^ 2 - A
          D = Fp2(A.t.c1 * non_residue, A.t.c0)
          E = C - D
          F = D + D + Fp2::one()
          G = E - Fp2::one()

          return Fp4(F, G);

          Enforced with 2 Fp2_sqr_gadget's that ensure that:

          elt.c1 ^ 2 = Fp2(self.t.result.t.c0.t.c1 / 2, (self.t.result.t.c0.t.c0 - 1) / (2 * non_residue)) = A
          (elt.c1 + elt.c0) ^ 2 = A + self.t.result.t.c1 + Fp2(A.t.c1 * non_residue + 1, A.t.c0)

          (elt.c1 + elt.c0) ^ 2 = Fp2(self.t.result.t.c0.t.c1 / 2 + self.t.result.t.c1.t.c0 + (self.t.result.t.c0.t.c0 - 1) / 2 + 1,
                                      (self.t.result.t.c0.t.c0 - 1) / (2 * non_residue) + self.t.result.t.c1.t.c1 + self.t.result.t.c0.t.c1 / 2)

          Corresponding test code:

            assert!(B.squared() == A + G + my_Fp2(A.t.c1 * non_residue + my_Fp::one(), A.t.c0));
            assert!(self.c1.squared().c0 == F.c1 * my_Fp(2).inverse());
            assert!(self.c1.squared().c1 == (F.c0 - my_Fp(1)) * (my_Fp(2) * non_residue).inverse());
        */
        let mut c0_expr_c0 = default_pb_lc::<FieldT>();
        let mut c0_expr_c1 = default_pb_lc::<FieldT>();
        let mut A_c0_plus_A_c1_c0 = default_pb_lc::<FieldT>();
        let mut A_c0_plus_A_c1_c1 = default_pb_lc::<FieldT>();
        let mut c1_expr_c0 = default_pb_lc::<FieldT>();
        let mut c1_expr_c1 = default_pb_lc::<FieldT>();

        c0_expr_c0.assign(
            &pb,
            &(result.t.c0.t.c1.clone() * field_to_pb_lc::<FieldT>(inverse::<FieldT>(2))),
        );
        c0_expr_c1.assign(
            &pb,
            &((result.t.c0.t.c0.clone() - 1) * (Fp4T::Fp2T::non_residue * 2).inverse()),
        );
        let c0_expr = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            c0_expr_c0.clone(),
            c0_expr_c1.clone(),
            prefix_format!(annotation_prefix, " c0_expr"),
        ));
        let compute_c0_expr = RcCell::new(Fp2_sqr_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            A.t.c1.clone(),
            c0_expr.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_c0_expr"),
        ));

        A_c0_plus_A_c1_c0.assign(&pb, &(A.t.c0.t.c0.clone() + A.t.c1.t.c0.clone()));
        A_c0_plus_A_c1_c1.assign(&pb, &(A.t.c0.t.c1.clone() + A.t.c1.t.c1.clone()));
        let A_c0_plus_A_c1 = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            A_c0_plus_A_c1_c0.clone(),
            A_c0_plus_A_c1_c1.clone(),
            prefix_format!(annotation_prefix, " A_c0_plus_A_c1"),
        ));

        c1_expr_c0.assign(
            &pb,
            &((result.t.c0.t.c1.clone() + result.t.c0.t.c0.clone() - 1) * inverse::<FieldT>(2)
                + result.t.c1.t.c0.clone()
                + i64_to_pb_lc::<FieldT>(1)),
        );
        c1_expr_c1.assign(
            &pb,
            &((result.t.c0.t.c0.clone() - 1) * (Fp4T::Fp2T::non_residue * 2).inverse()
                + result.t.c1.t.c1.clone()
                + result.t.c0.t.c1.clone() * inverse::<FieldT>(2)),
        );
        let c1_expr = RcCell::new(Fp2_variable::<Fp4T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            c1_expr_c0.clone(),
            c1_expr_c1.clone(),
            prefix_format!(annotation_prefix, " c1_expr"),
        ));

        let compute_c1_expr = RcCell::new(Fp2_sqr_gadget::<Fp4T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            A_c0_plus_A_c1.borrow().clone(),
            c1_expr.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_c1_expr"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                A,
                result,
                c0_expr_c0,
                c0_expr_c1,
                c0_expr,
                compute_c0_expr,
                A_c0_plus_A_c1_c0,
                A_c0_plus_A_c1_c1,
                A_c0_plus_A_c1,
                c1_expr_c0,
                c1_expr_c1,
                c1_expr,
                compute_c1_expr,
            },
        )
    }
}

impl<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp4_cyclotomic_sqr_gadgets<Fp4T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_c0_expr.borrow().generate_r1cs_constraints();
        self.t.compute_c1_expr.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_c0_expr.borrow().generate_r1cs_witness();

        self.t.A_c0_plus_A_c1_c0.evaluate_pb(&self.pb);
        self.t.A_c0_plus_A_c1_c1.evaluate_pb(&self.pb);
        self.t.compute_c1_expr.borrow().generate_r1cs_witness();

        let Aval = self.t.A.get_element();
        let Rval = Aval.squared();
        self.t.result.generate_r1cs_witness(&Rval);
    }
}
