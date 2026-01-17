//  Declaration of selector for the pairing gadget.

use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_variable,
};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, MessageConfig,
};
use ff_curves::{FpmConfig, Fr, G1, G2, PublicParams, PublicParamsType};
use ffec::scalar_multiplication::multiexp::KCConfig;

use ffec::{FieldTConfig, One, PpConfig, Zero};
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use rccell::RcCell;
use std::ops::{Add, Mul, Sub};
// pub const coeff_a: i64 = 0; //ffec::G1::<other_curve<ppT>>::coeff_a;
// pub const coeff_b: i64 = 0; //ffec::G1::<other_curve<ppT>>::coeff_b;
// pub type G1<ppT> = ppT; //ffec::G1<other_curve<ppT>>;
// pub type G2<ppT> = ppT; //ffec::G1<other_curve<ppT>>;
// pub type Fqe_variable<ppT, FieldT, PB> = <ppT as ppTConfig>::Fpk_variableT;
// pub type Fqe_sqr_gadget<ppT, FieldT, PB> = <ppT as ppTConfig>::Fpk_sqr_gadgetT;
// pub type Fqe_mul_gadget<ppT, FieldT, PB> = <ppT as ppTConfig>::Fpk_mul_gadgetT;
// pub type Fqk_variable<ppT, FieldT, PB> = <ppT as ppTConfig>::Fpk_variableT;
// pub type Fqk_sqr_gadget<ppT, FieldT, PB> = <ppT as ppTConfig>::Fpk_sqr_gadgetT;
// pub type Fqk_special_mul_gadget<ppT, FieldT, PB> = <ppT as ppTConfig>::Fpk_mul_gadgetT;
// pub type Fqe_mul_by_lc_gadget<ppT, FieldT, PB> = <ppT as ppTConfig>::Fpk_mul_gadgetT;

