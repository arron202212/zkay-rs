// Declaration of specializations of pairing_selector<ppT> to
// - pairing_selector<mnt4_pp>, and
// - pairing_selector<mnt6_pp>.

// use ff_curves::algebra::curves::mnt::mnt4::mnt4_pp;
// use ff_curves::algebra::curves::mnt::mnt6::mnt6_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::gadgets::pairing::weierstrass_final_exponentiation::{
    mnt4_final_exp_gadget, mnt6_final_exp_gadget,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_miller_loop::{
    mnt_e_over_e_miller_loop_gadget, mnt_e_times_e_over_e_miller_loop_gadget,
};
use crate::gadgetlib1::protoboard::{protoboard,PBConfig,ProtoboardConfig};
use ffec::FieldTConfig;

use crate::gadgetlib1::gadgets::fields::fp3_gadgets::{
    Fp3_mul_by_lc_gadget, Fp3_mul_gadget, Fp3_sqr_gadget, Fp3_variable, Fp3TConfig,
};

use crate::gadgetlib1::gadgets::fields::fp2_gadgets::{
    Fp2_mul_by_lc_gadget, Fp2_mul_gadget, Fp2_sqr_gadget, Fp2_variable, Fp2TConfig,
};
use crate::gadgetlib1::gadgets::fields::fp4_gadgets::{
    Fp4_mul_gadget, Fp4_sqr_gadget, Fp4_variable, Fp4TConfig,
};
use crate::gadgetlib1::gadgets::fields::fp6_gadgets::{
    Fp6_mul_by_2345_gadget, Fp6_mul_gadget, Fp6_sqr_gadget, Fp6_variable, Fp6TConfig,
};
use ff_curves::{Fqe, Fqk, PublicParams};
use std::marker::PhantomData;
// pub struct FrT<T>(PhantomData<T>);
// pub type Fr<T> = FrT<T>;
// pub struct FqeT<T>(PhantomData<T>);
// pub type Fqe<T> = FqeT<T>;
// pub struct FqkT<T>(PhantomData<T>);
// pub type Fqk<T> = FqkT<T>;
/**
 * Specialization for MNT4.
 */

pub trait pairing_selector4<
    mnt4_pp: PublicParams,
    mnt6_pp: PublicParams,
    ppT: ppTConfig,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp3T: Fp3TConfig<FieldT>,
    Fp6T: Fp6TConfig<FieldT>,
>
{
    type PB: PBConfig;
    type FieldT: FieldTConfig; // = Fr<mnt4_pp>;
    type FqeT = Fqe<mnt6_pp>;
    type FqkT = Fqk<mnt6_pp>;

    type Fqe_variable_type = Fp3_variable<Fp3T, FieldT, PB>;
    type Fqe_mul_gadget_type = Fp3_mul_gadget<Fp3T, FieldT, PB>;
    type Fqe_mul_by_lc_gadget_type = Fp3_mul_by_lc_gadget<Fp3T, FieldT, PB>;
    type Fqe_sqr_gadget_type = Fp3_sqr_gadget<Fp3T, FieldT, PB>;

    type Fqk_variable_type = Fp6_variable<Fp6T, FieldT, PB>;
    type Fqk_mul_gadget_type = Fp6_mul_gadget<Fp6T, FieldT, PB>;
    type Fqk_special_mul_gadget_type = Fp6_mul_by_2345_gadget<Fp6T, FieldT, PB>;
    type Fqk_sqr_gadget_type = Fp6_sqr_gadget<Fp6T, FieldT, PB>;

    type other_curve_type = mnt6_pp;

    type e_over_e_miller_loop_gadget_type;
    type e_times_e_over_e_miller_loop_gadget_type;
    type final_exp_gadget_type;

    // static pairing_loop_count:&constexpr bigint<mnt6_Fr::num_limbs> = mnt6_ate_loop_count;
}

//  type e_over_e_miller_loop_gadget_type = mnt_e_over_e_miller_loop_gadget<ppT,  PB>;
//     type e_times_e_over_e_miller_loop_gadget_type =
//         mnt_e_times_e_over_e_miller_loop_gadget<ppT,  PB>;
//     type final_exp_gadget_type = mnt4_final_exp_gadget<ppT,  PB>;
/**
 * Specialization for MNT6.
 */

pub trait pairing_selector6<
    mnt6_pp: PublicParams,
    mnt4_pp: PublicParams,
    ppT: ppTConfig,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp2T: Fp2TConfig<FieldT>,
    Fp4T: Fp4TConfig<FieldT>,
>
{
    type PB: PBConfig;
    type FieldT: FieldTConfig; // = Fr<mnt6_pp>;

    type FqeT = Fqe<mnt4_pp>;
    type FqkT = Fqk<mnt4_pp>;

    type Fqe_variable_type = Fp2_variable<Fp2T, FieldT, PB>;
    type Fqe_mul_gadget_type = Fp2_mul_gadget<Fp2T, FieldT, PB>;
    type Fqe_mul_by_lc_gadget_type = Fp2_mul_by_lc_gadget<Fp2T, FieldT, PB>;
    type Fqe_sqr_gadget_type = Fp2_sqr_gadget<Fp2T, FieldT, PB>;

    type Fqk_variable_type = Fp4_variable<Fp4T, FieldT, PB>;
    type Fqk_mul_gadget_type = Fp4_mul_gadget<Fp4T, FieldT, PB>;
    type Fqk_special_mul_gadget_type = Fp4_mul_gadget<Fp4T, FieldT, PB>;
    type Fqk_sqr_gadget_type = Fp4_sqr_gadget<Fp4T, FieldT, PB>;

    type other_curve_type = mnt4_pp;

    type e_over_e_miller_loop_gadget_type;
    type e_times_e_over_e_miller_loop_gadget_type;
    type final_exp_gadget_type;

    // static pairing_loop_count:&constexpr bigint<mnt4_Fr::num_limbs> = mnt4_ate_loop_count;
}
// type e_over_e_miller_loop_gadget_type = mnt_e_over_e_miller_loop_gadget<ppT, FieldT, PB>;
//     type e_times_e_over_e_miller_loop_gadget_type =
//         mnt_e_times_e_over_e_miller_loop_gadget<ppT, FieldT, PB>;
//     type final_exp_gadget_type = mnt6_final_exp_gadget<ppT, FieldT, PB>;
