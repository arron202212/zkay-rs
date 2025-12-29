pub mod weierstrass_g1_gadget;
pub mod weierstrass_g2_gadget;

use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_variable,
};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::FieldTConfig;
use crate::relations::variable::linear_combination;
use ffec::{One, Zero};
use rccell::RcCell;
use std::ops::{Add, Mul, Sub};
pub const coeff_a: i64 = 0; //ffec::G1::<other_curve<ppT>>::coeff_a;
pub const coeff_b: i64 = 0; //ffec::G1::<other_curve<ppT>>::coeff_b;
pub type G1<ppT> = ppT; //ffec::G1<other_curve<ppT>>;
pub type G2<ppT> = ppT; //ffec::G1<other_curve<ppT>>;
pub type Fqe_variable<ppT, FieldT, PB> = <ppT as ppTConfig<FieldT, PB>>::Fpk_variableT;
pub type Fqe_sqr_gadget<ppT, FieldT, PB> = <ppT as ppTConfig<FieldT, PB>>::Fpk_sqr_gadgetT;
pub type Fqe_mul_gadget<ppT, FieldT, PB> = <ppT as ppTConfig<FieldT, PB>>::Fpk_mul_gadgetT;
pub type Fqk_variable<ppT, FieldT, PB> = <ppT as ppTConfig<FieldT, PB>>::Fpk_variableT;
pub type Fqk_sqr_gadget<ppT, FieldT, PB> = <ppT as ppTConfig<FieldT, PB>>::Fpk_sqr_gadgetT;
pub type Fqk_special_mul_gadget<ppT, FieldT, PB> = <ppT as ppTConfig<FieldT, PB>>::Fpk_mul_gadgetT;
pub type Fqe_mul_by_lc_gadget<ppT, FieldT, PB> = <ppT as ppTConfig<FieldT, PB>>::Fpk_mul_gadgetT;

pub trait VariableTConfig<FieldT: FieldTConfig, PB: PBConfig, FpkT>:
    Default
    + Clone
    + Add<Output = Self>
    + std::ops::Neg<Output = Self>
    + Add<i64, Output = Self>
    + Mul<FieldT, Output = Self>
{
    fn Frobenius_map(&self, power: usize) -> Self;
    fn X(&self) -> FieldT;
    fn Y(&self) -> FieldT;
    fn old_RX(&self) -> FieldT;
    fn old_RY(&self) -> FieldT;
    fn gamma(&self) -> FieldT;
    fn gamma_X(&self) -> FieldT;
    fn all_vars(&self) -> pb_linear_combination_array<FieldT, PB>;
    fn size_in_bits() -> usize;
    fn num_variables() -> usize;
    fn get_element(&self) -> FpkT;
    fn to_field(&self) -> FieldT;
    fn mul_by_X(&self) -> Self;
    fn evaluate(&self);
    fn new(pb: RcCell<protoboard<FieldT, PB>>, annotation_prefix: String) -> Self;
    fn new2(pb: RcCell<protoboard<FieldT, PB>>, f: FieldT, annotation_prefix: String) -> Self;
    fn new22(
        pb: RcCell<protoboard<FieldT, PB>>,
        c0: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn newv(
        pb: RcCell<protoboard<FieldT, PB>>,
        c0: RcCell<Self>,
        c1: RcCell<Self>,
        annotation_prefix: String,
    ) -> Self;
    fn newvv(
        pb: RcCell<protoboard<FieldT, PB>>,
        c0: FieldT,
        c1: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn new3(
        pb: RcCell<protoboard<FieldT, PB>>,
        c0: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        c2: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn is_constant(&self) -> bool;
    fn generate_r1cs_constraints(&self);
    fn generate_r1cs_witness(&self, f: &FieldT);
    fn generate_r1cs_equals_const_constraints(&self, t: &FieldT);
    fn c0(&self) -> Self;
    fn c1(&self) -> Self;
    fn c2(&self) -> Self;
}
pub trait MulTConfig<FieldT: FieldTConfig, PB: PBConfig, Fpk_variableT>:
    Default + Clone + Mul<FieldT, Output = Self>
{
    fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        v: Fpk_variableT,
        v2: Fpk_variableT,
        v3: Fpk_variableT,
        annotation_prefix: String,
    ) -> Self;
    fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        v: Fpk_variableT,
        v2: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        v3: Fpk_variableT,
        annotation_prefix: String,
    ) -> Self;
    fn new3(
        pb: RcCell<protoboard<FieldT, PB>>,
        c0: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        c2: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn generate_r1cs_constraints(&self);
    fn generate_r1cs_witness(&self);
}
pub trait SqrTConfig<FieldT: FieldTConfig, PB: PBConfig, Fpk_variableT>: Default + Clone {
    fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        s: RcCell<Fpk_variableT>,
        s2: Fpk_variableT,
        annotation_prefix: String,
    ) -> Self;
    fn generate_r1cs_constraints(&self);
    fn generate_r1cs_witness(&self);
}
pub const M: usize = 4;
pub trait ppTConfig<FieldT: FieldTConfig, PB: PBConfig>:
    Clone
    + Default
    + One
    + Zero
    + std::cmp::PartialEq
    + std::ops::Neg<Output = Self>
    + Sub<Output = Self>
    + Mul<FieldT, Output = Self>
    + for<'a> std::ops::BitXor<&'a ffec::field_utils::bigint::bigint<M>, Output = Self>
    + crate::common::data_structures::accumulation_vector::AccumulationVectorConfig
{
    type Fr: FieldTConfig;
    type Fpk_variableT: VariableTConfig<FieldT, PB, Self>;
    type Fpk_mul_gadgetT: MulTConfig<FieldT, PB, Self::Fpk_variableT>;
    type Fpk_sqr_gadgetT: SqrTConfig<FieldT, PB, Self::Fpk_variableT>;

    // type Fqe_mul_by_lc_gadgets: MulTConfig<FieldT, PB, Self::Fpk_variableT>;
    // type Fqk_special_mul_gadget: SqrTConfig<FieldT, PB, Self::Fpk_variableT>;
    const M: usize;
    fn X(&self) -> FieldT;
    fn Y(&self) -> FieldT;
    fn c0(&self) -> Self;
    fn c1(&self) -> Self;
    fn c2(&self) -> Self;
    fn random_element() -> Self;
    fn squared(&self) -> Self;
    fn to_affine_coordinates(&self);
    fn Frobenius_map(&self, power: usize) -> Self;
    fn twist() -> FieldT;
    fn coeffs(&self) -> Vec<Self::Fpk_variableT>;
    fn PY_twist_squared(&self) -> Self::Fpk_variableT;
    fn coeff_a() -> i64;
    fn inverse(&self) -> Self;
    fn to_field(&self) -> FieldT;
    fn from_field(t: &FieldT) -> Self;
}
