// Declaration of interfaces for Fp6 gadgets.

// The gadgets verify field arithmetic in Fp6 = Fp3[Y]/(Y^2-X) where
// Fp3 = Fp[X]/(X^3-non_residue) and non_residue is in Fp.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::fields::fp2_gadgets::{
    Fp2_mul_gadget, Fp2_mul_gadgets, Fp2_sqr_gadget, Fp2_sqr_gadgets, Fp2_variable, Fp2_variables,
    Fp2TConfig,
};
use crate::gadgetlib1::gadgets::fields::fp3_gadgets::{
    Fp3_mul_gadget, Fp3_mul_gadgets, Fp3_variable, Fp3_variables, Fp3TConfig,
};
use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_variable,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::{One, Zero};
use rccell::RcCell;
use std::marker::PhantomData;
pub type protoboards<FieldT, PB> = protoboard<FieldT, PB>;
pub type protoboards2<FieldT, PB> = protoboard<FieldT, PB>;
pub type pb_linear_combinations<FieldT> =
    linear_combination<FieldT, pb_variable, pb_linear_combination>;
pub type pb_linear_combinations2<FieldT> =
    linear_combination<FieldT, pb_variable, pb_linear_combination>;
#[inline]
pub fn default_pb_lc<FieldT: FieldTConfig>()
-> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    linear_combination::<FieldT, pb_variable, pb_linear_combination>::default()
}

#[inline]
pub fn default_pb_lc2<FieldT: FieldTConfig>()
-> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    linear_combination::<FieldT, pb_variable, pb_linear_combination>::default()
}

#[inline]
pub fn i64_to_pb_lc<FieldT: FieldTConfig>(
    d: i64,
) -> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(FieldT::from(d))
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

#[inline]
pub fn to_field<FieldT: FieldTConfig>(d: i32) -> FieldT {
    FieldT::from(d)
}

/**
 * Gadget that represents an Fp6 variable.
 */
pub trait Fp6TConfig<FieldT: FieldTConfig>:
    FieldTConfig + Default + Clone + std::ops::Mul<Output = Self>
{
    // type FieldT: FieldTConfig;
    type Fp3T: Fp3TConfig<FieldT>;
    type Fp2T: Fp2TConfig<FieldT>;
    fn c0(&self) -> Self::Fp3T;
    fn c1(&self) -> Self::Fp3T;
    fn c0_mut(&mut self) -> &mut Self::Fp3T;
    fn c1_mut(&mut self) -> &mut Self::Fp3T;
    fn cyclotomic_squared(&self) -> Self;
    const non_residue: FieldT;
    const Frobenius_coeffs_c1: [FieldT; 2];
}

#[derive(Clone, Default)]
pub struct Fp6_variable<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp6T::my_Fp>
    // type FieldT=Fp6T::my_Fp;
    // type Fp3T=Fp6T::my_Fpe;
    pub c0: Fp3_variables<Fp6T::Fp3T, FieldT, PB>,
    pub c1: Fp3_variables<Fp6T::Fp3T, FieldT, PB>,
}

/**
 * Gadget that creates constraints for Fp6 multiplication.
 */
#[derive(Clone, Default)]
pub struct Fp6_mul_gadget<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    //     type Fp3T=Fp6T::my_Fpe;
    pub A: Fp6_variables<Fp6T, FieldT, PB>,
    pub B: Fp6_variables<Fp6T, FieldT, PB>,
    pub result: Fp6_variables<Fp6T, FieldT, PB>,
    pub v0_c0: pb_linear_combinations<FieldT>,
    pub v0_c1: pb_linear_combinations<FieldT>,
    pub v0_c2: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c0: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c1: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c2: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub v0: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub v1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub Bc0_plus_Bc1_c0: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1_c1: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1_c2: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub result_c1_plus_v0_plus_v1_c0: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1_c1: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1_c2: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub compute_v0: RcCell<Fp3_mul_gadgets<Fp6T::Fp3T, FieldT, PB>>,
    pub compute_v1: RcCell<Fp3_mul_gadgets<Fp6T::Fp3T, FieldT, PB>>,
    pub compute_result_c1: RcCell<Fp3_mul_gadgets<Fp6T::Fp3T, FieldT, PB>>,
}

/**
 * Gadget that creates constraints for Fp6 multiplication by a Fp6 element B for which B.t.c0.t.c0 = B.t.c0.t.c1 = 0.
 */