pub trait VariableTConfig:
    Default
    + Clone
    + Add<Output = Self>
    + std::ops::Neg<Output = Self>
    + Add<i64, Output = Self>
    + Mul<Self::FieldT, Output = Self>
{
    type FieldT: FieldTConfig;
    type PB: PBConfig;
    type FpkT;
    type Fqe_variable;
    fn Frobenius_map(&self, power: usize) -> Self;
    fn X(&self) -> Self::FieldT;
    fn Y(&self) -> Self::FieldT;
    fn old_RX(&self) -> Self::FieldT;
    fn old_RY(&self) -> Self::FieldT;
    fn gamma(&self) -> Self::FieldT;
    fn gamma_X(&self) -> Self::FieldT;
    fn all_vars(&self) -> pb_linear_combination_array<Self::FieldT, Self::PB>;
    fn size_in_bits() -> usize;
    fn num_variables() -> usize;
    fn get_element(&self) -> Self::FpkT;
    fn to_field(&self) -> Self::FieldT;
    fn mul_by_X(&self) -> Self;
    fn evaluate(&self);
    fn new(pb: RcCell<protoboard<Self::FieldT, Self::PB>>, annotation_prefix: String) -> Self;
    fn new2<FieldTT>(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        f: FieldTT,
        annotation_prefix: String,
    ) -> Self;
    fn new22(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        c0: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn newv(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        c0: RcCell<Self>,
        c1: RcCell<Self>,
        annotation_prefix: String,
    ) -> Self;
    fn newe(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        c0: RcCell<Self::Fqe_variable>,
        c1: RcCell<Self::Fqe_variable>,
        annotation_prefix: String,
    ) -> Self;
    fn newvv(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        c0: Self::FieldT,
        c1: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn new3(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        c0: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        c2: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn is_constant(&self) -> bool;
    fn generate_r1cs_constraints(&self);
    fn generate_r1cs_witness<FieldTT>(&self, f: &FieldTT);
    fn generate_r1cs_equals_const_constraints(&self, t: &Self::FieldT);
    fn c0(&self) -> Self;
    fn c1(&self) -> Self;
    fn c2(&self) -> Self;
}
pub trait MulTConfig: Default + Clone + Mul<Self::FieldT, Output = Self> {
    type FieldT: FieldTConfig;
    type PB: PBConfig;
    type Fpk_variableT;
    fn new(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        v: Self::Fpk_variableT,
        v2: Self::Fpk_variableT,
        v3: Self::Fpk_variableT,
        annotation_prefix: String,
    ) -> Self;
    fn new2(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        v: Self::Fpk_variableT,
        v2: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        v3: Self::Fpk_variableT,
        annotation_prefix: String,
    ) -> Self;
    fn new3(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        c0: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        c1: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        c2: linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> Self;
    fn generate_r1cs_constraints(&self);
    fn generate_r1cs_witness(&self);
}
pub trait SqrTConfig: Default + Clone {
    type FieldT: FieldTConfig;
    type PB: PBConfig;
    type Fpk_variableT;
    fn new(
        pb: RcCell<protoboard<Self::FieldT, Self::PB>>,
        s: RcCell<Self::Fpk_variableT>,
        s2: Self::Fpk_variableT,
        annotation_prefix: String,
    ) -> Self;
    fn generate_r1cs_constraints(&self);
    fn generate_r1cs_witness(&self);
}
pub const M: usize = 4;

pub trait ppTConfig:
    Clone
    + Default
    + One
    + Zero
    + std::cmp::PartialEq
    + std::ops::Neg<Output = Self>
    + Sub<Output = Self>
    + Mul<Self::FieldT, Output = Self>
    + for<'a> std::ops::BitXor<&'a ffec::field_utils::bigint::bigint<M>, Output = Self>
    + PublicParamsType
    + PublicParams<Fqk = Self::FieldT, Fr = Self::FieldT>
    + FieldTConfig
{
    type P: pairing_selector<
            FieldT = Self::FieldT,
            PB = Self::PB,
            G1 = Self::G1,
            G2 = Self::G2,
            Fr = <Self as PublicParams>::Fr,
        >;
    type FieldT: FieldTConfig
        + Mul<<Self as PublicParams>::G1, Output = <Self as PublicParams>::G1>
        + Mul<<Self as PublicParams>::G2, Output = <Self as PublicParams>::G2>
        + AsMut<[u64]>;
    type PB: PBConfig;
    type SV: SubVariableConfig;
    type SLC: SubLinearCombinationConfig;
    type my_Fp: FieldTConfig;
    // type Fr: FieldTConfig;
    type Fpk_variableT: VariableTConfig<FieldT = Fr<Self>, PB = Self::PB, FpkT = Self, Fqe_variable = Self>;
    type KC: KCConfig<FieldT = Self::FieldT, T = G1<Self>, T2 = G2<Self>>;
    type M: MessageConfig<FieldT = Self::FieldT>;
    type LD: LocalDataConfig<FieldT = Self::FieldT>;
    // type KC2: KCConfig;
    // type Fpk_mul_gadgetT: MulTConfig<Fr<Self>, Self::PB, Self::Fpk_variableT>;
    // type Fpk_sqr_gadgetT: SqrTConfig<Fr<Self>, Self::PB, Self::Fpk_variableT>;

    // type Fqe_mul_by_lc_gadgets: MulTConfig<FieldT, PB, Self::Fpk_variableT>;
    // type Fqk_special_mul_gadget: SqrTConfig<FieldT, PB, Self::Fpk_variableT>;
    // const M: usize = 4;
    // fn X(&self) -> FieldT;
    // fn Y(&self) -> FieldT;
    fn c0(&self) -> Self {
        Default::default()
    }
    fn c1(&self) -> Self {
        Default::default()
    }
    fn c2(&self) -> Self {
        Default::default()
    }
    // fn squared(&self) -> Self;
    fn to_affine_coordinates(&self) {}
    fn Frobenius_map(&self, power: usize) -> Self {
        Default::default()
    }
    fn twist() -> Self::FieldT {
        Default::default()
    }
    fn coeffs(&self) -> Vec<Self::Fpk_variableT> {
        vec![]
    }
    fn PY_twist_squared(&self) -> Self::Fpk_variableT {
        Default::default()
    }
    fn coeff_a() -> i64 {
        0
    }
    // fn inverse(&self) -> Self;
    // fn to_field(&self) -> Self::FieldT;
    fn from_field(t: &Self::FieldT) -> Self {
        Default::default()
    }
}

/**
 * The interfaces of pairing gadgets are templatized via the parameter
 * ec_ppT. When used, the interfaces must be invoked with
 * a particular parameter choice; let 'my_ec_pp' denote this choice.
 *
 * Moreover, one must provide a template specialization for the class
 * pairing_selector (below), containing typedefs for the typenames
 * - FieldT
 * - FqeT
 * - FqkT
 * - Fqe_variable_type;
 * - Fqe_mul_gadget_type
 * - Fqe_mul_by_lc_gadget_type
 * - Fqe_sqr_gadget_type
 * - Fqk_variable_type
 * - Fqk_mul_gadget_type
 * - Fqk_special_mul_gadget_type
 * - Fqk_sqr_gadget_type
 * - other_curve_type
 * - e_over_e_miller_loop_gadget_type
 * - e_times_e_over_e_miller_loop_gadget_type
 * - final_exp_gadget_type
 * and also containing a static constant
 * - const constexpr ffec::bigint<m> pairing_loop_count
 *
 * For example, if you want to use the types my_Field, my_Fqe, etc,
 * then you would do as follows. First declare a new type:
 *
 *   pub struct my_ec_pp;
 *
 * Second, specialize pairing_selector<ec_ppT> for the
 * case ec_ppT = my_ec_pp, type  the above types:
 *
 *   
 *   pub struct pairing_selector<my_ec_pp> {
 *       type FieldT=my_Field;
 *       type FqeT=my_Fqe;
 *       type FqkT=my_Fqk;
 *       type Fqe_variable_type=my_Fqe_variable_type;
 *       type Fqe_mul_gadget_type=my_Fqe_mul_gadget_type;
 *       type Fqe_mul_by_lc_gadget_type=my_Fqe_mul_by_lc_gadget_type;
 *       type Fqe_sqr_gadget_type=my_Fqe_sqr_gadget_type;
 *       type Fqk_variable_type=my_Fqk_variable_type;
 *       type Fqk_mul_gadget_type=my_Fqk_mul_gadget_type;
 *       type Fqk_special_mul_gadget_type=my_Fqk_special_mul_gadget_type;
 *       type Fqk_sqr_gadget_type=my_Fqk_sqr_gadget_type;
 *       type other_curve_type=my_other_curve_type;
 *       type e_over_e_miller_loop_gadget_type=my_e_over_e_miller_loop_gadget_type;
 *       type e_times_e_over_e_miller_loop_gadget_type=my_e_times_e_over_e_miller_loop_gadget_type;
 *       type final_exp_gadget_type=my_final_exp_gadget_type;
 *       static pairing_loop_count:&constexpr ffec::bigint<...> = ...;
 *   };
 *
 * Having done the above, my_ec_pp can be used as a template parameter.
 *
 * See mnt_pairing_params.hpp for examples for the case of fixing
 * ec_ppT to "MNT4" and "MNT6".
 *
 */

// pub struct pairing_selector;

/**
 * Below are various template aliases (used for convenience).
 */
pub trait pairing_selector:
    Sized
    + Clone
    + Default
    + std::cmp::PartialEq
    + std::ops::Neg<Output = Self>
    + std::ops::Mul<Output = Self>
{
    type G1: PpConfig + FpmConfig<Fr = Self::Fr>;
    type G2: PpConfig + FpmConfig<Fr = Self::Fr>;
    type Fr: FieldTConfig;
    type my_ec_pp: ppTConfig;
    type PB: PBConfig;
    type FieldT: FieldTConfig + std::cmp::PartialEq<Self>;
    type FqeT;
    type FqkT;
    type Fqe_variable_type: VariableTConfig<
            FieldT = Self::FieldT,
            PB = Self::PB,
            FpkT = Self::FieldT,
            Fqe_variable = Self::Fqk_variable_type,
        >;
    type Fqe_mul_gadget_type: MulTConfig<FieldT = Self::FieldT, PB = Self::PB, Fpk_variableT = Self::Fqe_variable_type>;
    type Fqe_mul_by_lc_gadget_type: MulTConfig<FieldT = Self::FieldT, PB = Self::PB, Fpk_variableT = Self::Fqe_variable_type>;
    type Fqe_sqr_gadget_type: SqrTConfig<FieldT = Self::FieldT, PB = Self::PB, Fpk_variableT = Self::Fqe_variable_type>;
    type Fqk_variable_type: VariableTConfig<
            FieldT = Self::FieldT,
            PB = Self::PB,
            FpkT = Self::FieldT,
            Fqe_variable = Self::Fqe_variable_type,
        >;
    type Fqk_mul_gadget_type: MulTConfig<FieldT = Self::FieldT, PB = Self::PB, Fpk_variableT = Self::Fqk_variable_type>;
    type Fqk_special_mul_gadget_type: MulTConfig<FieldT = Self::FieldT, PB = Self::PB, Fpk_variableT = Self::Fqk_variable_type>;
    type Fqk_sqr_gadget_type: SqrTConfig<FieldT = Self::FieldT, PB = Self::PB, Fpk_variableT = Self::Fqk_variable_type>;
    type other_curve_type: ppTConfig<G1 = Self::G1, G2 = Self::G2, Fr = Self::FieldT, FieldT = Self::FieldT>;
    type e_over_e_miller_loop_gadget_type;
    type e_times_e_over_e_miller_loop_gadget_type;
    type final_exp_gadget_type;
    const pairing_loop_count: u128;
    fn inverse(&self) -> Self {
        Default::default()
    }
}

pub type FqeT<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::FqeT;
pub type FqkT<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::FqkT; // TODO: better name when stable
pub type Fqe_variable<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqe_variable_type;
pub type Fqe_mul_gadget<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqe_mul_gadget_type;
pub type Fqe_mul_by_lc_gadget<ppT> =
    <<ppT as ppTConfig>::P as pairing_selector>::Fqe_mul_by_lc_gadget_type;
pub type Fqe_sqr_gadget<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqe_sqr_gadget_type;
pub type Fqk_variable<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqk_variable_type;
pub type Fqk_mul_gadget<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqk_mul_gadget_type;
pub type Fqk_special_mul_gadget<ppT> =
    <<ppT as ppTConfig>::P as pairing_selector>::Fqk_special_mul_gadget_type;
pub type Fqk_sqr_gadget<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqk_sqr_gadget_type;
pub type other_curve<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::other_curve_type;
// pub type e_over_e_miller_loop_gadget<ppT> =
//     <<ppT as ppTConfig>::P as pairing_selector>::e_over_e_miller_loop_gadget_type;
// pub type e_times_e_over_e_miller_loop_gadget<ppT> =
//     <<ppT as ppTConfig>::P as pairing_selector>::e_times_e_over_e_miller_loop_gadget_type;
// pub type final_exp_gadget<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::final_exp_gadget_type;

pub type Fpk_variableT<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqk_variable_type;
pub type Fpk_mul_gadgetT<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqk_mul_gadget_type;
pub type Fpk_sqr_gadgetT<ppT> = <<ppT as ppTConfig>::P as pairing_selector>::Fqk_sqr_gadget_type;

use crate::gadgetlib1::gadgets::pairing::weierstrass_final_exponentiation::{
    mnt4_final_exp_gadget, mnt4_final_exp_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_miller_loop::{
    mnt_e_over_e_miller_loop_gadget, mnt_e_over_e_miller_loop_gadgets,
    mnt_e_times_e_over_e_miller_loop_gadget, mnt_e_times_e_over_e_miller_loop_gadgets,
};

pub type e_over_e_miller_loop_gadget<ppT> = mnt_e_over_e_miller_loop_gadget<ppT>;
pub type e_times_e_over_e_miller_loop_gadget<ppT> = mnt_e_times_e_over_e_miller_loop_gadget<ppT>;
pub type final_exp_gadget<ppT> = mnt4_final_exp_gadget<ppT>;

pub type e_over_e_miller_loop_gadgets<ppT> = mnt_e_over_e_miller_loop_gadgets<ppT>;
pub type e_times_e_over_e_miller_loop_gadgets<ppT> = mnt_e_times_e_over_e_miller_loop_gadgets<ppT>;
pub type final_exp_gadgets<ppT> = mnt4_final_exp_gadgets<ppT>;