#[derive(Clone, Default)]
pub struct Fp6_mul_by_2345_gadget<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    //     type Fp3T=Fp6T::my_Fpe;
    pub A: Fp6_variables<Fp6T, FieldT, PB>,
    pub B: Fp6_variables<Fp6T, FieldT, PB>,
    pub result: Fp6_variables<Fp6T, FieldT, PB>,
    pub v0_c0: pb_linear_combinations<FieldT>,
    pub v0_c1: pb_linear_combinations<FieldT>,
    pub v0_c2: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c0: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c1: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1_c2: pb_linear_combinations<FieldT>,
    pub Ac0_plus_Ac1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub v0: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub v1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub Bc0_plus_Bc1_c0: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1_c1: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1_c2: pb_linear_combinations<FieldT>,
    pub Bc0_plus_Bc1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub result_c1_plus_v0_plus_v1_c0: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1_c1: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1_c2: pb_linear_combinations<FieldT>,
    pub result_c1_plus_v0_plus_v1: RcCell<Fp3_variables<Fp6T::Fp3T, FieldT, PB>>,
    pub compute_v1: RcCell<Fp3_mul_gadgets<Fp6T::Fp3T, FieldT, PB>>,
    pub compute_result_c1: RcCell<Fp3_mul_gadgets<Fp6T::Fp3T, FieldT, PB>>,
}

/**
 * Gadget that creates constraints for Fp6 squaring.
 */
#[derive(Clone, Default)]
pub struct Fp6_sqr_gadget<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    pub A: Fp6_variables<Fp6T, FieldT, PB>,
    pub result: Fp6_variables<Fp6T, FieldT, PB>,
    pub mul: RcCell<Fp6_mul_gadgets<Fp6T, FieldT, PB>>,
}

/**
 * Gadget that creates constraints for Fp6 cyclotomic squaring
 */
#[derive(Clone, Default)]
pub struct Fp6_cyclotomic_sqr_gadget<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    //     type Fp3T=Fp6T::my_Fp2;
    pub A: Fp6_variables2<Fp6T, FieldT, PB>,
    pub result: Fp6_variables2<Fp6T, FieldT, PB>,
    pub a: RcCell<Fp2_variables<Fp6T::Fp2T, FieldT, PB>>,
    pub b: RcCell<Fp2_variables<Fp6T::Fp2T, FieldT, PB>>,
    pub c: RcCell<Fp2_variables<Fp6T::Fp2T, FieldT, PB>>,
    pub asq_c0: pb_linear_combinations2<FieldT>,
    pub asq_c1: pb_linear_combinations2<FieldT>,
    pub bsq_c0: pb_linear_combinations2<FieldT>,
    pub bsq_c1: pb_linear_combinations2<FieldT>,
    pub csq_c0: pb_linear_combinations2<FieldT>,
    pub csq_c1: pb_linear_combinations2<FieldT>,
    pub asq: RcCell<Fp2_variables<Fp6T::Fp2T, FieldT, PB>>,
    pub bsq: RcCell<Fp2_variables<Fp6T::Fp2T, FieldT, PB>>,
    pub csq: RcCell<Fp2_variables<Fp6T::Fp2T, FieldT, PB>>,
    pub compute_asq: RcCell<Fp2_sqr_gadgets<Fp6T::Fp2T, FieldT, PB>>,
    pub compute_bsq: RcCell<Fp2_sqr_gadgets<Fp6T::Fp2T, FieldT, PB>>,
    pub compute_csq: RcCell<Fp2_sqr_gadgets<Fp6T::Fp2T, FieldT, PB>>,
}

pub type Fp6_variables<Fp6T, FieldT, PB> = gadget<FieldT, PB, Fp6_variable<Fp6T, FieldT, PB>>;
pub type Fp6_variables2<Fp6T, FieldT, PB> = gadget<FieldT, PB, Fp6_variable<Fp6T, FieldT, PB>>;
impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> Fp6_variable<Fp6T, FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboards<FieldT, PB>>,
        annotation_prefix: String,
    ) -> Fp6_variables<Fp6T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(
            pb.clone(),
            annotation_prefix.clone(),
            Self {
                c0: Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " c0"),
                ),
                c1: Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new(
                    pb.clone(),
                    prefix_format!(annotation_prefix, " c1"),
                ),
            },
        )
    }

    pub fn new2(
        pb: RcCell<protoboards<FieldT, PB>>,
        el: Fp6T,
        annotation_prefix: String,
    ) -> Fp6_variables<Fp6T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(
            pb.clone(),
            annotation_prefix.clone(),
            Self {
                c0: Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new2(
                    pb.clone(),
                    el.c0(),
                    prefix_format!(annotation_prefix, " c0"),
                ),
                c1: Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new2(
                    pb.clone(),
                    el.c1(),
                    prefix_format!(annotation_prefix, " c1"),
                ),
            },
        )
    }

    pub fn new3(
        pb: RcCell<protoboards<FieldT, PB>>,
        c0: Fp3_variables<Fp6T::Fp3T, FieldT, PB>,
        c1: Fp3_variables<Fp6T::Fp3T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp6_variables<Fp6T, FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { c0, c1 })
    }
}
impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig> Fp6_variables<Fp6T, FieldT, PB> {
    pub fn generate_r1cs_equals_const_constraints(&self, el: &Fp6T) {
        self.t.c0.generate_r1cs_equals_const_constraints(&el.c0());
        self.t.c1.generate_r1cs_equals_const_constraints(&el.c1());
    }

    pub fn generate_r1cs_witness(&self, el: &Fp6T) {
        self.t.c0.generate_r1cs_witness(&el.c0());
        self.t.c1.generate_r1cs_witness(&el.c1());
    }

    pub fn get_element(&self) -> Fp6T {
        let mut el = Fp6T::default();
        *el.c0_mut() = self.t.c0.get_element();
        *el.c1_mut() = self.t.c1.get_element();
        return el;
    }

    pub fn Frobenius_map(&self, power: usize) -> Fp6_variables<Fp6T, FieldT, PB> {
        let (mut new_c0c0, mut new_c0c1, mut new_c0c2, mut new_c1c0, mut new_c1c1, mut new_c1c2) = (
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
            default_pb_lc::<FieldT>(),
        );
        new_c0c0.assign(&self.pb, &self.t.c0.t.c0);
        new_c0c1.assign(
            &self.pb,
            &(self.t.c0.t.c1.clone() * Fp6T::Fp3T::Frobenius_coeffs_c1[power % 3].clone()),
        );
        new_c0c2.assign(
            &self.pb,
            &(self.t.c0.t.c2.clone() * Fp6T::Fp3T::Frobenius_coeffs_c2[power % 3].clone()),
        );
        new_c1c0.assign(
            &self.pb,
            &(self.t.c1.t.c0.clone() * Fp6T::Frobenius_coeffs_c1[power % 6].clone()),
        );
        new_c1c1.assign(
            &self.pb,
            &(self.t.c1.t.c1.clone()
                * (Fp6T::Frobenius_coeffs_c1[power % 6].clone()
                    * Fp6T::Fp3T::Frobenius_coeffs_c1[power % 3].clone())),
        );
        new_c1c2.assign(
            &self.pb,
            &(self.t.c1.t.c2.clone()
                * (Fp6T::Frobenius_coeffs_c1[power % 6].clone()
                    * Fp6T::Fp3T::Frobenius_coeffs_c2[power % 3].clone())),
        );

        return Fp6_variable::<Fp6T, FieldT, PB>::new3(
            self.pb.clone(),
            Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
                self.pb.clone(),
                new_c0c0.clone(),
                new_c0c1.clone(),
                new_c0c2.clone(),
                prefix_format!(self.annotation_prefix, " Frobenius_map_c0"),
            ),
            Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
                self.pb.clone(),
                new_c1c0,
                new_c1c1,
                new_c1c2,
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

pub type Fp6_mul_gadgets<Fp6T, FieldT, PB> = gadget<FieldT, PB, Fp6_mul_gadget<Fp6T, FieldT, PB>>;

impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_mul_gadget<Fp6T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboards<FieldT, PB>>,
        A: Fp6_variables<Fp6T, FieldT, PB>,
        B: Fp6_variables<Fp6T, FieldT, PB>,
        result: Fp6_variables<Fp6T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp6_mul_gadgets<Fp6T, FieldT, PB> {
        /*
            Karatsuba multiplication for Fp6 as a quadratic extension of Fp3:
                v0 = A.t.c0 * B.t.c0
                v1 = A.c1 * B.c1
                result.t.c0 = v0 + non_residue * v1
                result.c1 = (A.t.c0 + A.c1) * (B.t.c0 + B.c1) - v0 - v1
            where "non_residue * elem" := (non_residue * elem.c2, elem.c0, elem.c1)

            Enforced with 3 Fp3_mul_gadget's that ensure that:
                A.c1 * B.c1 = v1
                A.t.c0 * B.t.c0 = v0
                (A.t.c0+A.c1)*(B.t.c0+B.c1) = result.c1 + v0 + v1

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab
        */

        let v1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " v1"),
        ));

        let compute_v1 = RcCell::new(Fp3_mul_gadget::<Fp6T::Fp3T, FieldT, PB>::new(
            pb.clone(),
            A.t.c1.clone(),
            B.t.c1.clone(),
            v1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_v1"),
        ));

        let mut v0_c0 = default_pb_lc::<FieldT>();
        let mut v0_c1 = default_pb_lc::<FieldT>();
        let mut v0_c2 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c0 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c1 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c2 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c0 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c1 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c2 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c0 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c1 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c2 = default_pb_lc::<FieldT>();

        v0_c0.assign(
            &pb,
            &(result.t.c0.t.c0.clone() - v1.borrow().t.c2.clone() * Fp6T::non_residue),
        );
        v0_c1.assign(&pb, &(result.t.c0.t.c1.clone() - v1.borrow().t.c0.clone()));
        v0_c2.assign(&pb, &(result.t.c0.t.c2.clone() - v1.borrow().t.c1.clone()));
        let v0 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
            pb.clone(),
            v0_c0.clone(),
            v0_c1.clone(),
            v0_c2.clone(),
            prefix_format!(annotation_prefix, " v0"),
        ));

        let compute_v0 = RcCell::new(Fp3_mul_gadget::<Fp6T::Fp3T, FieldT, PB>::new(
            pb.clone(),
            A.t.c0.clone(),
            B.t.c0.clone(),
            v0.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_v0"),
        ));

        Ac0_plus_Ac1_c0.assign(&pb, &(A.t.c0.t.c0.clone() + A.t.c1.t.c0.clone()));
        Ac0_plus_Ac1_c1.assign(&pb, &(A.t.c0.t.c1.clone() + A.t.c1.t.c1.clone()));
        Ac0_plus_Ac1_c2.assign(&pb, &(A.t.c0.t.c2.clone() + A.t.c1.t.c2.clone()));
        let Ac0_plus_Ac1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
            pb.clone(),
            Ac0_plus_Ac1_c0.clone(),
            Ac0_plus_Ac1_c1.clone(),
            Ac0_plus_Ac1_c2.clone(),
            prefix_format!(annotation_prefix, " Ac0_plus_Ac1"),
        ));

        Bc0_plus_Bc1_c0.assign(&pb, &(B.t.c0.t.c0.clone() + B.t.c1.t.c0.clone()));
        Bc0_plus_Bc1_c1.assign(&pb, &(B.t.c0.t.c1.clone() + B.t.c1.t.c1.clone()));
        Bc0_plus_Bc1_c2.assign(&pb, &(B.t.c0.t.c2.clone() + B.t.c1.t.c2.clone()));
        let Bc0_plus_Bc1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
            pb.clone(),
            Bc0_plus_Bc1_c0.clone(),
            Bc0_plus_Bc1_c1.clone(),
            Bc0_plus_Bc1_c2.clone(),
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
        result_c1_plus_v0_plus_v1_c2.assign(
            &pb,
            &(result.t.c1.t.c2.clone() + v0.borrow().t.c2.clone() + v1.borrow().t.c2.clone()),
        );
        let result_c1_plus_v0_plus_v1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
            pb.clone(),
            result_c1_plus_v0_plus_v1_c0.clone(),
            result_c1_plus_v0_plus_v1_c1.clone(),
            result_c1_plus_v0_plus_v1_c2.clone(),
            prefix_format!(annotation_prefix, " result_c1_plus_v0_plus_v1"),
        ));

        let compute_result_c1 = RcCell::new(Fp3_mul_gadget::<Fp6T::Fp3T, FieldT, PB>::new(
            pb.clone(),
            Ac0_plus_Ac1.borrow().clone(),
            Bc0_plus_Bc1.borrow().clone(),
            result_c1_plus_v0_plus_v1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_result_c1"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb.clone(),
            annotation_prefix,
            Self {
                A,
                B,
                result,
                v0_c0,
                v0_c1,
                v0_c2,
                Ac0_plus_Ac1_c0,
                Ac0_plus_Ac1_c1,
                Ac0_plus_Ac1_c2,
                Ac0_plus_Ac1,
                v0,
                v1,
                Bc0_plus_Bc1_c0,
                Bc0_plus_Bc1_c1,
                Bc0_plus_Bc1_c2,
                Bc0_plus_Bc1,
                result_c1_plus_v0_plus_v1_c0,
                result_c1_plus_v0_plus_v1_c1,
                result_c1_plus_v0_plus_v1_c2,
                result_c1_plus_v0_plus_v1,
                compute_v0,
                compute_v1,
                compute_result_c1,
            },
        )
    }
}
impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_mul_gadgets<Fp6T, FieldT, PB>
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
        self.t.Ac0_plus_Ac1_c2.evaluate_pb(&self.pb);

        self.t.Bc0_plus_Bc1_c0.evaluate_pb(&self.pb);
        self.t.Bc0_plus_Bc1_c1.evaluate_pb(&self.pb);
        self.t.Bc0_plus_Bc1_c2.evaluate_pb(&self.pb);

        self.t.compute_result_c1.borrow().generate_r1cs_witness();

        let Aval = self.t.A.get_element();
        let Bval = self.t.B.get_element();
        let Rval = Aval * Bval;

        self.t.result.generate_r1cs_witness(&Rval);

        self.t.result_c1_plus_v0_plus_v1_c0.evaluate_pb(&self.pb);
        self.t.result_c1_plus_v0_plus_v1_c1.evaluate_pb(&self.pb);
        self.t.result_c1_plus_v0_plus_v1_c2.evaluate_pb(&self.pb);

        self.t.compute_result_c1.borrow().generate_r1cs_witness();
    }
}

pub type Fp6_mul_by_2345_gadgets<Fp6T, FieldT, PB> =
    gadget<FieldT, PB, Fp6_mul_by_2345_gadget<Fp6T, FieldT, PB>>;

impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_mul_by_2345_gadget<Fp6T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboards<FieldT, PB>>,
        A: Fp6_variables<Fp6T, FieldT, PB>,
        B: Fp6_variables<Fp6T, FieldT, PB>,
        result: Fp6_variables<Fp6T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp6_mul_by_2345_gadgets<Fp6T, FieldT, PB> {
        /*
            Karatsuba multiplication for Fp6 as a quadratic extension of Fp3:
                v0 = A.t.c0 * B.t.c0
                v1 = A.c1 * B.c1
                result.t.c0 = v0 + non_residue * v1
                result.c1 = (A.t.c0 + A.c1) * (B.t.c0 + B.c1) - v0 - v1
            where "non_residue * elem" := (non_residue * elem.c2, elem.c0, elem.c1)

            We know that B.t.c0.t.c0 = B.t.c0.t.c1 = 0

            Enforced with 2 Fp3_mul_gadget's that ensure that:
                A.c1 * B.c1 = v1
                (A.t.c0+A.c1)*(B.t.c0+B.c1) = result.c1 + v0 + v1

            And one multiplication (three direct constraints) that enforces A.t.c0 * B.t.c0
            = v0, where B.t.c0.t.c0 = B.t.c0.t.c1 = 0.

            Note that (u + v * X + t * X^2) * (0 + 0 * X + z * X^2) =
            (v * z * non_residue + t * z * non_residue * X + u * z * X^2)

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab
        */
        let v1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " v1"),
        ));
        let compute_v1 = RcCell::new(Fp3_mul_gadget::<Fp6T::Fp3T, FieldT, PB>::new(
            pb.clone(),
            A.t.c1.clone(),
            B.t.c1.clone(),
            v1.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_v1"),
        ));

        /* we inline result.t.c0 in v0 as follows: v0 = (result.t.c0.t.c0 - Fp6T::non_residue * v1->c2, result.t.c0.t.c1 - v1->c0, result.t.c0.t.c2 - v1->c1) */
        let v0 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " v0"),
        ));

        let mut v0_c0 = default_pb_lc::<FieldT>();
        let mut v0_c1 = default_pb_lc::<FieldT>();
        let mut v0_c2 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c0 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c1 = default_pb_lc::<FieldT>();
        let mut Ac0_plus_Ac1_c2 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c0 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c1 = default_pb_lc::<FieldT>();
        let mut Bc0_plus_Bc1_c2 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c0 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c1 = default_pb_lc::<FieldT>();
        let mut result_c1_plus_v0_plus_v1_c2 = default_pb_lc::<FieldT>();

        Ac0_plus_Ac1_c0.assign(&pb, &(A.t.c0.t.c0.clone() + A.t.c1.t.c0.clone()));
        Ac0_plus_Ac1_c1.assign(&pb, &(A.t.c0.t.c1.clone() + A.t.c1.t.c1.clone()));
        Ac0_plus_Ac1_c2.assign(&pb, &(A.t.c0.t.c2.clone() + A.t.c1.t.c2.clone()));
        let Ac0_plus_Ac1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
            pb.clone(),
            Ac0_plus_Ac1_c0.clone(),
            Ac0_plus_Ac1_c1.clone(),
            Ac0_plus_Ac1_c2.clone(),
            prefix_format!(annotation_prefix, " Ac0_plus_Ac1"),
        ));

        Bc0_plus_Bc1_c0.assign(&pb, &(B.t.c0.t.c0.clone() + B.t.c1.t.c0.clone()));
        Bc0_plus_Bc1_c1.assign(&pb, &(B.t.c0.t.c1.clone() + B.t.c1.t.c1.clone()));
        Bc0_plus_Bc1_c2.assign(&pb, &(B.t.c0.t.c2.clone() + B.t.c1.t.c2.clone()));
        let Bc0_plus_Bc1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
            pb.clone(),
            Bc0_plus_Bc1_c0.clone(),
            Bc0_plus_Bc1_c1.clone(),
            Bc0_plus_Bc1_c2.clone(),
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
        result_c1_plus_v0_plus_v1_c2.assign(
            &pb,
            &(result.t.c1.t.c2.clone() + v0.borrow().t.c2.clone() + v1.borrow().t.c2.clone()),
        );
        let result_c1_plus_v0_plus_v1 = RcCell::new(Fp3_variable::<Fp6T::Fp3T, FieldT, PB>::new4(
            pb.clone(),
            result_c1_plus_v0_plus_v1_c0.clone(),
            result_c1_plus_v0_plus_v1_c1.clone(),
            result_c1_plus_v0_plus_v1_c2.clone(),
            prefix_format!(annotation_prefix, " result_c1_plus_v0_plus_v1"),
        ));

        let compute_result_c1 = RcCell::new(Fp3_mul_gadget::<Fp6T::Fp3T, FieldT, PB>::new(
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
                v0_c2,
                Ac0_plus_Ac1_c0,
                Ac0_plus_Ac1_c1,
                Ac0_plus_Ac1_c2,
                Ac0_plus_Ac1,
                v0,
                v1,
                Bc0_plus_Bc1_c0,
                Bc0_plus_Bc1_c1,
                Bc0_plus_Bc1_c2,
                Bc0_plus_Bc1,
                result_c1_plus_v0_plus_v1_c0,
                result_c1_plus_v0_plus_v1_c1,
                result_c1_plus_v0_plus_v1_c2,
                result_c1_plus_v0_plus_v1,
                compute_v1,
                compute_result_c1,
            },
        )
    }
}
impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_mul_by_2345_gadgets<Fp6T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_v1.borrow().generate_r1cs_constraints();
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.t.c1.clone(),
                self.t.B.t.c0.t.c2.clone() * Fp6T::Fp3T::non_residue,
                self.t.result.t.c0.t.c0.clone()
                    - self.t.v1.borrow().t.c2.clone() * Fp6T::non_residue,
            ),
            prefix_format!(self.annotation_prefix, " v0.borrow().t.c0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.t.c2.clone(),
                self.t.B.t.c0.t.c2.clone() * Fp6T::Fp3T::non_residue,
                self.t.result.t.c0.t.c1.clone() - self.t.v1.borrow().t.c0.clone(),
            ),
            prefix_format!(self.annotation_prefix, " v0.borrow().t.c1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.A.t.c0.t.c0.clone(),
                self.t.B.t.c0.t.c2.clone(),
                self.t.result.t.c0.t.c2.clone() - self.t.v1.borrow().t.c1.clone(),
            ),
            prefix_format!(self.annotation_prefix, " v0.borrow().t.c2"),
        );
        self.t
            .compute_result_c1
            .borrow()
            .generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_v1.borrow().generate_r1cs_witness();

        let A_c0_val = self.t.A.t.c0.get_element();
        let B_c0_val = self.t.B.t.c0.get_element();
        assert!(B_c0_val.c0().is_zero());
        assert!(B_c0_val.c1().is_zero());

        let v0_val = A_c0_val * B_c0_val;
        self.t.v0.borrow().generate_r1cs_witness(&v0_val);

        self.t.Ac0_plus_Ac1_c0.evaluate_pb(&self.pb);
        self.t.Ac0_plus_Ac1_c1.evaluate_pb(&self.pb);
        self.t.Ac0_plus_Ac1_c2.evaluate_pb(&self.pb);

        self.t.Bc0_plus_Bc1_c0.evaluate_pb(&self.pb);
        self.t.Bc0_plus_Bc1_c1.evaluate_pb(&self.pb);
        self.t.Bc0_plus_Bc1_c2.evaluate_pb(&self.pb);

        self.t.compute_result_c1.borrow().generate_r1cs_witness();

        let Aval = self.t.A.get_element();
        let Bval = self.t.B.get_element();
        let Rval = Aval * Bval;

        self.t.result.generate_r1cs_witness(&Rval);

        self.t.result_c1_plus_v0_plus_v1_c0.evaluate_pb(&self.pb);
        self.t.result_c1_plus_v0_plus_v1_c1.evaluate_pb(&self.pb);
        self.t.result_c1_plus_v0_plus_v1_c2.evaluate_pb(&self.pb);

        self.t.compute_result_c1.borrow().generate_r1cs_witness();
    }
}

pub type Fp6_sqr_gadgets<Fp6T, FieldT, PB> = gadget<FieldT, PB, Fp6_sqr_gadget<Fp6T, FieldT, PB>>;

impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_sqr_gadget<Fp6T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboards<FieldT, PB>>,
        A: Fp6_variables<Fp6T, FieldT, PB>,
        result: Fp6_variables<Fp6T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp6_sqr_gadgets<Fp6T, FieldT, PB> {
        // We can't do better than 3 Fp3_mul_gadget's for squaring, so we just use multiplication.
        let mul = RcCell::new(Fp6_mul_gadget::<Fp6T, FieldT, PB>::new(
            pb.clone(),
            A.clone(),
            A.clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " mul"),
        ));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { A, result, mul })
    }
}
impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_sqr_gadgets<Fp6T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.mul.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.mul.borrow().generate_r1cs_witness();
    }
}

pub type Fp6_cyclotomic_sqr_gadgets<Fp6T, FieldT, PB> =
    gadget<FieldT, PB, Fp6_cyclotomic_sqr_gadget<Fp6T, FieldT, PB>>;

impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_cyclotomic_sqr_gadget<Fp6T, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboards2<FieldT, PB>>,
        A: Fp6_variables2<Fp6T, FieldT, PB>,
        result: Fp6_variables2<Fp6T, FieldT, PB>,
        annotation_prefix: String,
    ) -> Fp6_cyclotomic_sqr_gadgets<Fp6T, FieldT, PB> {
        /*
            my_Fp2 a = my_Fp2(c0.t.c0, c1.t.c1);
            my_Fp2 b = my_Fp2(c1.t.c0, c0.t.c2);
            my_Fp2 c = my_Fp2(c0.t.c1, c1.t.c2);

            my_Fp2 asq = a.squared();
            my_Fp2 bsq = b.squared();
            my_Fp2 csq = c.squared();

            result.t.c0.t.c0 = 3 * asq_a - 2 * a_a;
            result.c1.t.c1 = 3 * asq_b + 2 * a_b;

            result.t.c0.t.c1 = 3 * bsq_a - 2 * c_a;
            result.c1.t.c2 = 3 * bsq_b + 2 * c_b;

            result.t.c0.t.c2 = 3 * csq_a - 2 * b_b;
            result.c1.t.c0 = 3 * my_Fp3::non_residue * csq_b + 2 * b_a;

            return Fp6_2over3_model<n, mbodulus>(my_Fp3(A_a, C_a, B_b),
                                                 my_Fp3(B_a, A_b, C_b))
        */
        let a = RcCell::new(Fp2_variable::<Fp6T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            A.t.c0.t.c0.clone(),
            A.t.c1.t.c1.clone(),
            prefix_format!(annotation_prefix, " a"),
        ));
        let b = RcCell::new(Fp2_variable::<Fp6T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            A.t.c1.t.c0.clone(),
            A.t.c0.t.c2.clone(),
            prefix_format!(annotation_prefix, " b"),
        ));
        let c = RcCell::new(Fp2_variable::<Fp6T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            A.t.c0.t.c1.clone(),
            A.t.c1.t.c2.clone(),
            prefix_format!(annotation_prefix, " c"),
        ));
        let mut asq_c0 = default_pb_lc2::<FieldT>();
        let mut asq_c1 = default_pb_lc2::<FieldT>();
        let mut bsq_c0 = default_pb_lc2::<FieldT>();
        let mut bsq_c1 = default_pb_lc2::<FieldT>();
        let mut csq_c0 = default_pb_lc2::<FieldT>();
        let mut csq_c1 = default_pb_lc2::<FieldT>();
        asq_c0.assign(
            &pb,
            &((result.t.c0.t.c0.clone() + a.borrow().t.c0.clone())
                * FieldT::from(2)
                * inverse::<FieldT>(3)),
        );
        asq_c1.assign(
            &pb,
            &((result.t.c1.t.c1.clone() - a.borrow().t.c1.clone())
                * FieldT::from(2)
                * inverse::<FieldT>(3)),
        );

        bsq_c0.assign(
            &pb,
            &((result.t.c0.t.c1.clone() + c.borrow().t.c0.clone())
                * FieldT::from(2)
                * inverse::<FieldT>(3)),
        );
        bsq_c1.assign(
            &pb,
            &((result.t.c1.t.c2.clone() - c.borrow().t.c1.clone())
                * FieldT::from(2)
                * inverse::<FieldT>(3)),
        );

        csq_c0.assign(
            &pb,
            &((result.t.c0.t.c2.clone() + b.borrow().t.c1.clone())
                * FieldT::from(2)
                * inverse::<FieldT>(3)),
        );
        csq_c1.assign(
            &pb,
            &((result.t.c1.t.c0.clone() - b.borrow().t.c0.clone())
                * FieldT::from(2)
                * (to_field::<FieldT>(3) * Fp6T::Fp3T::non_residue).inverse()),
        );

        let asq = RcCell::new(Fp2_variable::<Fp6T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            asq_c0.clone(),
            asq_c1.clone(),
            prefix_format!(annotation_prefix, " asq"),
        ));
        let bsq = RcCell::new(Fp2_variable::<Fp6T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            bsq_c0.clone(),
            bsq_c1.clone(),
            prefix_format!(annotation_prefix, " bsq"),
        ));
        let csq = RcCell::new(Fp2_variable::<Fp6T::Fp2T, FieldT, PB>::new4(
            pb.clone(),
            csq_c0.clone(),
            csq_c1.clone(),
            prefix_format!(annotation_prefix, " csq"),
        ));

        let compute_asq = RcCell::new(Fp2_sqr_gadget::<Fp6T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            a.borrow().clone(),
            asq.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_asq"),
        ));
        let compute_bsq = RcCell::new(Fp2_sqr_gadget::<Fp6T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            b.borrow().clone(),
            bsq.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_bsq"),
        ));
        let compute_csq = RcCell::new(Fp2_sqr_gadget::<Fp6T::Fp2T, FieldT, PB>::new(
            pb.clone(),
            c.borrow().clone(),
            csq.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_csq"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                A,
                result,
                a,
                b,
                c,
                asq_c0,
                asq_c1,
                bsq_c0,
                bsq_c1,
                csq_c0,
                csq_c1,
                asq,
                bsq,
                csq,
                compute_asq,
                compute_bsq,
                compute_csq,
            },
        )
    }
}
impl<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>
    Fp6_cyclotomic_sqr_gadgets<Fp6T, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_asq.borrow().generate_r1cs_constraints();
        self.t.compute_bsq.borrow().generate_r1cs_constraints();
        self.t.compute_csq.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        let Aval = self.t.A.get_element();
        let Rval = Aval.cyclotomic_squared();

        self.t.result.generate_r1cs_witness(&Rval);

        self.t.asq.borrow().evaluate();
        self.t.bsq.borrow().evaluate();
        self.t.csq.borrow().evaluate();

        self.t.compute_asq.borrow().generate_r1cs_witness();
        self.t.compute_bsq.borrow().generate_r1cs_witness();
        self.t.compute_csq.borrow().generate_r1cs_witness();
    }
}
